# Product

What Hyleon does, in detail. Why it exists and what must stay true are in [TRUTHS.md](../TRUTHS.md); this document doesn't repeat them. Dan ratifies scope changes. Items marked *proposed* await his decision.

## What Hyleon is made of

This is everything Hyleon really needs (D-036). Everything else attaches to it.

Hyleon connects researched boundaries to the work (TRUTHS.md, D-045). Packs carry the research, the model carries the work, and checks join the two, whether a person or an AI did the work. The pack format is open, so any kind of work can be researched into packs; Hyleon's own packs start with physical design.

1. **One model.** Everything is one of four things:
   - a **Place** (site, area, room, enclosure)
   - a **Part** (a real component from the catalog, or a custom one)
   - a **Port** (where things attach: power, signal, data, pipe)
   - a **Connection** (a cable, wire, pipe or network link between ports)

   Every value records its source.
2. **Five tools,** the same in any environment: **Place**, **Connect**, **Configure** (settings and values), **Measure** (distances and clearances), **Mark up** (notes and site changes).
3. **Four views** of the same model:
   - **Layout:** where things are
   - **Diagram:** how they connect
   - **Sheets:** drawings to print and issue
   - **Lists:** parts, I/O, cables, budget
4. **One flagship job:** a pump station, meaning a VFD pump panel with its drive bus and discharge piping, checked, drawn, printed and recorded end to end. Everything else grows from it.

Environments differ only in data: the catalog's parts, the packs, and the region and owner profiles. A new environment needs new data, not new tools.

## Who uses it

**Authors** are the people who design and document the work:
- electricians and electrical contractors who design and wire panels and field circuits
- panel builders, such as UL 508A shops, who produce schematics, layouts, labels and short-circuit documentation
- controls integrators who connect controllers, drives, instruments and networks
- site and facility engineers who own a site's systems and its as-built record
- their AI agents: Claude Code or any MCP client, working for one of the people above

**Viewers** read and comment but don't author: clients, inspectors, installers, reviewers. Viewing is always free.

**Later:** enterprise engineering teams with company standards, roles and sign-off chains.

## The loop

1. **Brief.** A person, or their agent, states the job, for example "two pumps on drives, filling a tank, controlled over Modbus."
2. **Questions.** The packs for that job list what must be known. Hyleon asks only what is still unanswered, and records who answered.
3. **Design.** People and agents change the model through commands: add a device, connect two ports, set a rating. Every value carries its source.
4. **Checks.** Checks run on every change. Each finding is an *error*, a *warning* or *info*, and names its source and a fix. Nothing a check finds stops the work.
5. **Review.** A person sees what changed, in the model and on the sheets. They confirm or replace the AI's guesses, and accept or reject the limits it proposed.
6. **Issue.** A person issues a revision whenever they decide to.
   - Hyleon first lists everything still open: errors, warnings, and unconfirmed AI guesses.
   - The person acknowledges each one. Errors need a written reason; warnings and AI guesses don't (D-022).
   - The revision is frozen, records those acknowledgements, and gets a generated list of what changed.
7. **Present.** The issued set goes out through a share view with comments and markups, and as a submittal package.
8. **Build.** Installers work from the issued set on site, on a tablet or on paper.
9. **Record.** Site changes, with photos, measured values and markups, become the next revision.
   - Each element shows whether it was checked on site.
   - The record set is labelled "record drawings" or "as-built", as the person chooses (D-024).

The loop doesn't force an order. People can answer, design, check and issue in any order, by hand or with an agent. Hyleon informs; it never overrules ([TRUTHS.md](../TRUTHS.md), truth 6).

## The five parts

1. **Scenario packs.** A pack is a data file for one kind of job. It holds:
   - questions, each with why it matters, its options, a default with a source, and conditions
   - boundaries, each with its source
   - checks: rules, or calls to built-in calculations
   - deliverables: the sheets and schedules the job must produce

   Packs are versioned and they compose: a pump station uses the motor circuit, drive bus and spool packs together. Format: [formats/pack.md](formats/pack.md).
2. **One model.** It holds equipment, devices and their ports, links (power, signal, data, pipe), buses, networks, cables, locations, sheets and revisions. It changes only through commands, which are validated before they apply. Format: [formats/project.md](formats/project.md).
3. **Checks.** There are three kinds:
   - rules written in the pack
   - calculations built into Hyleon, such as conductor sizing or heat load. The flagship is the **fault-current chain**: from the utility's available fault current to every device's rating, and on to the printed labels, with the study date (D-021)
   - cross-references: tags are unique and follow the owner's naming grammar, references resolve, and the same thing has the same name on every sheet

   Findings never block anything. At issue, open ones are listed, acknowledged and recorded.
