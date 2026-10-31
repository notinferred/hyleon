# Appendix 4: Enterprise architecture and AI governance (agent report, 2026-10-31)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-31-scope.md](../2026-10-31-scope.md).

**The brief:** Map enterprise architecture tools (Avolution ABACUS and its peers) and AI governance and runtime guardrail products, with the standards behind them, then judge whether any of them offers domain-researched boundaries for AI-assisted work as Hyleon's packs do, where they overlap with Hyleon, what gaps remain, which of them act as a connecting layer, and which decisions only Dan can make.

---

## How this was done

- **Date and method.** One research agent, 2026-10-31. Web search, then direct reads of vendor pages, docs, GitHub repos, press releases and EU pages. Nothing was installed or run.
- **Markers:**
  - **\*** marks a vendor's claim about itself.
  - **(U)** marks a claim found only in a search snippet, a secondary source, or with conflicting figures. Treat it as unverified.
  - **(M)** marks a fact from my prior knowledge that I could not re-check this session. Treat it as unverified.
  - **(A)** marks my own inference or judgment.
  - "Not found" means I looked and found nothing. Nothing was guessed.
- **Dates.** Every claim carries the date of the event or of its source. "Read 2026-10-31" means the live page, which shows no date of its own.
- **Limits.**
  - The session's shared web-search budget ran out about a third of the way through. Everything after that came from fetching known official pages directly. Some items stayed unverified and are marked.
  - These sites refused automated reading: servicenow.com product pages, iso.org, sparxsystems.com shop and Kernaro pages, businesswire.com, help.sap.com.
  - Almost all enterprise pricing is quote-only.
  - The EU AI Act section is research, not legal advice.
- **Sources** are numbered in brackets, like [4], and listed at the end.

## Summary

1. **Avolution ABACUS is not a competitor to Hyleon today.** It models a company's IT and business: capabilities, applications, processes, data. It doesn't model physical systems. Its AI assistant and MCP server are read-only (read 2026-10-31) [2][3]. Its buyers are enterprise architects in large, regulated organisations, on quote-only plans [5]. It becomes a competitor only if Hyleon sells "boundaries for AI work" to enterprise leadership.
2. **In 2026 every enterprise architecture (EA) vendor pitches its repository as the context and the rules for AI agents.** Avolution's CEO wrote on 2026-10-21 that "the map becomes the rules and briefing agents work from" [8]. Ardoq, SAP LeanIX, Bizzdesign, BOC and Orbus all ship or are shipping MCP servers and AI-agent inventories [11][17][25][30]. MCP access is now table stakes, not a differentiator.
3. **AI governance platforms ship researched rule sets, but about the AI system, not the work.** Credo AI sells "policy packs" for the EU AI Act, NIST AI RMF, ISO 42001 and others, said to be "built and updated by agents and validated by human experts"\* (read 2026-10-31) [32]. They ask "is this AI system allowed, safe and documented?" None of them asks "is what the AI designed correct for its domain?"
4. **Runtime guardrails check text, not substance.** NeMo Guardrails, Guardrails AI, Lakera and Prompt Shields filter prompts and outputs for attacks, toxicity, personal data and format. Microsoft's own docs say Prompt Shields "don't determine whether content is harmful, factually accurate, grounded… compliant with policy, or aligned with the application's task" (doc dated 2026-08-28) [45].
5. **No product in these categories offers domain-researched boundaries for AI-assisted work** the way Hyleon's packs are meant to: questions first, limits that each cite a source, checks on every change, findings that inform and never block. The nearest in form are Credo AI's policy packs (law about AI) and Ardoq's agent workflow (AI proposes, a person approves, every fact keeps its source) [19][20].
6. **Both categories now call themselves a "connecting layer".** The generic connecting job is taken by well-funded vendors. The open part is the researched domain content and checking the AI's actual output against it (A).
7. **The markets are consolidating fast.** Bizzdesign merged with MEGA and Alfabet (2024–2025) [26]. Check Point agreed to buy Lakera on 2025-09-16 [44]. Harvey acquired Guardrails AI on 2026-09-08 [42]. ServiceNow offered AI Control Tower free for a year on 2026-05-06 [40]. Gartner's first Magic Quadrant for AI Governance Platforms came out in 2026 [38][39].
8. **Head-on competition here looks unrealistic for a solo founder (A).** The decision is Dan's; the options are at the end.

---

## A. Enterprise architecture tools

### What these tools are

Enterprise architecture tools hold a model of how a company runs: business capabilities, applications, technology, processes, data and projects, linked to each other. Architects use the links for impact analysis, roadmaps, cost roll-ups and rationalisation. "Architecture" here means IT and business architecture. None of these tools models buildings, panels, piping or other physical systems (A, from every product page read).

Gartner's Magic Quadrant for EA Tools is the market's main yardstick. The 2025 edition is dated 2025-10-06 [31]; the 2026 edition is dated 2026-10-13 [20]. Leader placements below are as each vendor reports them; I didn't read Gartner's reports.

