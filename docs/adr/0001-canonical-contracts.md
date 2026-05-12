# ADR 0001 — Canonical ingestion contracts first

Status: Proposed
Date: 2026-06-30

## Context

Rumble products need ingestion for notes, learning sessions, feeds, specs, blog content, and agent context. If each product parses HTML, Markdown, PDFs, Office files, feeds, URLs, code, OCR, or STT locally, the ecosystem duplicates security risk and loses provenance.

## Decision

`wrench-loader` owns canonical extraction contracts before heavy parser implementation.

The first public contracts are:

- `ExtractionRequest v0.1`
- `CanonicalSourceDocument v0.1`
- `LoaderEvidenceReport v0.1`
- `GearSourceCandidate v0.1`

Canonical truth is structured JSON. Markdown is only a readable projection.

`wrench-loader` may produce a `GearSourceCandidate`, but it does not create durable `SourceRef`; Gear Memory owns storage, indexing, lifecycle, deletion/anonymization, and retrieval.

## Boundaries

- Wrench Loader extracts, normalizes, detects risk, and emits evidence.
- Gear Memory stores/indexes and owns durable memory/source refs.
- Rumble products decide product meaning, UX, publication, curation, sessions, and notes.
- Bolt orchestrates and gates; it does not become a parser.

## Security requirements

- Treat all extracted content as untrusted data.
- No PII or secrets in logs/evidence metadata.
- Security findings travel with the canonical output.
- OCR/STT remain disabled until explicit policy and license/RGPD review.
- No SaaS US, AGPL, SSPL, BSL, or proprietary runtime dependency.

## Consequences

The first implementation is intentionally modest: UTF-8 text, Markdown, and simple HTML normalization. PDF, Office, feed parsing, code syntax chunks, OCR, and STT must plug into the same contracts instead of creating parallel output shapes.
