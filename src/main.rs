use std::{env, fs, process::ExitCode};

use gear_loader::{
    EXTRACTION_REQUEST_FORMAT, ExtractionInput, ExtractionPolicy, ExtractionRequest, FeatureToggle,
    FeedFormat, FindingMode, InputKind, NetworkPolicy, PromptInjectionMode, RawInput,
    SourceInputType, extract_code_text, extract_feed, extract_office_text, extract_pdf_text,
    extract_text_like, summary,
};

fn main() -> ExitCode {
    match run(env::args().skip(1).collect()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(1)
        }
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
    if args.is_empty() {
        println!("{}", summary());
        return Ok(());
    }

    match args[0].as_str() {
        "extract" => run_extract(&args[1..]),
        "--help" | "-h" | "help" => {
            print_help();
            Ok(())
        }
        unknown => Err(format!("unknown command '{unknown}'. Try --help.")),
    }
}

fn run_extract(args: &[String]) -> Result<(), String> {
    let mut input_path = None;
    let mut input_type = None;
    let mut media_type = None;
    let mut out_path = None;
    let mut evidence_path = None;
    let mut gear_candidate_path = None;
    let mut feed_format = None;
    let mut timestamp = "2026-06-30T00:00:00Z".to_owned();
    let mut secret_mode = FindingMode::Detect;
    let mut pii_mode = FindingMode::Detect;
    let mut prompt_mode = PromptInjectionMode::Detect;

    let mut i = 0;
    while i < args.len() {
        let key = args[i].as_str();
        let value = args
            .get(i + 1)
            .ok_or_else(|| format!("missing value for {key}"))?;
        match key {
            "--input" => input_path = Some(value.clone()),
            "--input-type" => input_type = Some(parse_input_type(value)?),
            "--media-type" => media_type = Some(value.clone()),
            "--out" => out_path = Some(value.clone()),
            "--evidence" => evidence_path = Some(value.clone()),
            "--gear-source-candidate" => gear_candidate_path = Some(value.clone()),
            "--feed-format" => feed_format = Some(parse_feed_format(value)?),
            "--timestamp" => timestamp = value.clone(),
            "--secret-mode" => secret_mode = parse_finding_mode(value)?,
            "--pii-mode" => pii_mode = parse_finding_mode(value)?,
            "--prompt-injection-mode" => prompt_mode = parse_prompt_mode(value)?,
            unknown => return Err(format!("unknown extract option '{unknown}'")),
        }
        i += 2;
    }

    let input_path = input_path.ok_or("--input is required")?;
    let input_type = input_type.ok_or("--input-type is required")?;
    let media_type = media_type.ok_or("--media-type is required")?;
    let out_path = out_path.ok_or("--out is required")?;

    let bytes =
        fs::read(&input_path).map_err(|err| format!("failed to read {input_path}: {err}"))?;
    let request = ExtractionRequest {
        format: EXTRACTION_REQUEST_FORMAT.to_owned(),
        request_id: "cli_req_01".to_owned(),
        actor_ref: "cli_actor".to_owned(),
        workspace_ref: "cli_workspace".to_owned(),
        input: ExtractionInput {
            kind: InputKind::FileRef,
            reference: input_path.clone(),
        },
        policy: ExtractionPolicy {
            allowed_media_types: vec![media_type.clone()],
            max_bytes: 25_000_000,
            network: NetworkPolicy::Disabled,
            ocr: FeatureToggle::Disabled,
            stt: FeatureToggle::Disabled,
            pii_mode,
            secret_mode,
            prompt_injection_mode: prompt_mode,
        },
        requested_outputs: vec![
            "canonical_document".to_owned(),
            "evidence_report".to_owned(),
            "gear_source_candidate".to_owned(),
        ],
    };

    if input_type == SourceInputType::Feed {
        let feed_format = feed_format.ok_or("--feed-format is required for feed input")?;
        let bundle = extract_feed(
            &request,
            RawInput {
                input_type,
                media_type: &media_type,
                bytes: &bytes,
                uri: None,
                filename: Some(&input_path),
            },
            feed_format,
            &timestamp,
        )
        .map_err(|err| err.to_string())?;
        if let Some(evidence_path) = evidence_path {
            let reports = bundle
                .items
                .iter()
                .map(|item| &item.evidence_report)
                .collect::<Vec<_>>();
            write_json(&evidence_path, &reports)?;
        }
        write_json(&out_path, &bundle)?;
        return Ok(());
    }

    let evidence_path = evidence_path.ok_or("--evidence is required")?;
    if input_type == SourceInputType::Pdf {
        let bundle = extract_pdf_text(
            &request,
            RawInput {
                input_type,
                media_type: &media_type,
                bytes: &bytes,
                uri: None,
                filename: Some(&input_path),
            },
            &timestamp,
        )
        .map_err(|err| err.to_string())?;
        write_json(&out_path, &bundle.canonical_document)?;
        write_json(&evidence_path, &bundle.evidence_report)?;
        if let (Some(path), Some(candidate)) = (gear_candidate_path, bundle.gear_source_candidate) {
            write_json(&path, &candidate)?;
        }
        return Ok(());
    }

    if input_type == SourceInputType::Office {
        let bundle = extract_office_text(
            &request,
            RawInput {
                input_type,
                media_type: &media_type,
                bytes: &bytes,
                uri: None,
                filename: Some(&input_path),
            },
            &timestamp,
        )
        .map_err(|err| err.to_string())?;
        write_json(&out_path, &bundle.canonical_document)?;
        write_json(&evidence_path, &bundle.evidence_report)?;
        if let (Some(path), Some(candidate)) = (gear_candidate_path, bundle.gear_source_candidate) {
            write_json(&path, &candidate)?;
        }
        return Ok(());
    }

    if input_type == SourceInputType::Code {
        let bundle = extract_code_text(
            &request,
            RawInput {
                input_type,
                media_type: &media_type,
                bytes: &bytes,
                uri: None,
                filename: Some(&input_path),
            },
            &timestamp,
        )
        .map_err(|err| err.to_string())?;
        write_json(&out_path, &bundle.canonical_document)?;
        write_json(&evidence_path, &bundle.evidence_report)?;
        if let (Some(path), Some(candidate)) = (gear_candidate_path, bundle.gear_source_candidate) {
            write_json(&path, &candidate)?;
        }
        return Ok(());
    }

    let bundle = extract_text_like(
        &request,
        RawInput {
            input_type,
            media_type: &media_type,
            bytes: &bytes,
            uri: None,
            filename: Some(&input_path),
        },
        &timestamp,
    )
    .map_err(|err| err.to_string())?;

    write_json(&out_path, &bundle.canonical_document)?;
    write_json(&evidence_path, &bundle.evidence_report)?;
    if let (Some(path), Some(candidate)) = (gear_candidate_path, bundle.gear_source_candidate) {
        write_json(&path, &candidate)?;
    }

    Ok(())
}