### Product map

| Product | Owner and recent news | AI and MCP (dated) | Boundaries it sets, and their source | Price | Buyers |
|---|---|---|---|---|---|
| **Avolution ABACUS** [1]–[10] | Private, founded 2001 in Australia [7][8]. Deloitte India alliance for EA and AI governance, likely 2026-10 (date inferred from the page) (U) [10]. Claims "Leader in Gartner MQ for EA Tools for 9 consecutive years"\* [6]; 2026 placement not found. | AI Assistant: plain-language questions over the repository, read-only, permission-scoped, model provider not named (read 2026-10-31) [3]. MCP server: read-only, "can't create, change or delete architecture data, so any change stays with a person" (2026-10-15) [9]. Works with Claude, Copilot, any MCP client [2]. | Customer-built metamodel; "hundreds of prebuilt standards and templates"\* such as TOGAF, BizBok, PCF [6]. No-code "algorithms" for cost, risk, fit and impact roll-ups [6]. AI Governance solution: inventory of AI, "automated compliance scoring" against EU AI Act, NIST AI RMF and DORA\*; how the requirements are sourced is not stated [4]. | Quote only. Foundation plan: 5 modelers, 25 editors, 500 collaborators; Enterprise: tailored. AI Assistant in all plans (read 2026-10-31) [5]. | Enterprise architects and IT in large or regulated organisations: banks, government, insurers, universities, manufacturers [1]. |
| **SAP LeanIX** [11]–[16] | Owned by SAP since 2023 (M). 2026 Gartner MQ Leader, "furthest in Completeness of Vision and highest in Ability to Execute"\* (2026-10-21) [15]. | MCP server, available to all customers (2025-11-04) [11]. AI agent hub: discovers agents and MCP servers from SAP, Google and Microsoft repositories; imports in-house agents via an Agent2Agent API (2025-11-04) [12]. 2026 plans add AWS Bedrock agent discovery and Microsoft MCP registry discovery (2026-01-30) [13]. | Lifecycle status and approval, so "only approved agents are deployed" [12]. Technology radar. All set by the customer. A vendor reference catalog of IT components with lifecycle data exists (M); its page didn't load. | Quote only; tiered by number of applications, unlimited users (U) [16]. UK G-Cloud reseller listings: £11,460 to £42,720 per 100-application unit per year (U) [16]. | CIO office, enterprise architects, SAP customers. SAP's own line: "Enterprise architects are not in the driving seat for AI governance" (2026-10-21) [15]. |
| **Ardoq** [17]–[23] | Norway. $125M Series D in 2022, led by EQT (U) [22]. 2026 Gartner MQ Leader, sixth year (MQ dated 2026-10-13; post 2026-10-19) [20]. | MCP server ("AI Gateway") GA 2025-09-30, claimed "first to market"\* [17]. Eight Architecture Agents plus no-code Custom Agents (2026-05-28) [19]. MCP single sign-on via OAuth [19]. External agents can write back, but every output lands in a Scenario branch and "human review is required" [19]. | Customer metamodel and "governance rules"; agents "scoped to their metamodel" [19]. "GraphLake": each fact records its source, who asserted it and when [20]. Deterministic code checks each AI reasoning step against the graph and ontology\* [20]. AI Lens (early Q4 2025): AI inventory and EU AI Act compliance tracking [18][23]. | Quote only. Priced per application, unlimited users; no public figures (read 2026-10-31) [21]. | Enterprise architects. Customer story: Tenneco built six agents in Teams via the MCP server, claiming 292% ROI\* [20]. |
| **Bizzdesign** (Horizzon, HOPEX, Alfabet, Unify) [24]–[27] | Main Capital owns Bizzdesign (since 2022) (U). Merged with MEGA (announced 2024-09-04) and bought Alfabet from Software GmbH (announced 2025-01-09); combined company from 2025-05 (U) [26]. 2026 Gartner MQ Leader, 19th year\* (2026-10-20) (U) [27]. | Unify launched 2026-04-16: whiteboard plus live EA data, "AI-native", agents that summarise designs and suggest improvements [24]. June 2026 roadmap: Horizzon MCP server in beta; HOPEX MCP server with OAuth; Unify "AI agents equipped with EA-specific skills" and "expanded MCP connectivity" planned [25]. | Industry metamodels and frameworks (M). "Model governance" named as a 2026 priority, no detail [25]. | Not found. | Enterprise architecture and portfolio teams; named Unify customers include Rolls-Royce SMR [24]. |
| **MEGA HOPEX** | Now part of Bizzdesign (see above) [26]. | MCP server with new OAuth authentication on the 2026 roadmap; status not stated [25]. | Not found beyond the Bizzdesign roadmap. | Not found. | Large enterprises (M). |
| **Sparx Enterprise Architect** [28] | Australia, private (M). | Kernaro, an "AI hub" for Enterprise Architect, introduced 2026-05-21, in beta (U) [28]. Model Chat, impact analysis, Jira/Confluence/Azure DevOps integration (U). Official MCP not found; a third-party read-only MCP server exists on npm (U). | Modelling-language rules (UML, SysML, ArchiMate, BPMN) and model validation (M). | Perpetual per-seat licences: Professional US$245, Unified US$535, Ultimate US$750 (standard licence; page undated) (U) [28]. | Software, systems and enterprise architects; widely used in government and defence (M). |
| **Orbus OrbusInfinity** (formerly iServer) [29] | London. SilverTree Equity since 2020/2021; FTV Capital co-control investment 2024-12-16 (U) [29]. FedRAMP Authorized, Moderate, in Q1 2026 (2026-05-14) [29]. 2026 Gartner MQ Leader, fifth year\* (U) [29]. | O.ai assistant, "O.ai Agent Network", AI dashboards (2026-05-14) [29]. MCP not mentioned [29]. | AI Governance offering: discover, catalog and risk-assess AI tools and agents (read 2026-10-31) [29]. | Quote only [29]. | Microsoft-centred enterprises and US government [29]. |
| **BOC ADOIT** [30] | Vienna, private (M). 2025 Gartner MQ Leader\* (2025-10-07) (U) [30]. | MCP server since ADOIT 17.5; OAuth 2.0/OIDC in 17.6; 18.0 announced 2025-11-07 [30]. No agent shipped: "customers need their own local agent" [30]. Admins turn any REST endpoint into an MCP tool [30]. | Admin decides which knowledge is exposed: "full control over what knowledge is exposed" [30]. | Community Edition free; commercial editions quote only (2025-11-07) [30]. | Enterprise architects; named customers include Allianz and PostFinance [30]. |

