//! The rule checks, run on the real workspace and the real ledger.

use std::path::Path;

use cargo_metadata::MetadataCommand;
use hyleon_guardrails::{LAYERS, Package, layering_violations, ledger_rows, long_ledger_rows};

fn workspace_root() -> &'static Path {
    Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../.."))
}

fn workspace_packages() -> Vec<Package> {
    let metadata = MetadataCommand::new()
        .manifest_path(workspace_root().join("Cargo.toml"))
        .no_deps()
        .exec()
        .expect("cargo metadata runs");
    let core = metadata.workspace_root.join("crates").join("core");
    metadata
        .workspace_packages()
        .into_iter()
        .map(|package| Package {
            name: package.name.to_string(),
            is_core: package.manifest_path.starts_with(&core),
            dependencies: package
                .dependencies
                .iter()
                .map(|d| d.name.clone())
                .collect(),
        })
        .collect()
}

#[test]
fn layering_holds() {
    let violations = layering_violations(&workspace_packages());
    assert!(
        violations.is_empty(),
        "layering broken:\n{}",
        violations.join("\n")
    );
}

#[test]
fn every_crate_in_the_layer_table_exists() {
    let packages = workspace_packages();
    for name in LAYERS.iter().flat_map(|layer| layer.iter()) {
        assert!(
            packages.iter().any(|package| package.name == *name),
            "{name} is in the layer table but not in the workspace"
        );
    }
}

#[test]
fn ledger_rows_stay_under_the_limit() {
    let ledger =
        std::fs::read_to_string(workspace_root().join("DECISIONS.md")).expect("DECISIONS.md reads");
    let rows = ledger_rows(&ledger).count();
    assert!(
        rows >= 36,
        "found only {rows} ledger rows; the row format may have changed"
    );
    let long = long_ledger_rows(&ledger);
    assert!(
        long.is_empty(),
        "ledger rows at or over 500 characters: {long:?}"
    );
}
