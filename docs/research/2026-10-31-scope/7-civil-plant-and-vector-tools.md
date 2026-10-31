# Appendix 7: Civil, plant and vector drawing tools (agent report, 2026-10-31)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-31-scope.md](../2026-10-31-scope.md).

**The brief:** Map the tools the round didn't cover (Autodesk Civil 3D and InfraWorks, Bentley's civil, utility and plant tools, Hexagon's plant tools now sold by Octave, and Inkscape with other free vector editors): what each designs or checks, where its rules come from, its AI and MCP features, price and buyers; then show the overlap with Hyleon, what to hand off or exchange, the gaps, how close each comes to a connecting layer, and the decisions only Dan can make.

---

## Summary

- **All three engineering vendors check designs only inside their own tool, against rules the customer or an agency sets.**
  - Civil 3D has pipe rules, pressure-network design checks, and design-criteria files (AASHTO tables ship with it). They only warn; the person decides [4–7].
  - Bentley designs against constraints (slope, cover, velocity) and agency standards libraries. US state transport departments publish these libraries as OpenRoads workspaces [25, 27, 33].
  - Octave's Facets tools are "rules-driven". They validate P&IDs "against engineering standards, safety practices and owner requirements" and electrical work for NEC tray fill [41, 43].
  - None of them records a source for each limit or asks questions before design.
- **MCP has arrived, unevenly.**
  - **Bentley** lists seven MCP servers (STAAD, MicroStation in early access, iTwin IoT, ProjectWise, AssetWise, PLS-CADD, TOWER/PLS-POLE). It calls them "the first in a planned rollout … across the full Bentley portfolio" [18]. None covers OpenRoads, OpenFlows or OpenPlant yet.
  - **Autodesk** has no official Civil 3D MCP server that I found. Its in-product Assistant is a tech preview in Civil 3D 2027. Only community and commercial third-party servers exist [2, 12–14].
  - **Octave** has no MCP server that I found. Its AI is closed: Octave Assist is live, and the Aria multi-agent framework is in private preview (2026-08-12) [47].
- **The closest thing to a connecting layer in this group is Bentley's OpenUtilities Substation+** (early access). Engineers upload design-criteria documents, and the AI "cross-reference[s] element properties against uploaded design criteria to flag conflicts" (2026-05-20) [36]. It is closed, for substations only, and it reads documents with AI rather than running tested, sourced rules.
- **The vendors now use Hyleon's trust language.** Bentley's CTO: "We are not generating text about engineering; we are executing real, verifiable engineering workflows" (2026-06-08) [22]. A commentator on Bentley's STAAD MCP server: "MCP does not validate engineering results by itself" [21]. Trust alone won't set Hyleon apart; open, sourced, cross-tool packs might.
- **Inkscape and the free vector editors check nothing.** They make pictures, the kind TRUTHS.md says is never the source.
  - Since 2026-09, community MCP servers let any agent draw in Inkscape [54, 55].
  - draw.io has an official MCP server [60].
  - So AI-drawn "technical" pictures are getting easier, not harder. That is the problem Hyleon starts from.
- **Overlap with Hyleon is at the edges, not the core.**
  - The civil tools design buried networks and the ground. Hyleon designs the pump station, its panel, its spool and its controls.
  - The plant tools (OpenPlant, Forte 3D, Facets) overlap on piping specs, P&IDs, instrument indexes and electrical load and cable lists, but they serve large EPC firms (engineering, procurement and construction contractors) and owner-operators.
  - No tool in this group covers OT networks or control panels.
- **Exchange formats that matter:**
  - **LandXML** (surfaces, alignments, gravity pipes) and **IFC 4.3** (civil and plant models) from Civil 3D, OpenRoads and OpenFlows.
  - **DEXPI 2.0.1** (2026-09-30) for P&IDs [51].
  - **SWMM** from OpenFlows Sewer [32].
  - **SVG** both ways with vector editors.
- **A word to settle:** D-001 says "site systems". In civil tools, "site design" means grading, parking and drainage (Civil 3D, OpenSite). Dan should say whether Hyleon's site stops at the station or building, or reaches into the ground.

