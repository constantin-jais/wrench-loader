# disc-loader

Sovereign rich-document ingestion worker/service backed by Xberg.

> Status: `0.0.0` skeleton — public repo created so the boundary, upstream policy,
> and CI gates are explicit before implementation starts.

## Why it exists

Document parsers and OCR are high-blast-radius dependencies; they belong in an isolated worker, not in the product server hot path.

## Upstream relationship

- Upstream: [Xberg](https://github.com/xberg-io/xberg)
- Policy: upstream-first, pinned releases/commits, no permanent fork.
- Fork rule: fork only for a blocking security/build/sovereignty patch; open the upstream PR and remove the fork once merged.

## Contract shape

- input: object-store key or uploaded bytes plus declared MIME type
- output: canonical extracted text, metadata, and extraction diagnostics
- limits: max bytes/pages/timeouts enforced before parser execution

## Non-goals

- no direct dependency from presto-server to Xberg internals
- no remote LLM/OCR providers by default
- no silent best-effort ingestion without diagnostics

## Development

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features
cargo test --workspace --all-features
```

## License

MIT © Constantin Jais
