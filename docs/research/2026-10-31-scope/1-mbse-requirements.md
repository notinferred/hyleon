# Appendix 1: Requirements management and MBSE (agent report, 2026-10-31)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-31-scope.md](../2026-10-31-scope.md).

**The brief:** Map the requirements-management and model-based systems engineering (MBSE) tools (what they check, where their rules come from, their AI and MCP support in 2026, platforms, prices, buyers and recent news), compare them with Hyleon's loop, find the gaps a cross-domain "researched boundaries for AI work" framework could fill, and judge which of them act as a connecting layer rather than an authoring tool.

---

## Bottom line

- **Checking how requirements are written is now a commodity.** Every big vendor uses AI to score requirement text against the same two public rule sets: INCOSE's writing rules and EARS. Jama, Siemens Polarion, PTC Codebeamer and IBM DOORS Next all do it [2,11,17,22].
- **They check structure, not substance.** The tools check trace links, coverage, links flagged as suspect after a change, baselines and signatures. None that I found ships researched, cited limits for the product itself, such as a rating, a clearance or a code clause. Each customer writes those rules.
- **MCP arrived in 2026.**
  - Official servers: Jama (2026-05-04, sold as an add-on) [7], Obeo's "AI for Capella" (planned for the end of July 2026) [32], Trace.Space [51] and Dalus [53].
  - Community servers only: Polarion, Codebeamer and Cameo [21,29,63].
  - IBM: not confirmed.
- **The best-funded newcomers describe themselves as a connecting layer.** Flow Engineering calls itself a "living system of record" and is building an "AI harness". It raised $50M at a $750M valuation, announced 2026-09-30 [42,45]. Trace.Space calls its AI layer "the AI harness to accelerate hardware R&D" [51]. Both still leave the boundaries to the customer.
- **SysML v2 is now a formal OMG standard (published 2025-09)** [36]. It is a text format that works with Git and has a standard API. Most vendors support it, so it is a realistic exchange format for a model or for packs.
- **Prices.**
  - The incumbents (IBM, Jama, Siemens, Dassault, PTC) sell by quote only.
  - The newcomers publish prices: Dalus runs from $0 to $249 per editor per month [53], Flow from $150 to $300 per editor per month [44], and Valispace costs $999 per user per year [47].
  - Buyers are regulated enterprises: aerospace, defense, automotive, medical devices and semiconductors.

## How this was done

- **Method.** One agent worked on 2026-10-31. It used web search plus direct reads of official pages. Nothing was installed or run.
- **Markers:**
  - **\*** marks a vendor's claim about itself.
  - **(U)** marks a claim that is unverified: it comes from a secondary, community or social-media source, or from a search snippet I could not open.
  - **"Not found"** means I looked and did not find it. It does not mean the thing doesn't exist.
- **Limits:**
  - The session's web-search budget ran out partway through. Later facts come from direct reads of known URLs.
  - Some pages refused automated reading (HTTP 403): Jama's support release notes, PTC's newsroom and a Dealroom page. Claims that rest only on their search snippets are marked (U).
- **Context.** This category was not in the 2026-10-28 competitor map, which covered electrical, controls and piping tools.

## 1. What each product checks, and where its rules come from

