# Appendix 1: Competitors and pricing (agent report, 2026-10-28)

The research agent's full report, verbatim. It's never edited. The synthesis is in [../2026-10-28-market.md](../2026-10-28-market.md).

**The brief:**
- Map the products relevant to Hyleon: disciplines, whether one model spans them, AI features, exports, platform, price and target.
- Cover the closest competitors in depth.
- List the gaps.

---

I mapped 35 products. Under 2026-10-28 conditions, no electrical, controls or piping design tool combines all four of Hyleon's core ideas, but two come close. WSCAD ELECTRIX OS covers the requirements and checks idea, and it is not on sale until 2027. Outerport, a small startup, covers checked design models, though it sells to enterprises.

Notes:
- Prices were read from vendor pages on 2026-10-28 unless dated otherwise. "Quote only" means no public price.
- \* = vendor claim. ~ = customer size I inferred from price or positioning; the vendor doesn't state it.
- The session's web-search limit ran out near the end. Cells marked "unknown" were not guessed.
- autodesk.com refused direct fetches, so the AutoCAD price comes from Autodesk pages as shown in search results.
- Changes since your brief:
  - EPLAN Platform 2027 shipped in 9/2026.
  - Hexagon's Smart tools now belong to Octave, which was spun off and listed in 5/2026.
  - The SEE Electrical page now redirects to etap.com.
  - The Autodesk Build URL now redirects to "Forma Build".

## 1. Product map

