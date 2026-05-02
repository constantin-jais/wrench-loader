//! wrench-loader — Sovereign rich-document ingestion worker/service backed by Xberg.
//!
//! This crate is intentionally a minimal skeleton. The first implementation
//! increments must keep the upstream boundary explicit and preserve the
//! sovereign constraints documented in `docs/adr/0001-scope-and-upstream-policy.md`.

/// Static project metadata used by the CLI and smoke tests.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProjectCard {
    pub name: &'static str,
    pub role: &'static str,
    pub upstream: &'static str,
    pub relationship: &'static str,
}

/// The repository's initial scope card.
pub const PROJECT: ProjectCard = ProjectCard {
    name: "wrench-loader",
    role: "rich document ingestion",
    upstream: "Xberg",
    relationship: "External ingestion worker/service for Presto-Matic; integrates by queue/HTTP/object-store contract.",
};

/// Human-readable summary for CLI smoke runs.
pub fn summary() -> String {
    format!(
        "{} — {} (upstream: {})",
        PROJECT.name, PROJECT.role, PROJECT.upstream
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn project_card_names_the_repo_and_upstream() {
        assert_eq!(PROJECT.name, "wrench-loader");
        assert_eq!(PROJECT.upstream, "Xberg");
        assert!(summary().contains(PROJECT.role));
    }
}