| Product (version, date) | What it checks | Where the rules come from | Who maintains them |
|---|---|---|---|
| **IBM DOORS / DOORS Next** (7.2, 2025-12-18) [5] | Requirements in modules, baselines, electronic signatures, multi-level trace links [1]. Links to other tools through the OSLC standard [6]. AI quality scores "against industry standards", with suggested rewording [2]. | IBM's page does not name the standards behind the AI score [2]. Project rules are the customer's own attributes, link types and scripts. | IBM (AI); the customer (projects) |
| **Jama Connect** (9.35, 2026-05) [7] | Trace coverage against a customer-defined "Traceability Information Model"\* [7]. The Advisor add-on scores requirement text against "a core subset of 40" INCOSE rules and EARS, one at a time or in bulk [11,12]. | The INCOSE rules for writing requirements and EARS notation. The customer defines the trace model. | Jama (Advisor); the customer (trace model) |
| **Siemens Polarion** (2606, 2026-05-28) [15] | Its Copilot checks text against INCOSE rules and proposes rewrites. It also finds similar items (duplicates) and runs a consistency checker that follows trace links several levels deep for "contradictions, gaps or mismatched details" [17]. | INCOSE. Partner add-ons cover EARS, INCOSE and the Sophist sentence template (reQlab 2.0, 2025-05) [19]. Workflows are configured per customer. | Siemens, partners, the customer |
| **PTC Codebeamer** (3.2 + Codebeamer AI 1.0, 2026-01-15) [22] | A Requirements Assistant "aligned with INCOSE and ISTQB guidance" detects quality issues. A Test Case Assistant drafts tests from requirements. | INCOSE and ISTQB (a software-testing standards body) | PTC |
| **Dassault CATIA Magic / Cameo** (2026x) | SysML v1 and v2 models. SysML v2 validation suites and rules, customizable from 2026x Refresh1 [26]. Requirement violations are flagged with a trace to the constraint broken. Also roll-ups, trade studies, and automated requirement checks through the API in build pipelines [23]. | OMG language rules plus the customer's validation suites. AI "Model Suggestions" come from queries defined in a JSON file that customers can edit [27]. | Dassault and the customer |
| **Eclipse Capella** (7.1.0, 2026-07-10) [35] | Models built with the Arcadia method, plus the model-validation rules built into Capella (U: background knowledge, not re-checked this round). Obeo's AI add-on lets agents discover, understand, analyze, edit and document a model, and refresh or export its diagrams [33]. "Model review and consistency checking" is listed as a future direction [32]. | The Arcadia method (Thales) plus the customer's own rules | Thales, Obeo, the community |
| **SysML v2 tools** (Syside, SysON, the OMG Pilot, SystemsModeling.com, SysGit) | Language validation of SysML v2 text, and evaluation of constraints. Sysand is an open-source package manager for model libraries [38,39]. | The OMG specification and shared libraries | The community and vendors |
| **Flow Engineering** (v3, 2026-06-04) [43] | Everything lives in one "systems graph". Agents run impact analysis on every change\*, track mass, cost and power budgets, check requirements against standards\* and check test coverage [45]. | The engineer "defines standards and encodes intent" [43]. Which standards ship with the product is not stated. | The customer |
| **Requirements Portal** (formerly Valispace, now Altium) | Requirements, components, a verification and validation module, analysis, simulation and scripting [47] | The customer | The customer |
| **Trace.Space** | Deterministic rules across its graph check requirement quality and consistency, broken links, coverage gaps, parameters out of range or tolerance, and the impact of a change. Templates are "aligned with" ISO 26262, ASPICE and DO-178C\* [51]. | Vendor templates plus the customer's rules | Vendor and customer |
| **Dalus** | SysML v2 models. Requirements are verified through linked behavior, simulation or Python scripts. Hazards are rated by severity and likelihood. Test cases are traced [53]. | The customer | The customer |

### The pattern: three layers of rules

1. **Rules for writing requirements.**
   - These come from the INCOSE Guide to Writing Requirements, version 4, dated 2023-07-01. It is copyrighted and sold through the INCOSE store [60]. The other source is EARS, a sentence template published in 2009 by authors at Rolls-Royce (background knowledge).
   - Every vendor automates a subset: Jama "a core subset of 40" [12], and QVscribe 24 of 44 rules in an earlier edition, per a 2020 paper [61].
   - I did not find INCOSE's licence terms for using the guide inside commercial tools.
2. **Process rules from safety standards.**
   - The standards are ISO 26262, DO-178C, IEC 62304 and ASPICE.
   - They ship as templates that fix the item types and the trace links required. They do not supply domain values.
   - The product pages I read did not describe these templates in detail (U).
3. **Product rules: the actual limits.**
   - The customer writes them, as SysML v2 constraints, CATIA validation suites, Dalus Python scripts or Flow's "standards you encode".
   - I found no vendor shipping researched, cited domain limits.

**This third layer is the main finding for Hyleon:** the market has engines that check rules, but nobody supplies researched, cited content for them.

## 2. AI, MCP, platforms, prices, buyers and news

