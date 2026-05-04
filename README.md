# wrench-loader

> Sovereign rich-document ingestion worker/service — PDF, Office, OCR, HTML, and archives into canonical text and metadata, backed by [Xberg](https://github.com/xberg-io/xberg).

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![Rust 1.95+](https://img.shields.io/badge/Rust-1.95%2B-orange.svg)](https://www.rust-lang.org)
[![CI](https://github.com/constantin-jais/wrench-loader/actions/workflows/ci.yml/badge.svg)](https://github.com/constantin-jais/wrench-loader/actions/workflows/ci.yml)

> **Status:** `0.0.0` skeleton — boundary, upstream policy, and CI gates are explicit before implementation starts.

## Why it exists

Document parsers and OCR are high-blast-radius dependencies; they belong in an isolated worker, not in the product server hot path. `wrench-loader` accepts documents, enforces resource limits, runs extraction, and returns structured output — without leaking parser internals into the calling service.

## Ecosystem

```mermaid
graph TB
    subgraph products["🎯 Rumble products"]
        RL["rumble-lm<br/>Collaborative learning platform"]
        RC["rumble-cos<br/>Public knowledge site"]
    end
    subgraph tools["🛠️ Sovereign tooling"]
        CM["cos-matic<br/>Agent config + autonomous code-ops"]
        WL["wrench-loader<br/>Document ingestion worker"]
        GM["gear-memory<br/>Local agent context"]
    end
    subgraph infra["⚙️ Infrastructure"]
        GC["gear-cable<br/>Distribution substrate"]
        GD["gear-depot<br/>Registry proxy/cache"]
        VI["vault-inspector<br/>Postgres security audit"]
    end
    RL --> WL
    RL --> GM
    RL --> VI
    RL --> GD
    RL --> GC
    RC --> RL
    CM --> GC
    WL --> GM
    style WL fill:#dbeafe,stroke:#2563eb,stroke-width:2px
```

## Contract

| Direction  | Shape                                                         |
| ---------- | ------------------------------------------------------------- |
| **Input**  | Object-store key or uploaded bytes + declared MIME type       |
| **Output** | Canonical extracted text, metadata, extraction diagnostics    |
| **Limits** | Max bytes / pages / timeouts enforced before parser execution |

## Non-goals

- No direct dependency from `presto-server` to Xberg internals
- No remote LLM / OCR providers by default
- No silent best-effort ingestion without diagnostics

## Upstream

|               |                                                                                                            |
| ------------- | ---------------------------------------------------------------------------------------------------------- |
| **Project**   | [Xberg](https://github.com/xberg-io/xberg)                                                                 |
| **Policy**    | Upstream-first, pinned releases/commits, no permanent fork                                                 |
| **Fork rule** | Only for a blocking security/build/sovereignty patch; open the upstream PR and remove the fork once merged |

## Development

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features
cargo test --workspace --all-features
```

## Related projects

| Repo                                                                  | Role                                           |
| --------------------------------------------------------------------- | ---------------------------------------------- |
| [rumble-lm](https://github.com/constantin-jais/rumble-lm)          | Primary consumer — ingestion pipeline for RAG  |
| [gear-memory](https://github.com/constantin-jais/gear-memory)         | Receives extracted text as agent context input |
| [cos-matic](https://github.com/constantin-jais/cos-matic)     | Config compiler and autonomous orchestrator    |
| [gear-cable](https://github.com/constantin-jais/gear-cable)           | Multi-platform distribution substrate          |
| [gear-depot](https://github.com/constantin-jais/gear-depot)       | Sovereign registry proxy / cache               |
| [vault-inspector](https://github.com/constantin-jais/vault-inspector) | Postgres security audit                        |

## License

MIT © Constantin Jais
