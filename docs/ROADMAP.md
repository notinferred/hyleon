# Roadmap

From foundations to ecosystem, end to end. Each phase has a goal, its scope, and **exit checks**. An exit check could fail if the work were wrong. A phase is done only when its exit checks pass, and the next five tasks live in [STEPS.md](STEPS.md). Dan re-plans this file.

The order of content follows D-021: every industry is supported, starting with the shared base packs.

## Phase 0: Foundations

**Goal.** The structures exist, and the stack is proven on one screen.

**Scope:**
- these documents (T-00, T-06, T-07)
- **Feedback by email, optional.** Hyleon doesn't depend on interviews (Dan, 2026-10-28). Integrators or panel shops may be emailed for feedback, but nothing waits on it.
- workspace guardrails and CI (T-01)
- the Dioxus trial (T-02)
- the model and project format (T-03)
- the pack format and the drive-bus pack (T-04)
- the sheet pipeline (T-05)

**Exit checks:**
- CI passes on a clean clone: format, lints, licences, layering, tests, and the wasm32 build of the core.
- `hyleon check samples/pump-station` reports exactly the two seeded drive-bus findings (a duplicate address and a missing termination) and nothing else. Fixing them clears them.
- One bus wiring sheet renders at A3 and ANSI B and passes veraPDF.
- The Dioxus trial runs as a web build and as macOS and Windows desktop bundles; Linux follows later. D-003 is confirmed or reversed.

## Phase 1: The framework, as a thin slice

**Goal.** A person, by hand or with an agent, designs a small, real job across three disciplines in one model. Everything still open is shown and acknowledged before a set is issued.

**Scope:**
- **Packs:** `site-basics`, `motor-branch-circuit`, `modbus-drive-bus` and `pump-discharge-spool`, all sharing one pump ([formats/pack.md](formats/pack.md)).
- **The engine:**
  - questions with conditions
  - sources, and acknowledgements at issue, with a reason required for errors (D-022)
  - rules, calculations and cross-reference checks
  - the start of the fault-current chain: the site's available fault current, the drive's short-circuit rating, and the panel nameplate
  - a verification status on every element (D-024)
- **Region profiles.** The format ([formats/region.md](formats/region.md)), plus two profiles: US (NEC 2023) and international (IEC).
- **A default owner profile** that the sheets are drawn to ([formats/owner.md](formats/owner.md)).
- **The catalog** ([formats/catalog.md](formats/catalog.md)): exact models picked and logged, and device settings for the drive bus.
- **Printing:** true size, and fit to paper with the real scale and a scale bar ([formats/print.md](formats/print.md)).
- **Building by hand.** Forms and tables for components, connections and answers. No agent needed.
- **The MCP server:** `status`, `questions`, `ask`, `propose`, `apply`, `check`, `explain`, `render`.
- **The desktop app:** questions, findings, sheets; confirming, accepting and issuing.
- **Outputs:** one-line diagram, bus wiring sheet, spool sheet, BOM (XLSX and CSV), register list.

**Exit checks:**
- From a written brief, Claude Code designs the sample through MCP. Every required question is answered by a person, through elicitation or in the app. Revision A is issued with zero errors.
- A person builds the same sample by hand in the app, with no agent, and gets the same sheets.
- Entering the pump twice under two tags, once from the power pack and once from the piping pack, produces the cross-discipline errors. They stay listed until the pumps are merged or the person acknowledges them at issue, with a reason.
- An agent's attempt to confirm, accept, acknowledge or issue is refused and logged.
- An AI-inferred value still open at issue is listed, and the revision records who confirmed or accepted it.
- Choosing the international region profile fills `site-basics` with sourced defaults. Changing one by hand records the person as its source.
- Raising the site's available fault current above the drive's rating produces an error at the drive and on the nameplate it prints.
- Rendering revision A again gives identical sheet SVGs, matched by hash.

