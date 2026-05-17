# Wrench Loader

[![CI](https://github.com/constantin-jais/wrench-loader/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/constantin-jais/wrench-loader/actions/workflows/ci.yml)
[![Security](https://github.com/constantin-jais/wrench-loader/actions/workflows/security.yml/badge.svg?branch=main)](https://github.com/constantin-jais/wrench-loader/actions/workflows/security.yml)
[![Contracts](https://github.com/constantin-jais/wrench-loader/actions/workflows/contracts.yml/badge.svg?branch=main)](https://github.com/constantin-jais/wrench-loader/actions/workflows/contracts.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)

**Layer:** Wrench — Tooling  
**Role:** document ingestion and canonical extraction  
**Mission:** transform heterogeneous raw sources into clean, structured, auditable content.

---

## Stack role

- **Layer:** Wrench — Tooling.
- **Role:** document ingestion and canonical extraction.
- **Mission:** transform heterogeneous raw sources into clean, structured, auditable content.
- **Maturity:** `dojo`.
- **Scale-ready:** no — CLI/contracts/fixtures exist, while richer parsers still fail closed or need hardening.
- **Current increment:** P1 CLI proof over P0 contracts.
- **Learning value:** canonical ingestion, hostile-content evidence, fail-closed parser policy, and Gear source candidate handoff.
- **Next quality step:** harden PDF/Office/feed/code adapters under license, security, and sandbox gates.

See the ecosystem cockpit in [`constantin-jais/ecosystem/status.md`](https://github.com/constantin-jais/constantin-jais/blob/main/ecosystem/status.md).

## Dogfooding

This repository is part of the forge dogfooding loop: the ecosystem should use its own tools to make specs, maturity, contracts, releases, and product documentation observable.

Current visible evidence:

- CI, contract, and security workflows exercise ingestion and source-candidate boundaries;
- fixtures frame parser, hostile-content, and fail-closed behavior;
- README maturity notes keep PDF/Office/feed/code adapter limits explicit.

Expected next evidence:

- publish example canonical extraction outputs;
- make sandbox and parser-policy evidence visible through fixtures and reports.

Dogfooding claims should stay backed by visible commands, fixtures, CI workflows, generated reports, or linked docs.

## Contributing

See:

- [CONTRIBUTING.md](CONTRIBUTING.md) for contribution guidelines;
- [ROADMAP.md](ROADMAP.md) for current contribution priorities;
- [issue templates](.github/ISSUE_TEMPLATE/) for bugs, docs issues, fixture/example requests, and design discussions.

## Forge role

`wrench-loader` is a Wrench capability used by Rumble products and Bolt plans when raw documents need deterministic extraction, normalization, and evidence before they can become trustworthy context.

## Boundary

It must not own long-term knowledge UX, product meaning, orchestration decisions, or durable memory. Rumble decides the user workflow, Bolt coordinates work, and Gear stores/indexes provenance and source references.

## Purpose

`wrench-loader` performs the dirty work of ingestion: PDF, Office, OCR, HTML, archives, and other rich documents are extracted into canonical text and metadata.

It produces evidence and structured outputs that higher layers can trust.

## Owns

- Source loading, extraction, cleanup, and normalization.
- Canonical text/metadata outputs.
- Deterministic ingestion pipelines and extraction evidence.
- Integration points for Bolt orchestration and Gear memory persistence.

## Does Not Own

- Long-term knowledge management UX: belongs to Rumble.
- Persistent truth or semantic memory: belongs to Gear.
- Strategic decisions about what to ingest next: belongs to Bolt or the product.
- Database/security inspection: belongs to `wrench-db-inspect` or `wrench-inspect` depending on scope.

## Allowed Dependencies

- Can be called by `cos-matic` or directly by a Rumble product.
- Can write structured outputs to `gear-memory`.
- Can emit artifacts that `gear-depot` may distribute or cache.

## Product Vision Challenge

`wrench-loader` must stay an ingestion worker/service, not a knowledge product. Its success is extraction quality, repeatability, and traceable evidence.

## Contract-first P0

The repo now starts with stable contract shapes before heavy parsers:

- `ExtractionRequest v0.1`
- `CanonicalSourceDocument v0.1`
- `LoaderEvidenceReport v0.1`
- `GearSourceCandidate v0.1`

P0 implementation begins with deterministic UTF-8 text, Markdown, and simple HTML normalization. PDF, Office, feeds, URL fetch, code parsing, OCR, and STT must plug into the same contracts instead of introducing product-local formats.

Strict boundary:

- Wrench Loader extracts and normalizes.
- Gear Memory stores, indexes, and owns durable `SourceRef` lifecycle.
- Rumble products decide product meaning and UX.
- Bolt orchestrates and gates.

See `docs/adr/0001-canonical-contracts.md`.

## CLI MVP

```bash
cargo run -- extract \
  --input fixtures/minimal.md \
  --input-type markdown \
  --media-type text/markdown \
  --out /tmp/wrench-loader.canonical.json \
  --evidence /tmp/wrench-loader.evidence.json \
  --gear-source-candidate /tmp/wrench-loader.gear-source-candidate.json
```

PDF and Office remain P0 in the product contract, but the default runtime currently fails closed until parser dependencies pass `cargo deny` advisory checks or sandboxed workers are selected. OCR remains explicit P1.

```bash
cargo run -- extract \
  --input fixtures/minimal.pdf \
  --input-type pdf \
  --media-type application/pdf \
  --out /tmp/wrench-loader.pdf.canonical.json \
  --evidence /tmp/wrench-loader.pdf.evidence.json
# expected today: fail-closed, no approved PDF parser enabled
```

Office modern files also fail closed by default:

```bash
cargo run -- extract \
  --input fixtures/minimal.docx \
  --input-type office \
  --media-type application/vnd.openxmlformats-officedocument.wordprocessingml.document \
  --out /tmp/wrench-loader.office.canonical.json \
  --evidence /tmp/wrench-loader.office.evidence.json
# expected today: fail-closed, no approved Office parser enabled
```

Code extraction is syntax-aware only at metadata level for now; tree-sitter symbol parsing remains a future adapter:

```bash
cargo run -- extract \
  --input fixtures/minimal.rs \
  --input-type code \
  --media-type text/rust \
  --out /tmp/wrench-loader.code.canonical.json \
  --evidence /tmp/wrench-loader.code.evidence.json
```

Feed parsing is a bounded parse-only primitive; polling and curation stay in Rumble/Bolt:

```bash
cargo run -- extract \
  --input fixtures/minimal.rss \
  --input-type feed \
  --feed-format rss \
  --media-type application/rss+xml \
  --out /tmp/wrench-loader.feed-bundle.json \
  --evidence /tmp/wrench-loader.feed-evidence.json
```

Security policy can fail closed:

```bash
cargo run -- extract \
  --input fixtures/hostile.html \
  --input-type html \
  --media-type text/html \
  --out /tmp/canonical.json \
  --evidence /tmp/evidence.json \
  --secret-mode block
```
