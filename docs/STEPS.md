# Steps

The next five tasks, in detail. Finished tasks move to [steps/README.md](steps/README.md) with their records.

- **Zero waste on Dan's Mac.** macOS builds, development runs and tests run locally only when necessary; CI runs the rest ([CLAUDE.md](../CLAUDE.md)).
- **The remote** is `github.com/notinferred/hyleon`. Verified work is pushed to `staging`, and CI moves `main` when its checks pass (D-038). No pull requests.

## Open

- [ ] **T-03: Model and project format v1**
  - **Needs:** T-01.
  - **Build:**
    - `units`, `model` and `store` (local folder only)
    - entities, sources and commands
    - the log, with sequence numbers claimed by create-if-absent writes
    - the projection and its rebuild
    - canonical JSON and the JSON Schema export
    - the made-up sample pump station in `samples/`, in two variants: US (NEC 2023, imperial) and international (IEC, metric) ([formats/project.md](formats/project.md))
  - **Verify:**
    - the sample loads, applies a command and writes back byte-for-byte canonical files
    - two writers racing for one sequence number: exactly one wins, and the other gets `stale`
    - deleting `model/` and rebuilding from `log/` gives identical files
    - an edit made outside Hyleon is reported
    - an agent's `issue_revision` is refused as `forbidden`
    - an agent's value is stored as `inferred`
- [ ] **T-04: Pack format v1 and the drive-bus pack**
  - **Needs:** T-03.
  - **Build:**
    - `packs`: the TOML loader, CEL through the `cel` crate, and Hyleon's bounds: parse at load, at most two levels of macro nesting, a dry run against the pack's test projects, and capped collections
    - `checks`: rules, findings with sources
    - `packs/modbus-drive-bus` and `packs/site-basics`, each with its test projects
    - `hyleon check <folder>`
    - the pack spec, open to any author (D-045): published in [formats/pack.md](formats/pack.md) with a version and a schema; every rule carries a source and at least one test project
    - `hyleon pack check <folder>`, which validates anyone's pack against the spec
  - **Verify:**
    - on the sample, a seeded duplicate address and a missing termination each produce exactly one error citing its source, and fixing them clears them
    - a rule that nests three macros is refused at load, with its file, line and column
    - a rule whose dry run fails against the test projects is refused
    - a collection over the cap yields an `info` finding instead of hanging
    - a malformed pack is refused with its line and column
    - a rule without a source, or without a test, is refused with its file and line
- [ ] **T-05: Sheet pipeline**
  - **Needs:** T-04.
  - **Build:**
    - `layout` (the bus wiring sheet)
    - `render` (SVG and symbols)
    - `sheets` (the Typst title block and revision table, at A3 and ANSI B)
    - `hyleon render`
  - **Verify:**
    - the sample's bus sheet renders at both sizes
    - the PDF passes veraPDF as PDF/A-2u (D-008)
    - rendering an unchanged model again gives the same SVG hash
    - the title block shows the project, sheet, revision and "PRELIMINARY, NOT FOR CONSTRUCTION"
- [ ] **T-18: The app on Leptos + Tauri**
  - **Needs:** T-01. It starts from the trial in `trials/tauri` ([T-02](steps/T-02.md)).
  - **Build:**
    - `crates/binaries/ui` becomes the app: a Leptos UI in a Tauri shell, with the core running inside the UI as wasm (D-043)
    - the native side does menus, files and, later, signed updates
    - the layering table gains the app's crates
  - **Verify:**
    - the gate passes, including the Leptos licences (D-044)
    - the app builds and runs on macOS (locally) and Windows (in CI), and as a web build
    - every control works by keyboard, and the accessibility tree names the sheet and its findings, as in T-02
