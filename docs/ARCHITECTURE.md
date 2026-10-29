# Architecture

The code map and the rules that shape it. **Status: planned.** No code exists yet, so everything here describes the intended structure until a step builds it. Why each choice was made is in [DECISIONS.md](../DECISIONS.md); the file and protocol contracts are in [formats/](formats/).

## Shape

- A Rust workspace (D-002). The core crates do no I/O, so the same code runs in the desktop app, the browser (wasm32), the CLI and its MCP server, and later the API.
- The app's UI is Dioxus, for web and desktop, with tablets later (D-003). The website is Astro with Starlight, in `site/`, outside the Cargo workspace.

```
crates/
  core (no I/O; builds for wasm32-unknown-unknown)
    units      typed quantities (uom) inside the math; explicit-unit values at every boundary; conductor-size tables
    model      entities, IDs, tags, ports, links, sources, commands; JSON Schema (schemars)
    catalog    parts: ports, ratings, dimensions, settings schemas, compatibility, sources (data)
    calc       engineering calculations and their tables (electrical, piping, network/bus)
    packs      pack, region-profile and owner-profile loading and validation, pack layers, question state, CEL conditions and rules (the `cel` crate, with Hyleon's own bounds)
    checks     runs rules, calculations and cross-references; produces findings
    layout     places elements on a sheet, per sheet type (deterministic, in Rust)
    render     SVG output and the symbol library
    sheets     Typst templates (title block, revision table, ISO and ANSI sizes) → PDF/A-2u; PDF/A-3u with the model attached later
    export     XLSX (rust_xlsxwriter), CSV and DXF (dxf) writers (bytes out)
    app        use cases behind ports: open, apply, answer, confirm, accept, check, issue, export
    sim        later (phase 6)
  adapters (I/O)
    store      the project folder: log, projection, revisions (object_store: local now, R2 later)
    mcp        the MCP server (rmcp); maps tools to app use cases; elicitation
    dwg        DWG writing through the Open Design Alliance library (phase 3; licensed, native only, never in wasm32)
  binaries
    cli        the `hyleon` binary: check, render, export, mcp
    ui         the Dioxus app (web, desktop)
    api        later (phase 3): accounts, sync, sharing, billing
site/        Astro + Starlight; a page per pack, generated from the pack files
packs/       first-party packs (data)
regions/     region profiles (data), kept current by the watcher (phase 4)
owners/      owner profiles (data): clients' drawing standards
samples/     the made-up sample project
```

## Layering

- **Direction.** Dependencies point inward. The order is: units → model → catalog, calc → packs → checks → layout → render → sheets, export → app → adapters → binaries.
  - A crate uses only crates earlier in that order.
  - Adapters implement the ports declared by `app`.
  - Binaries wire adapters to `app`. Nothing depends on a binary.
- **No I/O in the core.** Core crates never use `std::fs`, `std::net`, `std::process`, `tokio`, an HTTP client or a database driver. They receive time, IDs, files and fonts through ports or as bytes, and they must build for wasm32.
- **One home per outside system.** Each one is used from a single place:
  - files and object storage: only `store`
  - the MCP SDK: only `mcp`
  - the DWG library: only `dwg`
  - Postgres, Stripe and the auth provider: only `api`
- **No AI SDKs.** No crate depends on a language model's SDK. Hyleon never calls a model: people bring their own agent (TRUTHS.md, truths 7 and 8).
- **Enforcement (from T-01):**
  - a layering test that reads `cargo metadata` and fails on a backwards dependency, a banned dependency outside its home crate, or core I/O
  - cargo-deny: licences, advisories, banned crates, sources
  - workspace lints, including `unsafe_code = "forbid"`
  - a wasm32 build of the core in CI

## Runtime

- **Desktop.** One Dioxus process per open project: UI, core and `store`.
- **Agent.** The agent's client starts `hyleon mcp` as its own process, which writes to the same project folder. That makes two writers, so writes are optimistic and fenced by sequence number ([formats/project.md](formats/project.md); D-010).
- **Desktop and agent together.** The app watches the folder and reloads when the sequence advances.
- **Web (phase 3).** Dioxus compiled to wasm32 in the browser, with the API for accounts, sync and sharing.
- **CLI in CI (later).** `hyleon check` can gate a project in a customer's own CI, the way tests gate code ([BACKLOG.md](BACKLOG.md)).

## Trust boundaries

Everything crossing inward is hostile until parsed. It is parsed once, at the edge, into typed values.

| Boundary | What crosses | Rule |
|---|---|---|
| Project folder | Files that may come from anyone | Parsed against the format version and schema on load. Edits made outside Hyleon are detected, not trusted ([formats/project.md](formats/project.md)) |
| Packs | Third-party or company rule files | Parsed and validated. CEL can't do I/O or loop, and Hyleon bounds every rule (S-04) |
| Region profiles | Facts about what's in force where, from Hyleon and later from the watcher | Parsed and locked by sha256. A watcher's proposal stays unverified until a person releases it (S-10) |
| Catalog data, imported files | Ratings, text, drawings | Parsed as data. Text is never treated as instructions (S-03) |
| MCP | An agent's tool calls | Parsed per tool schema. The agent's identity is recorded, and person-only actions are refused (S-01, S-02) |
| Elicitation answers | A person's answers via the agent's client | Recorded as the person's, via elicitation. Issuing still requires the app (S-02) |
| API (phase 3) | Requests from the internet | Authentication by token, then authorization per request and per resource. Deny by default |