| Product [src] | Disciplines (simulation) | One model? | AI 2025–26 | Exports | Platform | Price/seat/yr | Target |
|---|---|---|---|---|---|---|---|
| Siemens COMOS [1,2] | Process/P&ID, automation, operations; SIMIT simulation link (announced) | Yes* | COMOS AI (specs and diagrams from text)*, announced 2024 for 2025; release unconfirmed | unknown | unknown | Quote only | ~Enterprise |
| Aucotec Engineering Base [3–6] | Process, I&C, electrical | Yes* | Engineering Advisor (6/2026); AI Diagram Import (9/2026) | Excel, DEXPI, AutomationML, DWG | unknown | Quote only | ~Enterprise |
| AVEVA Diagrams/Engineering [7] | P&ID, engineering data, 3D | Yes, "data-centric"* | Assistant, generative pipe routing, predictive design (1/2026) | unknown | unknown | Quote only | ~Enterprise |
| Hexagon Smart P&ID / Instrumentation (now Octave) [8,9] | P&ID; I&C | unknown | P&ID-to-graph for LLM chat and agent rule checks, shown 2/2026, status unclear | DEXPI 2.0 "upcoming" | unknown | Quote only | ~Enterprise |
| EPLAN Platform 2027 (Electric P8, Pro Panel, Preplanning, Cogineer) [10–15] | Electrical, fluid, 3D panel, P&ID preplanning; simulation via Rockwell Emulate 3D import | Within platform* | Eplan Copilot (knowledge answers, part lookup); Cogineer is rule-based | unknown | Desktop + Eplan Cloud | Quote only | ~Mid–Enterprise |
| AutoCAD Electrical [16–20] | Control schematics, panel layouts | No (separate toolsets) | Autodesk Assistant preview; built-in MCP server serves only that assistant, mostly read-only | DWG; others unknown | Desktop | $2,095, includes all toolsets (US) | ~SMB–Enterprise |
| SOLIDWORKS Electrical [21] | Schematics, 3D routing | unknown | SOLIDWORKS 2026 AI drawing generation*; nothing Electrical-specific | unknown | Desktop + 3DEXPERIENCE | Quote only | unknown |
| WSCAD ELECTRIX AI [22–24] | Electrical, cabinet, P&ID, fluid, installation, building automation | "Same platform"* | Copilot commands, cabinet layout from schematic, checks*; US/UL 508A edition 4/2026 | DXF/DWG, PDF, Excel, Komax/Rittal data | Desktop | From £999 (UK page) | Small–Enterprise* |
| Zuken E3.series [25] | Schematic, harness, formboard, panel | unknown | None in 2026 release | Excel/Word reports | Desktop; browser viewer | Quote only | ~Mid–Enterprise |
| SEE Electrical [26] | Electrical, panels | unknown | None found | PDF, DWG, DXF, DGN, EMF | Windows | Quote only | Installers, panel builders |
| PCSCHEMATIC Automation [27–29] | Electrical, automation, panels, installation | unknown | AI Assistant: lookups, validation, lists; cannot edit (DKK 1,740/yr) | PDF; Excel/CSV lists | Desktop | DKK 9,865 (limited) / DKK 17,000 | Electricians, panel shops |
| QElectroTech [30] | Electrical; fluid, P&ID, solar symbols | No | None | DXF; others unknown | Win/Mac/Linux | Free (GPL) | ~Individuals |
| ProfiCAD [31] | Electrical schematics | No | None found | unknown | Desktop | $277 one-time, no subscription | ~Small |
| Rittal RiTherm / RiPanel [32] | Enclosure heat calculation; enclosure configurator | No | None | unknown | Web (RiTherm via Eplan Cloud) | RiTherm free; RiPanel unknown | Any |
| Schneider Ecodial [33] | Low-voltage sizing | No | None | unknown | Desktop | Free | Any |
| ABB DOC (e-Design) [34] | Single-line diagrams, device selection, protection coordination | No | None | unknown | Windows | unknown | Any |
| Siemens SIMARIS 27 [35] | Network sizing, trip curves, budgeting | No | None | BIM data | Desktop + web | Free basic; paid editions via partners | Any |
| ETAP 2026 [36] | Power studies: load flow, short circuit, arc flash, transients | Electrical digital twin* | AI Copilot, AI auto-complete for one-lines* (5/2026) | unknown | unclear | Quote only | Utilities, data centers, industry |
| SKM PowerTools [37] | Fault, load flow, coordination, arc flash | Electrical only | None | unknown | Windows + cloud portal | Quote only | Electrical engineering firms |
| EasyPower (Bentley) [38] | Arc flash, short circuit, coordination, power flow | Electrical only | None | unknown | Windows | Quote only | Power engineers |
| AutoCAD Plant 3D [16,18,39] | P&ID, 3D piping | unknown | Autodesk Assistant (all of AutoCAD) | DWG | Desktop | Inside the $2,095 AutoCAD | ~SMB–Enterprise |
| Bentley OpenPlant Modeler [40] | 3D piping, equipment, isometrics | ISO 15926 model* | None for OpenPlant; Bentley-wide MCP offering mentioned | DGN, DWG, DXF, IGES | Windows | From $5,839 (US) | ~Mid–Enterprise |
| NetBox [41–44] | Network/data-center records | Network only | Copilot released 2/2026; MCP for Claude, Cursor, ChatGPT | API | Self-host or cloud | Open source free; cloud quote only | Any |
| Visio [45] | General diagrams | No | None confirmed | unknown | Web; desktop in Plan 2 | $60 / $180 | Any |
| Lucidchart + Lucid AI [46,47] | General diagrams | No | Lucid AI; MCP server for any client | unknown | Web | $108 | Any |
| draw.io [48] | General diagrams | No | Generate tool (8/2026), free hosted model or own keys; MCP server | unknown | Web/desktop | Free | Any |
| Cisco Network Sketcher [49] | L1–L3 network diagrams, device tables | One master file | Local MCP server; AI context files | SVG, PPTX, XLSX, draw.io | Local/web/desktop | Free (Apache-2.0) | Network engineers |
| Bluebeam Revu [50] | PDF markup and review | No | Max tier: MCP, AI drawing review/compare | PDF | Windows, web, mobile | $260 / $330 / $440 / $590 (Max, introductory) | AEC firms |
| Procore [51,52] | Construction management | No | Agents (search, submittal review, RFIs); human approval; cited sources | n/a | Web/mobile | Quote only (set by construction volume) | Contractors |
| Fieldwire [53] | Field management | No | unknown | unknown | Web/mobile | $468 / $768 / $1,068 | Field teams |
| Autodesk Build [54] | Construction management | No | unknown | unknown | Web/mobile | unknown | unknown |
| ChatDiagram P&ID Maker [55,56] | P&ID drawings from text | No | Text to P&ID; no engineering checks | SVG, PNG, PDF | Web | Free; $15.90 or $49.90 per month | Individuals |
| Eraser [57] | Software diagrams | No | AI diagrams; MCP | PDF, PNG, SVG, MD | Web | $540 | Software teams |
| Napkin [58] | Visuals from text | No | Text to visuals | PNG, PDF, PPT, SVG | Web | $9 or $22 per month | Individuals |
| Outerport (YC S24) [59–61] | Electrical/control panels, P&ID; control-logic simulation | Yes* | Agents generate and check designs*; digitized values cite their source* | PDF, DWG, XLSX, CSV, DEXPI, API | Web + API | Quote only | Enterprise* |

