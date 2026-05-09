# Wrench Loader

**Layer:** Wrench — Tooling  
**Role:** document ingestion and canonical extraction  
**Mission:** transform heterogeneous raw sources into clean, structured, auditable content.

---

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