## Phase 2: The workspace

**Goal.** People design, compare and issue with a clear history, to their clients' own standards.

**Scope:**
- **Topology editor.** Drag devices, connect ports with typed links (power, signal, data, pipe), and configure them: the Packet Tracer-like view.
- **Editable tables:** equipment, I/O and cable lists.
- **Options.** Branches of the model, compared on findings, BOM and cost.
- **Owner profiles and their validator** (D-023), including the tag grammar that keeps every name identical across documents.
- **Lifecycle** (D-024):
  - revisions with a purpose (30/60/90% reviews, submittal, for construction, record set)
  - reviewers' action codes and comments
  - resubmittals
- **On site:** marking elements as checked or differing; markups back into the model.
- **Swapping parts** with compatibility checks and a list of what changes; custom components; agent-researched catalog entries that a person approves (D-031).
- **Your company profile** (D-030).
- **Printing:** tiled printing, half-size sets, and the whole set to PDF, printer or plotter (D-029).
- **Revisions.** Issued revisions with generated change lists, and navigating from a tag to every sheet that shows it.
- **Exports and handoff:**
  - DXF with mapped layers
  - a DOCX installation guide
  - a deliverable with the model stripped out
  - CSV and JSON

**Exit checks:**
- A person builds the sample from scratch with the drag-and-connect topology editor.
- Two options are compared side by side, and one is chosen.
- Revision B is issued, and its change list matches the commands since A.
- An owner profile with a different tag grammar flags every non-conforming tag, naming the owner's rule.
- A reviewer's "revise and resubmit" is recorded, and the resubmittal names the revision it answers.
- The DXF opens in a CAD tool with correct layers and scale.

## Phase 3: Cloud and Pro launch

**Goal.** People pay for Pro, work across devices, and present to clients.

**Scope:**
- accounts (D-027) and subscriptions (D-028)
- free viewer seats for reviewers, inspectors and clients
- a per-project pricing test beside per-person seats
- sync of logs and revisions (R2)
- the web app (Dioxus on wasm32)
- share links with comments and markups
- the submittal package
- **DWG,** through an Open Design Alliance library (D-020). "DWG" names only the file format, never a product
- space reserved for the licensee's own seal or electronic signature
- pack defect notices to every affected project (D-025)
- a shared catalog: approved entries published for everyone (D-031)
- an offline licence for the desktop app
- the Astro site: landing, pricing, docs, and a page per pack
- terms of service and an end-user licence, drafted by counsel for each market. The industry pattern: Hyleon is a tool, not a professional service; the user verifies results; liability is capped. Some countries' laws limit what can be excluded. Counsel also checks EU product liability for software (from 2026-12-09), and E&O and product insurance.
- the API: Axum, tower, sqlx and Postgres (D-026). Its dormant invariants switch on: resilience, admission control, health, secrets ([ARCHITECTURE.md](ARCHITECTURE.md))

**Exit checks:**
- A paying user syncs a project between desktop and web.
- A client opens a share link without an account and leaves a comment.
- A DWG written by Hyleon opens in AutoCAD with the owner profile's layers.
- The desktop app works offline through the grace period.
- A lapsed licence still opens and exports every project.

## Phase 4: Depth

**Goal.** Base packs deep enough for real professional work in most industries, with panels as the flagship.

**Scope:**
- **Shared base packs first (D-021):** I/O and instrument loops (4–20 mA loop budgets), site networks (IP and VLAN plans, switch ports), pump stations, tanks and level, dosing, standby power, and commissioning (factory and site acceptance tests, loop checks).
- **The flagship VFD panel pack:**
  - the full fault-current chain
  - help with the SCCR calculation per UL 508A Supplement SB and its documentation; ratings come from UL's database or maker data, the shop's qualified representative decides, and no UL marks are used
  - heat load against cooling
  - backplate and door layout
  - terminal plans, wire lists, labels
