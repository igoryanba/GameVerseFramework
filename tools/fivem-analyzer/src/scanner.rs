use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeSet,
    fs,
    path::{Path, PathBuf},
};
use walkdir::WalkDir;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Framework {
    Standalone,
    Qbcore,
    Qbox,
    Esx,
    Unknown,
}
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CompatibilityCategory {
    Supported,
    Convertible,
    Manual,
    Blocked,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Finding {
    pub category: CompatibilityCategory,
    pub feature: String,
    pub detail: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceReport {
    pub resource_name: String,
    pub manifest: PathBuf,
    pub legacy_manifest: bool,
    pub framework: Framework,
    pub dependencies: Vec<String>,
    pub shared_scripts: Vec<String>,
    pub client_scripts: Vec<String>,
    pub server_scripts: Vec<String>,
    pub files: Vec<String>,
    pub ui_page: Option<String>,
    pub exports: Vec<String>,
    pub events: Vec<String>,
    pub callbacks: Vec<String>,
    pub natives: Vec<String>,
    pub sql_files: Vec<PathBuf>,
    pub license_files: Vec<PathBuf>,
    pub findings: Vec<Finding>,
}

#[derive(Serialize)]
struct GameVerseManifest<'a> {
    name: &'a str,
    client_scripts: &'a [String],
    server_scripts: &'a [String],
    shared_scripts: &'a [String],
    dependencies: &'a [String],
    ui_page: &'a Option<String>,
    files: &'a [String],
}

pub fn to_gameverse_toml(report: &ResourceReport) -> Result<String> {
    Ok(toml::to_string_pretty(&GameVerseManifest {
        name: &report.resource_name,
        client_scripts: &report.client_scripts,
        server_scripts: &report.server_scripts,
        shared_scripts: &report.shared_scripts,
        dependencies: &report.dependencies,
        ui_page: &report.ui_page,
        files: &report.files,
    })?)
}

pub fn analyze(root: &Path) -> Result<ResourceReport> {
    anyhow::ensure!(
        root.is_dir(),
        "resource path is not a directory: {}",
        root.display()
    );
    let modern = root.join("fxmanifest.lua");
    let legacy = root.join("__resource.lua");
    let manifest = if modern.is_file() {
        modern
    } else if legacy.is_file() {
        legacy.clone()
    } else {
        anyhow::bail!("no fxmanifest.lua or __resource.lua in {}", root.display())
    };
    let manifest_text =
        fs::read_to_string(&manifest).with_context(|| format!("read {}", manifest.display()))?;
    let mut contents = String::new();
    let mut sql_files = Vec::new();
    let mut license_files = Vec::new();
    for entry in WalkDir::new(root).follow_links(false) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        let relative = path.strip_prefix(root).unwrap_or(path).to_path_buf();
        let name = path
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if name.starts_with("license") || name.starts_with("copying") || name.starts_with("notice")
        {
            license_files.push(relative.clone());
        }
        match path
            .extension()
            .and_then(|s| s.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref()
        {
            Some("lua") | Some("js") | Some("ts") | Some("cs") => {
                if let Ok(text) = fs::read_to_string(path) {
                    contents.push('\n');
                    contents.push_str(&text);
                }
            }
            Some("sql") => sql_files.push(relative),
            _ => {}
        }
    }
    let combined = format!("{}\n{}", manifest_text, contents);
    let framework = detect_framework(&combined);
    let ui_page = single(&manifest_text, "ui_page");
    let dependencies = directives(&manifest_text, &["dependency", "dependencies"])?;
    let shared_scripts = directives(&manifest_text, &["shared_script", "shared_scripts"])?;
    let client_scripts = directives(&manifest_text, &["client_script", "client_scripts"])?;
    let server_scripts = directives(&manifest_text, &["server_script", "server_scripts"])?;
    let files = directives(&manifest_text, &["file", "files"])?;
    let events = captures(
        &combined,
        r#"(?:RegisterNetEvent|AddEventHandler|TriggerServerEvent|TriggerClientEvent)\s*\(\s*['\"]([^'\"]+)"#,
    )?;
    let callbacks = captures(
        &combined,
        r#"(?:CreateCallback|TriggerCallback|RegisterCallback)\s*\(\s*['\"]([^'\"]+)"#,
    )?;
    let exports = captures(&combined, r#"exports\s*\(\s*['\"]([^'\"]+)"#)?;
    let natives = detect_natives(&combined)?;
    let mut findings = vec![Finding {
        category: CompatibilityCategory::Convertible,
        feature: "manifest".into(),
        detail: "manifest can be converted to gameverse.toml".into(),
    }];
    if !events.is_empty() {
        findings.push(Finding {
            category: CompatibilityCategory::Supported,
            feature: "events".into(),
            detail: format!("{} statically named event(s)", events.len()),
        });
    }
    if !exports.is_empty() {
        findings.push(Finding {
            category: CompatibilityCategory::Convertible,
            feature: "exports".into(),
            detail: format!("{} export(s) require runtime registration", exports.len()),
        });
    }
    if ui_page.is_some()
        || combined.contains("SendNUIMessage")
        || combined.contains("RegisterNUICallback")
    {
        findings.push(Finding {
            category: CompatibilityCategory::Manual,
            feature: "nui".into(),
            detail: "browser UI needs a GameVerse host bridge".into(),
        });
    }
    if !sql_files.is_empty() {
        findings.push(Finding {
            category: CompatibilityCategory::Manual,
            feature: "database".into(),
            detail: "SQL dialect and schema require review before PostgreSQL migration".into(),
        });
    }
    if !matches!(framework, Framework::Standalone | Framework::Unknown) {
        findings.push(Finding {
            category: CompatibilityCategory::Manual,
            feature: "framework".into(),
            detail: format!("{:?} APIs require a compatibility package", framework),
        });
    }
    if manifest_text.contains("server_only") || manifest_text.contains("data_file") {
        findings.push(Finding {
            category: CompatibilityCategory::Manual,
            feature: "manifest_directives".into(),
            detail: "special FiveM manifest directives require explicit mapping".into(),
        });
    }
    if manifest_text.contains("dofile(") || manifest_text.contains("load(") {
        findings.push(Finding {
            category: CompatibilityCategory::Blocked,
            feature: "dynamic_manifest".into(),
            detail: "dynamic Lua is not executed by the analyzer".into(),
        });
    }
    Ok(ResourceReport {
        resource_name: root
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or("resource")
            .into(),
        legacy_manifest: manifest == legacy,
        manifest: manifest
            .strip_prefix(root)
            .unwrap_or(&manifest)
            .to_path_buf(),
        framework,
        dependencies,
        shared_scripts,
        client_scripts,
        server_scripts,
        files,
        ui_page,
        exports,
        events,
        callbacks,
        natives,
        sql_files,
        license_files,
        findings,
    })
}
fn detect_framework(text: &str) -> Framework {
    let lower = text.to_ascii_lowercase();
    if lower.contains("qbx_core") || lower.contains("qbox") {
        Framework::Qbox
    } else if lower.contains("qb-core") || lower.contains("qbcore") {
        Framework::Qbcore
    } else if lower.contains("es_extended") || lower.contains("esx:") {
        Framework::Esx
    } else {
        Framework::Standalone
    }
}
fn single(text: &str, name: &str) -> Option<String> {
    Regex::new(&format!(
        r#"(?m)\b{}\s*['\"]([^'\"]+)['\"]"#,
        regex::escape(name)
    ))
    .ok()?
    .captures(text)
    .and_then(|c| c.get(1))
    .map(|m| m.as_str().into())
}
fn directives(text: &str, names: &[&str]) -> Result<Vec<String>> {
    let mut out = BTreeSet::new();
    for name in names {
        if let Some(v) = single(text, name) {
            out.insert(v);
        }
        let block = Regex::new(&format!(r#"(?s)\b{}\s*\{{(.*?)\}}"#, regex::escape(name)))?;
        let quoted = Regex::new(r#"['\"]([^'\"]+)['\"]"#)?;
        for cap in block.captures_iter(text) {
            for item in quoted.captures_iter(&cap[1]) {
                out.insert(item[1].to_string());
            }
        }
    }
    Ok(out.into_iter().collect())
}
fn captures(text: &str, pattern: &str) -> Result<Vec<String>> {
    let regex = Regex::new(pattern)?;
    Ok(regex
        .captures_iter(text)
        .filter_map(|c| c.get(1).map(|m| m.as_str().to_string()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect())
}
fn detect_natives(text: &str) -> Result<Vec<String>> {
    let regex = Regex::new(r"\b([A-Z][A-Za-z0-9_]{2,})\s*\(")?;
    let excluded = [
        "RegisterNetEvent",
        "AddEventHandler",
        "TriggerServerEvent",
        "TriggerClientEvent",
        "CreateCallback",
        "TriggerCallback",
        "RegisterCallback",
        "Citizen",
        "Wait",
        "CreateThread",
        "SetTimeout",
    ];
    Ok(regex
        .captures_iter(text)
        .filter_map(|c| c.get(1).map(|m| m.as_str()))
        .filter(|name| !excluded.contains(name))
        .map(str::to_string)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn inventories_without_executing_lua() {
        let dir = tempfile::tempdir().unwrap();
        fs::write(dir.path().join("fxmanifest.lua"),"fx_version 'cerulean'\ngame 'gta5'\nclient_scripts {'client.lua'}\ndependency 'qb-core'\nui_page 'web/index.html'").unwrap();
        fs::write(dir.path().join("client.lua"),"RegisterNetEvent('bank:open')\nexports('balance', function() end)\nlocal p=GetEntityCoords(PlayerPedId())").unwrap();
        fs::write(dir.path().join("schema.sql"), "select 1;").unwrap();
        let report = analyze(dir.path()).unwrap();
        assert_eq!(report.framework, Framework::Qbcore);
        assert_eq!(report.events, vec!["bank:open"]);
        assert!(report.natives.contains(&"GetEntityCoords".into()));
        assert_eq!(report.sql_files.len(), 1);
        let converted = to_gameverse_toml(&report).unwrap();
        assert!(converted.contains("client.lua"));
    }
}
