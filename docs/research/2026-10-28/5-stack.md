# Appendix 5: Stack check (agent report, 2026-10-28)

The research agent's full report, verbatim. It's never edited. The synthesis is in [../2026-10-28-market.md](../2026-10-28-market.md).

**The brief:** for each proposed library, report its version, licence, maintenance health, fit for Hyleon and the gotchas to expect. The libraries are Dioxus 0.7 (with Leptos + Tauri as the fallback), rmcp elicitation, Typst as a library, cel, uom, object_store, sqlx, WorkOS, the output crates, and graph layout. Nothing was installed, built or run.

---

I checked all ten items. Three findings change the plan:

- **MCP elicitation works differently now.** The MCP spec version 2026-07-28 removes server-initiated requests. rmcp 3.x supports it, and current Claude Code uses it with stdio servers that support it. So the CLI's questions have to use the new "multi round-trip requests" pattern (MRTR) instead of `peer.elicit()`.
- **Typst removed `pdf.embed` in 0.15.** It is now `pdf.attach`.
- **Dioxus has no desktop auto-updater, and elkjs is EPL-2.0 (copyleft).**

The session's web-search quota ran out partway through. After that I used direct fetches of crates.io, GitHub and official docs. Nothing was installed, built or run.

## 1. Summary

| Item | Version (date) | License | Health | Fit |
|---|---|---|---|---|
| Dioxus | 0.7.10 (2026-07-30); 0.8.0-alpha.3 (2026-10-16) | MIT OR Apache-2.0 | Small company team; recent commits mostly one engineer; 0.7 now gets fixes only | ok |
| Leptos + Tauri (fallback) | 0.8.22 (2026-10-05) + 2.12.5 (2026-10-21) | MIT; Apache-2.0 OR MIT | Leptos has one lead maintainer; Tauri has a large team; Leptos 0.9-rc0 and Tauri 3-alpha are out | good |
| rmcp | 3.8.0 (2026-10-26) | Apache-2.0 | Very active; three major versions in 2026: 1.0 (Mar), 2.0 (Jun), 3.0 (Jul) | ok |
| typst, typst-pdf | 0.15.1 (2026-07-17) | Apache-2.0 | Typst GmbH; about 2 minor releases a year | good |
| cel (renamed from cel-interpreter) | 0.15.0 (2026-10-06) | MIT | Active; one lead maintainer merges everything | ok |
| regorus | 0.14.0 (2026-10-27) | MIT AND Apache-2.0 AND BSD-3-Clause | Microsoft; frequent releases | risky for non-programmers |
| datalogic-rs (JsonLogic) | 5.9.0 (2026-10-22) | Apache-2.0 | Weekly releases | ok |
| uom | 0.38.0 (2026-02-14) | Apache-2.0 OR MIT | One maintainer; last commit 2026-04-04 | ok |
| object_store | 0.14.2 (2026-09-15) | MIT/Apache-2.0 | Apache Arrow project | good native, risky wasm |
| sqlx | 0.9.0 (2026-05-21) | MIT OR Apache-2.0 | Many contributors; repo moved to transact-rs | good |
| WorkOS (`workos` Rust SDK) | 3.5.0 (2026-10-19) | MIT (SDK) | Official SDK, but 18 GitHub stars | good |
| dxf | 0.6.2 (2026-10-03) | MIT | One maintainer, slow | ok |
| rust_xlsxwriter | 0.101.0 (2026-10-20) | MIT OR Apache-2.0 | One very active maintainer | good |
| docx-rs | 0.4.22 (2026-07-21) | MIT | Owner plus contributors; 61 open issues | ok |
| schemars | 1.2.3 (2026-10-12) | MIT | One maintainer, steady | good |
| elkjs | 0.12.1 (2026-10-28) | **EPL-2.0 OR GPL-3.0-or-later (copyleft)** | Kiel University team | ok |
| Rust layered-layout crates | rust-sugiyama 0.4.0, layout-rs 0.1.3, dagre 0.1.1 | unknown | Small, low use | risky |

## 2. Gotchas

**1. Dioxus 0.7**
- **One codebase:** web runs as wasm in the browser. Desktop runs native Rust that drives the OS webview (WebView2, WKWebView, WebKitGTK).
- **Files on desktop:** plain `std::fs` works. `dioxus-desktop` already depends on rfd 0.17.2, which on Linux needs an xdg-desktop-portal service installed. rfd has no iOS or Android backend.
- **`dx bundle`:** makes .app/.dmg, .msi/.exe, .deb/.rpm/.AppImage, iOS .app and .apk. It builds only for the machine's own OS and architecture. The docs say signing and notarization aren't built in.
- **Auto-update:** there is none. The options are:
  - Velopack (MIT; its docs list Rust as "Ready")
  - self_update 1.3.3 (replaces the binary)
  - cargo-packager-updater (no release since July 2025)
  - Tauri's updater plugin only works with Tauri.
