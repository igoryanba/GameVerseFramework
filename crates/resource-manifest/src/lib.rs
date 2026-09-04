//! Static FiveM manifest parsing and safe GameVerse resource path resolution.
//! Manifest Lua is data: it is never executed.
use anyhow::{Context, Result};
use globset::Glob;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Component, Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct DataFile {
    pub kind: String,
    pub path: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceMetadata {
    pub manifest: String,
    pub legacy: bool,
    pub license: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceManifest {
    pub name: String,
    #[serde(default)]
    pub client_scripts: Vec<String>,
    #[serde(default)]
    pub server_scripts: Vec<String>,
    #[serde(default)]
    pub shared_scripts: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub files: Vec<String>,
    #[serde(default)]
    pub exports: Vec<String>,
    #[serde(default)]
    pub data_files: Vec<DataFile>,
    pub ui_page: Option<String>,
    pub source: SourceMetadata,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParsedManifest {
    pub manifest: ResourceManifest,
    pub blocked_dynamic: Vec<String>,
}

pub fn parse_fivem(root: &Path) -> Result<ParsedManifest> {
    anyhow::ensure!(
        root.is_dir(),
        "resource path is not a directory: {}",
        root.display()
    );
    let modern = root.join("fxmanifest.lua");
    let legacy = root.join("__resource.lua");
    let (path, is_legacy) = if modern.is_file() {
        (modern, false)
    } else if legacy.is_file() {
        (legacy, true)
    } else {
        anyhow::bail!("no fxmanifest.lua or __resource.lua in {}", root.display());
    };
    let raw = fs::read_to_string(&path).with_context(|| format!("read {}", path.display()))?;
    let text = strip_comments(&raw);
    let dynamic =
        Regex::new(r"(?m)\b(?:dofile|load|loadfile|require)\s*\(|\bfunction\b|\bwhile\b|\bfor\b")?;
    let blocked_dynamic = dynamic
        .find_iter(&text)
        .map(|value| value.as_str().trim().to_string())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let manifest = ResourceManifest {
        name: root
            .file_name()
            .and_then(|v| v.to_str())
            .unwrap_or("resource")
            .to_string(),
        client_scripts: directives(&text, &["client_script", "client_scripts"]),
        server_scripts: directives(&text, &["server_script", "server_scripts"]),
        shared_scripts: directives(&text, &["shared_script", "shared_scripts"]),
        dependencies: directives(&text, &["dependency", "dependencies"]),
        files: directives(&text, &["file", "files"]),
        exports: directives(
            &text,
            &["export", "exports", "server_export", "server_exports"],
        ),
        data_files: data_files(&text),
        ui_page: single(&text, "ui_page"),
        source: SourceMetadata {
            manifest: path.file_name().unwrap().to_string_lossy().into_owned(),
            legacy: is_legacy,
            license: detect_license(root),
        },
    };
    Ok(ParsedManifest {
        manifest,
        blocked_dynamic,
    })
}

pub fn to_gameverse_toml(manifest: &ResourceManifest) -> Result<String> {
    Ok(toml::to_string_pretty(manifest)?)
}

pub fn from_gameverse_toml(text: &str) -> Result<ResourceManifest> {
    Ok(toml::from_str(text)?)
}

pub fn resolve_and_validate(root: &Path, manifest: &ResourceManifest) -> Result<Vec<PathBuf>> {
    let canonical_root = root
        .canonicalize()
        .with_context(|| format!("canonicalize {}", root.display()))?;
    let patterns = manifest
        .shared_scripts
        .iter()
        .chain(&manifest.client_scripts)
        .chain(&manifest.server_scripts)
        .chain(&manifest.files)
        .chain(manifest.ui_page.iter())
        .chain(manifest.data_files.iter().map(|v| &v.path));
    let mut resolved = BTreeSet::new();
    let candidates: Vec<_> = WalkDir::new(root)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| entry.file_type().is_file())
        .collect();
    for pattern in patterns {
        if pattern.starts_with('@') {
            continue;
        }
        validate_relative(pattern)?;
        let matcher = Glob::new(&pattern.replace('\\', "/"))?.compile_matcher();
        let mut matched = false;
        for entry in &candidates {
            let relative = entry
                .path()
                .strip_prefix(root)?
                .to_string_lossy()
                .replace('\\', "/");
            if matcher.is_match(&relative) {
                let canonical = entry.path().canonicalize()?;
                anyhow::ensure!(
                    canonical.starts_with(&canonical_root),
                    "resource path escapes root: {relative}"
                );
                resolved.insert(entry.path().strip_prefix(root)?.to_path_buf());
                matched = true;
            }
        }
        anyhow::ensure!(matched, "resource pattern matched no files: {pattern}");
    }
    Ok(resolved.into_iter().collect())
}

/// Expands already-validated manifest patterns into deterministic normalized
/// paths relative to the resource root.
pub fn expand_patterns(patterns: &[String], resolved: &[PathBuf]) -> Result<Vec<String>> {
    let mut output = BTreeSet::new();
    for pattern in patterns {
        if pattern.starts_with('@') {
            output.insert(pattern.clone());
            continue;
        }
        let matcher = Glob::new(&pattern.replace('\\', "/"))?.compile_matcher();
        for path in resolved {
            let normalized = path.to_string_lossy().replace('\\', "/");
            if matcher.is_match(&normalized) {
                output.insert(normalized);
            }
        }
    }
    Ok(output.into_iter().collect())
}

pub fn validate_dependency_graph(resources: &BTreeMap<String, ResourceManifest>) -> Result<()> {
    fn visit(
        name: &str,
        resources: &BTreeMap<String, ResourceManifest>,
        visiting: &mut BTreeSet<String>,
        done: &mut BTreeSet<String>,
    ) -> Result<()> {
        if done.contains(name) {
            return Ok(());
        }
        anyhow::ensure!(
            visiting.insert(name.to_string()),
            "cyclic resource dependency at {name}"
        );
        if let Some(resource) = resources.get(name) {
            for dependency in &resource.dependencies {
                if resources.contains_key(dependency) {
                    visit(dependency, resources, visiting, done)?;
                }
            }
        }
        visiting.remove(name);
        done.insert(name.to_string());
        Ok(())
    }
    let mut done = BTreeSet::new();
    for name in resources.keys() {
        visit(name, resources, &mut BTreeSet::new(), &mut done)?;
    }
    Ok(())
}

fn validate_relative(value: &str) -> Result<()> {
    let path = Path::new(value);
    anyhow::ensure!(
        !path.is_absolute(),
        "absolute resource path is forbidden: {value}"
    );
    anyhow::ensure!(
        !path.components().any(|part| matches!(
            part,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )),
        "resource path traversal is forbidden: {value}"
    );
    Ok(())
}
fn strip_comments(text: &str) -> String {
    text.lines()
        .map(|line| line.split("--").next().unwrap_or(""))
        .collect::<Vec<_>>()
        .join("\n")
}
fn single(text: &str, name: &str) -> Option<String> {
    Regex::new(&format!(
        r#"(?m)\b{}\s*['\"]([^'\"]+)['\"]"#,
        regex::escape(name)
    ))
    .ok()?
    .captures(text)?
    .get(1)
    .map(|v| v.as_str().to_string())
}
fn directives(text: &str, names: &[&str]) -> Vec<String> {
    let quoted = Regex::new(r#"['\"]([^'\"]+)['\"]"#).unwrap();
    let mut output = BTreeSet::new();
    for name in names {
        if let Some(value) = single(text, name) {
            output.insert(value);
        }
        let block = Regex::new(&format!(r#"(?s)\b{}\s*\{{(.*?)\}}"#, regex::escape(name))).unwrap();
        for capture in block.captures_iter(text) {
            for item in quoted.captures_iter(&capture[1]) {
                output.insert(item[1].to_string());
            }
        }
    }
    output.into_iter().collect()
}
fn data_files(text: &str) -> Vec<DataFile> {
    let regex =
        Regex::new(r#"(?m)\bdata_file\s*['\"]([^'\"]+)['\"]\s*['\"]([^'\"]+)['\"]"#).unwrap();
    regex
        .captures_iter(text)
        .map(|v| DataFile {
            kind: v[1].to_string(),
            path: v[2].to_string(),
        })
        .collect()
}
fn detect_license(root: &Path) -> Option<String> {
    let entry = fs::read_dir(root)
        .ok()?
        .filter_map(Result::ok)
        .find(|entry| {
            let name = entry.file_name().to_string_lossy().to_ascii_lowercase();
            entry.path().is_file() && (name.starts_with("license") || name.starts_with("copying"))
        })?;
    let text = fs::read_to_string(entry.path()).ok()?.to_ascii_lowercase();
    Some(
        if text.contains("gnu affero") {
            "AGPL-3.0"
        } else if text.contains("gnu general public license") {
            "GPL-3.0"
        } else if text.contains("mit license") || text.contains("permission is hereby granted") {
            "MIT"
        } else if text.contains("apache license") {
            "Apache-2.0"
        } else {
            "unknown"
        }
        .into(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_resolves_and_rejects_escape_and_cycles() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join("client")).unwrap();
        fs::write(dir.path().join("client/main.lua"), "return true").unwrap();
        fs::write(
            dir.path().join("LICENSE"),
            "MIT License\nPermission is hereby granted",
        )
        .unwrap();
        fs::write(dir.path().join("fxmanifest.lua"), "-- ignored\nclient_scripts {'client/*.lua'}\ndata_file 'TEST' 'client/main.lua'\nexport 'ready'").unwrap();
        let parsed = parse_fivem(dir.path()).unwrap();
        assert_eq!(parsed.manifest.source.license.as_deref(), Some("MIT"));
        assert!(resolve_and_validate(dir.path(), &parsed.manifest)
            .unwrap()
            .contains(&PathBuf::from("client/main.lua")));
        let mut invalid = parsed.manifest.clone();
        invalid.files.push("../secret".into());
        assert!(resolve_and_validate(dir.path(), &invalid).is_err());
        let mut graph = BTreeMap::new();
        let mut a = parsed.manifest.clone();
        a.name = "a".into();
        a.dependencies = vec!["b".into()];
        let mut b = a.clone();
        b.name = "b".into();
        b.dependencies = vec!["a".into()];
        graph.insert("a".into(), a);
        graph.insert("b".into(), b);
        assert!(validate_dependency_graph(&graph).is_err());
    }

    #[test]
    fn parses_legacy_manifest_and_marks_dynamic_lua_without_running_it() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("client.lua"), "return true").unwrap();
        fs::write(
            dir.path().join("__resource.lua"),
            "client_script 'client.lua'\nfunction build() while true do end end",
        )
        .unwrap();
        let parsed = parse_fivem(dir.path()).unwrap();
        assert!(parsed.manifest.source.legacy);
        assert_eq!(parsed.manifest.client_scripts, ["client.lua"]);
        assert!(parsed.blocked_dynamic.contains(&"function".to_string()));
        assert!(parsed.blocked_dynamic.contains(&"while".to_string()));
    }

    #[test]
    fn rejects_absolute_and_unmatched_paths() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(
            dir.path().join("fxmanifest.lua"),
            "client_script 'missing/*.lua'",
        )
        .unwrap();
        let parsed = parse_fivem(dir.path()).unwrap();
        assert!(resolve_and_validate(dir.path(), &parsed.manifest).is_err());
        let mut absolute = parsed.manifest;
        absolute.client_scripts = vec![if cfg!(windows) {
            "C:/outside.lua".into()
        } else {
            "/outside.lua".into()
        }];
        assert!(resolve_and_validate(dir.path(), &absolute).is_err());
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlink_escape() {
        use std::os::unix::fs::symlink;
        let resource = tempfile::tempdir().unwrap();
        let outside = tempfile::tempdir().unwrap();
        fs::write(outside.path().join("outside.lua"), "return true").unwrap();
        symlink(
            outside.path().join("outside.lua"),
            resource.path().join("escape.lua"),
        )
        .unwrap();
        fs::write(
            resource.path().join("fxmanifest.lua"),
            "client_script 'escape.lua'",
        )
        .unwrap();
        let parsed = parse_fivem(resource.path()).unwrap();
        assert!(resolve_and_validate(resource.path(), &parsed.manifest).is_err());
    }
}
