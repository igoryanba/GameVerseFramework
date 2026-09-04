use anyhow::Result;
use gameverse_resource_manifest::{
    expand_patterns, parse_fivem, resolve_and_validate, DataFile, ResourceManifest, SourceMetadata,
};
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
    pub manifest: String,
    pub legacy_manifest: bool,
    pub source: SourceMetadata,
    pub framework: Framework,
    pub dependencies: Vec<String>,
    pub shared_scripts: Vec<String>,
    pub client_scripts: Vec<String>,
    pub server_scripts: Vec<String>,
    pub resolved_files: Vec<PathBuf>,
    pub files: Vec<String>,
    pub data_files: Vec<DataFile>,
    pub ui_page: Option<String>,
    pub exports: Vec<String>,
    pub events: Vec<String>,
    pub callbacks: Vec<String>,
    pub natives: Vec<String>,
    pub sql_files: Vec<PathBuf>,
    pub findings: Vec<Finding>,
}

pub fn to_gameverse_toml(report: &ResourceReport) -> Result<String> {
    gameverse_resource_manifest::to_gameverse_toml(&ResourceManifest {
        name: report.resource_name.clone(),
        client_scripts: report.client_scripts.clone(),
        server_scripts: report.server_scripts.clone(),
        shared_scripts: report.shared_scripts.clone(),
        dependencies: report.dependencies.clone(),
        files: report.files.clone(),
        exports: report.exports.clone(),
        data_files: report.data_files.clone(),
        ui_page: report.ui_page.clone(),
        source: report.source.clone(),
    })
}