| Product | AI features (date) | MCP as of 2026-10-31 | Platforms | Published price | Target buyers | Funding, ownership, releases |
|---|---|---|---|---|---|---|
| **IBM DOORS Next** | Engineering AI Hub 1.1 (2025-12-18): requirement quality analysis, a choice of LLM for each agent, and a use-case discovery agent for Rhapsody (IBM's SysML tool) [3,4] | None in the release notes [3,4]. One search snippet says an IBM page mentions "managed MCP tools" (U). | Web (Jazz platform). The classic DOORS client runs on the desktop. On-premises and cloud (third-party sources) | Quote only [1] | Large regulated enterprises. IBM names ASPICE, ISO 26262 and DO-178C [2]. | DOORS Next 7.2, 2025-12-18 [5] |
| **Jama Connect** | The Advisor add-on (language analysis; IBM-style quality scores, Cloud only) [11]. MCP is pitched as "spec-driven development" [7]. | **Official**, 2026-05-04, in 9.35. Clients: Claude, Codex, Cursor, GitHub Copilot, Visual Studio. "Enforces existing permissions, lifecycle workflows and audit requirements"\* [7]. A separately bought add-on (U: release-note snippet) [9]. | Cloud, Customer-Validated Cloud, self-hosted [9] | Quote only. Tiers are Reviewer, Stakeholder, Test Runner and Creator, with no prices shown [14]. Reviewer licenses are free\*. | Regulated product developers: medical, automotive, aerospace, semiconductors | Francisco Partners agreed to buy Jama for $1.2B, announced 2024-03-18 [13]. MCP followed 2026-05-04. |
| **Siemens Polarion** | Copilot: INCOSE checks, similarity, consistency [17]. Version 2512 added a help chatbot and similarity analysis [16]. Version 2606 (2026-05-28) adds a Copilot API and connectors for any LLM on on-premises installs [15]. | No official server found [15]. A community server exists [21]. | On-premises (Polarion ALM); cloud (Polarion X) [18] | Quote only [18]. Old list prices of $2,490 per named user and $8,690 per concurrent user date from about 2015 and are stale [20]. A third-party listing shows "Polarion X Base, $42 per user per month, 10 to 20 users" (U). | Regulated industries, automotive, medical | 2606 shipped 2026-05-28 [15]. At CES 2026, Siemens reportedly announced nine new AI copilots, including one for Polarion (U) [62]. |
| **PTC Codebeamer** | Codebeamer AI 1.0: Requirements Assistant and Test Case Assistant (2026-01-15) [22] | No official server found. Community servers exist, one with 18 read and write tools and one MIT-licensed, updated 2026-05-28 [63]. | Web; on-premises or cloud (U) | Not found | Automotive and software-defined products | Codebeamer 3.2 and AI 1.0, 2026-01-15 [22] |
| **CATIA Magic / Cameo** | The 2026x AI Assistant drafts a candidate architecture. Requirements extracted from documents and model suggestions land in an "AI sandbox" for review. A LEO companion answers questions about the model. All of this is in limited availability under a "Discovery Offer" (2026-07-28) [27]. The Aura, Leo and Marie companions were announced 2026-02-11 [28]. | No official server found. Third-party servers exist: Cameo Bridge, and jgs-magic with 130+ tools for SysML v1 [29]. | Desktop; Teamwork Cloud; 3DEXPERIENCE; MagicLab, a read-only browser viewer; on-premises for classified programs. A free SysML v2 Community Edition is available [23]. | Quote only [30]. A reseller reports SysML v2 bundles rose about 20% on 2026-01-01 [24]. | Defense (No Magic Cameo) and general industry (CATIA Magic) [24] | 2026x adds SysML v2. Hot Fix 1 came 2025-12-19 [25]. Refresh1 added validation suites [26]. |
| **Eclipse Capella** | AI for Capella (Obeo's commercial add-on) opens Capella to outside agents through MCP and APIs. It has no built-in assistant. Its tagline is "AI assists, engineers decide" [32,33]. | **Official (Obeo).** Planned for the end of July 2026 [32]. Tested clients include Claude Code and Desktop, Codex CLI, Gemini CLI and the ChatGPT desktop app [33]. | Desktop (Eclipse). SysON runs in a browser. | Free and open source (Eclipse Public License). Obeo's add-ons are quote only [33]. | Thales and large industrial firms: defense, space, rail (U) | Capella 7.1.0 on 2026-07-10 [35]. An Arcadia library for SysML v2 and an open-sourced Simulink add-on, both 2026-06 [31]. |
| **SysML v2 tools** | Syside lists "AI support" [38]. Obeo is prototyping an LLM assistant for SysON. | A community SysON server exists [40], plus an academic framework for SysML v2 MCP servers [41]. Dalus has an official one. | Syside Community Edition is a free, open-source VS Code extension. SysON runs in a browser. SystemsModeling.com and Ansys SAM run in the cloud. | Syside CE is free; the Pro price was not found. | Systems engineers in defense, space and automotive | SysML v2.0 became formal, published 2025-09 [36]. An editorial update in 2026-03 prepared it for ISO submission [37]. |
| **Flow Engineering** | Flow v3, an "agentic platform for systems engineering", 2026-06-04 [43]. An "AI harness" is planned [42]. | Not mentioned on its site or in coverage | Cloud. Enterprise customers get ITAR-compliant AWS GovCloud [44]. | Basic $150 per editor per month (500 requirements, 1 project). Pro $300 (5,000 requirements, 5 projects). Enterprise by quote [44]. | Hardware companies in space, automotive, defense and fusion. Customers named: Rivian, Anduril, Joby, Astranis, GM Performance Power Units [42]. | Series A $23M, 2025-10, led by Sequoia [46]. Series B $50M at a $750M valuation, 2026-09-30 [42]. |
| **Requirements Portal (formerly Valispace)** | ValiAssistant, in beta [47]. Altium Develop lists an "AI Requirements Assistant" [48]. | Not found | Cloud on Altium 365; on-premises for enterprise [47] | Valispace plan: $999 per user per year, minimum 3 users [47]. Altium Develop: $995 per year for each added author [48]. | Hardware and electronics teams | Altium bought Valispace for a reported $15.6M to $20M. Sources disagree on the date, which falls between 2023-12 and 2024-02 [49]. Renesas bought Altium in 2024. |
| **Trace.Space** | Trinity, its "AI harness". A Space Agent coordinates other agents. AI proposes trace links and the engineer accepts them [51]. | **Official**: an API and an MCP server, so customers can "bring your own AI" [51] | Cloud, private cloud, on-premises, air-gapped [51] | Not published; a free trial is offered [51] | Regulated hardware enterprises [51] | $4M seed led by Cherry Ventures, 2025-02-05 [52] |
| **Dalus** | AI Copilot drafts architectures from requirements [53] | **Official**: the "Dalus MCP Connector" for Claude and ChatGPT [53] | Web; GovCloud; on-premises for enterprise [53] | Free (1 model, 250 elements). Basic $99 per editor per month. Pro $249. Enterprise by quote [53]. | Energy, defense, aerospace, automotive, robotics [53] | In Y Combinator's Winter 2025 batch (U) [54]. Funding not confirmed. |
| **Celedon Davinci** | SysML v2 with automated CAD, documents and code generation [38] | Not found | Not found | About €17 per user per month for the core tier, from a single forum post (U) [34] | Aerospace and defense (U) | Not found |

**Other notes:**
- A Capella user wrote on the forum on 2026-02-03 that "lagging so far behind the AI wave is now a deal-breaker" [34]. That pressure is visible across the incumbents.
- The OMG list of SysML v2 tools names 24 tools from 2025. Only Dalus is called "AI-native" there [38].

## 3. Where they overlap Hyleon's loop

| Hyleon step | What this category does | Closest product | The difference |
|---|---|---|---|
| **What must be known** (questions first) | AI pulls requirements out of existing documents: CATIA's requirements extraction [27], IBM's use-case discovery [3], Polarion's splitting of tender documents (RFQs) into requirements [17]. | None | They extract from what already exists. None holds a researched list of what a given kind of job must answer. Not found anywhere. |
| **Limits, each with a source** | Requirements with attributes; SysML v2 constraints; parameter ranges | CATIA SysML v2 [23], Trace.Space [51], Dalus [53] | Limits are written by the customer. A limit's "source" is a link to a parent requirement or document, not a cited clause and edition. |
| **Checks on every change** | Links flagged as suspect, impact analysis, checks in build pipelines | Flow v3\* [43,45], Trace.Space [51], Jama [7], CATIA CI [23] | Mostly structural (links, coverage, text quality), or numeric constraints the customer wrote |
| **AI guesses stay marked until a person confirms** | CATIA's "AI sandbox" (accept or decline) [27]. Trace.Space: "AI suggests, engineer accepts" [51]. Jama MCP: AI actions are "user-initiated, permission-aware, fully traceable"\* [8]. | CATIA Magic | Review happens before acceptance. I found no tool that keeps an "AI-inferred" label on a value after it is accepted. Per-value source types (person, standard, catalog, calculated, AI) were not found. |
| **Issue, with open findings acknowledged** | Baselines, reviews and e-signatures: DOORS [1], Jama reviews, Codebeamer's Review Hub [22] | IBM, Jama, Codebeamer | These tools enforce workflows and approvals. Hyleon informs and never blocks. Regulated buyers expect enforcement. |
| **Open to any agent** | MCP servers | Obeo (the widest list of tested clients) [33], Jama [7] | Same stance as Hyleon's truth 8 |
| **Works without AI** | All the incumbents | — | Same |

## 4. Where they don't overlap

- **Scale and buyer.**
  - These tools serve whole product programs: aircraft, cars, satellites, medical devices. Most are sold to enterprises.
  - Hyleon starts with one pump panel, for electricians and panel shops. Today the buyers do not overlap.
- **Design output.**
  - None of them produces drawings, schedules, labels or cable lists.
  - MBSE models describe architecture. They are not a design someone can install.
- **Catalog.** None has a curated parts catalog with ratings. Flow and Valispace track components, but they do not curate them.
- **Clause-level sourcing.** None reproduces code text, but none cites a clause at the level of a single value either.
- **Gatekeeping.** These tools are built to enforce process: permissions, workflows, e-signatures. Hyleon is "not a gatekeeper" (TRUTHS.md).
- **Requirement wording.** Hyleon does not check how requirements are written. That is the most crowded part of this category.

## 5. Gaps a cross-domain "researched boundaries for AI work" framework could fill

1. **Researched domain content.** No tool I found ships cited limits for each kind of job. Every tool leaves that to the customer or a consultant. This is the clearest gap.
2. **Lists of questions to answer first.** No tool holds, for each kind of job, the questions that must be answered before design starts.
3. **A source on every value.** Each value would carry a source type, and an "AI-inferred" label would stay until a person confirms it. I found this nowhere in the category.
4. **Portable boundaries.** Packs could plug into any system of record through:
   - MCP
   - SysML v2 libraries, packaged the way Sysand does it
   - ReqIF, the standard format for exchanging requirements (background knowledge)

   Sysand shows that versioned, shareable model libraries are already a working idea in this field [39].
5. **Small buyers.** Apart from Dalus's free tier, the category is priced and sold for enterprises.
6. **Domains beyond engineering.**
   - Enterprise architecture is partly covered: Cameo Enterprise Architecture supports UAF, a standard architecture framework [24], and Sparx Enterprise Architect is on the SysML v2 list [38].
   - Leadership is not covered by any product in this category.
   - Jama and Polarion already claim to serve any discipline. What they lack is the researched content, not the generic engine.

**Caution.** The generic engine is crowded and well funded:
- Jama sold for $1.2B [13].
- Flow is valued at $750M [42].
- Siemens, IBM and Dassault are the incumbents.

A cross-domain engine without deep content would meet all of them.

## 6. Which products act as a connecting layer (Dan's addendum)

Dan's framing: Hyleon connects things after the research is done. It links the researched knowledge, the user's model and what an AI produces, then shows the consequences and the gaps.

| Product | How it positions itself | Researched knowledge? | User's model? | AI output? | Consequences and gaps? | How close |
|---|---|---|---|---|---|---|
| **Flow Engineering** | "A living system of record", "connect adjacent tools so the tree stays the source of truth", an "AI harness" [42,43,45] | No. The customer encodes the standards [43]. | Yes: the systems graph, plus links to CAD, simulation and tests | Yes, its own agents | Yes: impact, budgets, coverage on every change\* | **Closest.** It lacks shipped boundaries and a source on each value, MCP was not found, and it sells to enterprise hardware companies. |
| **Trace.Space** | "The AI harness"; one graph; bring your own AI through MCP; deterministic rules [51] | Partly: templates aligned with safety standards\* | Yes | Yes, any agent through MCP | Yes: coverage, values out of range, change impact | **Close** |
| **Jama Connect + MCP** | A governed gateway between AI agents and specifications, built on a "Traceability Information Model" [7] | No. Only the rules for writing requirements. | Yes: specifications and trace links | Yes, AI access is governed | Partly: coverage, suspect links, quality scores | **Medium.** It controls what AI may touch more than it judges what AI produced. |
| **Obeo AI for Capella** | Opens the model to any agent; a person validates [32,33] | No (the Arcadia method only) | Yes | Yes | Not yet. Consistency checking is listed as a future direction. | **Low to medium** |
| **Digital-thread connectors** (Intercax Syndeia, Violet Labs, Istari, SysGit, OpenMBEE) | Link many tools' data. Istari: "vendor-neutral infrastructure for trusted AI-native engineering and certification" [59]. Syndeia: an MCP server announced for 3.7.2 (U) [57]. | No | They link many tools | Syndeia, through MCP (U) | Links, not judgment | **They connect data, not knowledge.** |

**What this means for Hyleon:**
- "Connecting layer" is already a positioning that funded companies use in hardware, under the names "harness", "system of record" and "digital thread".
- None of them ships the researched-knowledge part.
- If Hyleon is the connecting layer, what sets it apart is its researched, cited boundaries and its question lists. The connecting itself is crowded, and MCP is making it standard.
- There are two ways to play it:
  - Hyleon supplies boundaries that other systems read through MCP or SysML v2.
  - Hyleon is its own system of record and competes with Flow and Trace.Space.

## 7. Decisions only Dan can make

1. **Which content to research first.** The choices are physical design only, or also other domains such as enterprise architecture and leadership. The evidence says the checking engine is a commodity and the content is the gap.
2. **Own the model, or connect to others' models.**
   - (a) Keep Hyleon's own model (Place, Part, Port, Connection) and import and export.
   - (b) Become a layer of boundaries and checks that reads other systems through MCP (Jama, Capella, Trace.Space, Dalus) and the SysML v2 API.
   - (c) Both.
3. **SysML v2.** Whether to adopt it as an exchange format or as the pack format. It became formal in 2025-09, and Sysand packages its libraries.
4. **Buyer.** Stay with small shops, or move to regulated enterprises, where these vendors already sell and where enforced approvals and e-signatures are expected.
5. **"Never block".** Whether it still holds in regulated settings, or whether customers may configure their own gates. TRUTHS.md says Hyleon is not a gatekeeper, so this is a decision at the level of the truths.
6. **Checks on requirement wording (INCOSE and EARS).** Ignore them because the field is crowded, or offer them as a pack. Whether INCOSE licenses its guide to tools is unknown.
7. **Price position.** The newcomers publish prices from $0 to $300 per editor per month.
8. **Partner or compete.** Offer packs to Jama, Polarion and Capella users through MCP, or compete with Flow and Trace.Space.

## Sources

All were read on 2026-10-31 unless a date is given.

1. IBM DOORS Next pricing page (no prices shown): https://www.ibm.com/products/requirements-management/pricing
2. IBM requirements management AI page: https://www.ibm.com/products/requirements-quality-assistant
3. IBM, "New AI automations for model-based systems engineering", 2025-12-18: https://www.ibm.com/new/announcements/new-ai-automations-for-model-based-systems-engineering
4. IBM community blog on ELM 7.2 and AI Hub 1.1, 2025-12-18: https://community.ibm.com/community/user/blogs/bhawana-gupta/2025/12/16/driving-engineering-excellence-new-releases-and-ai
5. DOORS Next downloads (7.2, 2025-12-18): https://jazz.net/downloads/requirements-management-doors-next
6. SodiusWillert on DOORS Next and OSLC: https://www.sodiuswillert.com/en/ibm-elm/ibm-engineering-requirements-management-doors-next
7. Jama press release, "Jama Software Launches MCP Server", 2026-05-04: https://www.jamasoftware.com/legacy/wp-json/wp/v2/press/86493
8. Jama MCP datasheet: https://www.jamasoftware.com/datasheet/accelerate-ai-driven-development-with-jama-connect-mcp/
9. Jama 9.35 release notes. Seen as search snippets only; the pages returned 403: https://support.jamasoftware.com/hc/en-us/articles/45272636845837-Version-9-35-Cloud and https://support.jamasoftware.com/hc/en-us/articles/45505900754573-Version-9-35-1-Self-hosted
10. Jama, "MCP vs REST API", 2026-07-08: https://support.jamasoftware.com/hc/en-us/articles/47235506164749-MCP-vs-REST-API-What-s-the-Difference
11. Jama Connect Advisor: https://www.jamasoftware.com/platform/jama-connect/jama-connect-advisor/
12. Jama Advisor blog, 2025-03-20: https://www.jamasoftware.com/blog/2025/03/20/utilize-artificial-intelligence-and-natural-language-processing-to-produce-high-quality-requirements-with-jama-connect-advisor/ ; help page: https://help.jamasoftware.com/ah/en/jama-connect-advisor-/interpreting-the-results.html
13. Francisco Partners to acquire Jama, 2024-03-18: https://www.franciscopartners.com/media/francisco-partners-to-acquire-jama-software-for-12b
14. Jama pricing listings: https://www.g2.com/products/jama-connect/pricing ; https://pricingsaas.com/companies/jamasoftware
15. Polarion ALM 2606, 2026-05-28: https://blogs.sw.siemens.com/polarion/polarion-alm-2606-whats-new-and-noteworthy/
16. Polarion ALM 2512: https://blogs.sw.siemens.com/polarion/polarion-alm-2512-whats-new-and-noteworthy/
17. Polarion industrial AI page: https://www.siemens.com/en-us/products/polarion/industrial-ai/
18. Polarion product page: https://www.siemens.com/en-us/products/polarion/
19. Polarion extensions (Semios; reQlab): https://extensions.polarion.com/extensions/463-semios-ai-powered-quality-assessment-similarities-and-acronyms-analyses-for-polarion
20. TechTarget on Polarion (old prices): https://www.techtarget.com/searchsoftwarequality/feature/Polarion-Software-offers-ALM-focused-on-regulatory-compliance
21. Community Polarion MCP server: https://glama.ai/mcp/servers/@Sdunga1/MCP-Polarion/blob/f72b0a9fd423af61c5c402e561bb1ca535a9a622/README.md
22. PTC, Codebeamer AI 1.0 and 3.2, 2026-01-15. The PTC page returned 403; read through syndicated copies: https://www.ptc.com/en/news/2026/ptc-delivers-new-ai-functionality-with-new-alm-releases ; https://www.iot-now.com/2026/01/19/154960-ptc-delivers-new-ai-functionality-for-software-driven-product-development-with-new-alm-releases/
23. CATIA SysML v2: https://www.3ds.com/products/catia/catia-magic/sysmlv2.md
24. GoEngineer on SysML v2 in 2026x, 2025-12-15: https://www.goengineer.com/en-us/blog/advantages-of-sysml-v2-now-available-in-no-magic-cameo-and-catia-magic-2026
25. No Magic 2026x Hot Fix 1: https://docs.nomagic.com/VN/2026x/2026x-hot-fix-1-278725663.html
26. 2026x Refresh1 post: https://3dswym.3dexperience.3ds.com/post/catia-mbse-cyber-systems/now-released-2026x-refresh1_nS0Ya-VaT9OtOoj6dnPSRg
27. "Generative Systems Engineering in CATIA Magic (CUSE 2026)", 2026-07-28: https://3dswym.3dexperience.3ds.com/post/catia-mbse-cyber-systems/generative-systems-engineering-in-catia-magic-cuse-2026_zmc1yOfsS4WvrYO7ETAcyw
28. Dassault, AI-powered Virtual Companions, 2026-02-11: https://www.3ds.com/newsroom/press-releases/dassault-systemes-unveils-new-way-working-industry-ai-powered-virtual-companions
29. Third-party Cameo MCP servers: https://mcpmarket.com/server/cameo-bridge ; https://enterprisedna.co/directories/mcp/jgsystemsconsulting-jgs-magic-sysmlv1-mcp
30. G2, Cameo pricing: https://www.g2.com/products/cameo-systems-modeler/pricing
31. Obeo, "Obeo strengthens its commitment to the future of Capella", 2026-06-10: https://blog.obeosoft.com/obeo-strengthens-its-commitment-to-the-future-of-capella
32. Obeo, "AI for Capella: agentic AI as a first step", 2026-06-23: https://blog.obeosoft.com/ai-for-capella-agentic-ai-as-a-first-step
33. Obeo, AI for Capella product page (last modified 2026-10-09): https://www.obeosoft.com/?p=21876
34. Capella forum, "Capella hasn't kept up with AI…", 2026-02-03 to 2026-06-19: https://forum.mbse-capella.org/t/capella-hasnt-kept-up-with-ai-and-new-mbse-capabilities-is-it-time-to-abandon-it/11010
35. Capella 7.1.0: https://metrics.eclipse.org/projects/polarsys.capella/ ; https://projects.eclipse.org/projects/polarsys.capella/releases/7.1.0/review ; https://forum.mbse-capella.org/t/capella-7-1-0-has-been-released/11192
36. OMG SysML specifications (2.0 formal, 2025-09): https://www.omg.org/spec/SysML
37. SysML v2 release repository: https://github.com/systems-modeling/sysml-v2-release
38. OMG list of SysML v2 tools: https://www.omg.org/sysml/sysmlv2/sysml-tool/
39. Sensmetry (Syside, Sysand): https://sensmetry.com/ ; INCOSE SE Lab, 2025-03-31: https://www.incose.org/events/news-detail/2025/03/31/new-se-lab-tool-vendor--sensmetry
40. Community SysON MCP server: https://jsr.io/@casys/mcp-syson/0.6.0/README.md
41. "Model context protocol for MBSE" (paper): https://cris.fau.de/publications/366869202
42. SiliconANGLE, Flow Engineering Series B, 2026-10-01: https://siliconangle.com/2026/10/01/flow-engineering-raises-50m-to-bring-the-power-of-ai-to-hardware-engineering/
43. Flow, "Agents have reached hardware" (Flow v3), 2026-06-04: https://www.flowengineering.com/blog/agents-have-reached-hardware
44. Flow pricing: https://www.flowengineering.com/pricing
45. Flow homepage: https://www.flowengineering.com/
46. Flow, Series A changelog, 2025-10-06: https://flow-engineering.productlane.com/changelog/2025-10-06-flow-raises-23m-series-a
47. Valispace / Requirements Portal pricing: https://www.valispace.com/pricing/
48. Altium Develop pricing: https://www.altium.com/develop/pricing
49. Altium acquires Valispace: https://pcdandf.com/pcdesign/index.php/editorial/menu-news/design-news/17930-altium-acquires-valispace-software-platform ; https://www.aviaspace-bremen.de/en/2024/05/22/elevating-engineering-with-valispaces-acquisition-by-altium/
50. SE Trends on Requirements Portal: https://www.se-trends.de/en/valispace-altium-requirements-portal/
51. Trace.Space: https://trace.space/
52. TechCrunch, Trace.Space seed round, 2025-02-05: https://techcrunch.com/2025/02/05/aiming-to-accelerate-product-design-with-ai-trace-space-raises-a-seed-round
53. Dalus: https://www.dalus.io/
54. Dalus in Y Combinator W25 (U): https://origami.chat/people/sebastian-volkl-co-founderceo-dalus-yc-w25 ; https://www.ycombinator.com/companies/dalus
55. Celedon Davinci: see [34] and [38]
56. SysGit: https://www.carahsoft.com/sysgit
57. Intercax Syndeia docs: https://intercax.atlassian.net/wiki/spaces/SYN37 . The MCP server in 3.7.2 comes from a staff post on LinkedIn that I did not open (U).
58. Violet Labs: https://www.vcbacked.co/company/violet-labs ; see also [38]
59. Istari: https://dealroom.co/news/141586-istari-raises-22-6m-for-defense-digital-engineering-push (2026-07, U) ; https://cn.weforum.org/organizations/istari-digital/
60. INCOSE Guide to Writing Requirements v4, 2023-07-01: https://www.incose.org/docs/default-source/working-groups/requirements-wg/gtwr/incose_rwg_gtwr_v4_040423_final_drafts.pdf
61. NLP4RE 2020 paper on QVscribe and INCOSE rules: https://ceur-ws.org/Vol-2584/NLP4RE-paper3.pdf
62. Siemens at CES 2026, nine AI copilots (secondary source, U): https://hyper.ai/en/stories/37d7f8d47de4e5c76ec6b791176556aa
63. Community Codebeamer MCP server: https://enterprisedna.co/directories/mcp/3knightcz-codebeamer-mcp
