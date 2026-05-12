use std::{fs, process::Command};

#[test]
fn cli_extract_writes_canonical_evidence_and_candidate() {
    let temp_dir =
        std::env::temp_dir().join(format!("wrench_loader_cli_test_{}", std::process::id()));
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");
    let canonical_path = temp_dir.join("canonical.json");
    let evidence_path = temp_dir.join("evidence.json");
    let candidate_path = temp_dir.join("candidate.json");

    let status = Command::new(env!("CARGO_BIN_EXE_wrench-loader"))
        .args([
            "extract",
            "--input",
            "fixtures/minimal.md",
            "--input-type",
            "markdown",
            "--media-type",
            "text/markdown",
            "--out",
            canonical_path.to_str().unwrap(),
            "--evidence",
            evidence_path.to_str().unwrap(),
            "--gear-source-candidate",
            candidate_path.to_str().unwrap(),
        ])
        .status()
        .expect("CLI should run");

    assert!(status.success());

    let canonical: serde_json::Value =
        serde_json::from_slice(&fs::read(&canonical_path).expect("canonical output should exist"))
            .expect("canonical output should be JSON");
    let evidence: serde_json::Value =
        serde_json::from_slice(&fs::read(&evidence_path).expect("evidence output should exist"))
            .expect("evidence output should be JSON");
    let candidate: serde_json::Value =
        serde_json::from_slice(&fs::read(&candidate_path).expect("candidate output should exist"))
            .expect("candidate output should be JSON");

    assert_eq!(canonical["format"], "wrench.canonical_source_document.v0.1");
    assert_eq!(evidence["format"], "wrench.loader_evidence_report.v0.1");
    assert_eq!(candidate["format"], "wrench.gear_source_candidate.v0.1");
    assert_eq!(evidence["status"], "passed");

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn cli_extract_code_writes_language_metadata() {
    let temp_dir = std::env::temp_dir().join(format!(
        "wrench_loader_cli_code_test_{}",
        std::process::id()
    ));
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");
    let canonical_path = temp_dir.join("code-canonical.json");
    let evidence_path = temp_dir.join("code-evidence.json");

    let status = Command::new(env!("CARGO_BIN_EXE_wrench-loader"))
        .args([
            "extract",
            "--input",
            "fixtures/minimal.rs",
            "--input-type",
            "code",
            "--media-type",
            "text/rust",
            "--out",
            canonical_path.to_str().unwrap(),
            "--evidence",
            evidence_path.to_str().unwrap(),
        ])
        .status()
        .expect("CLI should run");

    assert!(status.success());
    let canonical: serde_json::Value =
        serde_json::from_slice(&fs::read(&canonical_path).expect("code canonical should exist"))
            .expect("code canonical should be JSON");
    assert_eq!(canonical["metadata"]["code"]["language"], "rust");
    assert_eq!(canonical["canonical"]["structure"][0]["block_type"], "code");

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn cli_extract_pdf_fails_closed_without_approved_parser() {
    let temp_dir =
        std::env::temp_dir().join(format!("wrench_loader_cli_pdf_test_{}", std::process::id()));
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");

    let output = Command::new(env!("CARGO_BIN_EXE_wrench-loader"))
        .args([
            "extract",
            "--input",
            "fixtures/minimal.pdf",
            "--input-type",
            "pdf",
            "--media-type",
            "application/pdf",
            "--out",
            temp_dir.join("pdf-canonical.json").to_str().unwrap(),
            "--evidence",
            temp_dir.join("pdf-evidence.json").to_str().unwrap(),
        ])
        .output()
        .expect("CLI should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("no PDF parser is enabled"));

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn cli_extract_office_fails_closed_without_approved_parser() {
    let temp_dir = std::env::temp_dir().join(format!(
        "wrench_loader_cli_office_test_{}",
        std::process::id()
    ));
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");

    let output = Command::new(env!("CARGO_BIN_EXE_wrench-loader"))
        .args([
            "extract",
            "--input",
            "fixtures/minimal.docx",
            "--input-type",
            "office",
            "--media-type",
            "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            "--out",
            temp_dir.join("office-canonical.json").to_str().unwrap(),
            "--evidence",
            temp_dir.join("office-evidence.json").to_str().unwrap(),
        ])
        .output()
        .expect("CLI should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("no Office parser is enabled"));

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn cli_extract_feed_writes_feed_bundle_and_item_evidence() {
    let temp_dir = std::env::temp_dir().join(format!(
        "wrench_loader_cli_feed_test_{}",
        std::process::id()
    ));
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");
    let bundle_path = temp_dir.join("feed-bundle.json");
    let evidence_path = temp_dir.join("feed-evidence.json");

    let status = Command::new(env!("CARGO_BIN_EXE_wrench-loader"))
        .args([
            "extract",
            "--input",
            "fixtures/minimal.rss",
            "--input-type",
            "feed",
            "--feed-format",
            "rss",
            "--media-type",
            "application/rss+xml",
            "--out",
            bundle_path.to_str().unwrap(),
            "--evidence",
            evidence_path.to_str().unwrap(),
        ])
        .status()
        .expect("CLI should run");

    assert!(status.success());
    let bundle: serde_json::Value =
        serde_json::from_slice(&fs::read(&bundle_path).expect("feed bundle should exist"))
            .expect("feed bundle should be JSON");
    let evidence: serde_json::Value =
        serde_json::from_slice(&fs::read(&evidence_path).expect("feed evidence should exist"))
            .expect("feed evidence should be JSON");

    assert_eq!(bundle["format"], "wrench.feed_extraction_bundle.v0.1");
    assert_eq!(bundle["item_count"], 1);
    assert_eq!(evidence.as_array().unwrap().len(), 1);

    let _ = fs::remove_dir_all(temp_dir);
}

#[test]
fn cli_extract_fails_closed_when_secret_block_policy_matches() {
    let temp_dir = std::env::temp_dir().join(format!(
        "wrench_loader_cli_secret_test_{}",
        std::process::id()
    ));
    fs::create_dir_all(&temp_dir).expect("temp dir should be created");

    let output = Command::new(env!("CARGO_BIN_EXE_wrench-loader"))
        .args([
            "extract",
            "--input",
            "fixtures/hostile.html",
            "--input-type",
            "html",
            "--media-type",
            "text/html",
            "--out",
            temp_dir.join("canonical.json").to_str().unwrap(),
            "--evidence",
            temp_dir.join("evidence.json").to_str().unwrap(),
            "--secret-mode",
            "block",
        ])
        .output()
        .expect("CLI should run");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("blocked by policy"));

    let _ = fs::remove_dir_all(temp_dir);
}