- **The unlisted-panel package:** SCCR, nameplate, and a package ready for a field evaluation body.
- **Industry overlays,** starting with water and wastewater (Ten States, NFPA 820, alarms, standby power), then food and beverage.
- **Power-study export:** one-line data to SKM, ETAP and EasyPower.
- **Parameter sheets and commissioning checklists** from device settings, with firmware versions logged (D-032).
- **Other packs:** distribution boards, piping spools (materials, supports, cut lists).
- **On site.** Record work on a tablet: photos, measured values, markups.
- **Catalogs.** Manufacturer data imported via ETIM (open; its xChange JSON format) or BMEcat (BME's free implementation licence needed).
- **Codes and locales.** Code packs per country (NEC, CEC, IEC/EN), with licences from the standards bodies where their terms require them (D-017), plus units and locales.
- **Region profiles for every place the first customers work,** with effective-date rules and local triggers, kept current by the daily watcher. A person approves every update, and projects get advisories ([formats/region.md](formats/region.md); D-019).

**Exit checks:**
- A seeded change on a watched source produces a proposed profile update within a day, and nothing goes live until a person approves it.
- A watcher that stops running raises an alarm within a day.
- Every released pack names its reviewing licensed engineer and its code editions.
- A real panel shop issues a real panel set from Hyleon and reports which checks caught real errors, counted rather than estimated.

## Phase 5: Space

**Goal.** Designs with real positions and dimensions, so distances, clearances and routes are checked.

**Scope:**
- **The spatial model:**
  - the building shell (column grid, clear height, doors), with DWG, DXF or PDF drawings as backgrounds
  - placed objects with footprints and clearance envelopes, each tied to its source clause
  - zones whose properties trigger rules
  - egress paths
  - routes whose real lengths feed back into the checks (voltage drop, hydraulics)
- **Layout outputs:** elevations, sections, and dimensioned sheets.
- **Layout packs:**
  - equipment rooms with working clearances
  - warehouses: racking, aisles, egress, and high-piled storage plans

**Exit checks:**
- An equipment room with less working clearance than required is flagged, with its clause.
- A cable route's real length changes the voltage-drop result.
- An aisle narrower than its required width is flagged, with its source.

## Phase 6: Simulation

**Goal.** Catch behavior problems, not just static ones, before building.

**Scope:**
- **S1:** network reachability (VLANs, routes, firewall rules) and Modbus poll timing (bus load against the required cycle).
- **S2:** hydraulics (EPANET) and control logic (tank states, flow loops).

**Exit check:** a seeded subnet mismatch and an overloaded bus are each caught before anything is built.

## Phase 7: Enterprise

**Goal.** Large teams run Hyleon under their own standards and controls.

**Scope:**
- SSO and SCIM
- roles and sign-offs (checked by, approved by)
- an audit trail
- company packs and owner profiles, with governance over them
- on-premises and air-gapped deployment
- open export and source-code escrow
- DEXPI and AutomationML import and export
- PDF/A-3u or PDF/A-4f with the model attached
- real-time collaboration, if the need is measured
- a public API: idempotency keys, cursor pagination, versioning, signed webhooks

**Exit checks:**
- An enterprise pilot runs entirely on its own premises with SSO and its own pack.
- For any value, the audit trail answers who set it, who approved it, and when.

## Phase 8: Ecosystem

**Goal.** Others extend Hyleon, and Hyleon fits into the tools people already use.

**Scope:**
- **A pack library and marketplace.** Community and company packs, and manufacturers publishing device packs. Packs are signed.
- **Delegating packs.** A KiCad project, or a 3D CAD model via DXF or DEXPI, is linked as evidence, while that tool does its own checks.
- **Import.** A picture, a sketch or a PDF becomes model elements, every one marked AI-inferred until a person confirms it.

**Exit check:** a third-party pack installs, runs sandboxed, and cites its own sources in its findings.