### How EA vendors pitch the repository to AI agents (2025–2026)

| Vendor | The pitch, in their words | Date |
|---|---|---|
| Avolution | "Without rules of the road, they will run into each other." "That briefing is organizational context, and architecture is where it lives." "Exciting announcements to come in the next few months." [8] | 2026-10-21 |
| Avolution | "Gaps in the repository show up as gaps in the answers." [9] | 2026-10-15 |
| Ardoq | Architects are "keeping the architecture graph accurate enough for people and AI agents to rely on" and "deciding what agents can do on their own and where a human stays in the loop." "Architects don't have an AI shortage. They have a trust problem." [20] | 2026-10-19 |
| SAP LeanIX | EA data is "essential for leveraging agentic AI and governing it." [15] | 2026-10-21 |
| BOC | EA data is "reliable, structured, and context-rich," a "trusted single source of truth" for AI. [30] | ~2025-11 (U) |
| Bizzdesign | "AI agents equipped with EA-specific skills" (planned). [25] | 2026-06 |

**What this means (A).** The pitch is about two things: feeding AI agents accurate company context, and keeping an inventory of the agents themselves. None of these vendors says its tool checks what an agent produced against researched limits. Ardoq comes closest: agents propose into a branch, deterministic code checks the steps, and a person approves.

---

## B. AI governance platforms

These govern AI systems as assets: inventory, risk tiering, approvals, evidence, and now runtime control of agents. Gartner published its first Magic Quadrant for AI Governance Platforms in 2026: IBM says May 2026 and Leader [38]; OneTrust says Visionary [39]; Holistic AI says Challenger, posted 2026-07-07 [37].