- **SVG with about 5,000 elements:** I found no Dioxus benchmark. The webview does the painting. Dioxus pays for diffing and for the stream of edits and events between Rust and the webview.
  - Static sheets can go in as one SVG string (`dangerous_inner_html`).
  - In the editor, give each node its own signal.
  - Test on WebKitGTK first, the weakest of the three webviews. NVIDIA problems are tracked in tauri#9394.
- **Accessibility:** the app is a real DOM, so screen readers see the webview's accessibility tree, nothing more. Dioxus Primitives follow WAI-ARIA; Dioxus Native's accessibility is still work in progress. The SVG sheets and the drag-and-connect editor will need hand-built labels and keyboard alternatives.
- **Mobile:** the docs call it "first-class" (webview based). Native APIs such as camera are on the 0.8 roadmap, and 0.8-alpha.3 still fixes a blank Android webview.
- **API stability:**
  - 0.6 to 0.7 breaks were moderate: forms submit by default, server functions use a JSON codec, and the asset options API changed.
  - 0.8 alphas break more: element names are no longer namespaced, keys become `Key::Named(NamedKey::Enter)`, libxdo is removed, Windows 7 is dropped, and the 0.8 CLI can't run apps older than 0.7.2.
  - No date for 0.8 stable.
  - The CLI sends telemetry by default.
- **Production users:** the homepage shows Airbus, ESA, Cognition, YC and Futurewei logos. I found no case studies.
- **Leptos + Tauri:**
  - Gains: a signed updater (NSIS/MSI, .app.tar.gz, AppImage), dialog and file plugins with permission scopes, and stable iOS/Android targets.
  - Cost: the UI runs as wasm and calls a native backend over IPC, so there are two layers.
  - Same webviews, so the same accessibility and WebKitGTK caveats.
  - Leptos 0.9-rc0 is described as "mostly compatible" with 0.8.

**2. rmcp, MCP and Claude Code**
- **The spec change:** rmcp implements MCP 2026-07-28 and stays compatible with 2025-11-25. In 2026-07-28 a server can no longer send its own requests to the client. Elicitation must use MRTR:
  - The tool returns `InputRequiredResult`, carrying `inputRequests` and `requestState`.
  - The client asks the user, then repeats the tool call with `inputResponses`.
  - `requestState` must be treated as attacker input. rmcp's `request-state` feature provides an HMAC-signed codec.
- **rmcp API shape:** `ctx.peer.elicit::<T>()` (with `elicit_safe!`) and `elicit_url` are the old server-initiated style. The README doesn't say what happens if you call them on a 2026-07-28 connection.
  - The `elicitation` feature only adds the `url` dependency.
  - Enum fields need `#[schemars(inline)]` and `extend("type"="string")`.
- **What Claude Code negotiates:** its v2 runtime picks 2026-07-28 with stdio servers that support it, from v2.1.232 or v2.1.274 depending on the session type. Setting `MCP_PROTOCOL_NEGOTIATION=legacy` forces the old handshake. Expect rmcp 3.x with current Claude Code to use MRTR.
- **When Claude Code added it:** Anthropic's CHANGELOG confirms 2.1.76 (GitHub release dated 14 March). The entry reads "Added MCP elicitation support … form fields or browser URL", plus `Elicitation` and `ElicitationResult` hooks.
  - Form and URL modes on older-protocol stdio connections need 2.1.287 or later.
  - The current version is 2.1.312.
- **Field limits:** Anthropic documents none beyond the spec.
  - The spec allows a flat object of strings (email, uri, date, date-time), numbers or integers, booleans, and single- or multi-select enums. No nested objects or arrays of objects.
  - URL mode is capped at about 8,000 characters, or about 4,000 if the URL is heavily percent-escaped.
  - Headless runs can answer through the `Elicitation` hook.
  - For MCP tasks, elicitation only works in interactive sessions, with at most 16 input keys per task.
- **Tool schemas via schemars:** the root must be an object. Doc comments on the type are dropped; doc comments on fields become descriptions.
- **Stdio:** stdio needs the `transport-io` feature, and stdout carries only protocol messages, so logs go to stderr.