pub fn analyze(root: &Path) -> Result<ResourceReport> {
    let parsed = parse_fivem(root)?;
    let manifest = parsed.manifest;
    let resolved_files = resolve_and_validate(root, &manifest)?;
    let shared_scripts = expand_patterns(&manifest.shared_scripts, &resolved_files)?;
    let client_scripts = expand_patterns(&manifest.client_scripts, &resolved_files)?;
    let server_scripts = expand_patterns(&manifest.server_scripts, &resolved_files)?;
    let mut contents = String::new();
    let mut sql_files = Vec::new();
    for entry in WalkDir::new(root).follow_links(false) {
        let entry = entry?;
        if !entry.file_type().is_file() {
            continue;
        }
        let path = entry.path();
        match path
            .extension()
            .and_then(|v| v.to_str())
            .map(str::to_ascii_lowercase)
            .as_deref()
        {
            Some("lua") | Some("js") | Some("ts") | Some("cs") => {
                if let Ok(text) = fs::read_to_string(path) {
                    contents.push('\n');
                    contents.push_str(&text);
                }
            }
            Some("sql") => sql_files.push(path.strip_prefix(root)?.to_path_buf()),
            _ => {}
        }
    }
    let framework = detect_framework(&contents);
    let events = captures(
        &contents,
        r#"(?:RegisterNetEvent|AddEventHandler|TriggerServerEvent|TriggerClientEvent)\s*\(\s*['\"]([^'\"]+)"#,
    )?;
    let callbacks = captures(
        &contents,
        r#"(?:CreateCallback|TriggerCallback|RegisterCallback)\s*\(\s*['\"]([^'\"]+)"#,
    )?;
    let code_exports = captures(&contents, r#"exports\s*\(\s*['\"]([^'\"]+)"#)?;
    let exports = manifest
        .exports
        .iter()
        .chain(&code_exports)
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    let natives = detect_natives(&contents)?;
    let mut findings = vec![finding(
        CompatibilityCategory::Convertible,
        "manifest",
        "static manifest can be converted to gameverse.toml",
    )];
    if !events.is_empty() {
        findings.push(finding(
            CompatibilityCategory::Supported,
            "events",
            &format!("{} statically named event(s)", events.len()),
        ));
    }
    if !callbacks.is_empty() {
        findings.push(finding(
            CompatibilityCategory::Supported,
            "callbacks",
            &format!("{} callback name(s)", callbacks.len()),
        ));
    }
    if !exports.is_empty() {
        findings.push(finding(
            CompatibilityCategory::Supported,
            "exports",
            &format!("{} export(s)", exports.len()),
        ));
    }
    if !natives.is_empty() {
        findings.push(finding(
            CompatibilityCategory::Convertible,
            "natives",
            &format!(
                "{} native candidate(s) require allow-list review",
                natives.len()
            ),
        ));
    }
    if manifest.ui_page.is_some()
        || contents.contains("SendNUIMessage")
        || contents.contains("RegisterNUICallback")
    {
        findings.push(finding(
            CompatibilityCategory::Manual,
            "nui",
            "browser UI host is outside the first import milestone",
        ));
    }
    if !sql_files.is_empty() {
        findings.push(finding(
            CompatibilityCategory::Manual,
            "database",
            "SQL schema requires explicit PostgreSQL migration",
        ));
    }
    if !manifest.data_files.is_empty() {
        findings.push(finding(
            CompatibilityCategory::Manual,
            "data_files",
            "GTA asset data_file mounting is not implemented",
        ));
    }
    if !matches!(framework, Framework::Standalone) {
        findings.push(finding(
            CompatibilityCategory::Manual,
            "framework",
            &format!("{framework:?} APIs require a separate bridge package"),
        ));
    }
    if manifest
        .source
        .license
        .as_deref()
        .is_none_or(|value| !matches!(value, "MIT" | "Apache-2.0"))
    {
        findings.push(finding(
            CompatibilityCategory::Manual,
            "license",
            "resource license is unknown or copyleft; keep it outside GameVerse binaries",
        ));
    }
    for token in parsed.blocked_dynamic {
        findings.push(finding(
            CompatibilityCategory::Blocked,
            "dynamic_manifest",
            &format!("manifest contains dynamic token {token}; it was not executed"),
        ));
    }
    Ok(ResourceReport {
        resource_name: manifest.name,
        manifest: manifest.source.manifest.clone(),
        legacy_manifest: manifest.source.legacy,
        source: manifest.source,
        framework,
        dependencies: manifest.dependencies,
        shared_scripts,
        client_scripts,
        server_scripts,
        resolved_files,
        files: manifest.files,
        data_files: manifest.data_files,
        ui_page: manifest.ui_page,
        exports,
        events,
        callbacks,
        natives,
        sql_files,
        findings,
    })
}

fn finding(category: CompatibilityCategory, feature: &str, detail: &str) -> Finding {
    Finding {
        category,
        feature: feature.into(),
        detail: detail.into(),
    }
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
fn captures(text: &str, pattern: &str) -> Result<Vec<String>> {
    let regex = Regex::new(pattern)?;
    Ok(regex
        .captures_iter(text)
        .filter_map(|c| c.get(1).map(|v| v.as_str().to_string()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect())
}
fn detect_natives(text: &str) -> Result<Vec<String>> {
    let regex = Regex::new(r"\b([A-Z][A-Za-z0-9_]{2,})\s*\(")?;
    let excluded = [
        "RegisterNetEvent",
        "AddEventHandler",
        "RemoveEventHandler",
        "TriggerEvent",
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
        .filter_map(|c| c.get(1).map(|v| v.as_str()))
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
    fn reports_supported_and_manual_facts() {
        let dir = tempfile::tempdir().unwrap();
        fs::create_dir(dir.path().join("client")).unwrap();
        fs::write(
            dir.path().join("LICENSE"),
            "MIT License\nPermission is hereby granted",
        )
        .unwrap();
        fs::write(
            dir.path().join("fxmanifest.lua"),
            "client_script 'client/main.lua'\nui_page 'web.html'",
        )
        .unwrap();
        fs::write(dir.path().join("client/main.lua"),"RegisterNetEvent('bank:open')\nexports('balance', function() end)\nGetEntityCoords(PlayerPedId())").unwrap();
        fs::write(dir.path().join("web.html"), "").unwrap();
        let report = analyze(dir.path()).unwrap();
        assert_eq!(report.events, vec!["bank:open"]);
        assert!(report
            .findings
            .iter()
            .any(|v| v.feature == "nui" && v.category == CompatibilityCategory::Manual));
        assert!(to_gameverse_toml(&report)
            .unwrap()
            .contains("client/main.lua"));
    }
}