| Product | Owner and recent news | AI agents and MCP (dated) | Boundaries it sets, and their source | Price | Buyers |
|---|---|---|---|---|---|
| **Credo AI** [32]–[36] | Private, US. Funding not re-checked this session (M: Series B in 2024). Fast Company's Most Innovative Companies 2026, #6\* [32]. Joined the Microsoft for Startups Pegasus Program 2026-07-07 [33]. | GAIA governance agent GA 2026-05-13 [33]. **Agent Governor**, research preview, 2026-07-14: installs "governance-as-code" into the agent harness (Claude Code only so far) as hooks, skills and settings; each action resolves to allow, block, escalate or **advise** [34][35]. MCP security post 2026-05-26 [33]. AIUC-1 agent standard added 2026-09-22 [36]. | **Policy packs** for EU AI Act, NIST AI RMF, ISO 42001, OMB M-25, Colorado ADMT, NAIC AI\*. Content "built and updated by agents and validated by human experts," by a team involved in ISO, NIST, EU Parliament, NATO and OECD work\* [32]. Agent Governor baselines at three postures (permissive, balanced, strict) from its control library; custom policies in progress [35]. | Quote only [32]. | Enterprise risk and AI governance teams in insurance, finance, federal, healthcare; customers include Mastercard [32]. |
| **Holistic AI** [37] | UK and US (M). Funding not found. | "Guardian Agents": Sentinel agents score agent actions and tool calls against policy; Operative agents block, pause or redirect and escalate to a person\* (read 2026-10-31) [37]. MCP not mentioned. | Controls mapped to EU AI Act, NIST AI RMF and ISO/IEC 42001; 40+ tests for security, safety and bias. Sourcing of mappings not stated [37]. | Quote only [37]. | Security, legal, boards; customers include Unilever, Siemens, GSK [37]. |
| **IBM watsonx.governance** [38] | IBM. Gartner MQ AI Governance Platforms Leader, May 2026\* [38]. | "AI-native governance" with embedded assistants; agent features not described on the page (read 2026-10-31) [38]. | "One of the industry's largest AI regulatory and risk content ecosystems"\*: EU AI Act, NIST, ISO 42001, Data & Trust Alliance. Maintainer not stated [38]. | Public: Risk & Compliance Basic from $3,500/month (1 instance, 1 module, 1 concurrent user); Advanced from $6,450/month; on AWS $42,000 (unit not stated); 14-day free trial (read 2026-10-31) [38]. | Risk, compliance and model-risk teams in large enterprises (A). |
| **OneTrust AI Governance** [39] | Private, US. New CEO John Heyman (undated on page) [39]. Gartner MQ Visionary, May 2026\* [39]. | Agents registered with purpose and allowed actions; "Guardian Agents" for runtime oversight; **MCP policies** define which tools an agent may call and what data it can reach, with audit logs\* [39]. AI Guard Python SDK. | Built-in templates for EU AI Act, NIST AI RMF, ISO 42001; seeded GenAI guardrails. Content source not stated [39]. | Quote only; metered on admin users and AI inventory size (read 2026-10-31) [39]. | Privacy, risk and compliance teams (A). |
| **ServiceNow AI Control Tower** [40][41] | ServiceNow. Introduced at Knowledge 2025 (2025-05) [40]. Acquired Moveworks (2025) [40]; Armis and Veza also named, amounts and dates not confirmed in text (U) [40]. | CEO calls ServiceNow the "AI agent of the agents"; rivals' agents "must come through the control tower" (2026-05-06) [40]. ServiceNow MCP Server 1.1.0, OAuth 2.0 (2025-12) [41]. | Agents get identities, rights and privileges, like employees [40]. Framework content not verified (product page blocked). | **Free for a year**, announced at Knowledge 2026 (2026-05-06) [40]. | CIOs and CISOs already on ServiceNow [40]. |

## C. Runtime guardrails

These sit in the request path of an AI app and check each prompt, retrieved document, tool call or output.

