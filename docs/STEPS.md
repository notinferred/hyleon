# Steps

The next five tasks, in detail. Finished tasks move to [steps/README.md](steps/README.md) with their records.

- **Zero waste on Dan's Mac.** macOS builds, development runs and tests run locally only when necessary; CI runs the rest ([CLAUDE.md](../CLAUDE.md)).
- **The remote** is `github.com/notinferred/hyleon`. Verified work is pushed to `main`, with no pull requests.

## Open

- [ ] **T-02: Dioxus trial**
  - **Needs:** T-01.
  - **Build:** one screen, built for web and macOS (locally when necessary) and Windows (in CI), with the Dioxus CLI's telemetry switched off. Linux comes later, and the code stays portable:
    - a question form generated from a JSON Schema
    - a findings list
    - a 5,000-element SVG sheet loaded from a fixed file, with accessible labels
    - native menus and Cmd/Ctrl shortcuts
    - a Velopack update from one test build to the next
  - **Verify:**
    - the web, macOS and Windows builds all run
    - every control works by keyboard alone
    - a screen reader announces the sheet's title and its findings
    - the Velopack update installs and relaunches
    - Dan, when he chooses, opens the macOS bundle and confirms the sheet pans and zooms smoothly
    - the result is recorded: confirm D-003, or switch to the Leptos + Tauri fallback
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
  - **Verify:**
    - on the sample, a seeded duplicate address and a missing termination each produce exactly one error citing its source, and fixing them clears them
    - a rule that nests three macros is refused at load, with its file, line and column
    - a rule whose dry run fails against the test projects is refused
    - a collection over the cap yields an `info` finding instead of hanging
    - a malformed pack is refused with its line and column
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
