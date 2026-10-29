//! Rule checks for the workspace: layering, one home per outside system, and the ledger row limit.
//!
//! The rules come from docs/ARCHITECTURE.md (Layering) and CLAUDE.md (Permanent IDs). The checks
//! are pure functions; `tests/workspace.rs` runs them on the real workspace and DECISIONS.md.

/// Workspace crates by layer, innermost first. A crate may depend only on crates in an earlier layer.
pub const LAYERS: &[&[&str]] = &[
    &["hyleon-units"],
    &["hyleon-model"],
    &["hyleon-catalog", "hyleon-calc"],
    &["hyleon-packs"],
    &["hyleon-checks"],
    &["hyleon-layout"],
    &["hyleon-render"],
    &["hyleon-sheets", "hyleon-export"],
    &["hyleon-app"],
    &["hyleon-store", "hyleon-mcp"],
    &["hyleon-cli", "hyleon-ui"],
];

/// Workspace crates outside the layers. Nothing may depend on them.
pub const STANDALONE: &[&str] = &["hyleon-guardrails"];

/// Outside systems, and the one crate allowed to depend on each.
pub const HOMES: &[(&str, &str)] = &[
    ("object_store", "hyleon-store"),
    ("rmcp", "hyleon-mcp"),
    ("sqlx", "hyleon-api"),
    ("postgres", "hyleon-api"),
    ("tokio-postgres", "hyleon-api"),
];

/// Dependencies no core crate may take: async runtimes, HTTP clients and database drivers.
pub const CORE_BANNED: &[&str] = &[
    "tokio",
    "mio",
    "async-std",
    "reqwest",
    "hyper",
    "ureq",
    "sqlx",
    "postgres",
    "tokio-postgres",
    "rusqlite",
];

/// Ledger rows must stay under this many characters.
pub const LEDGER_ROW_LIMIT: usize = 500;

/// A workspace crate and the names of its direct dependencies, of every kind.
pub struct Package {
    pub name: String,
    pub is_core: bool,
    pub dependencies: Vec<String>,
}

/// The layer a workspace crate belongs to, counted from the innermost.
pub fn layer_of(name: &str) -> Option<usize> {
    LAYERS.iter().position(|layer| layer.contains(&name))
}

/// Every broken layering or home rule in the workspace, as a plain sentence.
pub fn layering_violations(packages: &[Package]) -> Vec<String> {
    let is_workspace = |name: &str| packages.iter().any(|p| p.name == name);
    let mut violations = Vec::new();

    for package in packages {
        let name = package.name.as_str();
        let layer = layer_of(name);
        if layer.is_none() && !STANDALONE.contains(&name) {
            violations.push(format!("{name} isn't in the layer table"));
        }

        for dependency in &package.dependencies {
            let dependency = dependency.as_str();
            if is_workspace(dependency) {
                if STANDALONE.contains(&dependency) {
                    violations.push(format!(
                        "{name} depends on {dependency}, which nothing may depend on"
                    ));
                } else if let (Some(own), Some(theirs)) = (layer, layer_of(dependency))
                    && theirs >= own
                {
                    violations.push(format!(
                        "{name} (layer {own}) depends on {dependency} (layer {theirs}); a crate may use only earlier layers"
                    ));
                }
                continue;
            }
            if let Some((_, home)) = HOMES.iter().find(|(system, _)| *system == dependency)
                && *home != name
            {
                violations.push(format!(
                    "{name} depends on {dependency}, which only {home} may use"
                ));
            }
            if package.is_core && CORE_BANNED.contains(&dependency) {
                violations.push(format!(
                    "core crate {name} depends on {dependency}; the core does no I/O"
                ));
            }
        }
    }
    violations
}

/// The ledger's decision rows: every line that starts a `D-` row.
pub fn ledger_rows(text: &str) -> impl Iterator<Item = &str> {
    text.lines().filter(|line| line.starts_with("| D-"))
}

/// Ledger rows at or over the limit, as (decision ID, length in characters).
pub fn long_ledger_rows(text: &str) -> Vec<(String, usize)> {
    ledger_rows(text)
        .map(|row| (row, row.chars().count()))
        .filter(|(_, length)| *length >= LEDGER_ROW_LIMIT)
        .map(|(row, length)| {
            let id = row.split('|').nth(1).unwrap_or_default().trim();
            (id.to_string(), length)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn package(name: &str, is_core: bool, dependencies: &[&str]) -> Package {
        Package {
            name: name.to_string(),
            is_core,
            dependencies: dependencies.iter().map(|d| d.to_string()).collect(),
        }
    }

    #[test]
    fn a_backwards_dependency_is_caught() {
        let packages = [
            package("hyleon-model", true, &["hyleon-store"]),
            package("hyleon-store", false, &[]),
        ];
        let violations = layering_violations(&packages);
        assert_eq!(violations.len(), 1, "{violations:?}");
        assert!(
            violations[0].starts_with("hyleon-model (layer 1) depends on hyleon-store (layer 9)")
        );
    }

    #[test]
    fn a_long_ledger_row_is_caught() {
        let long_row = format!("| D-999 | {} |", "x".repeat(600));
        let text = format!("# Decisions\n\n{long_row}\n| D-998 | short |\n");
        assert_eq!(
            long_ledger_rows(&text),
            [("D-999".to_string(), long_row.chars().count())]
        );
    }
}