## 2. Closest competitors

**EPLAN Cogineer (+ Copilot).** Cogineer uses rules, not AI. In its Designer module, teams group Eplan macros into "Macro-Typicals" and "Typical-Groups" with dependencies. In Project Builder, they enter configuration values and generate Electric P8 or Fluid documents; the libraries live in the Eplan Cloud [14]. In 2017 Eplan claimed 30–80% engineering savings* [15]. Cogineer only reproduces variants defined in advance. It does not ask open requirement questions, carry limits with their sources, or check designs. Platform 2027 extends this generation to 3D panels [10]. Eplan Copilot adds knowledge answers and part lookup; bills of materials and wiring lists are on the roadmap [11,12]. It is Eplan's own assistant; no MCP is mentioned.

**WSCAD ELECTRIX OS** [62–64]. This is the closest concept to Hyleon. It premiered on 2026-10-13 and goes on sale in 2027; until then WSCAD is taking design partners.
- It breaks specifications into verifiable requirements and records where each one came from.
- When information is missing or conflicting, it asks clarifying questions and leaves the affected steps unresolved.
- WSCAD's own agents propose power, control and safety concepts. These are checked for structure, completeness and inconsistencies before they enter the project model.
- Every step is logged, and decision records cite the relevant codes and the approving engineer\*.
- The AI cannot change the knowledge base, and no public cloud is required.

It covers electrical design only. ELECTRIX AI is the first CAD system it feeds. It targets OEMs, machine builders, integrators and panel builders with recurring variants. No price has been published, and there is no MCP or outside-agent access.

**AutoCAD Electrical** [16–20]. It is included with AutoCAD at $2,095/yr (US). Its AI path is Autodesk Assistant, still a tech preview. AutoCAD 2027 bundles a local MCP server, but a third-party review (10/2026) found that only Autodesk's assistant can use it, on Windows only, and mostly read-only. Autodesk's public MCP server serves help documentation, not drawings. AutoCAD Electrical has no layer for requirements, value sources or rule checks.

**Outerport (YC S24)** [59–61]. This is the closest AI-native product on the market. YC describes it as design automation for industrial electrical controls, written in an electrical design language "backed by thousands of unit tests." Its website claims\*:
- Specifications and drawings become a structured design model.
- Outputs include drawing sets, bills of materials, terminal tables and calculations.
- P&IDs are checked for continuity, tags and instrument coverage.
- Control logic is simulated, and checks re-run after each revision.
- Digitized values cite the area of the sheet they came from.

It is sold to enterprises (Daikin is named). It lists its investors but not the amounts; an aggregator shows a $500K pre-seed round (9/2024). I found no MCP support, network coverage or public price.

