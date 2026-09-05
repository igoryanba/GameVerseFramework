use fivem_analyzer::{analyze, CompatibilityCategory, Framework};
use std::path::PathBuf;

fn fixture(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests/fixtures")
        .join(name)
}

#[test]
fn modern_manifest_reports_exact_supported_and_manual_features() {
    let report = analyze(&fixture("modern")).unwrap();
    assert!(!report.legacy_manifest);
    assert_eq!(report.framework, Framework::Standalone);
    assert_eq!(report.client_scripts, ["client/main.lua"]);
    assert_eq!(report.server_scripts, ["server/main.lua"]);
    assert_eq!(report.exports, ["ready"]);
    assert_eq!(report.metadata.fx_version.as_deref(), Some("cerulean"));
    assert_eq!(report.metadata.games, ["gta5"]);
    assert_eq!(report.commands, ["modern"]);
    assert_eq!(report.convars, ["modern:locale"]);
    let command = report
        .required_capabilities
        .iter()
        .find(|value| value.capability == "lua.commands")
        .unwrap();
    assert_eq!(command.category, CompatibilityCategory::Manual);
    assert!(command
        .evidence
        .iter()
        .any(|value| value.path.ends_with("client/main.lua") && value.line == 2));
    assert!(report.findings.iter().any(
        |finding| finding.feature == "nui" && finding.category == CompatibilityCategory::Manual
    ));
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.feature == "data_files"
            && finding.category == CompatibilityCategory::Manual));
}

#[test]
fn legacy_qbcore_and_esx_detection_is_evidence_based() {
    let legacy = analyze(&fixture("legacy")).unwrap();
    assert!(legacy.legacy_manifest);
    assert_eq!(legacy.events, ["legacy:ready"]);

    let qbcore = analyze(&fixture("qbcore")).unwrap();
    assert_eq!(qbcore.framework, Framework::Qbcore);
    assert!(qbcore
        .findings
        .iter()
        .any(|finding| finding.feature == "framework"
            && finding.category == CompatibilityCategory::Manual));

    let esx = analyze(&fixture("esx")).unwrap();
    assert_eq!(esx.framework, Framework::Esx);
    assert!(esx
        .findings
        .iter()
        .any(|finding| finding.feature == "database"
            && finding.category == CompatibilityCategory::Manual));
}

#[test]
fn dynamic_manifest_is_blocked_without_execution() {
    let report = analyze(&fixture("dynamic")).unwrap();
    assert!(report
        .findings
        .iter()
        .any(|finding| finding.feature == "dynamic_manifest"
            && finding.category == CompatibilityCategory::Blocked));
}

#[test]
fn external_corpus_is_pinned_and_copyleft_is_analysis_only() {
    let path =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../Research/resource-corpus.lock.json");
    let value: serde_json::Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    let resources = value["resources"].as_array().unwrap();
    assert!(resources.len() >= 7);
    for resource in resources {
        let commit = resource["commit"].as_str().unwrap();
        let sha = resource["archive_sha256"].as_str().unwrap();
        assert_eq!(commit.len(), 40);
        assert!(commit.bytes().all(|value| value.is_ascii_hexdigit()));
        assert_eq!(sha.len(), 64);
        assert!(sha.bytes().all(|value| value.is_ascii_hexdigit()));
        let license = resource["license"].as_str().unwrap();
        if license.starts_with("GPL") || license == "mixed-per-resource" {
            assert_eq!(resource["mode"], "analysis_only");
        }
    }
}

#[test]
fn missing_build_output_is_reported_but_unsafe_paths_still_fail() {
    let missing = tempfile::tempdir().unwrap();
    std::fs::write(
        missing.path().join("fxmanifest.lua"),
        "fx_version 'cerulean'\ngame 'gta5'\nclient_script 'dist/index.js'\n",
    )
    .unwrap();
    let report = analyze(missing.path()).unwrap();
    assert_eq!(report.missing_patterns, ["dist/index.js"]);
    assert!(report.findings.iter().any(|finding| {
        finding.feature == "missing_file" && finding.category == CompatibilityCategory::Manual
    }));

    let unsafe_resource = tempfile::tempdir().unwrap();
    std::fs::write(
        unsafe_resource.path().join("fxmanifest.lua"),
        "fx_version 'cerulean'\ngame 'gta5'\nclient_script '../escape.lua'\n",
    )
    .unwrap();
    assert!(analyze(unsafe_resource.path())
        .unwrap_err()
        .to_string()
        .contains("unsafe"));
}