4. **Sources.** Every value is labeled with where it came from:

   | Label | Recorded with it |
   |---|---|
   | person | who, and whether it came through the app or a question the agent relayed |
   | standard | the code and clause, never the text |
   | catalog | the entry and its version |
   | calculated | the calculation, its version and its inputs |
   | AI-inferred | the agent and its session |

   An AI-inferred value stays marked until a person confirms it or replaces it. At issue, the person confirms, replaces or accepts each one still open, and the record says which.
5. **Generators.** Everything below is generated from the model:
   - drawing sheets: SVG, then PDF and PDF/A, in ISO A and ANSI sizes, with a title block and revision table, drawn to the owner profile
   - schedules: I/O list, cable list and BOM, as XLSX and CSV
   - guides, and test procedures with sign-off forms
   - CAD files, with layers and attributes mapped to the owner profile: DXF from phase 2, DWG from the Pro launch (D-020)

   **Handoff choices.** People can hand over:
   - the whole project, with its model and sources
   - a deliverable with the model stripped out
   - CAD files
   - open CSV and JSON for owners' maintenance systems

   DOCX, DEXPI and AutomationML come later ([ROADMAP.md](ROADMAP.md)).

## Boundaries

- A boundary comes from a selected code edition (for example NEC 2023 with UL 508A, or IEC 60204-1), a client or company specification, or a person's answer.
- An agent may propose a boundary. It stays inactive until a person accepts it.
- A boundary appears on the elements it limits, so an installer sees "BSP threads only" on the fitting, not in a note on sheet 1.
- Hyleon cites clauses and never reproduces code text, figures, notes or whole tables. Standards bodies' terms forbid using their documents to build software without permission.
- **Table values from a code** (an ampacity, a motor full-load current) come from one of two places:
  - the person, from their own copy, recorded with the clause
  - a table Hyleon is licensed to use (D-017)
- **No standards text enters a project or reaches an agent.** Projects store clause references, not standards documents.

## Professional responsibility

- **Hyleon never seals or certifies.** Until a revision is issued, every sheet is marked "PRELIMINARY, NOT FOR CONSTRUCTION".
- **Issuing.** The person chooses the issue's purpose, for example "for review" or "for construction".
- **Sealing.** Where a jurisdiction requires a licensed engineer's seal, the licensee applies their own after review: in the title block, or as an electronic signature on the PDF/A, as the owner requires. Hyleon reserves the space and never applies a seal.
- **Wording.** Hyleon says "checked against clause X, edition Y", never "compliant" or "certified" (D-025).
- **Records.** The log keeps every input, rule version, AI suggestion and approval. That lets a licensee reproduce the result and show they were in responsible charge, which licensing boards' AI guidance asks for ([research](research/2026-10-28-market.md)).
- **Jurisdiction notes.** Who may design what varies by state and country. Hyleon can show notes on that, never legal determinations.
- **UL 508A.** Hyleon helps with the short-circuit rating calculation and its documentation. Component ratings come from UL's database or the maker's data, and the shop's qualified technical representative decides. Hyleon never uses UL's marks.

## Region profiles

- **Picking a region** applies what's in force there:
  - code editions, and local amendments cited by clause
  - units, paper sizes, the symbol set and wire colours
  - notes on who may design what
- **Every profile shows** when it was last verified, its sources, and "confirm with your authority having jurisdiction". Hyleon informs; it doesn't certify.
- **The answers it supplies** (for example, which code and edition govern) carry the profile as their source.
- **Updates.** A watcher checks the sources daily and proposes changes. A person approves them before they're released.
- **Projects** record the profile version they were designed under, and are told when it changes.

Format: [formats/region.md](formats/region.md); decision D-019.

## Owner profiles

- **A client's drawing standard, as data:** title block, sheet sizes, fonts and text heights, line weights, layers, numbering and tag grammar, revision scheme, and how sheets are sealed.
- **Every sheet is generated to it,** and a validator checks the whole set before issue. Each finding names the owner's rule and the owner's document.
- **Projects** lock the version they follow.

Format: [formats/owner.md](formats/owner.md); decision D-023.

## Components and catalog

- **Exact models,** picked by hand or proposed by an agent.
  - Every pick is logged.
  - Each part is marked as existing on site, in stock, or picked to buy.
- **Swaps** check compatibility and list everything they change.
- **Missing parts** are added by hand, or researched by an agent from official sources with a source for every fact. A person approves them before use.

Format: [formats/catalog.md](formats/catalog.md); D-031.

## Device settings

Each device's parameters, addresses and firmware follow its catalog schema. They're checked across devices, and exported as parameter sheets and commissioning checklists (D-032).

## Energy and running cost (phase 4)

From each part's researched ratings and the person's run hours, load profile, tariff and location, Hyleon estimates yearly energy, cost and carbon per part, system and site (D-045). Every figure shows its sources and stays an estimate. Some energy facts are also design checks: a pump far from its best efficiency point is flagged.

## Printing and branding