fn write_json<T: serde::Serialize>(path: &str, value: &T) -> Result<(), String> {
    let json = serde_json::to_string_pretty(value).map_err(|err| err.to_string())?;
    fs::write(path, format!("{json}\n")).map_err(|err| format!("failed to write {path}: {err}"))
}

fn parse_input_type(value: &str) -> Result<SourceInputType, String> {
    match value {
        "markdown" => Ok(SourceInputType::Markdown),
        "html" => Ok(SourceInputType::Html),
        "text" => Ok(SourceInputType::Text),
        "code" => Ok(SourceInputType::Code),
        "feed" => Ok(SourceInputType::Feed),
        "pdf" => Ok(SourceInputType::Pdf),
        "office" => Ok(SourceInputType::Office),
        other => Err(format!("unsupported --input-type '{other}'")),
    }
}

fn parse_feed_format(value: &str) -> Result<FeedFormat, String> {
    match value {
        "rss" => Ok(FeedFormat::Rss),
        "atom" => Ok(FeedFormat::Atom),
        "json-feed" => Ok(FeedFormat::JsonFeed),
        other => Err(format!("unsupported --feed-format '{other}'")),
    }
}

fn parse_finding_mode(value: &str) -> Result<FindingMode, String> {
    match value {
        "detect" => Ok(FindingMode::Detect),
        "redact" => Ok(FindingMode::Redact),
        "block" => Ok(FindingMode::Block),
        other => Err(format!("unsupported finding mode '{other}'")),
    }
}

fn parse_prompt_mode(value: &str) -> Result<PromptInjectionMode, String> {
    match value {
        "detect" => Ok(PromptInjectionMode::Detect),
        "quarantine_on_high" => Ok(PromptInjectionMode::QuarantineOnHigh),
        other => Err(format!("unsupported prompt injection mode '{other}'")),
    }
}

fn print_help() {
    println!(
        "gear-loader\n\nCommands:\n  extract --input <path> --input-type <markdown|html|text|code|pdf|office> --media-type <type> --out <canonical.json> --evidence <evidence.json> [--gear-source-candidate <candidate.json>] [--secret-mode detect|redact|block] [--pii-mode detect|redact|block] [--prompt-injection-mode detect|quarantine_on_high]\n\n  extract --input <feed> --input-type feed --feed-format <rss|atom|json-feed> --media-type <type> --out <feed-bundle.json> [--evidence <item-evidence-array.json>]"
    );
}
