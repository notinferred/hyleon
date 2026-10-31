# Appendix 3: Model checking and code compliance (agent report, 2026-10-31)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-31-scope.md](../2026-10-31-scope.md).

**The brief:** Map model-checking and code-compliance products for buildings, plants and infrastructure (what they check, how their rules are sourced and licensed, AI and MCP, price, buyers, recent news), then show the overlap with Hyleon, the gaps, how rules are kept current across jurisdictions, which products act as a connecting layer, and the decisions only Dan can make.

---

## Summary

- The category has five parts:
  - **Model checkers** run user-set geometry and data rules on IFC or Revit models: Solibri, Navisworks, BIMcollab, Speckle, Forma, AVEVA.
  - **AI plan review** reads drawings against building codes: UpCodes, CivCheck, Archistar, CodeComply, Structured AI, Nomic and Symbium.
  - **Government engines** check submitted models or drawings: Singapore, Japan, Dubai and Estonia.
  - **Open standards:** IDS, the IFC Validation Service and ASHRAE 229.
  - **Code publishers** now sell AI over their own text. ICC launched AI Navigator in 2025-10, and NFPA launched CASI on 2026-01-13.
- Every vendor calls its output a "first pass" that doesn't replace a reviewer. A person approves. That matches Hyleon's truths 4 and 6, so the advisory stance alone won't set Hyleon apart.
- Products handle code text in three ways:
  - **License it.** Archistar and CivicPlus are ICC Code Connect partners.
  - **Publish adopted code as public law, and defend that in court.** UpCodes does this.
  - **Encode the rules and cite the clause.** Solibri, Symbium and CORENET X do this. Hyleon's route is this one.
- Two things weren't found. No product checks what an outside AI agent produced while tracking each value's source. No product checks panel, controls, piping or network models against NEC or UL 508A.
- The building segment is crowded and funded. Authorities are starting to require pre-checks: Honolulu has required one since 2026-09-01, Singapore requires model submissions, and Japan plans checks on the model data itself from 2029.

