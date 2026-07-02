# ADR-0001 — Scope and upstream policy

- Status: Accepted
- Date: 2026-06-29
- Upstream: [Xberg](https://github.com/xberg-io/xberg)

## Context

`gear-loader` is a companion repository in the Presto-Matic / cos-matic ecosystem. Its role is **rich document ingestion**. It is intentionally separate from the Presto-Matic product repo so heavy dependencies, operational lifecycle, and upstream tracking stay isolated.

## Decision

Build `gear-loader` as an upstream-first, sovereign Rust project:

- track upstream releases/tags/commits explicitly;
- keep local patches small and temporary;
- expose stable contracts rather than leaking upstream internals to consumers;
- enforce permissive OSS licensing and vulnerability gates in CI;
- default to self-hosted/EU-resident operation and avoid US hyperscaler requirements.

## Integration contract

- input: object-store key or uploaded bytes plus declared MIME type
- output: canonical extracted text, metadata, and extraction diagnostics
- limits: max bytes/pages/timeouts enforced before parser execution

## Non-goals

- no direct dependency from presto-server to Xberg internals
- no remote LLM/OCR providers by default
- no silent best-effort ingestion without diagnostics

## Consequences

- The companion can iterate independently from Presto-Matic.
- Presto-Matic avoids accidental dependency bloat and can roll back integration by switching contracts off.
- Upstream changes are absorbed deliberately through version bumps, changelog review, and contract tests.