**Other AI-native startups.**
- **Endra:** AI design of building mechanical, electrical and plumbing systems (electrical models, single-line diagrams, schedules, load calculations; Revit sync). $50M Series A led by a16z, 2026-06-03 [65,66].
- **Operon (YC S26):** turns P&IDs and plant documents into a model of the plant you can query; funding undisclosed [67].
- **Structured AI (YC F25):** AI review of drawing sets against building codes; $4.2M seed [68].
- **Augmenta:** AI agent for building electrical design; $10M seed, 3/2025 [69].
- **Solvera and Marengo:** sell AI-assisted engineering services, not software. Solvera raised a $6.3M pre-seed in 10/2026; Marengo is in YC S26 [70,71].

I found no startup aimed at UL 508A panel shops or small integrators.

## 3. Gaps

- **Verification that works with any AI agent: not found in design tools.** No electrical, controls or piping design product lets any MCP agent propose model changes that the product checks before a person confirms. Open MCP access exists only in tools for diagrams and records (Lucid, draw.io, Eraser, Network Sketcher, NetBox, Bluebeam Max). Design vendors (Eplan, WSCAD, ETAP, AVEVA, Autodesk) use their own assistants.
- **Cross-discipline site systems for small teams: not found.** Multi-discipline single-model suites are quote-only enterprise products. WSCAD (from £999/yr) is the broadest small-team tool, but it leaves out networks, power studies and 3D piping. Network tools are not connected to panel or power-study models.
- **Every value carrying its source: not on sale.** The nearest are ELECTRIX OS requirement origins and decision records (2027) and Outerport's source-cited digitized values\*. Procore and Structured AI cite sources on answers and findings, not on design values.
- **Questions answered before design starts: only ELECTRIX OS (2027, electrical only).** Cogineer and COMOS "variant drivers" ask for configuration values of pre-built designs, not open requirement questions.
- **Per-job scenario packs (questions, sourced limits, checks, deliverables): not found as a product.** The nearest are Cogineer configurators and Endra's "Playbooks" of firm standards [65].
- **Vendor calculators stay isolated.** Ecodial, SIMARIS, DOC and RiTherm each check one area using one vendor's parts. Their results do not feed a shared model with sources; RiTherm's link to the Eplan Cloud is a partial exception.

## 4. Sources