**Notes**
- All pages were read on 2026-10-31 unless a date is given.
- The session's shared web-search budget ran out after about 30 of my searches. After that I fetched only official URLs I already had. Some topics are thinner because of this; see section 10.
- † means I saw it only in a search-engine summary, not on the page itself (the page refused fetches or I didn't open it). Treat it as unverified.
- \* means a vendor claim. "unknown" means not found; nothing was guessed.
- autodesk.com, inkscape.org and mcpservers.org refused fetches (HTTP 403). help.autodesk.com and aps.autodesk.com worked.
- Nothing was installed or run.

## 1. Autodesk Civil 3D and InfraWorks

| Item | Finding |
|---|---|
| **What it designs** | Surfaces and grading (feature lines, daylighting), alignments, profiles, corridors (roads), gravity pipe networks (storm and sanitary), pressure networks (water, gas), parcels. Drainage analysis through Autodesk Drainage Tools (InfoDrainage inside Civil 3D, since 2026.2) [2]. |
| **What it checks** | **Pipe network part rules:** Cover and Gradient, Cover Only, Length Check, Pipe Drop Across Structure, Maximum Pipe Size Check and others. They "flag criteria that aren't met", and the person edits the design or accepts the violation [4]. **Pressure network design checks:** deflection, diameter, open connections, radius of curvature. Warning symbols appear at each violation [5]. A separate depth-of-cover check exists† (not confirmed on the page read). **Alignment and profile criteria:** design speed, minimum radius and superelevation tables in an XML design-criteria file, plus user-written "design checks" that "always return a true or false value" [6, 7]. |
| **Where rules come from** | **Pipe rules:** two editable files, `C3DPipeRules` (a VB script) and `C3DPipeRules.xml`, in ProgramData. The page cites no standard; they are "adjustable defaults" [4]. **Design criteria:** files with AASHTO tables ship with Civil 3D (a tutorial shows AASHTO 2001 tables)†. Agencies edit them in the Design Criteria Editor [7]. **Pressure checks:** limits are set in the parts list [5]. Values carry no clause citation. |
| **AI** | **Autodesk Assistant:** a tech preview in Civil 3D 2027. It can "check a drawing's standards against a standards file" (layers and styles, not engineering limits) and query and change object styles; prompts can be saved (2026-03-25) [2]. **Horizontal Regression Analysis:** adds a pre-trained machine-learning fit that "does not learn from customer data" [2]. **Daylight feature line:** a new tech preview that creates daylight lines from grading criteria [2]. **Autodesk's plan:** the Assistant is being rebuilt "as an agentic AI partner, built on MCP servers". The vision includes checking a drawing "against known standards and regulations for safety, accessibility, and drainage" (2025-11-17)† [8]. A standalone, agent-first Assistant across products is planned. Timing, pricing and terms come in 2027 (AU 2026, about 2026-09-15) [9]. |
| **MCP** | **Official:** I found no Civil 3D MCP server. Autodesk's 2026-04-15 list had Revit, Fusion, Fusion Data and Product Help servers, with InfoWorks "coming soon" (appendix 3). An AU 2026 class covers three "production" servers: Revit deployment, Dynamo and an APS data pipeline† [12]. **Third-party:** a community server (`barbosaihan/civil3d-mcp`, MIT) lets an agent run arbitrary C# against the open drawing (`civil3d_execute`, `civil3d_query`) [13]. A commercial one, HMK Pilot, covers Revit, AutoCAD, Civil 3D and Navisworks, runs locally and has a 14-day trial† [14]. |
| **Data exchange** | **Civil 3D Cloud Connector:** generally available 2026-09-08. It sends corridors, pipe networks, surfaces, alignments, profiles and bridges to Autodesk Data Exchange, for Revit, Forma and Power BI. Pressure networks weren't mentioned [11]. **Forma:** Civil 3D becomes a "Forma Connected Client" (tech preview) with a shared geolocation engine (AU 2026, 2026-09-15) [10]. **Forma Street Design:** closed beta [10]. **IFC 4.3:** a plug-in exports and imports corridors, bridges, and pressure and gravity pipe networks (2023)† [16]. **LandXML:** import and export are long-standing features (not re-checked this round)†. |
| **InfraWorks** | **Status unknown.** I found only reseller listings and training catalogs for "InfraWorks 2027", and no release notes. The AU 2026 coverage I read doesn't mention InfraWorks; Autodesk's new conceptual tools are Forma Street Design and Forma Site Design [10]. Treat InfraWorks as a legacy concept tool until checked. |
| **Price** | Civil 3D: $2,870 a year, $8,610 for 3 years (Autodesk FAQ)† [15]. The AEC Collection price conflicts across sources (about $3,465 to $4,350 a year)†. |
| **Buyers** | Civil engineering firms, land developers, transport departments, utilities, contractors. |

## 2. Bentley

### Products

| Product | What it designs or checks | Where rules come from | AI, dated | Price (12-month Virtuoso subscription, "starting at") | Formats |
|---|---|---|---|---|---|
| **OpenRoads Designer** [25, 26] | Roads, corridors, superelevation, terrain, site grading, storm and sanitary drainage, culverts, ponds, 3D utilities | **Standards:** horizontal and vertical design standards, feature definitions and civil cells in standards libraries†. Agencies publish their own workspaces (Georgia DOT, Connecticut DOT, Tennessee DOT, FHWA)† [27]. The page says it supports "agency-specific standards … out of the box". | **2025-10-15:** AI for OpenRoads announced. An annotation agent was due for general availability in 2025-11, and Copilot in early 2026† [24]. **Product page (read 2026-10-31):** names only "AI-powered label organization", with no status [25]. **OpenRoads 2026:** regression table, "file health auditing" [26]. | $7,226 | Imports and exports DWG, DXF, LandXML and IFC (version not stated). Native DGN. |
| **OpenSite Designer** [28] | Site grading (optimised on "construction cost"), parking, parcels, driveways, sanitary and storm networks, ponds | User-set minimum and maximum slopes, plus customisable "design standards for site layout" | None; it automates | $3,168 | 3D PDF, iModel; LandXML and IFC not named |
| **OpenSite+** [29] | Layout, grading ("evaluates thousands of grading scenarios within project constraints"), stormwater with OpenFlows | Constraints "embedded in site objects". The page doesn't say who sets them or name any standard. | Generative AI for grading; Bentley Copilot (natural language, "evidence-based answers" from designs and documents). Runs on Azure, US-hosted only. Not generally released. | Request pricing | iModels; CAD lines out |
| **OpenFlows Water** (WaterCAD, WaterGEMS, HAMMER) [30, 31] | Pressure water networks: pressures, flows, velocities, fire flow, transients, pipe sizing and rehabilitation optimisation "under cost and hydraulic constraints", pump scheduling (Darwin Scheduler) | User-set constraints | Copilot tech preview: query the model and run commands in plain language (OpenFlows 2026, 2026-05-29). "AI-powered calibration" in the Advanced and Ultimate tiers. | $1,021 (100 pipes) to $16,782 (unlimited) | GIS shapefiles, ArcGIS, SCADA, terrain; iModel publishing |
| **OpenFlows Sewer** (SewerCAD, SewerGEMS) [32, 33] | Sanitary, combined and storm sewers; automated constraint-based gravity design (Standard tier and up) | **Constraints:** user-set minimum and maximum slope, velocity and cover, as defaults or per pipe. The solver ranks them, so minimum cover can give way to existing structures† [33]. **Bentley's own wiki:** "the final design should be as per the engineer's judgement"†. | Copilot tech preview | $1,021 to $16,782 | SWMM, LandXML, "drainage formats", ArcGIS, CAD |
| **OpenPlant Modeler, OpenPlant PID** [34, 35] | 3D piping, equipment, HVAC, cable tray, steel; P&IDs; clash detection | **Specs:** "specification-driven piping". Users create or edit specs. EN, DIN and AWWA catalogs. | **OpenPlant 2026** (blog dated 2026-09-29; a search listing showed 2026-09-23): a Python Assistant that builds reusable automation from prompts, and Bentley Copilot (tech preview) for in-app help [35]. | Modeler $5,839 | ISO 15926 data model; iModel; DGN, DWG, DXF, STEP, SVG out; IFC export now carries OpenPlant schemas (2026). DEXPI not mentioned. |
| **OpenUtilities Substation+** [36, 37] | Model-based substation design | **Design criteria:** uploaded by the engineer. "Engineers can ask the AI to cross-reference element properties against uploaded design criteria to flag conflicts." No standard named. | **2026-05-20:** document query, cross-referencing and agents with "domain-specific skills" are described as current. Optimisation across design variants is future. "Every action is traceable." | Early access; no price | Not stated |
| **iTwin Platform, Infrastructure Cloud** [19, 38] | Digital twins; iModels as the shared format across Bentley apps; open-source iTwin.js; Cesium | n/a | **iTwin IoT MCP server:** sensor queries and alerts [18]. The iTwin GitHub org has no MCP repository [38]. | Not stated | iModel, ECSQL |

### Bentley MCP servers (page read 2026-10-31; it gives no launch dates or prices) [18]

| Server | Status on the page | What it does | Clients |
|---|---|---|---|
| STAAD.Pro and STAAD Foundation | Downloadable; submitted as a Claude connector (2026-06) [20] | Builds and revises models in natural language, applies loads, runs analyses. "No access to the file system or operating system outside of STAAD.Pro." | Claude, GitHub Copilot, Gemini |
| MicroStation | "Early Access … restricted set of users" | Full Python API (read, write, create); queries across DGN data. The FAQ warns that key-ins run through MCP may reach the file system. | Claude, GitHub Copilot |
| ProjectWise | By request | Search, permission-aware Q&A, summaries, "AI-assisted reviews and compliance checks" | Microsoft Copilot, Claude, Gemini |
| iTwin IoT; AssetWise ALIM | By request | Sensor and alert queries; asset-information queries | Claude and others; Microsoft Copilot |
| PLS-CADD; TOWER and PLS-POLE | By request | Read-only queries of transmission-line data; "model checks", allowable spans | Any MCP client; local loopback only |

**Timing:**
- **YII 2026** (Singapore, 2026-10-06 to 07) was awards only.
- **Bentley Tech Summit** (Toronto, 2027-04-13 to 15) will carry product announcements from now on [23]. Watch it for OpenRoads, OpenFlows or OpenPlant MCP servers.

**Stance:**
- Bentley says it connects agents to its "validated engineering software", starting with STAAD.
- It says firms should keep control of their design data [22].

## 3. Octave (Hexagon's former Asset Lifecycle Intelligence)

**Spin-off.** Octave began trading on 2026-05-29 (Nasdaq New York: OCTV; Nasdaq Stockholm: OCTV SDB) [40].

**Renames** (page updated 2026-04-08; "do not impact product functionality, contracts or entitlements") [39]:

| Old name | New name |
|---|---|
| Intergraph Smart 3D | Octave Forte 3D |
| Intergraph Smart P&ID (and P&ID Design Validation) | Octave Facets P&ID (and Design Validation) |
| Intergraph Smart Instrumentation | Octave Facets Instrumentation |
| Intergraph Smart Electrical | Octave Facets Electrical |
| Smart Reference Data | Octave Forte Reference Data |
| CADWorx Plant Professional; CADWorx P&ID | Octave Forte 3DWorx; Facets P&IDWorx |
| HxGN SDx2 / SDx | Octave InConcert / InConcert Core |
| CAESAR II; PV Elite | Octave Aspect Pipe Stress; Aspect Pressure Vessel |
| Smart Construction; Spoolgen | Octave OnSite Construction Planning; OnSite Spool Design |

| Product | What it designs or checks | Where rules come from | AI, dated | Price, buyers |
|---|---|---|---|---|
| **Forte 3D** [44] | 3D plants for "medium to large scale mega projects"; "rule implementation"; "simplified error-checking" | Forte Reference Data: corporate and project material libraries (piping specs) | None on the page | Talk to sales. "Over 90% of the leading plant engineering companies and owner-operators."\* |
| **Facets P&ID** [41] | Data-centric P&IDs; Design Validation checks a P&ID "against isometrics" (attributes and topology) | "Engineering standards, safety practices and owner requirements". The page doesn't say whether customers write their own rules. | **On the page:** none. **2026-02-27 webinar** (Hexagon staff): SPID2GRAPH turns Smart P&IDs into graphs; one use case is "agentic AI to check … if it complies with engineering rules". An "upcoming" SPID2DEXPI exports DEXPI 2.0 [50]. Shipping status unknown. | Quote. Chemicals, oil and gas, power, pharma, EPCs. |
| **Facets Instrumentation** [42] | Instrument index, specs, loops, wiring, calibration; "a rules engine checks designs as you work" | Not named; "industry standards" | None on the page | Quote |
| **Facets Electrical** [43] | Loads, cables and equipment as "a single source of truth"; cable sizing; schedules; "NEC tray fill validation"; links to ETAP for analysis | "Standards and safety requirements"; NEC named for tray fill only | None on the page | Quote |
| **InConcert Assistant** [45] | AI over engineering data: tags, datasheets, P&IDs, HAZOP studies, 3D. "Precise, cited answers with full data traceability"; flags "compliance gaps and inconsistencies" | The owner's own data | Native to InConcert; "nothing leaves your environment". No date or MCP on the page. | Not stated |

**Octave AI overall:**
- **Octave Assist** runs "more than 2 million assists a day" [47].
- **Aria,** the multi-agent framework, is "in private preview". Customers have no date [47].
- Nine agents launched in Q2 2026 [47].
- Management has "not 100% decided how to monetize AI" (call on 2026-08-12) [47].
- **CoLabs** (2026-07-16) builds agentic workflows with Bechtel and Fluor in 11-week engagements [48].
- **MAIRE** (2026-08-31) uses Forte, OnSite, Loop and InConcert with a "humans in the loop approach" [49].
- **MCP:** none of these sources mentions it.

## 4. Inkscape and similar free vector editors

| Tool | What it is (dated) | Used for technical diagrams? | Checks | AI and MCP | Price |
|---|---|---|---|---|---|
| **Inkscape** [52–57] | GPL-2.0+. Stable 1.4.4 (2026-05-06). The 1.5 milestone is open, 81% done, due 2027-05-31 [53]. The project hired developers for 1.5 in 2026-03†. Python extensions; SVG native, with Inkscape-specific additions (live path effects, node types). | Yes. Wikipedia describes "artistic and technical illustration, including diagrams and flowcharts" [52]. Extensions add circuit symbols (`circuitSymbols`, GPL-3.0, IEEE Std 315 or IEC styles; last changelog 2022-02-19) [57], logic gates (IEEE 91), ray optics and LaTeX labels. | None. Pictures only; a symbol is a group of paths with no connection or rating. | **No official AI.** Community MCP servers: `inksmcp` 0.3.0 (MIT, 2026-10-06, beta: shapes, connectors, charts, PNG previews for the agent) [54]; `mcpinkscape` 0.0.2 (BSD-3, 2026-09-13, alpha, no raw SVG or shell access) [55]; `tinta-mcp`† [56]. None is affiliated with the Inkscape project. | Free |
| **draw.io** (diagrams.net) [59, 60] | Diagram editor (XML; exports SVG, PNG, PDF) | Widely used for network, P&ID-style and flow diagrams (earlier round, 2026-10-28) | Shapes with connections, but no engineering checks | "Generate" makes a diagram from text. An **official MCP server** (`jgraph/drawio-mcp`, Apache-2.0) lets an AI model create and open diagrams in XML, Mermaid or CSV. | Free |
| **QElectroTech** [58] | GPL electrical-diagram editor, version 0.100 (date not shown); "over 8000 symbols"; also plumbing, hydraulics, pneumatics, P&ID | Yes, for industrial electrical schematics | Not described | None on the page | Free |
| **Affinity** (Canva) [61] | Vector, pixel and layout in one app; "free for individuals" (start date not on the page) | General design; opens DWG with layers | None | AI tools only on a paid Canva plan | Free; AI paid |
| **Graphite** [62] | Open-source, node-based procedural vector editor; alpha (desktop builds at release candidate) | Not aimed at technical diagrams | None | None | Free |
| **Penpot** [63] | Open-source UI design platform; open MCP server marked "New" | UI and design systems, not engineering | None | MCP server; "any agent or LLM" | Free sign-up |

**Who uses them.** Integrators, students and small firms draw one-lines, panel layouts, network diagrams and P&ID-style sketches in Inkscape, draw.io or QElectroTech. These tools are free and need no CAD licence. I found no survey that counts them; this is unverified. The symbols are pictures, so nothing ties a drawn breaker to a rating or a drawn pipe to a pressure class.

**SVG as interchange:**
- SVG is a good *output*. Hyleon already renders sheets as SVG (`docs/formats/mcp.md`, `docs/PRODUCT.md`).
- It is a weak *input*. Meaning lives in groups, ids and labels that each author makes up. Inkscape adds its own namespaces.
- Importing a hand-drawn SVG means inferring a model, so every element would be AI-inferred until a person confirms it. That matches phase 8's "Import" item in `docs/ROADMAP.md`.

## 5. Where each overlaps Hyleon

Hyleon's packs: `site-basics`, `motor-branch-circuit`, `modbus-drive-bus` and `pump-discharge-spool`. Its base packs cover I/O loops, site networks, pump stations, tanks, dosing, standby power and commissioning, with a water and wastewater overlay (Ten States, NFPA 820) (`docs/ROADMAP.md`).

| Hyleon area | Civil 3D | Bentley | Octave | Vector editors |
|---|---|---|---|---|
| **Pump station and wet well** | Pipes, structures and inverts up to the station; InfoDrainage | OpenFlows: system curves, pump duty, transients (HAMMER), pump energy scheduling (Darwin) | n/a | Sketches only |
| **Buried pipe at the station boundary** (depth, cover, material) | Cover rules, pressure checks | Sewer and water constraints | n/a | n/a |
| **Pump discharge spool and piping specs** | n/a | OpenPlant: spec-driven piping, AWWA catalogs | Forte 3D and Reference Data; Aspect Pipe Stress | Sketches only |
| **P&ID** | n/a | OpenPlant PID | Facets P&ID with Design Validation | draw.io and QElectroTech drawings |
| **Instruments, I/O, loops** | n/a | n/a | Facets Instrumentation: index, loops, wiring | Drawings only |
| **Motor circuits, cables, loads** | n/a | Substation+ (upstream, utility side) | Facets Electrical: loads, cable sizing, NEC tray fill | QElectroTech schematics |
| **Control panels (UL 508A, NEC)** | None | None | None found | Drawings only |
| **OT and site networks** | None | None | None | draw.io drawings |
| **Energy and cost** | None | OpenFlows pump energy and scheduling (water only) | None found | None |
| **Drawings from one model** | Yes, inside Civil 3D | Yes, inside each tool | Yes, inside each tool | No model at all |

**Reading the table:**
- The civil tools own the ground and the buried networks.
- The plant tools own large-plant P&ID, 3D piping, instrument and electrical databases, but are sold by quote to EPCs and owner-operators.
- Hyleon's original ground has no competitor here: panels, controls, OT networks and the pump station as one checked job. The nearest is Facets Instrumentation and Electrical, at enterprise scale.

## 6. What Hyleon could hand off or exchange

These are options for Dan, not decisions.

| Format | Who writes and reads it | What Hyleon could take or give | Status and caveats |
|---|---|---|---|
| **LandXML** | Civil 3D†, OpenRoads, OpenFlows Sewer [25, 32] | **Take:** existing ground and invert levels at the station; incoming pipe size and material, recorded as values from that file. **Give:** little. | Old (1.2) and road-focused; the landxml.org page didn't load. Pressure networks are poorly covered† |
| **IFC 4.3** | Civil 3D (plug-in, pipe and pressure networks)†, OpenRoads, OpenPlant (2026 export carries its schemas) [16, 25, 35] | **Give:** station equipment, panels and pipe spools as objects for coordination. **Take:** pipe-network geometry. | IFC 4.3 is ISO 16739-1:2024† (iso.org refused the fetch). Pairs with IDS and BCF (appendix 3). |
| **DEXPI 2.0.1** | Facets P&ID via SPID2DEXPI ("upcoming") [50], AVEVA and Aucotec (earlier round); OpenPlant not stated | **Give and take:** P&ID topology and tags | 2.0.1 released 2026-09-30, Creative Commons licence [51]. Already on Hyleon's phase-7 list. |
| **SWMM, and EPANET†** | OpenFlows Sewer (SWMM) [32]; EPANET import in WaterGEMS† | **Take:** duty point, static head, force-main pressure, recorded as calculated values with their source model. This mirrors the existing handoff to power-study tools. | Open formats; not checked further |
| **SVG** | Inkscape, draw.io, every browser | **Give:** sheets (already planned). **Take:** a hand-drawn diagram imported as AI-inferred elements (phase 8). | Hyleon's SVG could carry model ids so markups can be traced back. Edits made in Inkscape must never become the source (truth 2). |
| **MCP, side by side** | Bentley (seven servers), the community Civil 3D server, the draw.io and Inkscape servers | One agent drives a specialist tool through its MCP server. Hyleon's MCP server checks the result against packs and records where each value came from. | No vendor here checks another vendor's output. This is the connecting-layer slot. |

**Handoff table.** `docs/PRODUCT.md` lists EDA, 3D CAD and power-study tools. This research suggests two more rows: **hydraulic modelling** (OpenFlows, InfoDrainage, InfoWorks, EPANET, SWMM) and **civil site design** (Civil 3D, OpenRoads, OpenSite). That's Dan's call.

## 7. Gaps none of them fill

- **Questions first.** No tool asks what must be known before design. Civil 3D asks for a design speed only because superelevation needs it.
- **A source on every limit.**
  - Civil rules are editable XML or VB defaults. Criteria files carry tables, not clause citations per value.
  - Bentley constraints are numbers the user types in.
  - Octave says only "engineering standards, safety practices and owner requirements".
- **Checks across tools.** Each vendor checks its own model. Nothing checks the civil hydraulic model against the station's pump, panel and P&ID, or the P&ID against the panel's I/O. The exception is Octave's P&ID-against-isometric check, which stays inside Octave.
- **Checking an outside AI's output.**
  - Bentley's MCP servers let any agent *act*, but "MCP does not validate engineering results by itself" [21].
  - Substation+ checks a model against uploaded criteria with AI, inside Bentley, for substations [36].
- **Open, tested packs.** Agency workspaces (DOT ORD workspaces) and design-criteria files are the nearest things to packs. They are tool-specific, untested as packs, and unsourced per value.
- **Small firms.** The plant tools are sold by quote to EPCs. Small integrators draw in free vector editors that check nothing.

## 8. How close each comes to a connecting layer

The test: researched knowledge, plus the user's model, plus what an AI produces, gives consequences and gaps.

| Product | Researched knowledge | User's model | AI output checked | Consequences and gaps shown | Closeness |
|---|---|---|---|---|---|
| Civil 3D (rules, criteria, Assistant) | Partly: AASHTO tables; editable defaults | Yes, its own | No; the Assistant checks drawing standards only | Warnings at the object | Low–medium |
| OpenRoads, OpenSite, OpenFlows | Partly: agency workspaces; user constraints | Yes, its own | No | Warnings; the solver relaxes lower-priority constraints | Low–medium |
| OpenSite+ | Constraints in objects; no standard named | Yes | Generative AI inside the tool | Ranked scenarios | Medium (in-tool) |
| **Substation+** (early access) | Uploaded criteria documents, read by AI | Yes | Yes, inside the tool | Conflicts flagged; "traceable" | **Medium–high** in intent, closed, one domain |
| Bentley MCP servers | No | Yes, through the API | They let agents act; no validation | Engine results (STAAD) | Medium as a channel, not a checker |
| Facets P&ID, Instrumentation, Electrical | Owner and standard rules, closed | Yes, its own | Not yet (graph rule checks shown 2026-02) | In-tool validation | Low–medium |
| InConcert Assistant | Owner data with cited answers | Yes | Answers, and flags compliance gaps | Cited, traceable answers | Medium, closed, enterprise |
| Inkscape, draw.io, QElectroTech | None | A picture, not a model | No; MCP lets AI draw more pictures | None | None (the opposite) |

**What this means:**
- **Bentley is moving fastest toward Hyleon's idea.** Its CTO talks about "verifiable" work, its MCP servers are open to any agent, and Substation+ checks a model against criteria.
- **Octave is closest on sourced answers,** but its AI stays inside its own platform.
- **Autodesk is behind on civil MCP,** but it plans an agent that checks drawings "against known standards and regulations"†.
- **None of the three publishes the boundaries as an open, sourced, tested artefact.** That is still the unclaimed piece the synthesis named.

## 9. Decisions only Dan can make

1. **What "site" covers.**
   - **A.** Hyleon stops at the station or building and its connections, and reads civil data through LandXML or IFC.
   - **B.** Hyleon adds packs for buried utilities at the boundary (cover, material, crossings), but not grading or roads.
   - **C.** Hyleon takes on civil site design.
   - Civil 3D and Bentley are strong in C.
2. **New handoff rows.** Whether to add hydraulic modelling (OpenFlows, InfoDrainage, EPANET, SWMM) and civil design (Civil 3D, OpenRoads) to the handoff table, with their results recorded as calculated values, like ETAP's.
3. **Which formats, and when.**
   - LandXML read in an early phase, or later.
   - IFC 4.3 export, and when.
   - Whether DEXPI moves earlier than phase 7, given 2.0.1 (2026-09-30) and Octave's SPID2DEXPI.
4. **Hand-drawn SVG import.**
   - Whether a diagram drawn in Inkscape, draw.io or QElectroTech can become a starting model, every element marked AI-inferred. If so, which phase (phase 8 now).
   - Or Hyleon refuses pictures as input.
5. **SVG sheets with model ids.**
   - Sheets could carry ids so a markup made in Inkscape traces back to an element (as a markup, never as the source).
   - Or sheets stay plain output.
6. **Plant buyers.**
   - Whether Hyleon courts EPC and owner-operator users of OpenPlant, Forte and Facets (crowded, sold by quote).
   - Or stays with integrators and small firms, who draw in free tools today.
7. **Packs from agency standards.** Whether Hyleon writes packs that cite public agency standards (for example a state DOT's utility criteria, or Ten States). Licensing and citation rules may differ for each agency.
8. **Driving other tools through MCP.**
   - Whether Hyleon's MCP server is designed to run side by side with vendor MCP servers (Bentley's now; Autodesk's later) and record their results as sourced values.
   - Or it stays standalone.
9. **Words.** Civil 3D already uses "design criteria", "design checks", "rules" and "rule sets", and Octave uses "rules-driven". Whether Hyleon's terms ("limits", "checks", "packs") need to stay clear of these.

## 10. Not verified or not covered

- **Not verified (†):**
  - Civil 3D LandXML and IFC 4.3 coverage in 2027
  - Civil 3D pressure-network depth check
  - Civil 3D price
  - Autodesk's 2025-11-17 MCP plan for the Civil 3D Assistant
  - The AU 2026 MCP class
  - HMK Pilot
  - OpenRoads standards details
  - The SewerGEMS constraint priority order
  - EPANET import in WaterGEMS
  - IFC 4.3 as ISO 16739-1:2024
  - `tinta-mcp`
  - Inkscape's 1.5 hiring
- **Not found:**
  - An official Civil 3D, OpenRoads, OpenFlows, OpenPlant or Octave MCP server
  - InfraWorks 2027 release notes
  - Octave prices
  - OpenSite+ and Substation+ prices
  - Whether SPID2DEXPI has shipped
  - Whether OpenPlant reads or writes DEXPI
- **Not covered, because the search budget ran out:**
  - LibreOffice Draw and other diagram editors
  - Surveys of how many engineers draw in Inkscape
  - AVEVA's civil and plant AI (covered in appendix 3 and the earlier round)
  - Autodesk Plant 3D 2027 (earlier round only)
  - 12d Model, Trimble Business Center and other civil tools outside the brief

## 11. Sources

1. https://www.autodesk.com/blogs/aec/2026/04/02/whats-new-in-civil-3d-2027/ (403; search summary only)
2. https://www.symetri.co.uk/insights/blog/what-s-new-in-civil-3d-2027-a-guide-to-the-latest-features/ (2026-03-25)
3. https://blog.hagerman.com/whats-new-in-autocad-civil-3d-2027-practical-enhancements-for-everyday-design-work (2026-07-13)
4. https://help.autodesk.com/cloudhelp/2024/ENG/Civil3D-UserGuide/files/GUID-4FBF470E-16EC-497A-82A9-8E84580F3963.htm
5. https://help.autodesk.com/cloudhelp/2023/ENU/Civil3D-UserGuide/files/GUID-EA890449-5A77-4FAB-8185-86396231CC5E.htm
6. https://help.autodesk.com/cloudhelp/2024/ENG/Civil3D-UserGuide/files/GUID-A35495A0-3C0B-468B-A632-67E75861755E.htm
7. https://help.autodesk.com/cloudhelp/2024/ENG/Civil3D-UserGuide/files/GUID-98DDEE62-94A8-4A15-BE95-575FD9DFC35D.htm and https://help.autodesk.com/cloudhelp/2025/ENU/Civil3D-Tutorials/files/GUID-E781CDDC-6D6C-4DEB-AF87-74A758671FAF.htm (search summary only)
8. https://www.autodesk.com/blogs/aec/2025/11/17/agentic-ai-developments-for-autodesk-assistant-in-civil-3d/ (403; search summary only)
9. https://www.engineering.com/au-2026-autodesks-ai-assistant-is-evolving/
10. https://aecmag.com/civil-engineering/autodesk-pushes-forma-upstream-and-connects-civil-3d/ (2026-09-15)
11. https://aps.autodesk.com/blog/civil-3d-data-exchange-directly-autodesk-forma-now-generally-available (2026-09-08)
12. https://www.autodesk.com/autodesk-university/class/MCP-on-the-Autodesk-Platform-Three-Production-Servers-Across-the-Full-Stack-2026 (search summary only)
13. https://github.com/barbosaihan/civil3d-mcp
14. https://mcpservers.org/servers/hmktools-com-revit-mcp (403; search summary only)
15. https://www.autodesk.com/solutions/aec/civil-3d-subscription-faq (403; search summary only)
16. https://www.autodesk.com/blogs/aec/2023/05/12/whats-new-in-openbim-and-infrastructure-ifc-4-3-for-civil-3d/ (search summary only)
17. Appendix 3 of this round: [3-model-code-checking.md](3-model-code-checking.md) (Autodesk's MCP list, 2026-04-15)
18. https://www.bentley.com/en/infrastructure-ai/mcp-servers/
19. https://www.bentley.com/en/lp/build-with-us/
20. https://blog.cadalyst.com/architecture-infrastructure-construction-solutions/meets-ai-infrastructure-the-rise-of-open-engineering-workflows (2026-06-18, sponsored by Bentley)
21. https://logisticsviewpoints.com/2026/06/08/bentleys-mcp-server-shows-how-ai-can-work-in-engineering-without-guessing/ (2026-06-08)
22. https://www.pbctoday.co.uk/news/digital-construction-news/precision-over-plausibility-ai-in-infrastructure-demands-a-different-standard/162724/ (2026-06-08)
23. https://blog.cadalyst.com/architecture-infrastructure-construction-solutions/bentley-announces-2026-yii-award-winners (2026-10-08)
24. https://www.bentley.com/en/news/bentley-systems-advances-infrastructure-ai-with-new-applications-and-industry-collaboration/ and https://www.bentley.com/wp-content/uploads/infrastructure-ai_vfinalweb_10.15.25_.pdf (2025-10-15; search summary only)
25. https://www.bentley.com/software/openroads-designer/
26. https://www.bentley.com/en/blog/openroads-designer-2026-and-the-roads-behind-the-summer-of-football/ (search summary only)
27. https://docs.bentley.com/LiveContent/web/OpenRoads%20Designer-v2024.0.2/Help/en/topics/243631/GUID-6105A5EE-FDBB-4D27-BA54-D94AABB3ADC9.html ; https://www.dot.ga.gov/PartnerSmart/DesignSoftware/OpenRoads/ORD%20Design%20Guidelines.pdf ; https://portal.ct.gov/DOT/Engineering-Applications/CTDOT-CONNECT-DDE-ebook-Volume-003---OpenRoads-Designer-Roadway-Modeling/2_1-Horizontal-Alignments (all search summary only)
28. https://www.bentley.com/software/opensite-designer/
29. https://www.bentley.com/software/opensite-plus/
30. https://www.bentley.com/en/blog/introducing-openflows-2026/ (2026-05-29)
31. https://bentley.com/en/products/openflows-water
32. https://www.bentley.com/software/openflows-sewer/
33. https://DOCS.BENTLEY.COM/LiveContent/web/Bentley%20SewerGEMS%20SS5-v2/en/GUID-73F4F92E-4F53-4962-98DA-096CED12833B.html and https://communities.bentley.com/products/hydraulics___hydrology/w/hydraulics_and_hydrology__wiki/18162.order-of-automated-design-priorities-for-stormcad-gvf-rational-and-sewercad-gvf-convex (search summary only)
34. https://bentley.com/software/openplant-modeler
35. https://www.bentley.com/en/blog/openplant-2026-plant-design-automation-collaboration/ (2026-09-29)
36. https://www.bentley.com/en/blog/how-agentic-ai-is-poised-to-reshape-substation-design/ (2026-05-20)
37. https://www.bentley.com/en/products/substation-plus/
38. https://github.com/orgs/iTwin/repositories?q=mcp
39. https://www.octave.com/legal/product-naming (updated 2026-04-08)
40. https://itbrief.co.uk/story/octave-begins-trading-on-nasdaq-after-hexagon-spin-off (2026-05-29)
41. https://www.octave.com/products/schematics-3d-modeling/facets/pid
42. https://www.octave.com/products/schematics-3d-modeling/facets/instrumentation
43. https://www.octave.com/products/schematics-3d-modeling/facets/electrical
44. https://www.octave.com/forte
45. https://www.octave.com/products/engineering-information-management/inconcert/assistant
46. https://www.octave.com/newsroom/press-releases/2026/second-quarter-2026-financial-results (2026-08-12)
47. https://www.webull.com/news/15393658071507968 (Q2 2026 call transcript, 2026-08-12)
48. https://www.octave.com/newsroom/press-releases/2026/octave-launches-colabs (2026-07-16)
49. https://www.octave.com/newsroom/press-releases/2026/octave-and-maire-collaborate-on-the-application-of-ai-across-engineering-and-construction-workflows (2026-08-31)
50. https://www.icheme.org/knowledge-networks/communities/special-interest-groups/process-management-and-control/events/27-02-2026-using-pids-with-ai-for-process-insights/ (2026-02-27)
51. https://dexpi.org/specifications/
52. https://en.wikipedia.org/wiki/Inkscape (last edited 2026-10-30)
53. https://gitlab.com/groups/inkscape/-/milestones/16
54. https://pypi.org/project/inksmcp/
55. https://pypi.org/project/mcpinkscape/
56. https://pypi.org/project/tinta-mcp/0.2.3/ (search summary only)
57. https://github.com/fsmMLK/inkscapeCircuitSymbols
58. https://qelectrotech.org/
59. https://www.drawio.com/docs/manual/insert/insert-generate/
60. https://github.com/jgraph/drawio-mcp
61. https://www.affinity.studio/
62. https://graphite.art/
63. https://penpot.app/