**Notes**
- All pages were read on 2026-10-31 unless a date is given.
- The web-search budget, shared by all agents in this session, ran out about halfway through. After that I fetched only official URLs I already had.
- † means I saw it only in a search-engine summary, not on the page itself (the page refused fetches or I didn't open it). Treat it as unverified.
- \* means a vendor claim. "unknown" means not found; nothing was guessed.
- autodesk.com, help.solibri.com, buildingsmart.org and denverite.com refused fetches (HTTP 403).

## 1. Model checkers (buildings, plants, infrastructure)

| Product [src] | What it checks | Where rules come from | AI and MCP, 2026 | Price | Buyers | Recent, dated |
|---|---|---|---|---|---|---|
| **Solibri** (Nemetschek) [1–6] | IFC models: clashes, clearances, accessibility (door clearance, handrail continuity, a new "accessible area" rule #248), required information, IDS (rule #244), point cloud vs model (new) | Rule templates shipped by Solibri. Users set the parameters and save rulesets (.cset). Premium adds "rule governance" and company extensions. No code text. | AI Assistant beta in 26.6.0 (2026-06-30), Premium only. It can run checks and create or update rulesets. It runs on "Solibri's AI infrastructure", with no outside data and no named model. A help page reportedly mentions an MCP server for outside AI clients† (page blocked). Community-built Solibri MCP servers exist [6]. | Starter €99, Essential €1,428, Advanced €2,109, Premium €2,772 per licence per year. Security+ by quote. Automation tokens: $250 per 5,000. | Architects, engineers, contractors, BIM managers | 26.4.0 (2026-04-13): new tiers; Office, Site, Anywhere and CheckPoint become legacy products; WebChecker cloud beta. 26.6.0 (2026-06-30): AI assistant; IfcAlignment for infrastructure. |
| **Navisworks Manage** (Autodesk) [7,8,12] | Clashes (hard, clearance, duplicates) across combined models | User-set tests and tolerances. No code content. | None found for 2027†. Autodesk's MCP list (2026-04-15) has no Navisworks server. | $3,128.15/yr at reseller CDW. Simulate $1,110/yr†. Autodesk's own pages blocked. | General contractors, MEP trades, coordinators | 2027: Clash Detective redesign, grouping, more export formats† |
| **BIMcollab** (KUBUS, NL) [9,10] | Zoom: clashes, rule-based clashes, Smart Views (colour by property), IDS checks. Nexus: BCF issues, IDS authoring. | User-made Smart Views, clash rules and IDS, shared through the cloud | None on the pages read | Platform €12.50, €18.75 or €25 per user per month, billed annually. Zoom licences are priced separately (not shown). | BIM coordinators | Plans page footnote "releasing in 2026" (feature not named) |
| **Speckle** [15,16] | Scores every published model version against rules. Rules can follow relationships (door to host wall). Imports IDS and COBie; failures export as BCF. | User-written rules, or imported IDS/COBie. "Define with AI" drafts rules; the user accepts, edits or dismisses each one. | Speckle Intelligence answers questions on model data and shows the query and source models behind each answer. MCP not mentioned. | unknown | AEC data teams | 2026 page, undated |
| **Autodesk Forma** [11–14] | New Drawing Compliance (AU 2026): AI flags drawing issues, such as walls too close or missing dimensions, against **user-built rule sets** (design standards, owner requirements, coordination). Drawing Change Analysis. | User-built rule sets. No code content stated. | Next-generation Autodesk Assistant (agents) in preview. MCP servers on 2026-04-15, all in tech preview: Revit MCP (read), Fusion MCP, Fusion Data MCP and Product Help MCP; InfoWorks MCP coming soon. On 2026-06-17 a "Revit Public MCP Server" became the official server for Revit 2027†. An AU 2026 session pitches MCP as a "governed layer" for agents to run QA/QC checks†. | Forma Site and Building Design prices not found | Architects, owners, contractors | AU 2026 (article dated 2026-09-16): Drawing Compliance, Forma Street Design beta, Civil 3D as a Forma client |
| **AVEVA E3D Design** (plants) [17] | Clashes across disciplines, "enforced component compatibility", user-configured design rules and contract requirements, laser scan vs design | User-configured. No code packs. | "LLM industrial assistant" built in. No MCP. | Quote | Owner-operators, EPC contractors | Undated page |

## 2. AI plan review and code compliance

| Product [src] | What it checks | Code content: source and licence | AI, MCP, human role | Price | Buyers | Recent, dated |
|---|---|---|---|---|---|---|
| **UpCodes** [18–25] | Copilot: code questions, answered with links to the adopted section. Copilot Intelligence (2025-12-24): project memory, compliance checklists. AI Plan Review (launched 2026-06, early access): code review, spec coordination, firm standards, constructability, cross-discipline. Civil review "coming soon" (~4,000 municipalities). | UpCodes publishes adopted code text itself: "11M+ code sections"\*. Plan Review covers 73 jurisdictions and 14 code editions (2026-10-13)\*. This rests on the position that adopted codes are law (see §5). | Multimodal AI\*, model not named. No API or MCP found. "First-pass analysis"; doesn't replace a licensed professional or the authority. | Essentials $39, Professional $59 per user per month, billed annually (Copilot is in Professional). Plan Review is Enterprise only, by quote. | Architects, engineers, contractors, owners (DFW Airport, BDG, Hereford Dooley) | $3.5M Series A (2023-05). 1,300+ plan sets reviewed and 180,000 issues found since June\* (2026-10-13). |
| **Archistar AI PreCheck** (formerly eCheck) [26–29,59] | PDF, CAD or BIM against local zoning and building codes, in 2D or 3D; pass/fail reports | "Smart Packs": California ADU, California Residential Code, IRC Essentials, plus custom rules per city. Listed as an ICC Code Connect partner (licensed ICC content). Who maintains the rules isn't stated. | "AI-powered engine". API terms exist; no MCP. Cities say formal staff review still applies. | Demo or quote | Cities, 30+\* (Surrey, Kingston, Cambridge, Burnaby, Burlington, Ottawa, Austin, Los Angeles County, Cape Coral, Yonkers), and developers, architects and certifiers | Surrey public use from 2026-03-10. Cambridge's rollout is funded by the federal Housing Accelerator Fund. Funding unknown. |
| **CivCheck** (owned by Clariti since 2025-10-08) [30–34,65] | Applicant PDFs, including hand-drawn ones: completeness, zoning, building, fire, accessibility, plumbing, structural. Calculations shown step by step. | Vendor template library built on code families (CBC, CRC). Each city configures the ~200–250 checks it enforces. Rule updates take 2–3 minutes in a "Rules Management System", then ~24 h of testing, with grace periods. | Staff approve every AI interpretation; the AI "should never make a decision directly". **When unsure, it asks the user instead of guessing.** Accuracy 97–98%\*. | Quote. Denver: five-year, $4.6M contract (council 2026-02 to 03)†. Vendor says HUD funds up to $1.5M per city (vendor claim). | Local governments: Honolulu, NYC, San Jose, Denver, Greeley, Chanhassen, NCTCOG, Calgary; Seattle exploring (2026-06) | Honolulu: first residential submissions need a CivCheck watermark from 2026-09-01†. TXShare (Texas) contract to 2027-09-30. |
| **Symbium** [35,36] | Instant permits for retrofits (heat pumps, HVAC, EV chargers, building-integrated solar, additions, structural); contractor licence checks | "100% deterministic" checks showing "referenced code sections"\*. How rules are encoded isn't described. | No AI mentioned | Not listed | Building officials; contractors, owner-builders | Bakersfield live since 2023. 100+ jurisdictions\*. ~$4M raised (CB Insights, undated)†. |
| **Structured AI** (YC F25) [37] | Whole drawing sets (PDF, Revit) checked before senior review; field work vs documents; MEP coordination | Its own computer-vision models. Code sourcing not stated. | No MCP found. The company says it won't take decisions from engineers. | Not public | MEP and structural engineers (Syska Hennessy is partner and investor) | $4.2M seed, 2026-06-10, led by FCVC; ~$5M total |
| **CodeComply.ai** [38,39] | Floor plans against IBC, IMC, IPC, IRC, ADA, FHA, UFC, state amendments and NFPA 101, 70, 72, 13, 14; version comparison | How code text is licensed isn't stated | "Informational... does not replace professional review"; final decisions stay with qualified reviewers | Not stated | Building departments, through CivicPlus | $2M seed, 2025-01-16†. CivicPlus partnership, 2026-03-19. |
| **Nomic** (moved into AEC) [40–42] | Agents for drawing review against codes, submittals, RFIs and QA/QC. "380+ codes and standards". Its fire-code page lists 119 codes, 108 supported. | Sourcing and licensing not stated | **MCP client:** connects to Gmail, Drive, Autodesk Forma, Smartsheet and a customer's own remote MCP server. Changes ask first. Agent API for in-house teams. | Individual plans from $20/month; Enterprise by quote | AEC firms (Arcadis case study) | 2026 pages, undated |

Other names in search results, not researched: InspectMind AI (YC, "AI plan checker"), Mason AI, PlanChecker.ai, AutoSitu (YC).

## 3. Government rule engines

| Program [src] | Status, dated | What is checked, and how | Who writes the rules | AI |
|---|---|---|---|---|
| **Singapore CORENET X** [43–46] | Mandatory for new projects of 30,000 m² or more since 2025-10-01; of 5,000 m² or more from 2026-10-01; voluntary below that (circular, 2026-07-23) | IFC-SG models. The Model Checker runs schema checks, quality checks against the Code of Practice, and regulatory checks (headroom, bicycle parking). Regulatory checks run only when the needed data is in the model. Deterministic rules only; the rest is manual†. Its beta date was "to be announced"† (undated page). | Seven agencies (BCA, URA, LTA, NEA, NParks, PUB, SCDF), each supplying IFC-SG parameters | None in the checker found. Nemetschek (2026-09-22) urges AI-assisted checks with audit trails and an engineer's signed accept, change or reject. |
| **Japan, MLIT** [47,48] | BIM drawing examination since 2026-04-01: drawings exported from BIM, checked by people. BIM data examination planned for spring 2029 (concept published 2026-09-18). | IFC uploaded to a dedicated common data environment. "Digital review": programs and AI assist the reviewer. Fully automated review, including generative AI, is excluded for now. Priority checks: layout, lighting and ventilation, smoke control, emergency lighting, finishes, fire compartments, zoning. No structural calculations at first. | MLIT plans to publish its examination methods, modelling rules and IFC specification in fiscal 2027 | Assists reviewers only |
| **Dubai Municipality** [49,50] | BIM e-permit with automated checks against the Dubai Building Code since 2023-05†. AI agent for villa permits announced 2026-08-09: 26 weeks, three phases. | Drawings against the Dubai Building Code: plot, architecture, MEP, structure. It also cross-checks drawings against each other and fills the building card. | The municipality. Vendor not named. | Reports disagree on how much human review remains |
| **Estonia** [51] | Model checks within the building registry: a rule engine with 47 criteria, semi-automated† (chapter undated, ~2025) | n/a | Ministry | unknown |
| **EU ACCORD** (Horizon Europe) [52] | Ran 2022-09 to about 2025-08. Built a compliance ontology (AEC3PO), open-source components and pilots in the UK, Finland, Estonia, Germany and Spain. Partners include Solibri and the City of Hamburg. | n/a | Research consortium | n/a |
| **US cities** [30–34,64] | Pre-check pilots in Honolulu, Denver, Los Angeles, Austin and Seattle (exploring). A federal FY2025 demonstration of $3M for AI permitting†. Toronto pilot from 2026-08-27, vendor unconfirmed†. | Vendor products (§2) | Vendors plus each city | Vendors |

## 4. Open standards and reference tools

| Item [src] | What it is | Notes |
|---|---|---|
| **IDS 1.0** (buildingSMART) [53–55] | A final standard since 2024-06†. An XML file that states which IFC objects need which attributes, properties, classifications and materials, with allowed values. It checks that information was delivered, not that a design is correct. Free and open. | Supported by Solibri (rule #244, IDS Editor), BIMcollab, Speckle (import) and IfcTester (open source, from IfcOpenShell; command line, web and Python; reports include BCF†). A Rust crate, `openbim-ids`, exists† (not evaluated). IDS 1.1 not confirmed. |
| **IFC Validation Service** (buildingSMART) [56] | Free. Checks IFC syntax, schema and buildingSMART's normative rules, and feeds its software certification. | Doesn't check national, project or organisation rules†. Rules were added 2025-04 to 06†. |
| **buildingSMART Regulatory Room** [57] | Has worked on automated code checking since 2017. A call for sponsorship on "regulatory information requirements" (undated)†. | No rule-language standard found as of 2026-10-31 |
| **ASHRAE 229P + PNNL Ruleset Checking Tool** [58] | A proposed standard for testing how energy-modelling software implements a ruleset. RCT is the reference implementation for 90.1-2019 Appendix G (90.1-2022 optional). MIT licence. Each rule is a Python class; test cases are JSON files. | The closest public analogue to Hyleon packs with tests. Final release waits for 229 to be published. |
| **BCF, IFC 4.3** | BCF is the issue-exchange format that Solibri, BIMcollab and Speckle export. IFC 4.3 adds infrastructure (ISO 16739-1:2024). | From general knowledge; not re-checked this round |

## 5. Code as data, and the licence picture

| Item [src] | Facts, dated |
|---|---|
| **ICC Digital Codes Premium + AI Navigator** [61,62] | AI Navigator answers code questions and summarises sections. It was embedded in the code pages around the ICC Annual Conference (2025-10-19 to 22). Premium required†. A Group A collection costs $102/month†. |
| **ICC Code Connect API** [59,60] | Launched 2022-05-11. It returns code sections, tables, figures and whole chapters as JSON, including historical editions. A vendor signs an integration licence and pays an implementation fee; content is licensed per title per year. Vendor price depends on employee count and titles; a jurisdiction's on population. Partners include Archistar, CivicPlus, Tyler, Municity and GovPilot. The page doesn't say whether AI use is allowed. Price on request. |
| **NFPA LiNK 3.0 + CASI** [63] | Launched 2026-01-13. 300+ codes. CASI answers "grounded in official NFPA information", with citations. No API or third-party access mentioned. Price not found. NFPA's old subscription closed 2025-12-22†. |
| **Court cases** [23–25] | 2020: a New York federal court (SDNY) held the I-Codes **as adopted** are public domain (as reported by UpCodes and TechCrunch, 2020-11-16). 2022-07-29: the Second Circuit sent part of the ICC case back on procedure†. Final outcome not found. 2023-09-12: the D.C. Circuit held that posting standards incorporated into law was mostly lawful (*ASTM v. Public.Resource.Org*, per EFF). The status of the federal "Pro Codes Act" bill is unverified. |

**What this means for Hyleon's assumption.** The rulings cover the text of codes **adopted into law**. They don't cover model codes before adoption, product standards such as UL 508A or IEC, or publishers' terms against building software from their documents. So Hyleon's assumption that code text isn't freely licensed still holds for its domains. The code owners now sell AI answers over their own text. A user's own agent can bring that text in from outside Hyleon.

## 6. How rule content is kept current across jurisdictions

| Model | Who maintains it | How updates reach users | Examples | Cost per new jurisdiction |
|---|---|---|---|---|
| Vendor templates plus local setup | Vendor library; each city picks and configures its ~200–250 checks | Rules Management System: edits in minutes, ~24 h of testing, grace periods | CivCheck | Residential checks "testing-ready within a few months"\* |
| Vendor packs per code family | Vendor | "Live code logic and rule updates"\* | Archistar Smart Packs | Packs "can be adapted to local amendments"\* |
| Licensed or public code text read by AI | Vendor ingests adopted text and amendments | Continuous | UpCodes, Nomic, CodeComply | Low to encode, but accuracy is the risk. UpCodes' plan review covers 73 jurisdictions; its library claims 6,000+. |
| Government publishes the rules | Agencies | Data requirements (IFC-SG) and codes of practice | CORENET X, Japan, Dubai | One jurisdiction each |
| User-configured rule templates | The firm or user | Ruleset files; company extensions | Solibri, BIMcollab, Speckle, Navisworks, Forma Drawing Compliance | No code content; the firm carries it |
| Open standard plus test cases | Committees | Versioned specifications; reference tests | IDS, ASHRAE 229/RCT | Slow, shared |

**Observations**
- Coverage is the bottleneck, not the engine. Even UpCodes, after years of building its library, reviews plans in 73 jurisdictions. CivCheck sets up one city at a time.
- Deterministic engines (Symbium, CORENET X, Solibri, RCT) cover narrow, well-defined checks. AI engines (UpCodes, CivCheck, Structured AI, Nomic, Dubai) cover more ground but describe themselves as a first pass.
- Hyleon's region profiles (D-019: adoption facts, a daily watcher, a person approves) look like CivCheck's Rules Management System without the code text.

## 7. Overlap with Hyleon

| Hyleon idea | Who does something like it here | The difference |
|---|---|---|
| Packs: questions, limits, checks and deliverables per kind of job | Archistar Smart Packs, CivCheck templates, Solibri rulesets, IDS, ASHRAE RCT (rules plus test cases) | None combines questions, sourced limits, checks and deliverables. IDS is the nearest to "what must be known", but only as data. RCT's test cases match pack tests. |
| Ask before designing | CivCheck asks when unsure; CORENET X checks only when the data exists; IDS lists required data | They ask at submission, not before design |
| Every value has a source | UpCodes, Structured AI and CodeComply cite the code section or drawing page on each finding. Speckle Intelligence shows its query and source models. | Sources are on findings, not on model values. No product marks AI-guessed values until a person confirms them. |
| Inform, never block | Every vendor: "first pass", "doesn't replace review" | Authorities use these tools as gates (Honolulu's watermark; Dubai's automatic approval) |
| Cite the clause, never copy the text | Solibri, Symbium ("referenced code sections"), CORENET X | UpCodes, ICC and NFPA ship the text. Code owners now sell AI over it themselves. |
| Works with any AI agent (MCP) | Autodesk MCP servers (tech preview; official Revit 2027 server†). Nomic as an MCP client. Community Solibri MCP servers. | No checker found offers its checks to any outside agent as a verification step |
| Documents generated from the model | None in this category; they check, they don't produce | n/a |
| Panels, controls, piping, networks | CodeComply lists NFPA 70 and 72 for building plan review; AVEVA has plant design rules | Nothing checks panel or UL 508A models |

## 8. Gaps

- **Nothing checks an outside AI's output with provenance.** Vendors run their own AI; none takes in another agent's proposed changes, checks them and records who guessed what.
- **No sources on values.** Findings cite clauses; model values don't record where they came from.
- **Questions come late.** Pre-checks ask at permit intake, not before design starts.
- **Narrow domains.** Products check buildings (zoning, life safety, accessibility, MEP coordination). Industrial electrical, controls and process piping against NEC, UL 508A, NFPA 79 or ASME B31 weren't found as model checks.
- **Rules are locked in each tool.** Solibri .cset files, BIMcollab Smart Views and city templates don't move between tools. IDS moves, but covers only information requirements.
- **No shared rule language.** buildingSMART's regulatory work has no published rule standard as of 2026-10-31. ACCORD built an ontology, but its uptake is unknown.

## 9. Connecting layer (Dan's addendum)

Here, "connecting layer" means a product that takes researched knowledge (codes, standards, owner rules), the user's model or drawings, and what an AI produced, and returns consequences and gaps without authoring the design itself.

| Product | What it connects | What's missing against that definition | How close |
|---|---|---|---|
| Solibri | Rules + IFC from any authoring tool → issues (BCF); its AI now writes rules | Sources on knowledge; awareness of AI output; questions | Medium |
| Speckle | A data hub across Revit, Tekla, IFC and Civil 3D + a rule checker + AI that shows its query; imports IDS | Code knowledge; sources on values | Medium (closest on connecting data) |
| BIMcollab / BCF | Issues across tools; IDS requirements | A knowledge layer | Low to medium |
| IDS (standard) | A machine-checkable contract between the requirement and the model | Everything beyond whether information is present | A building block |
| UpCodes | Code text + project memory + drawings → issues | It is the AI itself; closed; no MCP | Medium to high on knowledge to consequence; low on working with any agent |
| CivCheck | Code templates + applicant documents → findings; asks instead of guessing; people approve every interpretation | Government buyer; PDFs only; no model | Medium (closest to Hyleon's principles) |
| Nomic | Firm knowledge + codes + outside tools through MCP → agents | It does the work; it doesn't check another AI's output | Medium |
| Autodesk MCP / Forma | The platform as a "governed layer" where agents run QA/QC† | Tied to Autodesk data; tech preview; its own assistant first | Medium and rising |
| CORENET X, Japan 2029 | Submitted model + agency rules → findings; a person approves | One jurisdiction; at submission only | Medium |

**Conclusion.** No product found positions itself as a layer that:
- checks what any AI produced against researched sources
- keeps who said what on each value
- turns the result into consequences and open questions

The pieces exist separately:
- IDS: the requirements contract
- BCF: exchanging findings
- Speckle: the data hub
- CivCheck: asking instead of guessing
- Autodesk's MCP: the direction of travel

**What this implies for Hyleon.**
- As a connecting layer in buildings, Hyleon could read IFC and IDS and emit BCF rather than build new checkers. But that is where code-text licensing is hardest and the incumbents are funded (UpCodes, Nemetschek, Autodesk, Clariti, Archistar).
- Every vendor already takes the advisory stance. So Hyleon's difference would have to be sources on each value, working with any agent, and connecting across tools.

## 10. Decisions only Dan can make

1. **Does the wider scope include building code compliance?**
   - (a) No: stay with panels, controls, piping and networks. This keeps the fewest competitors, but the market there is smaller.
   - (b) Yes, as a connector: read IFC and IDS, link to other checkers, emit BCF. This reuses open standards, but Hyleon depends on other tools' rules.
   - (c) Yes, as a checker with its own building packs. This means taking on funded incumbents, free government engines and the code-text problem.
2. **Code content.**
   - (a) Citations only, with the person entering table values (D-017 as it stands). Legally safest, but more work for users.
   - (b) License the text. ICC Code Connect is open to vendors, priced by employee count and titles; no NFPA programme was found. This costs money, and the terms on AI use are unknown.
   - (c) Rely on adopted law being public. That's UpCodes' path, with years of litigation, and it covers neither model codes nor product standards.
3. **Who keeps jurisdiction rules current?** Hyleon staff, licensed reviewers (the current rule for releasing packs), firms or authorities writing their own packs, or a community. The evidence shows this is the main cost of scale.
4. **AI in the checks.**
   - The market reads PDFs with AI vision.
   - Hyleon checks deterministically with CEL.
   - Should Hyleon accept AI-read inputs (for example, values taken from a vendor's PDF) as "AI-inferred" values that feed deterministic checks?
5. **Who buys, and may a third party use Hyleon as a gate?** Authorities are making pre-checks mandatory (Honolulu since 2026-09-01). The truths say Hyleon informs and never blocks, but a buyer could still use its findings as a gate. The wording for that case is Dan's.
6. **Open formats.** Should Hyleon adopt IFC, IDS and BCF in or out? That fits truth 8 ("open to any tool"), but it widens the formats Hyleon has to own.
7. **The name "packs".** Archistar sells "Smart Packs" for code checks. This is minor; flagged only so the name is chosen knowingly.

**Things you might not see from here**
- **Mandates are spreading.** Authorities increasingly require checks of models or drawings. A tool that "never blocks" can still sit inside a mandated gate, which raises its liability exposure (see D-025).
- **AI over code text is now sold by the code owners.** ICC and NFPA both do this. "No standards text reaches an agent" can hold only for Hyleon's own channels. A user's agent can fetch the text elsewhere.
- **Coverage takes years.** Each jurisdiction is slow to add (UpCodes: 73 jurisdictions in plan review; CivCheck: months per city). A "framework for any kind of work" multiplies this per domain.

## 11. Not verified or not covered

- **Not verified:**
  - Solibri's own MCP server (help page blocked)
  - Details of Revit's public MCP server and which outside clients can use it
  - Navisworks 2027 features
  - Forma prices
  - Funding for Archistar and Symbium
  - The Pro Codes Act
  - The final outcome of ICC v. UpCodes
- **Not covered,** because the search budget ran out:
  - Hexagon/Octave Smart 3D and Bentley design checks (a Bentley product page returned 404)
  - Civil 3D and OpenRoads design-criteria checks
  - E-permitting in Korea, Hong Kong, Norway, the UK, New Zealand and Australia
  - Machine-readable code programs in Australia (ABCB) and Canada (NRC)
- **Not read:** a Florida League of Cities buyers' guide to AI permitting tools (2026-08). The PDF was saved but couldn't be read without installing a tool.

## 12. Sources

1. https://www.solibri.com/pricing
2. https://www.solibri.com/solutions/bim-quality-assurance/solibri-ai-powered-assistant
3. https://www.solibri.com/articles/solibri-release-notes-june-2026
4. https://www.solibri.com/articles/solibri-release-notes-april-2026
5. https://help.solibri.com/hc/en-us/articles/42109674663959-AI-Chatbot-Beta (403; search summary only)
6. https://glama.ai/mcp/servers/EdvardGK/solibri-mcp-server/tree and https://glama.ai/mcp/servers/Vanderjohnny/solibri-mcp/tree (search summary only)
7. https://www.cdwg.com/product/autodesk-navisworks-manage-2027-new-subscription-annual-1-seat/9115559 (search summary only)
8. https://forums.autodesk.com/t5/navisworks-forum/product-update-navisworks-2027-what-s-new/m-p/14121733 (403; search summary only)
9. https://www.bimcollab.com/en/plans/bimcollab-platform/
10. https://www.bimcollab.com/en/products/bimcollab-zoom-campaign/
11. https://architosh.com/2026/09/au2026-autodesk-expands-forma-and-leverages-ai/
12. https://aps.autodesk.com/blog/building-agentic-ai-whats-new-autodesk-platform-services
13. https://www.autodesk.com/blogs/aec/2026/06/17/revit-public-mcp-server/ (403; search summary only)
14. https://conferences.autodesk.com/flow/autodesk/au2026/sessioncatalog/page/digital/session/1783105092955001PXJZ (search summary only)
15. https://speckle.systems/
16. https://speckle.systems/bim-model-checker
17. https://www.aveva.com/en/products/e3d-design/
18. https://cms-v2.up.codes/pricing
19. https://cms-v2.up.codes/features/plan-review
20. https://www.constructiondive.com/press-release/20261013-upcodes-ai-reviews-1300-plan-sets-identifying-more-than-180000-potenti/
21. https://www.buildingenclosureonline.com/articles/94432-upcodes-unveils-copilot-intelligence
22. https://techcrunch.com/2023/05/30/upcodes-copilot-series-a/ (search summary only)
23. https://techcrunch.com/2020/11/16/a-court-decision-in-favor-of-startup-upcodes-may-help-shape-open-access-to-the-law (search summary only)
24. https://dockets.justia.com/docket/circuit-courts/ca2/21-826 (search summary only)
25. https://www.eff.org/cases/publicresource-freeingthelaw
26. https://www.archistar.ai/
27. https://www.archistar.ai/aiprecheck/smart-packs/
28. https://www.surrey.ca/renovating-building-development/building/digital-compliance-tool-building-plans (search summary only)
29. https://www.cambridge.ca/business-building-development/building-planning/ai-precheck/ (search summary only)
30. https://www.claritisoftware.com/products/civcheck-ai-plan-review-software
31. https://www.claritisoftware.com/blog/clariti-acquires-civcheck-to-accelerate-permit-approvals-with-ai
32. https://www.honolulu.gov/dpp/getting-started-with-civcheck/
33. https://denverite.com/2026/02/20/denver-hopes-4-6m-for-artificial-intelligence-will-speed-up-development/ and https://denverite.com/2026/03/10/denver-complyai-ai-permit-review/ (403; search summary only)
34. https://www.claritisoftware.com/civcheck-hud-grant (vendor claim; search summary only)
35. https://symbium.com/automated-permitting-comparison/
36. https://www.cbinsights.com/company/symbium (search summary only)
37. https://www.enr.com/articles/63139-construction-quality-startup-structured-ai-raises-42m-seed-round
38. https://www.civicplus.com/news/nn/civicplus-brings-ai-building-plan-review-codecomply-ai/
39. https://refreshmiami.com/news/with-2m-in-tow-ai-startup-codecomply-ai-aims-to-accelerate-construction-approvals-and-reshape-compliance-checks/ (search summary only)
40. https://www.nomic.ai/
41. https://www.nomic.ai/integrations
42. https://www.nomic.ai/use-cases/code-compliance-of-drawings/codes/fire-code
43. https://www.ura.gov.sg/guidelines/circulars/dc26-08/
44. https://info.corenet.gov.sg/overview/about-corenet-x/overview-of-corenet-x
45. https://stg.bca.cloud.sitefinity.com/overview/corenet-x-submission-portal/model-checker and https://www1.bca.gov.sg/regulatory-info/building-control/corenet-x/corenet-x-faq/new-data-requirements-and-technology (search summary only)
46. https://technode.global/2026/09/22/nemetschek-jonathan-ng-corenet-x-ai-assisted-compliance-digital-delivery-qa/
47. https://thegoodbuilder.com.au/japan-will-check-building-approvals-straight-from-bim-data-from-spring-2029/
48. https://www.kenchikushikai.or.jp/data/news/2026/2026-04-01-02.pdf (search summary only)
49. https://www.khaleejtimes.com/uae/dubai-ai-system-villa-construction-permits
50. https://mediaoffice.ae/en/news/2023/May/26-05/Dubai-Municipality-launches-new-online-system-for-building-permits-and-control-services (search summary only)
51. https://www.taylorfrancis.com/chapters/oa-edit/10.1201/9781003650287-22/bim-based-building-permit-christopher-robin-raitviir-pille-riin-peet-irene-lill-raido-puust (search summary only)
52. https://cordis.europa.eu/project/id/101056973 (search summary only)
53. https://www.buildingsmart.org/?p=30065 (search summary only)
54. https://docs.rs/openbim-ids (search summary only)
55. https://docs.ifcopenshell.org/ifctester.html (search summary only)
56. https://technical.buildingsmart.org/services/validation-service/ (403; search summary only)
57. https://www.buildingsmart.org/standards/calls-for-participation/regulatory-information-requirements/ (search summary only)
58. https://github.com/pnnl/ruleset-checking-tool
59. https://solutions.iccsafe.org/codeconnect
60. https://www.iccsafe.org/about/periodicals-and-newsroom/the-international-code-council-introduces-icc-code-connect-api/
61. https://mailing.iccsafe.org/october-updates-icc-ai-navigator-is-now-embedded-directly-in-the-codes
62. https://support.iccsafe.org/ht_kb/digital-codes/understanding-types-of-premium-subscription-levels/ (search summary only)
63. https://www.facilitiesdive.com/news/nfpa-digital-codes-and-standards-platform-gets-ai-upgrade/809622/
64. https://apply07.grants.gov/grantsws/rest/opportunity/att/download/352481 (search summary only)
65. https://www.calbo.org/post/ai-and-plan-review-practical-lessons-building-departments