1 https://www.siemens.com/global/en/products/automation/industry-software/plant-engineering-software-comos.html
2 https://www.automation.com/en-us/articles/june-2024/siemens-accelerates-hydrogen-ramp-up-genai
3 https://www.aucotec.com/en/products/engineering-base-plants/feed
4 https://www.aucotec.com/en/trend-topics/cfihos-and-engineering-base
5 https://www.aucotec.com/fileadmin/user_upload/aucotec/Presse/2026/Engineering_Advisor/PM-AUCOTEC_Engineering_Advisor_EN.pdf
6 https://www.aucotec.com/en/about-us/media/aucotec-makes-legacy-diagrams-usable-for-data-oriented-engineering-with-ai-diagram-import
7 https://www.aveva.com/en/about/news/press-releases/2026/aveva-unveils-new-artificial-intelligence-offering-across-its-unified-engineering-solution/
8 https://www.icheme.org/knowledge-networks/communities/special-interest-groups/process-management-and-control/events/27-02-2026-using-pids-with-ai-for-process-insights/
9 https://itbrief.co.uk/story/octave-begins-trading-on-nasdaq-after-hexagon-spin-off
10 https://www.eplan.com/my-en/about-us/press/now-is-best-to-test-the-eplan-platform-2027/
11 https://timestech.in/the-eplan-platform-2027-has-arrived/
12 https://www.eplan.com/gb-en/about-us/press/eplan-copilot-ai-on-its-way-to-become-a-reliable-knowledge-partner-in-engineering/
13 https://www.eplan.com/us-en/about-us/press/eplan-and-rittal-at-sps-2025-in-nuremberg/
14 https://www.eplan.help/help/cogineercloud/en-us/Content/htm/cogineer_intro.htm
15 https://www.eplan-software.com/company/news/now-available-eplan-cogineer-simply-automating-engineering
16 https://blog.autodesk.io/autocad-2027-and-agentic-ai/
17 https://www.meltflexai.com/blog/autocad-mcp (third party)
18 https://www.autodesk.com/products/autocad/buy
19 https://www.autodesk.com/products/autocad-electrical/overview
20 https://www.digitalengineering247.com/article/autodesk-unveils-product-help-mcp-server
21 https://www.3ds.com/newsroom/media-alerts/dassault-systemes-announces-solidworks-2026-ai-powered-design-and-collaboration-generative-economy
22 https://www.wscad.com/en/electrix
23 https://lp.wscad.com/en/electrical-cad-software/
24 https://www.digitalengineering247.com/article/wscad-unveils-electrix-ai-powered-electrical-cad
25 https://www.zuken.com/us/resource/zuken-announces-e3-series-2026-release/
26 https://etap.com/product/see-electrical
27 https://www.pcschematic.com/
28 https://www.pcschematic.com/da/priser/
29 https://www.pcschematic.com/da/ai-assistant/
30 https://qelectrotech.org/
31 https://www.proficad.com/purchase.aspx
32 https://www.rittal.com/RiPanel
33 https://www.se.com/ae/en/faqs/FA316360/
34 https://new.abb.com/low-voltage/solutions/selectivity/tools-support/doc
35 https://www.siemens.com/en-us/products/simaris/
36 https://etap.com/company/news/2026/05/21/etap-announces-etap-2026-powering-continuous-energy-intelligence
37 https://www.skm.com/
38 https://www.easypower.com/
39 https://www.cad-notes.com/one-autocad-seven-specialized-toolsets/
40 https://bentley.com/software/openplant-modeler
41 https://github.com/netbox-community/netbox
42 https://www.globenewswire.com/news-release/2026/02/10/3235154/0/en/NetBox-Labs-Announces-General-Availability-of-NetBox-Copilot-Delivering-Enterprise-Ready-AI-Grounded-in-Accurate-Infrastructure-Data.html
43 https://netboxlabs.com/docs/mcp/
44 https://netboxlabs.com/pricing/
45 https://www.microsoft.com/en-us/microsoft-365/visio/visio-plans-and-pricing
46 https://community.lucid.co/community-news-and-announcements-9/introducing-the-lucid-model-context-protocol-mcp-server-12230
47 https://lucid.app/pricing/lucidchart
48 https://www.drawio.com/blog/tags/generate/
49 https://github.com/cisco-open/network-sketcher
50 https://www.bluebeam.com/pricing/
51 https://www.procore.com/pricing
52 https://www.procore.com/helix
53 https://www.fieldwire.com/pricing/
54 https://construction.autodesk.com/products/autodesk-build/ (redirects to "forma-build")
55 https://www.chatdiagram.com/tool/p-and-id-maker
56 https://www.chatdiagram.com/pricing
57 https://www.eraser.io/pricing
58 https://www.napkin.ai/pricing
59 https://outerport.com
60 https://www.ycombinator.com/companies/outerport
61 https://www.vcbacked.co/company/outerport (aggregator)
62 https://www.wscad.com/us/wscad-presents-electrix-os/
63 https://www.financialcontent.com/article/bizwire-2026-10-20-wscad-unveils-electrix-os-introducing-engineering-intelligence-as-a-new-category-of-software
64 https://www.wscad.com/us/electrixos/
65 https://www.endra.ai/
66 https://www.dlapiper.com/news/2026/06/dla-piper-advises-andreessen-horowitz-on-usd50-million-investment-in-endra
67 https://www.ycombinator.com/companies/operon
68 https://www.ycombinator.com/companies/structured-ai
69 https://www.constructiondive.com/press-release/20250312-augmenta-secures-10-million-usd-in-funding-to-pioneer-ai-driven-building-1
70 https://bebeez.eu/2026/10/16/norwegian-startup-raises-6-3-million-pre-seed-round-to-build-ai-native-engineering-firm/
71 https://www.ycombinator.com/companies/marengo