**3. Typst**
- **Glue code:** the `World` trait has 7 required methods, and the World must do its own caching because the compiler doesn't.
  - typst-kit was reworked in 0.15 to make this easier; all its features are off by default.
  - My estimate is about 100 to 200 lines. `typst-as-lib` 0.17.0 (MIT) wraps it.
- **PDF/A:** supports A-1b, 1a, 2b, 2u, 2a, 3b, 3u, 3a, 4, 4f, 4e, plus PDF/UA-1.
  - Attachments are allowed only with A-3 or A-4f.
  - A-1b forbids transparency.
- **`pdf.embed` was removed in 0.15;** use `pdf.attach`.
- **Fonts:** Typst only sees fonts the World supplies, so bundle them. In wasm there are no system fonts.
- **Speed:** there is no official figure. A third-party Flutter wrapper (typst_flutter) claims "most documents under 100 ms". The first compile is dominated by font loading, so keep the World alive between renders.
- **wasm32:** the compiler runs in browsers (typst.ts), but Typst's own CI doesn't build for wasm32. I couldn't verify PDF export in wasm.
- **Upgrades:** every minor release breaks something; 0.15 also bans backslashes in paths.

**4. Rules engines**
- **The rename:** `cel-interpreter` stopped at 0.10.0 (2025-07-23); the same project continues as `cel` from 0.10.0 (2025-07-25).
- **Spec coverage:** 0.15 passes 1,688 of 2,507 conformance tests (67%).
  - Fully passing: the macros `has`, `all`, `exists`, `exists_one`, `map` and `filter`; strings; lists; and the strings extension.
  - Missing: the type checker, protobuf messages, two-variable comprehensions, and `cel.bind`/`cel.block`.
- **Custom functions:** `add_function`, `add_overload` and `add_member_overload`. Custom macros and types arrived in 0.15.
- **Sandboxing and termination:** CEL does no I/O except through functions you add, and the language has no unbounded loops.
  - There is **no execution budget**: PR #309, which proposed one, was closed unmerged.
  - Issue #340 reported quadratic `map` evaluation; it was closed as a duplicate.
  - With no type checker, mistakes only show up when an expression is evaluated.
  - `Context` has not been `Send` since 0.13.
- **Performance:** no published benchmarks.
- **regorus:** mostly compliant with OPA v1.2.0 and works in no_std and wasm. But Rego is a logic-programming language, which is hard for non-programmers.
- **JsonLogic (datalogic-rs):** rules are JSON trees. That suits a visual rule builder but is unreadable when written by hand. The older `jsonlogic-rs` has had no release since December 2024.

**5. uom**
- **Ergonomics:** quantities are written as `Length::new::<meter>(x)`, and type errors are long.
- **serde:** only the number in base units is written, so the unit is lost. uom also has no schemars `JsonSchema` implementation.
- **Compile time:** code is generated for each enabled number type. The defaults enable both f32 and f64, so set `default-features = false, features = ["f64","si","std"]`. I found no tracked compile-time issue.
- **wasm32:** works, as does no_std.
- **Alternatives:** diman 0.6.0 needs nightly Rust; dimensioned 0.8.0 has had no release since 2022; or plain newtypes.

**6. object_store**
- **Local folder vs R2:** local writes are atomic, but `PutMode::Update` is rejected there. R2 supports conditional writes (If-Match and If-None-Match), so concurrency behaves differently locally and on R2.
- **R2:** use the S3 builder with your endpoint and region `auto`. Presigned URLs come through the `Signer` trait.
- **wasm32:** no local filesystem. Use the `*-base` features with `ring`.
  - Issue #624 (open): a crash on wasm from `Instant::now`.
  - Issue #26 (open): wasm has no CI.
  - A browser also needs CORS and presigned URLs, never keys.

**7. sqlx**
- **Offline mode for CI:**
  - Run `cargo sqlx prepare --workspace` and commit `.sqlx/`.
  - In CI, set `SQLX_OFFLINE=true` and run `cargo sqlx prepare --check`.
- **0.9 breaking changes:**
  - `query()` now needs a `SqlSafeStr`; wrap dynamic SQL in `AssertSqlSafe`.
  - The `Migrate` trait changed.
  - The combined runtime+TLS features were removed.
  - Config can live in a new `sqlx.toml`.
  - `cargo install --locked sqlx-cli` no longer works.
- **Migrations:** `sqlx migrate add -r` creates reversible migrations. `migrate!` embeds the files; add sqlx's build script so new files trigger a rebuild.

