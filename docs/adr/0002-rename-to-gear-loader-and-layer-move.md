# ADR 0002 — Rename to gear-loader and layer move

- Status: Accepted
- Date: 2026-07-02

## Context

The repository was initially named `wrench-loader` to reflect a layer classification scheme based on deployment context (Wrench = ingestion tools isolated from product delivery). As the ecosystem matured, a new classification criterion emerged: **client consumption pattern**. Rumble products (feed-mind, Canvas, …) and Bolt orchestration now both depend on `gear-loader` as a runtime substrate, not a standalone utility tool.

## Decision

Rename the repository and package from `wrench-loader` to `gear-loader`. The layer classification moves from Wrench (tooling) to Gear (runtime substrate) per **D15 layer-classification-by-client**.

Specifically:

- Package name in `Cargo.toml`: `wrench-loader` → `gear-loader`
- Repository URL: `wrench-loader` → `gear-loader`
- Documentation and examples: all references to the tool name updated consistently
- Temporary directories in tests: `wrench_loader_*` → `gear_loader_*`
- Binary references in test macros: `CARGO_BIN_EXE_wrench-loader` → `CARGO_BIN_EXE_gear-loader`

## Rationale

**One contract, two clients:** The canonical ingestion contracts (`ExtractionRequest`, `CanonicalSourceDocument`, `LoaderEvidenceReport`, `GearSourceCandidate`) are consumed by:

1. **Rumble products** (feed-mind, Canvas) as runtime dependencies for deterministic ingestion.
2. **Bolt orchestration** as a factory-time ingestion service.

A library that is linkable and consumed by product runtimes belongs to **Gear** (runtime substrate), not **Wrench** (isolated tooling). See [`ecosystem/adr/D15-layer-classification-by-client.md`](https://github.com/constantin-jais/ecosystem/blob/main/adr/D15-layer-classification-by-client.md) for the full classification criterion.

## Non-changes

Contract format identifiers remain unchanged:

- `wrench.canonical_source_document.v0.1`
- `wrench.loader_evidence_report.v0.1`
- `wrench.gear_source_candidate.v0.1`
- `wrench.feed_extraction_bundle.v0.1`

These identifiers are part of the durable contract, not the tool name.

## Consequences

- The repository now reflects its actual role in the product architecture.
- Consumers (Rumble, Bolt) link a Gear-layer substrate, not an isolated Wrench tool.
- Repository README, documentation, and examples accurately document layer ownership and boundaries.
- CI workflows and binary names align with package metadata.
