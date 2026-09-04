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