| Product | Owner and recent news | What it checks | How its rules are sourced | Price | Buyers |
|---|---|---|---|---|---|
| **Guardrails AI** [42] | **Acquired by Harvey** (legal AI), announced 2026-09-08; future of the open-source project not stated [42]. Validators moving from the Hub to plain PyPI packages, cutoff 2026-08-25; hosted remote inference discontinued (notice 2026-07-06) [42]. | Input and output validators: regex, competitor mentions, toxic language, PII, structure via Pydantic [42]. | Pre-built validators in a community Hub; developers write their own [42]. | Open source, Apache-2.0 [42]. No price listed. | Developers of LLM apps. |
| **NVIDIA NeMo Guardrails** [43] | NVIDIA. v0.24.1 on 16 Sep, latest (year inferred as 2026 from the release sequence) (A) [43]. | Five rail types: input, dialog, retrieval, execution (tool calls), output [43]. Tool calling in IORails from v0.23.0 (01 Jul) [43]. | Developers write Colang flows and YAML config; library of NVIDIA safety models and third-party integrations [43]. | Open source, Apache-2.0 [43]. Production support via NVIDIA AI Enterprise (M); price not found. | Developers and platform teams. |
| **Lakera** [44] | **Check Point agreed to acquire it 2025-09-16**, closing expected Q4 2025 [44]. Site now carries Check Point branding (read 2026-10-31) [44]. | Prompt injection, jailbreaks, data leakage, indirect injection, across prompts, RAG and MCP\* [44]. | Threat data from the Gandalf game community, "80 million-plus adversarial patterns"\* [44]. | "Start for free"; no prices listed [44]. | Security teams. |
| **Microsoft Prompt Shields** (Azure AI Content Safety, Foundry) [45] | Microsoft. Doc dated 2026-08-28 [45]. | User prompt attacks and document (indirect) attacks; tool-response scanning for agents in preview; "Spotlighting" in preview [45]. | Microsoft-trained classifiers. Explicitly **not** for policy compliance or task alignment [45]. | Free tier 5,000 text records/month; paid per 1,000 text records (figure didn't render) [45]. | Developers on Azure. |

---

## D. Standards

| Standard | What it is | Key dates |
|---|---|---|
| **EU AI Act** (Regulation (EU) 2024/1689) [46]–[48] | Binding EU law on AI systems, by risk level. Fines for prohibited practices up to €35M or 7% of worldwide turnover [47]. | In force 2024-08-01. Prohibitions and AI literacy 2025-02-02. GPAI model rules 2025-08-02. Most of the Act applies 2026-08-02, with transparency rules. New prohibition 9 (non-consensual sexual content, CSAM) December 2026. **High-risk, Annex III uses: 2027-12-02. High-risk AI in regulated products, Annex I: 2028-08-02.** The last three come from the AI Omnibus: proposal 2025-11-19, political agreement 2026-05-07, in force 2026-07-27. Page updated 2026-10-19 [46]. |
| **ISO/IEC 42001** [50] | A certifiable AI management system standard: policy, roles, risk and impact assessment, controls (M). | Published December 2023 (M). iso.org refused the fetch, so not re-checked. |
| **NIST AI RMF** [49] | Voluntary US framework, four functions: Govern, Map, Measure, Manage. | 1.0 released 2023-01-26. Generative AI profile (NIST AI 600-1) 2024-07-26. Concept note for a critical-infrastructure profile 2026-04-07. "Being revised as part of the White House AI Action Plan," undated [49]. |
| **AIUC-1** [36] | Certification standard for AI agents by the Artificial Intelligence Underwriting Company; refreshed quarterly; certificates valid twelve months; independent audit. | Integrated into Credo AI 2026-09-22 [36]. |

**Relevance to Hyleon (A, not legal advice).**
- Hyleon's checks are deterministic rules (CEL) and versioned calculations. Under the Act's definition they may not be an "AI system" at all. The AI is the user's own agent, whose provider carries the provider duties. Counsel should confirm this.
- Annex III point 2 covers AI "used as safety components in the management and operation of… the supply of water, gas, heating or electricity" [48]. Hyleon's flagship is a pump station. A design-time checking tool is not a safety component in operation, so this likely doesn't apply. Counsel should confirm this too, especially before any pack touches safety functions.
- Enterprise buyers will ask how Hyleon supports their ISO 42001 or NIST AI RMF programme. Hyleon's per-value sources and its record of AI guesses and human acceptances are the kind of evidence those programmes collect.

---

## Does anyone offer domain-researched boundaries for AI-assisted work?

**No, not in these categories as of 2026-10-31.** Each one researches a different thing:

| Who | What their researched content is about | Who researches it | Checks the AI's work product? |
|---|---|---|---|
| EA tools | The customer's own company: apps, capabilities, costs. Plus generic frameworks (TOGAF, BizBok, PCF) and, for AI governance, AI regulations [4][6]. | The customer's architects. The vendor supplies templates. | No. AI reads the model; Ardoq's agents also propose changes for human review [19]. |
| AI governance | Law and standards about AI systems [32][37][38][39]. | Vendor experts (Credo says agents plus human experts\*) [32]. | No. They check the AI system's documentation, risk and actions, not the substance of what it produced. |
| Runtime guardrails | Attacks, toxicity, PII, output format [42]–[45]. | Vendor models or developers. | Only form and safety, never domain correctness [45]. |
| **Hyleon packs** (planned) | One kind of physical job: questions, limits, checks, deliverables, each value citing a code clause, specification or person ([PRODUCT.md](../../PRODUCT.md)). | Pack author; release needs a named licensed engineer as reviewer ([formats/pack.md](../../formats/pack.md)). | Yes. Checks run on every change, by hand or by agent. |

### Closest analogues, element by element

| Hyleon element | Closest thing found | Gap |
|---|---|---|
| Questions before designing | Credo GAIA and OneTrust intake assessments ask about the AI use case [32][39]. | They ask about the AI system, not about the job. |
| Limits that each cite a source | Credo policy packs map controls to regulations [32]. | About AI law; proprietary; per-clause sourcing not shown. |
| Every value records its source | Ardoq GraphLake: source, asserter, time per fact [20]. | IT facts only; no "AI-inferred until a person confirms" state found. |
| AI proposes, a person confirms | Ardoq Scenario branches with required review [19]; ABACUS and LeanIX keep AI read-only [2][9]. | Closest match. Domain is IT architecture. |
| Deterministic checks | Ardoq's deterministic step checks\* [20]; ABACUS algorithms for cost and risk roll-ups [6]. | No researched domain limits behind them. |
| Inform, never block | Credo's "advise" outcome [35]; Prompt Shields' annotate mode [45]. | Most governance products sell block and escalate as the main value [34][37][39]. |
| Works without AI | EA tools do. | Governance tools exist only to govern AI. |
| Open, versioned, file-based packs | AIUC-1 is versioned quarterly but is a certification standard [36]; Guardrails validators are open code [42]. | No open, sourced, domain pack format found in these categories. |

## Overlap with Hyleon

- **Words.** "Packs" (Credo AI policy packs), "guardrails", "rules of the road", "context for agents", "human in the loop". A buyer may hear Hyleon's pitch as one of these (A).
- **Mechanisms.** MCP access under the user's own permissions [2][17]; OAuth for MCP [19][30]; AI output held for human approval [19]; per-fact provenance [20]; harness hooks and skills as the place to apply rules (Credo targets Claude Code first) [34][35].
- **Buyers, if Hyleon goes "enterprise and leadership".** CIOs, enterprise architects, risk and AI governance boards. Those are exactly the buyers of every product above (A).
- **No overlap today.** Physical design, code clauses, engineering calculations, generated drawings and schedules. None of the products above touches them.

## Gaps none of them fill

1. **Checking the substance of AI work** against a researched body of domain knowledge, with clause-level sources.
2. **Asking what must be known before the work starts**, for the job itself rather than for the AI system.
3. **Marking every AI guess** until a named person confirms it, at the level of single values.
4. **Open, portable, reviewed domain packs** that work with any agent and without one.
5. **Individual professionals and small firms as buyers.** Everything here except Sparx, the open-source guardrails and Prompt Shields is enterprise and quote-only.
6. **Physical systems.** EA tools model IT. ServiceNow's Armis deal covers operational technology for security visibility, not design (U) [40].

---

## Connecting layers (Dan's addendum)

Dan's framing: Hyleon "connects things after research has been done… connect the dots on what AI does or gets". In other words, researched knowledge + the user's model + what an AI produces → consequences and gaps.

| Product | Positions as a connecting layer? | Connects | Produces consequences and gaps? | How close to Dan's framing (A) |
|---|---|---|---|---|
| **Ardoq** | Yes. "Trusted architecture context wherever people already work" [20]. | Documents and agent output → Scenario branch → human approval → graph, with provenance [19][20]. | Yes: blast radius, Foundation Insights flags "gaps and anomalies" [19]. | **Closest in mechanism.** Missing: external researched domain limits; the "research" is the customer's own model. |
| **Avolution ABACUS** | Yes. "Connects business capabilities, applications, technology, processes, and data into one live model" [1]; the map as "rules and briefing agents work from" [8]. | Company model → AI tools, read-only [9]. | Impact analysis; "compliance gaps automatically" for AI governance\* [4]. | Close in narrative, far in mechanism: AI only reads; its output isn't checked. |
| **SAP LeanIX** | Partly. Agent hub links agents to applications and capabilities [12]. | Agent inventory ↔ company model; MCP for outside AI [11]. | Gaps where agents could be deployed; approval status [12]. | Low: about which agents exist, not what they produce. |
| **Bizzdesign Unify** | Yes. Whiteboard plus live dependencies, risks and constraints [24]. | Collaboration ↔ EA data; agents suggest improvements [24]. | Shows risks and constraints in context [24]. | Low to medium; agent checks not described. |
| **Credo AI** | Yes. "AI governance operating layer" [36]. | Researched regulation packs + AI registry + evidence; Agent Governor applies policy to live agent actions [32][34]. | Yes: compliance gaps, risks, escalations [32][34]. | **Closest in "researched knowledge + what AI does".** Missing: the work's own domain; it judges actions and documentation, not designs. |
| **OneTrust** | Yes. Policy → controls → runtime [39]. | Policies + inventory + MCP tool limits [39]. | Risk tiers, findings, blocks [39]. | Medium-low; same gap as Credo. |
| **ServiceNow AI Control Tower** | Yes. "AI agent of the agents" [40]. | All agents and workflows on one platform [40]. | Not verified. | Low; generic, platform-bound. |
| **Holistic AI** | Yes. Identify, Protect, Enforce [37]. | Inventory + tests + policy [37]. | Risk scores, blocks [37]. | Low-medium. |
| **Runtime guardrails** | No. They are filters in the request path. | Prompt or output ↔ generic policy [42]–[45]. | Pass or fail on text. | Low. Prompt Shields explicitly excludes task alignment [45]. |

**What this implies for Hyleon (A).**
1. "Connecting layer" is already how both categories describe themselves in 2026. A generic connecting layer for AI work would enter a crowded field with SAP, IBM, ServiceNow and PE-backed vendors.
2. What none of them has is the researched domain content and the check of what the AI actually produced against it. In Dan's framing that is the "after research has been done" part. That is where Hyleon is different, and it's the expensive part to build: each domain needs its own research and a qualified reviewer.
3. The connectors are converging on MCP, OAuth and harness hooks. Hyleon could plug into them instead of competing: read a company's model through an EA vendor's MCP server, and hand its findings and provenance to a governance platform as evidence. These are options, not recommendations; Dan decides.
4. "Inform, never block" sets Hyleon apart from governance products, which sell blocking and escalation. Credo's own framework argues that rules which can't be reduced to a detector need "steering", not blocking [35]. That supports Hyleon's stance, but enterprise buyers may still ask for enforcement.

---

## Decisions only Dan can make

1. **Scope.** Stay with physical design, or widen to "any kind of work that involves AI"? TRUTHS.md defines the problem around physical design, and it says a change to its truths makes a different product. Options:
   - (a) Physical design only. Tightest fit with the truths, and no direct competitor found here.
   - (b) Physical design first, with a pack format that could later carry other domains. Keeps the option open without selling it yet.
   - (c) A cross-domain framework sold to enterprise and leadership now. Puts Hyleon into the EA and AI governance markets above.
2. **Buyer.** Individual professionals and small firms (per seat, bottom-up), or enterprise leadership (quote-only, long sales cycles, security certifications such as Orbus's FedRAMP) [29]?
3. **Compete or plug in?** If Hyleon widens, does it compete with EA and governance platforms, or feed them? For example, read their MCP servers and export findings as evidence.
4. **Who researches non-physical packs?** Physical packs need a named licensed engineer to release. Who would research and sign off a pack for enterprise architecture, finance or leadership work, and what would count as its "source"?
5. **Blocking.** Enterprise governance buyers expect block and escalate [34][37][39]. Does "inform, never block" stay absolute in every market?
6. **Naming.** Credo AI already sells "policy packs" [32]. Does that matter for Hyleon's "packs"?
7. **EU AI Act position.** Ask counsel whether Hyleon is an AI system, and whether any planned pack touches Annex I or Annex III uses (see section D).
8. **Is head-on competition realistic for a solo founder? My judgment (A): not in these two categories.**
   - Gartner evaluated 15 EA vendors in 2026 (U) [29]. All are established, PE-backed or owned by SAP, with enterprise sales teams.
   - AI governance has IBM, ServiceNow (free for a year [40]), OneTrust and well-funded specialists.
   - Startups are being bought, not outgrown: Lakera by Check Point [44], Guardrails AI by Harvey [42].
   - A niche with deep researched content in one domain they don't cover looks more realistic. Whether to accept that trade is Dan's call.

---

## Sources

All read 2026-10-31 unless a date is given. "Snippet" means only a search-result snippet was seen.

1. Avolution home: https://www.avolutionsoftware.com/
2. ABACUS MCP Server: https://www.avolutionsoftware.com/abacus/abacus-mcp-server/
3. ABACUS AI Assistant: https://www.avolutionsoftware.com/abacus-ai-assistant/
4. Avolution AI Governance: https://www.avolutionsoftware.com/solutions/ai-governance/
5. Avolution pricing: https://www.avolutionsoftware.com/pricing/
6. ABACUS features and algorithms: https://www.avolutionsoftware.com/abacus/ ; https://www.avolutionsoftware.com/abacus/algorithms/
7. Avolution about: https://www.avolutionsoftware.com/about/
8. "Avolution at 25" (2026-10-21): https://www.avolutionsoftware.com/our-resources/avolution-25-years-enterprise-architecture-ai-governance/
9. "What is an MCP server?" (2026-10-15): https://www.avolutionsoftware.com/our-resources/what-is-an-mcp-server/
10. Avolution press releases: https://www.avolutionsoftware.com/resource-type/press-release/
11. SAP LeanIX MCP server (2025-11-04): https://www.leanix.net/en/blog/mcp-server-for-sap-leanix-solutions
12. SAP LeanIX AI agent hub (2025-11-04): https://www.leanix.net/en/blog/sap-leanix-announces-launch-of-ai-agent-hub-and-key-industry-partnerships
13. SAP LeanIX 2026 plans (2026-01-30): https://www.leanix.net/en/blog/sap-leanix-2026-building-on-momentum
14. AI in SAP LeanIX: https://www.leanix.net/en/solutions/artificial-intelligence-in-sap-leanix
15. SAP News, Gartner MQ 2026 (2026-10-21): https://news.sap.com/2026/10/sap-leanix-six-time-leader-gartner-magic-quadrant-ea-tools/
16. LeanIX pricing (snippets): https://www.g2.com/products/leanix-enterprise-architecture-management/pricing ; https://www.applytosupply.digitalmarketplace.service.gov.uk/g-cloud/services/379962414817680
17. Ardoq MCP GA (2025-09-30): https://www.ardoq.com/blog/announcing-mcp-ga
18. Ardoq Q3 2025 AI roundup (2025-09-29): https://www.ardoq.com/blog/q3-2025-ardoq-ai-roundup
19. Ardoq agentic workforce (2026-05-28): https://www.ardoq.com/blog/new-agentic-ai-workforce-spring-2026
20. Ardoq Gartner MQ 2026 (2026-10-19): https://www.ardoq.com/blog/leader-gartner-magic-quadrant-ea-2026
21. Ardoq pricing: https://www.ardoq.com/pricing
22. Ardoq funding (snippet, 2022): https://siliconcanals.com/ardoq-raises-114m/
23. Ardoq help, Enterprise AI Governance: https://help.ardoq.com/en/collections/1423227-enterprise-ai-governance
24. Bizzdesign Unify launch (2026-04-16): https://itbrief.com.au/story/bizzdesign-launches-unify-to-link-collaboration-data
25. Bizzdesign 2026 roadmaps (June 2026): https://bizzdesign.com/customer/bizzdesign-product-roadmaps-2026
26. Bizzdesign deals (snippets): https://bizzdesign.com/news/news-bizzdesign-adds-alfabet ; https://www.forrester.com/blogs/the-enterprise-architecture-tool-market-starts-2025-with-a-notable-merger/ ; https://bizzdesign.com/press-releases/bizzdesign-announces-new-leadership-team-following-merger-mega-and-alfabet
27. Bizzdesign Gartner MQ 2026 (snippet, 2026-10-20): https://hipther.com/news/2026/10/20/161216/bizzdesign-recognized-for-19-consecutive-years-as-a-leader-in-gartner-magic-quadrant-for-ea-tools
28. Sparx (snippets): https://sparxsystems.com/buy ; https://sparxsystems.com/press/newsletter/2026-05.html ; https://kernaro.sparxsystems.com/ (refused fetch)
29. Orbus: https://www.orbussoftware.com/company/newsroom/orbus-software-accelerates-into-2026-with-customer-growth (2026-05-14) ; https://www.orbussoftware.com/llms.txt ; FTV investment (snippet, 2024-12-16): https://www.businesswire.com/news/home/20241216585552/en/Orbus-Software-Announces-Strategic-Growth-Investment-from-FTV-Capital-and-SilverTree-Equity ; Gartner MQ 2026 (snippet): https://www.orbussoftware.com/product/why-orbus/awards-and-recognition/gartner-magic-quadrant-download
30. BOC ADOIT: https://www.boc-group.com/en/blog/ea/bringing-ea-data-to-ai-with-the-adoit-mcp-server ; https://docs.boc-group.com/adoit/en/docs/17.6/news/ (snippet) ; ADOIT 18.0 (2025-11-07): https://www.prnewswire.com/de/pressemitteilungen/adoit-18-0-redefining-enterprise-architecture-with-built-in-intelligence-302607721.html ; 2025 MQ (snippet, 2025-10-07): https://tools.prnewswire.com/en-us/live/20823/release/20251007EN91164
31. Bizzdesign, 2025 Gartner MQ for EA Tools, report dated 2025-10-06 (snippet): https://bizzdesign.com/press-releases/bizzdesign-recognized-leader-2025-gartnerr-magic-quadranttm-enterprise-architecture
32. Credo AI home: https://www.credo.ai/
33. Credo AI blog index: https://www.credo.ai/blog
34. Credo AI Agent Governor (2026-07-14): https://www.credo.ai/blog/introducing-credo-ai-agent-governor
35. Credo AI Agent Governance Configuration (2026-07-14): https://www.credo.ai/blog/agent-governance-configuration-a-framework-for-governing-autonomous-ai-agents-at-the-harness
36. Credo AI and AIUC-1 (2026-09-22): https://www.credo.ai/blog/credo-ai-brings-aiuc-1-the-worlds-first-ai-agent-standard-into-the-ai-governance-operating-layer
37. Holistic AI: https://www.holisticai.com/
38. IBM watsonx.governance: https://www.ibm.com/products/watsonx-governance ; https://www.ibm.com/products/watsonx-governance/pricing
39. OneTrust AI Governance: https://www.onetrust.com/solutions/ai-governance/ ; https://www.onetrust.com/pricing/
40. diginomica, ServiceNow Knowledge 2026 (2026-05-06): https://diginomica.com/knowledge-2026-ceo-bill-mcdermott-was-playing-chess-last-year-now-he-says-servicenow-manages
41. ServiceNow MCP Server release notes (2025-12-11): https://www.servicenow.com/docs/r/C17ZqvSaMwUVpMBg8Nc5sg/6gAc8my0zIOsHaiId4ONrA
42. Guardrails AI: https://github.com/guardrails-ai/guardrails ; https://www.guardrailsai.com/ ; https://www.guardrailsai.com/blog ; Harvey acquisition (2026-09-08): https://www.guardrailsai.com/blog/guardrails-ai-joins-harvey
43. NeMo Guardrails: https://github.com/NVIDIA/NeMo-Guardrails ; https://github.com/NVIDIA/NeMo-Guardrails/releases ; https://docs.nvidia.com/nemo/guardrails/latest/release-notes.html
44. Lakera: https://www.lakera.ai/ ; Check Point acquisition (2025-09-16): https://www.checkpoint.com/press-releases/check-point-acquires-lakera-to-deliver-end-to-end-ai-security-for-enterprises/
45. Prompt Shields (doc 2026-08-28): https://learn.microsoft.com/en-us/azure/ai-services/content-safety/concepts/jailbreak-detection ; pricing: https://azure.microsoft.com/en-us/pricing/details/cognitive-services/content-safety/
46. European Commission, AI Act page (updated 2026-10-19): https://digital-strategy.ec.europa.eu/en/policies/regulatory-framework-ai
47. Credo AI, EU AI Omnibus explainer: https://www.credo.ai/downloadsopen/eu-ai-omnibus-playbook-what-changed-in-the-eu-ai-act
48. AI Act text, Annex III and Article 4 (secondary, Future of Life Institute site): https://artificialintelligenceact.eu/annex/3/ ; https://artificialintelligenceact.eu/article/4/
49. NIST AI RMF: https://www.nist.gov/itl/ai-risk-management-framework
50. ISO/IEC 42001 (refused fetch; facts from memory): https://www.iso.org/standard/42001