## Engineering invariants

These are Dan's invariants (architecture-invariants and ENGINEERING-CONCEPTS), each bound to how Hyleon will keep it, or marked dormant with the change that switches it on. Status is "planned" throughout until code exists.

| Invariant | Status | How it's kept, or what switches it on |
|---|---|---|
| Parse, don't validate | planned | serde into newtypes at every edge; schemars schemas as the contracts |
| Single-owner schema evolution | planned | The project format version and its migrators live only in `store`; the server's DB later uses sqlx migrations, applied by one actor |
| Idempotent writes | planned | Every command carries an ID; applying the same ID twice is a no-op |
| Append, then project | planned | The command log is append-only; the model files are a rebuildable projection |
| Immutable blob tier | planned | Issued revisions and media are content-addressed (sha256) and never rewritten |
| Atomic claim and fencing | planned | Each command claims the next sequence number with a create-if-absent write. A stale writer is refused |
| No wall-clock ordering | planned | Order comes from sequence numbers; wall-clock time is for display only |
| Canonical units, meaningless IDs | planned | SI at rest (uom); UUIDv7 identities; tags are names, not keys |
| Correlated structured events | planned | A tracing span per command, carrying the command ID and the actor |
| Real dependencies in tests | planned | Real Typst, real files, veraPDF for PDF/A; testcontainers Postgres from phase 3 |
| Decision records | holds | [DECISIONS.md](../DECISIONS.md) |
| Second machine that writes | **active** | The app and the agent write the same project: fencing as above |
| Resilience (timeouts, retries, breakers) | dormant | Switches on with the first remote call (phase 3): tower middleware on every outbound call |
| Admission control, rate limits | dormant | Switches on with the API: tower concurrency and rate limits |
| Health: alive, ready, fresh | dormant | Switches on with the API |
| Absence detection | dormant | Switches on with the region watcher (phase 4): an alarm when a daily run is missed, since silence must never read as "no change" |
| Runtime off-switches | dormant | Switches on with cloud features |
| Secrets and identity | dormant | Phase 3: the licence or session token lives in the OS keychain, and project files hold no secrets |
| Jobs, workflows, brokers, outbox, CDC | dormant | Switches on with server-side rendering or sync, when measured need exists |
| Public write API rules | dormant | Phase 7: idempotency keys, cursor pagination, versioning, signed webhooks |
| Second region, second team | dormant | Enterprise, on measured need |

## Platform leaks to plan for

Most of these were found by the stack check in [research/2026-10-28-market.md](research/2026-10-28-market.md).

- **Dioxus is pre-1.0.**
  - 0.7 gets fixes only. 0.8, now in alpha, renames elements and keys and drops Windows 7.
  - The UI is the outermost layer, so a migration never touches the core. Exact pins; the T-02 trial comes first.
  - It has no desktop updater, so updates go through Velopack (D-003).
  - `dx bundle` builds only for the machine it runs on, so CI needs one runner per operating system.
  - Its CLI sends telemetry by default; it's switched off in CI and in development.
- **Webviews differ.**
  - Desktop renders through the operating system's webview: WebKit on macOS, WebView2 on Windows, WebKitGTK on Linux.
  - Linux (WebKitGTK, the weakest webview) comes later. The code stays portable, and large sheets are tested there before Linux ships.
  - Sheets and SVG are checked on every platform before release.
- **Accessibility.** Screen readers see only the webview's accessibility tree. SVG sheets and the topology editor need their own labels and keyboard paths.
- **wasm32 has no threads, files or clock by default.**
  - The core receives time, IDs and fonts through ports, and fonts are embedded as bytes.
  - `object_store` stays out of wasm32: the web app reaches storage through the API.
- **Storage semantics differ.** The local filesystem and R2 behave differently for conditional writes. Hyleon relies only on create-if-absent, which both support, and never on update-if-unchanged.
- **MCP changes fast.**
  - rmcp had three major versions in 2026, and MCP 2026-07-28 replaced server-initiated elicitation with multi round-trip requests.
  - All MCP code lives in the thin `mcp` crate, with exact pins.
  - Clients that can't elicit leave the question open in the app ([formats/mcp.md](formats/mcp.md)).
- **CEL has no execution budget and no type checker.** Hyleon bounds rules itself when a pack loads and when a rule runs ([formats/pack.md](formats/pack.md)).
- **uom loses the unit when serialized,** and has no JSON Schema support. uom stays inside the math, and every value that crosses a boundary carries its unit explicitly.
- **Typst:**
  - every minor release breaks something, so it gets exact pins
  - its `World` needs about 100–200 lines of glue and its own caching
  - fonts must be bundled
  - compile errors are mapped to findings, never shown raw
  - attachments are `pdf.attach`, allowed only in PDF/A-3 or A-4f
- **PDF/A conformance is easy to break.** Every issued PDF is validated with veraPDF in CI.
- **DXF has gaps.** The `dxf` crate can't write HATCH entities, so filled areas export as outlines.
- **DWG is licensed and native-only.**
  - The Open Design Alliance library is C++, under a paid licence whose fees aren't known yet.
  - It lives in the `dwg` adapter, never in the core.
  - The web app gets DWG through the desktop app or the API.
- **Graph layout.**
  - No Rust crate matches ELK's ports and orthogonal routing.
  - The phase 2 topology view runs elkjs in the webview, unmodified (EPL-2.0, D-016).
  - Sheets use the deterministic Rust layouts (D-009).