**8. WorkOS**
- **AuthKit:** free up to 1M monthly active users, then $2,500 a month per extra 1M.
- **SSO and Directory Sync (SCIM):** priced per connection per month — $125 each for 1–15, $100 for 16–30, $80 for 31–50, $65 for 51–100. A custom domain is $99 a month.
- **JWT verification in Rust:**
  - Fetch the keys from `https://api.workos.com/sso/jwks/<client_id>`.
  - Verify with `jsonwebtoken` 11.1.0, checking `exp`, `iss` (`https://api.workos.com/` or your custom domain) and `client_id`.
  - The docs don't name the signing algorithm, so take it from the key set and pin it.
- **Official SDK:** the `workos` crate fetches and caches the keys and handles sealed sessions, but pins `jsonwebtoken = 10.3.0`.

**9. Output crates**
- **dxf:** writes R10 through R2018 with 45 entity types, but **no HATCH**. Issue #25 has been open since 2022 and PR #102 is stalled.
- **rust_xlsxwriter:** write-only (can't read or modify files); has a `wasm` feature.
- **docx-rs:** its README says "support is not exhaustive", and it is still 0.4.x.
- **schemars 1.x:** emits JSON Schema 2020-12 by default. Named types go into `$defs` with `$ref`, so inline them for elicitation forms.

**10. Graph layout**
- **Rust crates:** none offers ELK-style ports plus orthogonal edge routing. The candidates are tiny or new.
- **Practical route:** run elkjs inside the webview, since web and desktop both have JavaScript. The CLI would need an embedded JS engine or Node, which I haven't tested. I found no WASM build of ELK on npm.
- **License:** choose the EPL-2.0 option, which is weak, file-level copyleft.
  - Keep its notice and tell recipients where to get the elkjs source.
  - Any changes to elkjs's own files must be published under EPL.
  - Your code that calls it is not affected.
- **Version:** 0.13.0 was published today and already deprecated, so pin 0.12.1.

## 3. Recommendations and red flags

These are for you to decide.

1. **Red flag: the elicitation design.** Two options:
   - (a) Write tools in the MRTR style: the handler is re-entered and its state is signed. This is future-proof.
   - (b) Advertise only the 2025-11-25 protocol and use `peer.elicit()`. Simpler now, but deprecated.
   - I'd lean (a).
2. **Red flag: rmcp changes fast.** Pin exact versions and keep the MCP glue in one thin crate.
3. **Dioxus vs Leptos + Tauri:** Dioxus is workable if you accept a 0.8 migration, add Velopack for updates, and a 5,000-element SVG test passes on WebKitGTK. Leptos + Tauri is the better choice if signed updates and mobile maturity matter more than one codebase.
4. **Typst:** use `pdf.attach` with PDF/A-3u (or A-3b) when you attach data, and A-2u otherwise. Update anything that mentions `pdf.embed`.
5. **Rules language: I recommend CEL (`cel`).** It reads like ordinary conditions and always terminates. Add your own checks, because it has no type checker and no execution budget:
   - parse every expression;
   - dry-run it against test data;
   - cap input sizes.
   - Choose JsonLogic only if authors will always use a visual builder. Skip Rego.
6. **uom:** keep it inside the core math. Where values cross serde, MCP or storage, use types that name the unit explicitly.
7. **object_store:** don't rely on `PutMode::Update` with the local folder, and keep storage code out of wasm.
8. **dxf:** check whether the drawing sheets need hatched fills.
9. **Licenses:**
   - elkjs is EPL-2.0.
   - If a Linux AppImage bundles WebKitGTK (LGPL-2.1), include its notice and a source offer.
   - `cargo deny check licenses` would catch MPL-2.0 dependencies pulled in indirectly, such as `option-ext`. It resolves dependencies, so I'd ask before running it on this computer.
10. **Benchmarks** for Typst compile time and SVG rendering need a build. Ask before running them here.

## 4. Sources

- **Dioxus:**
  - [crates.io API](https://crates.io/api/v1/crates/dioxus)
  - [releases](https://github.com/DioxusLabs/dioxus/releases)
  - [0.7 blog](https://dioxuslabs.com/blog/release-070/)
  - [bundle](https://dioxuslabs.com/learn/0.7/tutorial/bundle/llms.txt)
  - [mobile](https://dioxuslabs.com/learn/0.7/guides/platforms/mobile)
  - [migration](https://dioxuslabs.com/learn/0.7/migration/to_07)
  - [desktop deps](https://docs.rs/crate/dioxus-desktop/0.7.10/source/Cargo.toml.orig)
  - [components](https://github.com/DioxusLabs/components)
  - [homepage](https://dioxuslabs.com/)
  - [rfd](https://github.com/PolyMeilex/rfd)
  - [tauri#9394](https://github.com/tauri-apps/tauri/issues/9394)
  - [Velopack](https://docs.velopack.io/)
  - [self_update](https://crates.io/crates/self_update)
- **Fallback:**
  - [Tauri updater](https://v2.tauri.app/plugin/updater/)
  - [Leptos releases](https://github.com/leptos-rs/leptos/releases)
- **MCP:**
  - [rust-sdk](https://github.com/modelcontextprotocol/rust-sdk)
  - [3.x migration](https://github.com/modelcontextprotocol/rust-sdk/discussions/969)
  - [features](https://docs.rs/crate/rmcp/latest/features)
  - [InputRequiredResult](https://docs.rs/rmcp/latest/rmcp/model/struct.InputRequiredResult.html)
  - [spec changelog](https://modelcontextprotocol.io/specification/2026-07-28/changelog)
  - [elicitation](https://modelcontextprotocol.io/specification/2026-07-28/client/elicitation)
  - [MRTR](https://modelcontextprotocol.io/specification/2026-07-28/basic/patterns/mrtr)
- **Claude Code:**
  - [CHANGELOG](https://github.com/anthropics/claude-code/blob/main/CHANGELOG.md)
  - [v2.1.76](https://github.com/anthropics/claude-code/releases/tag/v2.1.76)
  - [MCP docs](https://code.claude.com/docs/en/mcp)
  - [hooks](https://code.claude.com/docs/en/hooks)
- **Typst:**
  - [PDF](https://typst.app/docs/reference/pdf/)
  - [0.15 changelog](https://typst.app/docs/changelog/0.15.0/)
  - [World](https://docs.rs/typst/latest/typst/trait.World.html)
  - [typst-kit](https://docs.rs/typst-kit/0.15.1/typst_kit/)
  - [CI](https://github.com/typst/typst/blob/main/.github/workflows/ci.yml)
  - [typst.ts](https://github.com/Myriad-Dreamin/typst.ts)
  - [typst_flutter](https://pub.dev/packages/typst_flutter/versions/2.2.0)
- **Rules:**
  - [cel-rust](https://github.com/cel-rust/cel-rust)
  - [v0.15.0](https://github.com/cel-rust/cel-rust/releases/tag/v0.15.0)
  - [#309](https://github.com/cel-rust/cel-rust/pull/309)
  - [#340](https://github.com/cel-rust/cel-rust/issues/340)
  - [cel versions](https://crates.io/api/v1/crates/cel/versions)
  - [regorus](https://github.com/microsoft/regorus)
  - [datalogic-rs](https://crates.io/crates/datalogic-rs)
  - [jsonlogic-rs](https://crates.io/crates/jsonlogic-rs)
- **Units:**
  - [uom](https://github.com/iliekturtles/uom)
  - [serde impl](https://github.com/iliekturtles/uom/blob/master/src/system.rs)
  - [diman](https://docs.rs/diman)
- **Storage:**
  - [object_store](https://docs.rs/object_store/latest/object_store/)
  - [local.rs](https://github.com/apache/arrow-rs-object-store/blob/main/src/local.rs)
  - [#624](https://github.com/apache/arrow-rs-object-store/issues/624)
  - [#26](https://github.com/apache/arrow-rs-object-store/issues/26)
  - [R2 S3 API](https://developers.cloudflare.com/r2/api/s3/api/)
- **sqlx:**
  - [CHANGELOG](https://github.com/launchbadge/sqlx/blob/main/CHANGELOG.md)
  - [CLI](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)
- **WorkOS:**
  - [pricing](https://workos.com/pricing)
  - [sessions](https://workos.com/docs/user-management/sessions)
  - [tokens](https://workos.com/docs/reference/authkit/session-tokens)
  - [workos-rust](https://github.com/workos/workos-rust)
- **Outputs:**
  - [dxf entities](https://docs.rs/dxf/latest/dxf/entities/enum.EntityType.html)
  - [dxf#25](https://github.com/ixmilia/dxf-rs/issues/25)
  - [rust_xlsxwriter](https://github.com/jmcnamara/rust_xlsxwriter)
  - [docx-rs](https://github.com/bokuweb/docx-rs)
  - [schemars](https://docs.rs/schemars/latest/schemars/)
- **Layout:**
  - [elkjs](https://github.com/kieler/elkjs)
  - [elkjs license](https://github.com/kieler/elkjs/blob/master/LICENSE.md)
  - [npm](https://registry.npmjs.org/elkjs)
  - [crates search](https://crates.io/search?q=sugiyama)
