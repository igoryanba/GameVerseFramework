use anyhow::Result;
use gameverse_resource_manifest::{
    expand_patterns, parse_fivem, resolve_and_validate, DataFile, ManifestMetadata,
    ResourceManifest, SourceMetadata,
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
pub struct Evidence {
    pub path: PathBuf,
    pub line: usize,
    pub symbol: String,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct CapabilityRequirement {
    pub capability: String,
    pub category: CompatibilityCategory,
    pub evidence: Vec<Evidence>,
    pub required_bridge: Option<String>,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ResourceReport {
    pub resource_name: String,
    pub manifest: String,
    pub legacy_manifest: bool,
    pub source: SourceMetadata,
    pub metadata: ManifestMetadata,
    pub framework: Framework,
    pub dependencies: Vec<String>,
    pub shared_scripts: Vec<String>,
    pub client_scripts: Vec<String>,
    pub server_scripts: Vec<String>,
    pub resolved_files: Vec<PathBuf>,
    pub missing_patterns: Vec<String>,
    pub files: Vec<String>,
    pub data_files: Vec<DataFile>,
    pub ui_page: Option<String>,
    pub exports: Vec<String>,
    pub cross_resource_exports: Vec<String>,
    pub events: Vec<String>,
    pub callbacks: Vec<String>,
    pub natives: Vec<String>,
    pub commands: Vec<String>,
    pub convars: Vec<String>,
    pub state_bag_apis: Vec<String>,
    pub nui_apis: Vec<String>,
    pub ace_apis: Vec<String>,
    pub runtimes: Vec<String>,
    pub stream_assets: Vec<PathBuf>,
    pub sql_files: Vec<PathBuf>,
    pub findings: Vec<Finding>,
    pub required_capabilities: Vec<CapabilityRequirement>,
}

pub fn to_gameverse_toml(report: &ResourceReport) -> Result<String> {
    gameverse_resource_manifest::to_gameverse_toml(&ResourceManifest {
        name: report.resource_name.clone(),
        metadata: report.metadata.clone(),
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
    validate_declared_patterns(&manifest)?;
    let resolved_files = match resolve_and_validate(root, &manifest) {
        Ok(files) => files,
        Err(error)
            if error
                .to_string()
                .starts_with("resource pattern matched no files:") =>
        {
            inventory_files(root)?
        }
        Err(error) => return Err(error),
    };
    let (shared_scripts, mut missing_patterns) =
        expand_available(&manifest.shared_scripts, &resolved_files);
    let (client_scripts, missing_client) =
        expand_available(&manifest.client_scripts, &resolved_files);
    let (server_scripts, missing_server) =
        expand_available(&manifest.server_scripts, &resolved_files);
    missing_patterns.extend(missing_client);
    missing_patterns.extend(missing_server);
    let mut declared_files = manifest.files.clone();
    declared_files.extend(manifest.data_files.iter().map(|value| value.path.clone()));
    if let Some(page) = &manifest.ui_page {
        declared_files.push(page.clone());
    }
    missing_patterns.extend(expand_available(&declared_files, &resolved_files).1);
    missing_patterns.sort();
    missing_patterns.dedup();
    let mut contents = String::new();
    let mut sources = Vec::<(PathBuf, String)>::new();
    let mut sql_files = Vec::new();
    let mut stream_assets = Vec::new();
    let mut runtimes = BTreeSet::new();
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
                    let relative = path.strip_prefix(root)?.to_path_buf();
                    match path.extension().and_then(|value| value.to_str()) {
                        Some("lua") => {
                            runtimes.insert("lua54".to_string());
                        }
                        Some("js") | Some("ts") => {
                            runtimes.insert("javascript".to_string());
                        }
                        Some("cs") => {
                            runtimes.insert("csharp".to_string());
                        }
                        _ => {}
                    }
                    contents.push('\n');
                    contents.push_str(&text);
                    sources.push((relative, text));
                }
            }
            Some("sql") => sql_files.push(path.strip_prefix(root)?.to_path_buf()),
            _ => {}
        }
        if path
            .strip_prefix(root)?
            .components()
            .next()
            .is_some_and(|value| {
                value
                    .as_os_str()
                    .to_string_lossy()
                    .eq_ignore_ascii_case("stream")
            })
        {
            stream_assets.push(path.strip_prefix(root)?.to_path_buf());
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
    let cross_resource_exports = captures(&contents, r#"exports\s*\[\s*['\"]([^'\"]+)['\"]\s*\]"#)?;
    let natives = detect_natives(&contents)?;
    let commands = captures(&contents, r#"RegisterCommand\s*\(\s*['\"]([^'\"]+)"#)?;
    let convars = captures(
        &contents,
        r#"(?:GetConvar|GetConvarInt|GetConvarBool|SetConvar|SetConvarReplicated)\s*\(\s*['\"]([^'\"]+)"#,
    )?;
    let state_bag_apis = symbol_matches(
        &contents,
        &[
            "GlobalState",
            "LocalPlayer.state",
            "Player.state",
            "Entity.state",
            "AddStateBagChangeHandler",
        ],
    );
    let nui_apis = symbol_matches(
        &contents,
        &[
            "SendNUIMessage",
            "RegisterNUICallback",
            "SetNuiFocus",
            "SetNuiFocusKeepInput",
        ],
    );
    let ace_apis = symbol_matches(
        &contents,
        &[
            "IsPlayerAceAllowed",
            "add_ace",
            "remove_ace",
            "add_principal",
            "remove_principal",
        ],
    );
    let required_capabilities = build_capabilities(
        &sources,
        &events,
        &callbacks,
        &exports,
        &cross_resource_exports,
        &natives,
        &commands,
        &convars,
        &state_bag_apis,
        &nui_apis,
        &ace_apis,
        &runtimes,
        !stream_assets.is_empty(),
        !manifest.data_files.is_empty(),
    )?;
    let mut findings = vec![finding(
        CompatibilityCategory::Convertible,
        "manifest",
        "static manifest can be converted to gameverse.toml",
    )];
    for pattern in &missing_patterns {
        findings.push(finding(
            CompatibilityCategory::Manual,
            "missing_file",
            &format!("manifest pattern matched no files: {pattern}"),
        ));
    }
    if !events.is_empty() {
        findings.push(finding(
            CompatibilityCategory::Convertible,
            "events",
            &format!("{} event name(s); M2 routing must be enabled", events.len()),
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
    if !cross_resource_exports.is_empty() {
        findings.push(finding(
            CompatibilityCategory::Manual,
            "cross_resource_exports",
            &format!(
                "{} provider reference(s) require cluster routing",
                cross_resource_exports.len()
            ),
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
    if manifest.source.license.as_deref().is_none_or(|value| {
        !matches!(
            value,
            "MIT" | "Apache-2.0" | "BSD-2-Clause" | "BSD-3-Clause" | "Unlicense"
        )
    }) {
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
        metadata: manifest.metadata,
        framework,
        dependencies: manifest.dependencies,
        shared_scripts,
        client_scripts,
        server_scripts,
        resolved_files,
        missing_patterns,
        files: manifest.files,
        data_files: manifest.data_files,
        ui_page: manifest.ui_page,
        exports,
        cross_resource_exports,
        events,
        callbacks,
        natives,
        commands,
        convars,
        state_bag_apis,
        nui_apis,
        ace_apis,
        runtimes: runtimes.into_iter().collect(),
        stream_assets,
        sql_files,
        findings,
        required_capabilities,
    })
}

fn inventory_files(root: &Path) -> Result<Vec<PathBuf>> {
    let canonical_root = root.canonicalize()?;
    let mut files = Vec::new();
    for entry in WalkDir::new(root).follow_links(false) {
        let entry = entry?;
        if entry.file_type().is_symlink() {
            anyhow::bail!("resource contains a symlink: {}", entry.path().display());
        }
        if !entry.file_type().is_file() {
            continue;
        }
        let canonical = entry.path().canonicalize()?;
        anyhow::ensure!(
            canonical.starts_with(&canonical_root),
            "resource path escapes its root"
        );
        files.push(entry.path().strip_prefix(root)?.to_path_buf());
    }
    files.sort();
    Ok(files)
}

fn validate_declared_patterns(manifest: &ResourceManifest) -> Result<()> {
    let patterns = manifest
        .shared_scripts
        .iter()
        .chain(&manifest.client_scripts)
        .chain(&manifest.server_scripts)
        .chain(&manifest.files)
        .chain(manifest.data_files.iter().map(|value| &value.path))
        .chain(manifest.ui_page.iter());
    for pattern in patterns {
        let normalized = pattern.replace('\\', "/");
        let drive_absolute = normalized.as_bytes().get(1) == Some(&b':');
        anyhow::ensure!(
            !normalized.starts_with('/')
                && !drive_absolute
                && !normalized.split('/').any(|part| part == "..")
                && !normalized.contains('\0'),
            "resource path is unsafe: {pattern}"
        );
    }
    Ok(())
}

fn expand_available(patterns: &[String], files: &[PathBuf]) -> (Vec<String>, Vec<String>) {
    let mut expanded = BTreeSet::new();
    let mut missing = Vec::new();
    for pattern in patterns {
        match expand_patterns(std::slice::from_ref(pattern), files) {
            Ok(matches) => expanded.extend(matches),
            Err(_) => missing.push(pattern.clone()),
        }
    }
    (expanded.into_iter().collect(), missing)
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

fn symbol_matches(text: &str, symbols: &[&str]) -> Vec<String> {
    symbols
        .iter()
        .filter(|symbol| text.contains(**symbol))
        .map(|symbol| (*symbol).to_string())
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn build_capabilities(
    sources: &[(PathBuf, String)],
    events: &[String],
    callbacks: &[String],
    exports: &[String],
    cross_resource_exports: &[String],
    natives: &[String],
    commands: &[String],
    convars: &[String],
    state_bags: &[String],
    nui: &[String],
    ace: &[String],
    runtimes: &BTreeSet<String>,
    has_stream_assets: bool,
    has_data_files: bool,
) -> Result<Vec<CapabilityRequirement>> {
    let specifications = [
        (
            "lua.events.network",
            !events.is_empty(),
            CompatibilityCategory::Convertible,
            Some("m2-resource-events"),
        ),
        (
            "lua.callbacks",
            !callbacks.is_empty(),
            CompatibilityCategory::Supported,
            None,
        ),
        (
            "lua.exports",
            !exports.is_empty(),
            CompatibilityCategory::Supported,
            None,
        ),
        (
            "lua.exports.cross_resource",
            !cross_resource_exports.is_empty(),
            CompatibilityCategory::Manual,
            Some("resource-cluster-export-router"),
        ),
        (
            "gta.natives",
            !natives.is_empty(),
            CompatibilityCategory::Convertible,
            Some("adapter-native-host"),
        ),
        (
            "lua.commands",
            !commands.is_empty(),
            CompatibilityCategory::Manual,
            Some("command-registry"),
        ),
        (
            "lua.convars",
            !convars.is_empty(),
            CompatibilityCategory::Manual,
            Some("convar-registry"),
        ),
        (
            "entity.state_bags",
            !state_bags.is_empty(),
            CompatibilityCategory::Manual,
            Some("state-bag-bridge"),
        ),
        (
            "ui.nui",
            !nui.is_empty(),
            CompatibilityCategory::Manual,
            Some("webview2-nui-bridge"),
        ),
        (
            "server.ace",
            !ace.is_empty(),
            CompatibilityCategory::Manual,
            Some("permission-bridge"),
        ),
        (
            "assets.stream",
            has_stream_assets,
            CompatibilityCategory::Blocked,
            Some("asset-mounter"),
        ),
        (
            "assets.data_files",
            has_data_files,
            CompatibilityCategory::Blocked,
            Some("data-file-mounter"),
        ),
        (
            "runtime.javascript",
            runtimes.contains("javascript"),
            CompatibilityCategory::Blocked,
            Some("javascript-runtime"),
        ),
        (
            "runtime.csharp",
            runtimes.contains("csharp"),
            CompatibilityCategory::Blocked,
            Some("csharp-runtime"),
        ),
    ];
    let mut output = Vec::new();
    for (capability, present, category, bridge) in specifications {
        if !present {
            continue;
        }
        let tokens: Vec<&str> = match capability {
            "lua.events.network" => vec![
                "RegisterNetEvent",
                "TriggerServerEvent",
                "TriggerClientEvent",
            ],
            "lua.callbacks" => vec!["CreateCallback", "TriggerCallback", "RegisterCallback"],
            "lua.exports" => vec!["exports"],
            "lua.exports.cross_resource" => vec!["exports[", "exports ["],
            "gta.natives" => natives.iter().map(String::as_str).collect(),
            "lua.commands" => vec!["RegisterCommand"],
            "lua.convars" => vec!["GetConvar", "SetConvar"],
            "entity.state_bags" => state_bags.iter().map(String::as_str).collect(),
            "ui.nui" => nui.iter().map(String::as_str).collect(),
            "server.ace" => ace.iter().map(String::as_str).collect(),
            _ => vec![],
        };
        output.push(CapabilityRequirement {
            capability: capability.into(),
            category,
            evidence: evidence_for(sources, &tokens),
            required_bridge: bridge.map(str::to_string),
        });
    }
    Ok(output)
}

fn evidence_for(sources: &[(PathBuf, String)], tokens: &[&str]) -> Vec<Evidence> {
    let mut output = Vec::new();
    for (path, text) in sources {
        for (index, line) in text.lines().enumerate() {
            for token in tokens {
                if !token.is_empty() && line.contains(token) {
                    output.push(Evidence {
                        path: path.clone(),
                        line: index + 1,
                        symbol: (*token).to_string(),
                    });
                    if output.len() >= 128 {
                        return output;
                    }
                }
            }
        }
    }
    output
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