- **Printing:**
  - true size
  - fit to paper, with the real scale and a scale bar
  - tiled across small pages
  - half-size sets
  - the whole set at once, to PDF, printer or plotter

  Format: [formats/print.md](formats/print.md); D-029.
- **Your company profile:** logo, details, colours and licensed fonts. The client's own standard comes first (D-030).

## Scopes and collaboration

- **Every module is optional, to fit each person's scope of work.** A designer may never see the budget, and an estimator may never touch wiring.
- **Enterprise means many people on one project,** each in their own scope, finishing their part until the whole project is done, with or without AI. That's how Hyleon scales and earns from larger users (D-033).

## Working by hand, or with an AI agent

Hyleon is a normal design application first. People build and change everything by hand: forms, tables and, from phase 2, the topology view. An agent is optional.

- The agent connects through MCP: `hyleon mcp`, described in [formats/mcp.md](formats/mcp.md).
- **What the agent can do:** read the project's status, ask the person questions, propose and apply changes, run checks and render sheets.
- **What the agent can't do:**
  - answer for the person: a value it supplies is recorded as AI-inferred
  - confirm its own guesses
  - accept a boundary
  - acknowledge open findings
  - issue a revision
- Questions reach the person directly through MCP elicitation, where the agent's client supports it. Otherwise they wait in the app.
- Everything works without an agent: every action exists in the app.

## Options, presenting, records

- **Options.** A design option is a branch of the model. Options are compared side by side on findings, BOM and cost, and one becomes the design.
- **Presenting:**
  - a read-only share view with comments and markups
  - a submittal package: cover, index, sheets, schedules and datasheets, as one bookmarked PDF with clickable cross-references
- **Records:**
  - every element links to the answer, clause, catalog entry or photo behind it
  - issued revisions never change
  - past projects can be used as templates
  - **pack defects:** when a pack is found wrong, every project that used the affected version is told what changed and which checks it affects, with a corrected version (D-025)
  - **what Hyleon caught:** each project keeps a count of the findings raised and what changed because of them, on the person's device

## Lifecycle

- **Revisions carry their purpose:** design reviews at 30%, 60% and 90%, submittals, for construction, and the record set (D-024).
- **Reviewers' responses** are recorded against the revision: the action code and the comments. A resubmittal names the revision it answers.
- **Test procedures and their forms** are generated from the model, with spaces to sign: factory and site acceptance, loop checks and calibration.
- **On site:**
  - each element can be marked as checked (by whom and when), or as differing from the design
  - markups flow back into the model as changes for a person to accept

## The first version: a thin slice of three disciplines

| Discipline | Pack |
|---|---|
| Power | Motor branch circuit |
| Controls | Modbus RTU drive bus |
| Piping | Pump discharge spool |

All three packs share one pump, so the slice proves that one model works across disciplines: rename the pump in one place and every sheet follows, and a reference that doesn't resolve is caught.

The slice also starts the flagship fault-current chain: the site's available fault current, compared with the drive's short-circuit rating, and printed on the panel nameplate.

## Order of content

Hyleon supports every industry; industries are layers of packs ([formats/pack.md](formats/pack.md)). Content comes in this order (D-021):
1. **Shared base packs:** motor and VFD circuits, panels, I/O and loops, networks, pump stations, tanks and level, dosing, standby power, commissioning.
2. **Panels** with the full fault-current chain, as the flagship.
3. **Industry overlays,** starting where integrators already work: water and wastewater, and food and beverage.
4. **The spatial model,** then layout packs such as warehouses ([ROADMAP.md](ROADMAP.md), phase 5).

## Tiers (*proposed*)

| Tier | For | Includes |
|---|---|---|
| Viewer | Reviewers, inspectors, clients | Always free: open shared sets, comment, mark up |
| Free | Individuals trying it | Local projects, community packs, checks, PDF |
| Pro (subscription, per person) | Power users | All packs and exports including DWG, agent access over MCP, options, cloud sync, share links |
| Enterprise (later) | Large teams | SSO and SCIM, roles and sign-offs, audit trail, company packs and owner profiles, on-premises or air-gapped deployment, DEXPI and AutomationML, source-code escrow |

- A lapsed subscription never locks people out of their own projects: they can still open and export them (TRUTHS.md, truth 7).
- Prices aren't set. The inputs are in [research/](research/).
- Per-project pricing will be tested beside per-person seats, since many integrators bill by project.

## Where Hyleon hands off

| Work | Specialist tool | What Hyleon does |
|---|---|---|
| Electronics and PCBs | KiCad and other EDA tools, which have their own rule checks | A later pack can hold the requirements and evidence and link to the EDA project |
| 3D piping and mechanical design | AutoCAD Plant 3D, SolidWorks | Exchanges DXF, and later DEXPI |
| Power-system studies (short-circuit, arc flash) | SKM, ETAP, EasyPower | Exports one-line data to them, and records their results as sourced values |
