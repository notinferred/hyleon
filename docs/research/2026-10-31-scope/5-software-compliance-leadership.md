# Appendix 5: Software guardrails, compliance automation and leadership tools (agent report, 2026-10-31)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-31-scope.md](../2026-10-31-scope.md).

**The brief:** Research how software and infrastructure guardrails, compliance automation, and leadership and strategy tools source, maintain and apply boundaries (including for AI agents), to inform whether Hyleon's sourced-pack idea should widen beyond physical design into a cross-domain framework for AI-assisted work.

---

## How this was done

- **Date and method.** One research agent, 2026-10-31. Web search plus direct reads of vendor pages, docs, GitHub repos and press releases. Nothing was installed or run.
- **Markers:**
  - **\*** marks a vendor's claim about itself.
  - **(U)** marks a claim found only in secondary or aggregator sources, or with conflicting figures. Treat it as unverified.
  - **(A)** marks my own inference or assumption.
  - "Not found" means I looked and found nothing; nothing was guessed.
- **Dates.** Every claim carries the date of the event or of the source. "Fetched 2026-10-31" means I read the live page today.
- **Limits.**
  - The session's web search budget ran out near the end. The last checks were done by fetching known pages directly. A few items stayed unverified and are marked.
  - Several vendor pages returned 404 or gave no pricing. Most enterprise pricing is quote-only.
  - Licenses of open-source tools are from memory and were not re-checked.
- **Sources** are numbered in brackets, like [4], and listed at the end.

## Summary

1. **Every category already sells "packs".** AWS conformance packs, Credo AI policy packs, Kyverno policy sets inside an MCP server, Vanta and Drata test libraries, Agent Skills, Cisco's CodeGuard rules, the Secure Controls Framework. Packaged, maintained boundaries are a proven product shape. What differs is who researches them, whether each rule names its source, and whether they are checks or prose.
2. **The formats AI agents actually adopted are the least structured.** AGENTS.md has no required fields [18]. SKILL.md requires only a name and a description; it has no field for sources, versions or tests [19]. They won by being free, minimal and backed by platform vendors through a Linux Foundation body.
3. **Unresearched rule files barely help.** A February 2026 ETH Zurich study found developer-written context files raised agent success by about 4%, LLM-written ones lowered it by about 3%, and both raised cost by over 20% [29]. Skill marketplaces without review have real malware problems [22][23].
4. **Software guardrails are moving from "check" to "enforce on agents".** AWS AgentCore Policy (GA 2026-03-03) blocks agent tool calls with Cedar rules [7]. Drata's AI Agent Governance (limited availability 2026-08-04) blocks policy-violating tool calls through an MCP proxy [32]. Kyverno says it will cover MCP gateways [4]. Hyleon's truth "inform, never block" runs against this trend.
5. **Compliance automation is the closest existing business to Hyleon's shape.** Framework, then controls, then automated tests on live data, then evidence, then a human auditor decides. Vanta was valued at $4.15B in July 2025 [31]. The content (control and test libraries) is proprietary and vendor-maintained. The Delve allegations of March 2026 show what happens when evidence isn't real [35].
6. **Leadership tools connect AI to strategy data, but set no researched boundaries.** WorkBoard, Cascade and Lattice give agents read and write access to OKRs and goals [40][41][42]. No tool I found checks AI-assisted leadership work against sourced limits. AI governance platforms (Credo AI, ServiceNow) govern AI systems, not the content of leadership decisions [46][47].
7. **No cross-domain, open, sourced, checkable pack format exists.** The nearest pieces are each partial: SCF and OSCAL are sourced but not agent-facing checks; Agent Skills and AGENTS.md are agent-facing but unsourced; Vanta, Drata and Credo AI are sourced and checked but closed and single-domain.
8. **Hyleon's CEL choice matches where software policy is going.** Kubernetes and Kyverno (1.17, 2026-02) moved to CEL [3][4]. If packs ever cover software, the rule language is already common ground.

---

## A. Boundaries for AI-written software and infrastructure

### A1. Policy as code

| Tool | What it checks | How rules are sourced and maintained | AI / MCP | Price | Buyers | Recent, dated |
|---|---|---|---|---|---|---|
| **Open Policy Agent (OPA)** + **Conftest** | Any structured input: Kubernetes, Terraform plans, CI, API requests. Conftest runs OPA rules as tests on config files. | Each team writes Rego. No shared library of sourced rules. Rego is hard for non-programmers (Hyleon's own stack research, 2026-10-28) [50]. | No AI or MCP features in the last three releases [1]. Harness generates Rego from plain English and tests it before enforcing\* (2026-09-11) [3]. | Free, open source (Apache-2.0, from memory). | Platform and security teams. | OPA creators and many Styra staff joined Apple, announced 2025-08; OPA stays a CNCF graduated project [2]. v1.22.0 on 2026-10-29 adds `rego.v2`; v1.21.3 on 2026-10-19 fixed two bundle-signature bypasses [1]. Styra DAS status after the Apple move: not found. |
| **Kyverno** | Kubernetes resources at admission and in CI. | Policies in YAML and CEL. A community policy library. Nirmata's MCP server embeds three curated sets: Kubernetes Pod Security Standards, RBAC best practice, operational security (2026-01) [5]. Policy metadata has no required source field (A, from the library format). | Nirmata open-sourced a Kyverno MCP server (2026-01) [5]. CNCF says upcoming releases add "support for artificial intelligence and MCP gateways" [4]. | Free, open source. Nirmata sells the enterprise layer; price not found. | Platform teams. Named users include Bloomberg, LinkedIn, Spotify, Vodafone [4]. | CNCF graduation 2026-03-24; 9,000+ GitHub stars [4]. Kyverno 1.17 (2026-02) made the CEL engine v1 (U). |
| **Cedar** | Authorization: may this principal do this action on this resource. | Each team writes policies against a schema. Language is formally verified (Lean spec) [6]. | Basis of AgentCore Policy (below). | Free, open source. | App and platform teams. | Accepted to CNCF Sandbox 2025-10-08 [6]. Version 4.5 on 2026-04-23 (U). |
| **AWS Bedrock AgentCore Policy** | Every call an AI agent makes to a tool through AgentCore Gateway. Default deny. | Customer writes Cedar, or describes the rule in English. AWS translates it, validates it against the tool schema and uses automated reasoning to check safety conditions [7]. A third party warns translation errors can be silent (U) [7]. | It is an agent guardrail. Added Bedrock Guardrails support 2026-06-17 (U) [7]. | Not checked. | AWS customers building agents. | Preview 2025-12; GA 2026-03-03 [7]. |

**Reading.** Policy as code is mature, free and engine-centric. Rules are written by each customer. Nobody ships a researched, sourced rule set except for narrow, well-defined standards (Pod Security Standards, CIS). The 2026 move is to put the same engines in front of AI agents and to let people write rules in English.

### A2. IaC and cloud configuration checks

| Tool | What it checks | How rules are sourced and maintained | AI / MCP | Price | Recent, dated |
|---|---|---|---|---|---|
| **Checkov** (Palo Alto Networks, Prisma Cloud) | Terraform, CloudFormation, Kubernetes, Helm, Bicep, ARM, Dockerfiles, CI files; also SCA and secrets. | "Over 1000 built-in policies"\* in Python or YAML, maintained by Prisma Cloud (README, fetched 2026-10-31) [8]. Cortex Cloud's IaC scanning is built on it [8]. | Palo Alto mentions AI-assisted policy creation; details not confirmed [8]. | Free, open source; Cortex Cloud is quote-only. | No 2026 release found. |
| **Trivy** (Aqua Security; absorbed tfsec) | Containers, IaC, Kubernetes, cloud misconfigurations, vulnerabilities. | Checks live in `trivy-checks`, written with Rego helpers, documented on avd.aquasec.com [9]. tfsec's move into Trivy is from memory (2023), not re-checked. | Not found. | Free, open source. | **Supply-chain compromise, 2026-03-19 to -21:** stolen credentials were used to publish a malicious Trivy v0.69.4 and to repoint trivy-action tags; CVE-2026-33634 assigned 2026-03-24 [10]. Barracuda updated its notice on 2026-08-24 after finding an unrotated secret [10]. |
| **AWS Config conformance packs** | AWS resources, against bundles of Config rules. | AWS publishes sample packs per framework (NIST 800-53, NIST CSF, CIS Level 1 and 2, HIPAA, FedRAMP) [11]. AWS states the samples "are not designed to ensure your compliance" [11]. | AWS MCP Server GA 2026-05-06 (U). | $0.001 per conformance-pack evaluation for the first 100,000 a month, per account and region [11]. | Not found for 2026. |
| **Azure Policy** | Azure resources. | Built-in "Regulatory Compliance" initiatives map policy definitions to controls of standards (for example FedRAMP High, NIST SP 800-171 R2, Microsoft cloud security benchmark), with a responsibility per control: customer, Microsoft or shared. The feature is in preview. Microsoft says the mappings "can change over time" and points to its GitHub repo to watch changes [12]. | No official Copilot or MCP authoring found. | Free for Azure resources (from memory, not re-checked). | Not found for 2026. |

**Reading.** Cloud vendors already turn frameworks into checks, then disclaim that passing them means compliance. That is the same stance Hyleon takes ("checked against clause X", never "compliant"). The Trivy incident shows that the checking tool itself is attack surface.

### A3. Architecture fitness functions

- **ArchUnit** (Java) and **dependency-cruiser** (JavaScript and TypeScript) run layer and dependency rules as tests in CI. They are precise for "package A may not import package B" and cannot express intent or behaviour (developertoolkit.ai, undated) [13].
- **Rules are written by each team.** There are no external sources and no shared research. Both are free and open source.
- **The 2026 trend is feeding results back to agents.**
  - Thoughtworks' Technology Radar (2026-04) reportedly recommends fitness functions against the loss of shared understanding that fast AI change causes (U; seen only via Qodo's blog) [13].
  - A vendor survey says 18.7% of engineering leaders report AI review tools miss architectural violations\* [13].
  - archfit, a small Go tool, reads dependency facts across languages and emits "repair tasks" for agents\* (U) [13].
  - Guides recommend a "ratchet": agents may not add new violations even while old ones remain [13].

**Reading.** This is the software analogue of Hyleon's cross-reference checks. It has no notion of sources, and no product layer to speak of.

### A4. Developer portals with scorecards

| Product | What it checks | Where rules come from | AI / MCP | Price | Funding and releases, dated |
|---|---|---|---|---|---|
| **Backstage** (CNCF) + **Spotify plugins** | Soundcheck runs checks and tracks on catalog entities. | Customers write checks; Spotify ships the engine. | Backstage's MCP Actions Backend exposes registered actions as MCP tools [14]. Soundcheck exposes 42 actions: checks, tracks, campaigns, exemptions, facts, compliance analytics [14]. Spotify also sells AiKA and Xirp ("AI coding context grounded in your services & docs")\* [14]. | Not public ("Talk to us"), fetched 2026-10-31 [14]. | No dated 2026 release found. |
| **Port** | Scorecards on any catalog entity. | Customers define them. | Rebranded as an "Agentic Engineering Platform"\* [15]. Its "context lake" puts "policies and controls directly into the metadata layer" so agents are checked before acting\* [15]. | Free $0 (15 seats); Basic $30 per seat per month; Standard $40; Enterprise custom (fetched 2026-10-31) [15]. | $100M Series C at $800M valuation, 2025-12-11, led by General Atlantic [15]. |
| **Cortex** | Scorecards, including "AI Readiness" and "AI Maturity" templates [16]. | Vendor templates; customers edit. No external sources cited. | Cortex MCP answers questions over catalog, scorecards, metrics [16]. | Not public. | $60M Series C, 2024-09, at $470M [16]. A $100M Series D in 2026-06 appears only on an aggregator (U) [16]. |
| **OpsLevel** | Checks and rubrics on services. | Customers define them. | Hosted MCP server, read-only, in Claude's connectors directory [17]. | Not public; no free plan (U) [17]. | No dated 2026 release found. |

**Reading.** Portals are the software world's "one model": a catalog of everything. Their scorecards are the checks. But the rules are written by each customer, with no research and no sources. All four now feed agents through MCP.

### A5. Agent rule files and their ecosystems

| Format or ecosystem | Structure | Sources, versions, tests | Distribution | Dated facts |
|---|---|---|---|---|
| **AGENTS.md** | Plain Markdown. "No required fields"\* [18]. Nested files; nearest wins. | None. Versioned only through git [18]. | Read by ~25 tools listed on agents.md, including Codex, Cursor, Copilot's agent, Gemini CLI, Devin [18]. | "Used by over 60k open-source projects"\* (agents.md, fetched 2026-10-31) [18]. Stewarded by the Agentic AI Foundation under the Linux Foundation, founded 2025-12 with MCP and goose [18]. |
| **CLAUDE.md** | Claude Code's project memory file, free-form Markdown. | None. | Claude Code. | Claude Code plugins bundle skills, agents, hooks and MCP servers; an official Anthropic marketplace is added by default; managed settings can restrict marketplaces (U, secondary) [21]. |
| **Cursor rules** | `.cursor/rules/*.mdc` with `description`, `globs`, `alwaysApply`. Team Rules from the dashboard; admins can enforce them. Precedence: Team, then Project, then User. Also reads AGENTS.md [20]. | No source or version fields; no schema validation beyond the extension (docs, fetched 2026-10-31) [20]. | Rules ship inside plugins from Cursor's Marketplace ("manually reviewed"\*) or team marketplaces (Cursor 2.6, 2026-03) [20]. | Forum bug: plugin rules lose `alwaysApply` and become optional [20]. |
| **Agent Skills (SKILL.md)** | Folder with `SKILL.md`: required `name` and `description`; optional `license`, `compatibility`, `metadata`, experimental `allowed-tools`; optional `scripts/`, `references/`, `assets/` [19]. | No source field, no version field (only free-form `metadata`), no tests. The validator checks frontmatter only [19]. | Published as an open standard 2025-12-18 (U). Adopted by 30 to 40+ tools (U, figures vary) [19]. | — |
| **Skill marketplaces** | skills.sh (Vercel), ClawHub, Tessl, PRPM, Claude and Cursor marketplaces. | skills.sh: ~669,670 skills listed (2026-06), no review board, install counts only; Snyk, Socket and Gen audit pages (U) [22]. Tessl: skills with evals and quality scores; reframed 2026-01-29 as "the package manager for agent skills"\* [24]. PRPM converts rules between Cursor, Claude and other formats; 1,800+ packages\* [25]. | — | Snyk's "ToxicSkills" audit of 3,984 skills: 534 (13.4%) critical issues, 1,467 (36.8%) some flaw, 76 confirmed malicious (U; seen only in secondary summaries; date not confirmed) [23]. |
| **Project CodeGuard** (Cisco, now CoSAI) | Security rules for AI coding agents in one Markdown format, translators to each agent's format, and validators [26]. | Whether each rule cites OWASP or CWE: not confirmed [26]. Cisco says the rules don't guarantee secure output [26]. | Works with Claude Code, Codex, Cursor, Windsurf, Copilot and others\* [26]. | Open-sourced 2025-10; donated to the Coalition for Secure AI (OASIS) 2026-02-09 [26]. |
| **OpenSSF guide for AI code assistant instructions** | A prose guide for writing instruction files. | Most recommendations carry citation tags to a source appendix (papers, ANSSI, BSI, OpenSSF) [27]. Advises short instructions and warns that "act as a security expert" prompts performed poorly in studies [27]. | Copy into your own files. | Published 2025-08-01 [27]. |
| **GitHub Spec Kit** | A "constitution" of project principles, then specify, plan, tasks, implement. | Constitution is free-form. A clarify step asks questions before planning (from memory, not re-checked). | Supports Claude Code, Copilot, Cursor, Gemini CLI and 25+ agents\* [28]. | 82,000+ stars (2026-06, U) [28]. Thoughtworks Radar (2026-04): teams hit "instruction bloat" and "context rot" [28]. |
| **Semgrep registry** | Code rules with metadata. | Security rules must carry a CWE field; OWASP tags are common [30]. | Registry rulesets. | — |

**Evidence on whether rule files work.** Gloaguen et al., ETH Zurich, arXiv:2602.11988 (2026-02), tested Claude Code, Codex and Qwen Code on 138 tasks from 12 Python repos [29]:
- developer-written AGENTS.md files: about +4% success
- LLM-generated files: about −3%
- both: over 20% more inference cost
- codebase overviews did not help agents find files faster
- explicit tool instructions (use `uv`, not `pip`) were followed closely

Another January 2026 study found efficiency gains instead (U) [29]. The effects are small either way.

**Reading.** The agent-rule ecosystem is huge, open and nearly unstructured. It has distribution (60k+ repos, tens of tools, marketplaces) but no research, no sources, no tests and weak curation. The one exception that cites sources (OpenSSF) is prose, not checks. CodeGuard is the closest to "researched boundaries pack for AI-written software": open, agent-consumable, validated, governed by a standards body. But it covers software security only.

### What A means for Hyleon

- **Format.** If Hyleon packs ever cover software, CEL is already the shared language (Kubernetes, Kyverno). Rego is the incumbent but hard to read. Cedar fits authorization, not design checks.
- **Distribution.** Agents read AGENTS.md and SKILL.md everywhere. A pack could be exported to these formats for reach, while its sources, checks and tests stay in the pack (A). That is a choice for Dan (see decisions).
- **Differentiation.** Nobody in A combines questions first, sourced limits, executable checks, tests and per-value provenance. Each tool has one or two of these.
- **Stance.** Most of A blocks: admission control, gateways, CI gates. Hyleon informs. In software, buyers often want blocking (A).

---

## B. Compliance automation

### How a framework becomes checks with evidence

1. **Framework.** SOC 2 (AICPA criteria) or ISO 27001 (ISO text, copyrighted).
2. **Controls.** The vendor writes a control library and maps each control to framework requirements. Custom frameworks are allowed [32].
3. **Tests.** Automated tests run against live data pulled through integrations: cloud, identity, HR, device management. Tests are vendor-owned and pre-mapped to controls. Turning on a framework provisions its tests [32].
4. **Custom tests.**
   - **Drata:** a Test Builder with a simple and an advanced editor; Advanced plan or higher; there is a cap on the number of tests; data can come from "custom connections" [32].
   - **Vanta:** custom tests on one resource type at a time, with a subset of logic. They can't be edited after creation: you copy one and edit the copy, and the original is kept for audit [31].
5. **Evidence.** A passing or failing test produces audit-visible evidence. Mapped tests drive "control readiness" per framework. Unmapped tests show status but don't count [32].
6. **Human decision.** A licensed CPA firm (SOC 2) or an accredited certification body (ISO 27001) issues the opinion. The platform doesn't certify.

This is close to Hyleon's loop: pack, then checks on the model, then findings, then a person signs. The difference is that the vendor's library is closed and the "model" is the customer's live systems, not a design.

### Vendors

| Vendor | AI and MCP, dated | Price | Funding and news, dated | Buyers |
|---|---|---|---|---|
| **Vanta** | Vanta AI Agent on all plans\* [31]. Remote MCP server in public preview for all customers from 2026-04-15; a Claude plugin finds failing tests and opens pull requests across "500+ cloud tests"\* [31]. Earlier self-hosted MCP server deprecated [31]. | Quote-only [31]. Vendr data: about $12k to $25k a year for 1 to 50 employees, one framework; about $20k to $40k for 51 to 200 [31]. Audit fees extra. | $150M Series D at $4.15B, 2025-07-23; $504M raised in total [31]. About $300M ARR by 2026-04 (U). "400+ tools" integrated\* [31]. | Startups to mid-market; security and GRC leads. |
| **Drata** | Experimental MCP server, 2025-06-12 [32]. AIUC-1 support, 2026-07-16 [32]. **AI Agent Governance:** early access 2026-06-10; limited availability 2026-08-04 [32]. It finds shadow agents, logs actions, scores trust, and checks each tool call against policy before it runs, blocking violations. Policies are written in plain English, compiled to rules, and can be simulated against a year of past traffic. Anthropic is the first supported platform\* [32]. | Quote-only. Vendr: about $20k to $45k a year for a mid-size company doing SOC 2 Type II [32]. | Acquired SafeBase, 2025 [32]. | Mid-market and enterprise. |
| **Secureframe** | "Comply AI" for remediation as IaC (2023-06), risk (2023-10), control mapping (2024-05-02), and AI security assessments (2025) [33]. No 2026 AI announcement found. | Quote-only. | $56M Series B, 2022; no later round found [33]. | SMB to mid-market; FedRAMP content. |
| **Sprinto** | Sprinto AI (2025); rebranded as "Autonomous Trust Platform", 2026-03 (U) [34]. | Quote-only; competitor-published estimates of about $5k to $16k+ a year (U) [34]. | $20M Series B led by Accel; date unclear in sources (U) [34]. | SMB, India and US. |
| **Delve** (cautionary) | AI-first compliance. | — | 2026-03-21: a whistleblower alleged fabricated SOC 2 and ISO 27001 evidence and rubber-stamping auditors. Reportedly 493 of 494 reports were nearly identical (U). Delve denies it and says independent CPA firms issue all opinions [35]. | — |

### Shared and open sources of controls

- **Secure Controls Framework (SCF).**
  - A free metaframework: "1,400+ controls across 200+ laws, regulations and frameworks"\* [36].
  - Release 2026.1 came out on 2026-04-01. The 2026.2 newsletter (2026-07-07) says every law, regulation and framework now has a published Set Theory Relationship Mapping (STRM), following NIST IR 8477 [36].
  - This is the closest thing to "researched, sourced boundaries" across many domains, but only for cybersecurity and privacy. It has no executable checks.
- **OSCAL and FedRAMP 20x.**
  - OSCAL is NIST's machine-readable format for control catalogs, profiles and assessment results.
  - FedRAMP's consolidated 2026 rules launched on 2026-06-24 and replace narrative documents with "Key Security Indicators" and machine-readable evidence [37].
  - The KSI count conflicts between sources (46 vs 56/61) (U). An OSCAL export deadline of 2026-09-30 appears only in a vendor blog (U) [37].
- **Open-source GRC platforms.** Comp AI (AGPLv3, open core), Probo (open source plus a service), Openlane, VerifyWise [38]. No independent evaluations found.
- **AIUC-1.**
  - A standard for AI agents, crosswalked to ISO 42001, the EU AI Act, NIST AI RMF, MITRE ATLAS and OWASP [39].
  - Schellman became the first authorized auditor on 2026-02-03 (U). It is refreshed quarterly (U) [39].
  - Drata supports it [32]. This is a "researched boundary set" for AI agents, but it is a certification, not an open pack format.

### What B means for Hyleon

- **Willingness to pay is proven** where a third party (auditor, customer, regulator) demands evidence. Buyers pay $10k to $100k a year mainly for maintained control and test libraries, integrations and the path to an auditor (A).
- **The wording discipline matches.** AWS, Azure and the platforms all say the checks don't equal compliance. Hyleon's D-025 wording ("checked against", never "compliant") is the norm here.
- **Evidence integrity is the product.** Delve shows the failure mode: AI-made evidence nobody checked. Hyleon's "AI-guessed until a person confirms" is the right guard. No compliance vendor exposes per-value provenance like that (not found).
- **Copyright.** ISO 27001 text is copyrighted like UL 508A. Vendors map to clauses, as Hyleon plans to.
- **Competition.** Drata and Vanta are already moving into governing AI agents, with blocking. A cross-domain Hyleon would meet them (A).

---

## C. Leadership and strategy tooling

### Strategy and OKR platforms

| Product | What it does with AI | Boundaries? | Price | Dated facts |
|---|---|---|---|---|
| **WorkBoard** (now including Quantive) | AI agents including a "Chief of Staff" and a "Leadership Coach" (2025); 2026 adds Strategic Portfolio Management and "Portfolio Analyst" agents (U) [40]. An MCP server lets Claude, ChatGPT, Copilot and Cursor read and write WorkBoard data: create KPIs, update key results and due dates, add action items, build business reviews [40]. | The MCP page describes no role scoping, approval for writes or audit log. The tool count is inconsistent: "nearly 60" and "29" on the same page (fetched 2026-10-31) [40]. | Not public. MCP needs a subscription and a CSM to enable it [40]. | Acquired Quantive (formerly Gtmhub) 2025-05; Quantive's product is no longer sold separately [40]. |
| **Cascade** | "Tapestry AI" drafts check-ins, flags risks, writes board summaries, scores OKR alignment\* [41]. Remote MCP server (U) [41]. | None found. | About $10 per user per month (U); G2 lists a free entry plan [41]. | No dated 2026 release found. |
| **Lattice** (goals inside a people platform) | Lattice AI Agent, grounded in reviews, feedback, goals, engagement and 1:1s\*. Goals data was to reach the agent in "early 2026" [42]. | None found. | Not checked. | Bought Mandala's AI coaching technology 2026-03-31 (U); winding down HRIS and payroll (U) [42]. |
| **Microsoft Viva Goals** | — | — | — | Retired 2025-12-31; Microsoft named no replacement and pointed customers to third-party OKR tools [43]. |

### Decision rights and governance

- **Board tools: Diligent.**
  - Announced an "AI Board Member" on 2026-04-23, with general availability expected in the fall [44].
  - Its GovernAI runs inside Diligent's permissioned environment, gives source citations and doesn't train on customer data\* [44].
  - Diligent's survey of 104 US public-company directors (2026-06): 82% used generative AI for board work in the past six months, up from 66% in 2025-09. About 30% used it to summarize board books [44]. Diligent says directors often do this "without guidelines, policies or oversight"\* [44].
  - Its answer is a policy template, not checks [44].
- **Decision intelligence: Cloverpop.** Sells a "decision intelligence layer that connects AI to business outcomes"\*. It captures decision interactions as structured data and names a "Human ^ AI framework", with no specific guardrails described (fetched 2026-10-31). Pricing not public [45].
- **Decision-rights frameworks** (RACI, DACI, Bain's RAPID) live in consulting methods and templates, not software that checks them. A Harvard Business Review piece (2026-07/08, Greer, Jordan and Sytch, via Workpath) argues most organizations set them up wrong (U) [48]. No software found that encodes decision rights as checkable rules for AI-assisted work.
- **AI governance platforms** govern AI systems, not leadership decisions:
  - **Credo AI** has "pre-built policy packs for the EU AI Act, NIST AI RMF, ISO 42001, and 20+ standards"\* that produce a risk register with control gaps per AI use case [46]. Who maintains the packs isn't stated. No MCP found.
  - **ServiceNow AI Control Tower** expanded at Knowledge 2026 (May) to discovery, ownership, access, observability, value and remediation across any AI; "500+ customers live"\* [47]. SiliconANGLE (2026-10-17) says the claim of a complete enforcement plane "has not been broadly validated" [47].
- **Regulation as a boundary source.**
  - The EU AI Act treats AI used in employment and worker management as high-risk (Annex III).
  - A provisional "Digital Omnibus" deal on 2026-05-07 moves Annex III obligations to 2027-12-02, and Annex I (products) to 2028-08-02 [49].
  - Final adoption in the Official Journal was not confirmed by 2026-10-31 (U).

### Does any tool put researched boundaries around AI-assisted leadership work?

**Not found.**
- Strategy platforms give agents data and write access. They don't check the agent's proposals against sourced limits.
- AI governance platforms (Credo AI, ServiceNow, Drata's agent governance) check AI systems against regulations and policies. They don't check the content of a strategy, a plan or a decision.
- The closest pieces:
  - Diligent's cited, permissioned AI for boards: provenance, but no boundaries.
  - Credo AI's policy packs: sourced boundaries, but on AI systems.
  - Cloverpop's structured decision records: a model of decisions, but no researched limits.

**Why it may be hard (A).** In physical design, limits are numbers with clauses: a rating, a clearance. In leadership work most "limits" are judgments: risk appetite, priorities, decision rights. Some can be made checkable, such as "who may approve spend over X", "a hiring decision touched by AI is high-risk under EU law", or "every key result needs an owner and a metric". Most are organizational choices, not researched facts. A pack would hold more questions and fewer checks. That fits Hyleon's "ask before designing", but the value per check is lower and harder to prove.

---

## Is there a market for a cross-domain "researched boundaries pack" format?

### Evidence for

1. **Packs exist in every domain.** Buyers already pay for maintained boundary libraries: Vanta, Drata, Credo AI, Cortex Cloud. They also use free ones: AWS samples, Kyverno sets, CodeGuard, SCF.
2. **Agents now consume rules at scale.** 60k+ repos use AGENTS.md\*; SKILL.md is read by dozens of tools (U).
3. **Everyone is looking for trust in AI output.** Drata, Vanta, AWS, ServiceNow, Kyverno and Diligent all launched AI-agent governance or AI-with-citations features in 2026.
4. **The research gap is documented.** Unresearched rule files barely help (ETH, 2026-02). Marketplaces carry malware (ToxicSkills). Directors use AI without policies (Diligent, 2026-06).

### Evidence against

1. **No shared format crosses domains today.** Each pack is bound to an engine (Rego, Cedar, CEL, Python) or to a closed platform.
2. **The formats that won are minimal and free.** AGENTS.md and SKILL.md deliberately leave out sources, versions and tests. Their backers are OpenAI, Anthropic and others through the Linux Foundation. A small company is unlikely to set a competing cross-domain standard (A).
3. **The value is the content and its upkeep, not the format.** Vanta's moat is its test library, integrations and auditor network, not a file format (A).
4. **The trend in software and AI governance is enforcement.** Hyleon's "never block" is a deliberate difference that some buyers will see as a gap (A).
5. **Each domain's "model" is owned by someone else.** Cloud accounts (AWS, Azure), catalogs (Port, Backstage), strategy data (WorkBoard), compliance data (Vanta, Drata). Hyleon owns the model only in physical design (A).

### Who comes closest to an open, readable, sourced, agent-consumable pack

| Candidate | Open | Readable | Each rule names its source | Executable checks | Questions first | Versioned and tested | Agents consume it | Domains |
|---|---|---|---|---|---|---|---|---|
| Hyleon pack (planned) | yes | yes (TOML) | yes, required | yes (CEL) | yes | yes (semver, test projects, lockfile) | yes (MCP) | physical design |
| Project CodeGuard [26] | yes (CoSAI) | yes (Markdown) | not confirmed | validators | no | not confirmed | yes (translators) | software security |
| OpenSSF guide [27] | yes | yes (prose) | mostly | no | no | no | by copy | software security |
| Agent Skills [19] | yes | yes | no | optional scripts | no | no | yes | any |
| AGENTS.md / Cursor rules [18][20] | yes / vendor | yes | no | no | no | git only | yes | software |
| Tessl registry [24] | listing open | yes | no | evals and scores | no | yes | yes | software |
| Kyverno policies [4][5] | yes | medium (YAML, CEL) | not required | yes | no | yes (CLI tests) | via MCP | Kubernetes |
| OPA / Conftest [1] | yes | low (Rego) | no | yes | no | yes (`opa test`) | indirect | any config |
| AgentCore Policy / Cedar [6][7] | language open; service AWS | medium | no | yes, enforced | no | formally analyzable | governs agent calls | authorization |
| Checkov / Trivy [8][9] | yes | low to medium | partial guideline links | yes | no | releases | via CI | cloud, IaC |
| AWS conformance packs / Azure initiatives [11][12] | vendor | medium | maps to framework controls | yes | no | vendor versions | indirect | one cloud |
| Semgrep registry [30] | mostly | medium | CWE required for security rules | yes | no | rule tests | indirect | code |
| SCF [36] | free | yes (spreadsheets) | yes (STRM to 200+ laws) | no | no | releases | no | cyber, privacy |
| OSCAL [37] | yes (NIST) | low (XML, JSON) | yes | assessment results only | no | yes | no | security |
| Vanta / Drata libraries [31][32] | no | UI | mapped to controls | yes, on live data | no | vendor-managed | yes (MCP) | security, privacy, AI |
| Credo AI policy packs [46] | no | UI | yes (regulations) | assessments | partly (U) | vendor-managed | not found | AI governance |

**Closest overall (A).**
- **CodeGuard** for "open, researched, agent-consumable rules" in software.
- **SCF** for "sourced mappings across many laws".
- **Vanta and Drata** for "boundaries checked against a live model, with evidence and a human decision".

None combines all three, and none asks questions first or tracks AI-guessed values until a person confirms them.

### Gaps nobody fills (as far as found)

1. **One readable file with questions, sourced limits, checks and its own tests.** Each of these exists somewhere; together, nowhere.
2. **Per-value provenance**, including "an AI guessed this" until a person confirms it. Compliance tools track evidence per control, not per value.
3. **Ask first.** Except Spec Kit's clarify step and Credo-style assessments, no tool makes missing information a question before work starts.
4. **Inform, with recorded acknowledgement, instead of block.** It is rare. Compliance platforms come closest, because the auditor, not the tool, decides.
5. **Leadership work** has no researched boundary packs at all.
6. **Curated, reviewed distribution.** Open skill marketplaces lack review. Reviewed ones (Cursor, Tessl, Claude's official marketplace) don't require sources.

### Decisions only Dan can make

These need his answer; I have not assumed any of them.

1. **Scope.**
   - (a) Stay physical design, and use this research only to sharpen packs.
   - (b) Make the pack format domain-neutral, but ship only physical design.
   - (c) Ship packs for other domains (which first: software, compliance, leadership?).
2. **Open standard or product format.**
   - (a) Keep the format inside Hyleon.
   - (b) Publish it as an open spec in Hyleon's repo.
   - (c) Take it to a neutral body (AAIF, CoSAI/OASIS) later.
3. **Relationship to Agent Skills and AGENTS.md.**
   - (a) Ignore them.
   - (b) Export packs as skills for reach, keeping sources and checks in the pack.
   - (c) Extend SKILL.md through its `metadata` field.
4. **Blocking.** Truth 6 and "not a gatekeeper" say never block. Software and agent-governance buyers often want enforcement.
   - (a) Keep "inform only" everywhere.
   - (b) Let a customer's own CI or gateway block on Hyleon findings, outside Hyleon.
   - Changing the truth itself would make it a different product, by TRUTHS.md's own test.
5. **Who researches and reviews packs outside physical design.** Physical packs need a licensed engineer to release. Who plays that role for software, compliance or leadership packs, and who pays for upkeep?
6. **Business model for content.**
   - (a) Open packs plus paid tooling.
   - (b) Paid maintained packs, as in Vanta's model.
   - (c) A community registry with review.
7. **Buyer.** Practitioners (today's authors) or enterprise leadership (Dan's new phrase). They buy differently: per seat versus enterprise contracts.

---

## The connecting layer (Dan's addendum)

Dan's framing: Hyleon connects things after research is done. It joins researched knowledge, the user's model and what an AI produces, and shows the consequences and gaps. Which products in these categories position themselves that way, rather than as authoring tools?

| Product | Researched knowledge | User's model | What the AI produces | Consequences and gaps shown | How close (A) |
|---|---|---|---|---|---|
| **Vanta** | Vendor-maintained framework, control and test library | Live data from 400+ integrations\* | Agent fixes and pull requests via the MCP server and Claude plugin (2026-04) | Failing test, then failing control, then framework readiness | **Closest in shape.** Closed content; security and privacy only; consequences stop at "readiness". |
| **Drata** (incl. AI Agent Governance) | Library plus AIUC-1, ISO 42001, EU AI Act mappings | Live systems plus every agent tool call | Agent actions, checked before they run | Trust scores, drift, blocked calls, evidence log | Very close, but it **enforces**, and the content is closed. |
| **Credo AI** | Policy packs from regulations | Registry of AI use cases | Model and system evaluations | Risk register with control gaps | Close for AI governance; doesn't touch work content. |
| **Port** ("context lake") | None researched; customer policies | Software catalog as a graph | Agents act through it | Scorecards and policy checks before actions\* | Connects, but doesn't research. |
| **Cortex, OpsLevel, Backstage** | Templates; customer rules | Catalog | Agents query via MCP | Scorecard levels | Same as Port, read-mostly. |
| **ServiceNow AI Control Tower** | Customer policies | CMDB and inventory of all AI | Any AI agent | Ownership, access, value, remediation\* | Broad connector; unvalidated; no research. |
| **AgentCore Policy, Kyverno** | Customer rules (plus a few standard sets) | Tool schemas, cluster state | Agent calls, manifests | Allow or deny | Connects at the point of action; enforcement only; no consequences or gaps. |
| **SCF** | Researched mappings across 200+ laws | None | None | Which controls satisfy which laws | Knowledge only; it's the "research done" half. |
| **WorkBoard** | None | Strategy: OKRs, KPIs, reviews | Agents read and write via MCP | Progress and risk on goals | Model plus AI, no researched limits. |
| **Diligent** | Board materials, cited | Board data | Summaries, recall | Citations only | Provenance only. |

### What it implies for Hyleon (A)

1. **The connecting layer is a proven, valuable position, but each winner owns one domain's model.** Vanta and Drata connect compliance research to the customer's live systems. Port connects agents to the software catalog. None connects across domains, because each needs deep integrations into a domain's real model.
2. **Hyleon already has this shape for physical design.** Packs are the research. The one model is the user's work. The MCP commands carry what the AI produces. Checks and provenance surface consequences and gaps. The fault-current chain is a true consequence chain (utility fault current to device ratings to labels). Nothing found in A, B or C traces consequences that far; they stop at pass/fail or readiness.
3. **Going cross-domain as a connector means connecting to models Hyleon doesn't own.** That means cloud accounts, catalogs, strategy tools and compliance platforms, mostly over MCP. That is integration work against strong incumbents, and a different product from "the model is the design" (truth 2).
4. **The unclaimed piece is the research-and-provenance layer itself.**
   - Sourced, versioned, tested packs that any connector or agent can load.
   - Values marked "AI-guessed" until a person confirms them.
   - That would complement Vanta, Port or WorkBoard rather than replace them.
   - Whether to pursue it, and in which domains, is decision 1 and 2 above.
5. **Inform-not-block is the clearest difference from every connector found.** It is a strength for professionals who answer for their work. In software and agent governance it is a gap. Dan should weigh which buyers this serves (decision 4).

---

## Sources

1. OPA releases (fetched 2026-10-31): https://github.com/open-policy-agent/opa/releases
2. Apple hires Styra's OPA team (Cloud Native Now): https://cloudnativenow.com/editorial-calendar/best-of-2025/apple-buys-styra-brains-opa-remains-open-2/
3. Harness, "Policy as Code in 2026" (2026-09-11): https://www.harness.io/blog/policy-as-code-in-2026-opa-kyverno-cedar-and-what-s-next
4. CNCF, Kyverno graduation (2026-03-24): https://www.cncf.io/announcements/2026/03/24/cloud-native-computing-foundation-announces-kyvernos-graduation/
5. Nirmata Kyverno MCP: https://nirmata.com/?p=12179 · https://help.nirmata.com/docs/n4k/kyverno-mcp
6. Cedar in CNCF: https://www.cncf.io/projects/cedar/ · https://infoq.com/news/2026/01/cedar-joins-cncf-sandbox/ · https://github.com/cedar-policy/cedar
7. AgentCore Policy: https://aws.amazon.com/about-aws/whats-new/2025/12/amazon-bedrock-agentcore-policy-evaluations-preview · https://aws.amazon.com/it/about-aws/whats-new/2026/03/policy-amazon-bedrock-agentcore-generally-available/ · https://docs.aws.amazon.com/bedrock-agentcore/latest/devguide/policy.html · https://dev.classmethod.jp/en/articles/20260617-amazon-bedrock-agentcore-policy-guardrails/ · https://hidekazu-konishi.com/entry/amazon_bedrock_agentcore_policy_implementation_guide.html
8. Checkov README (fetched 2026-10-31): https://github.com/bridgecrewio/checkov · Palo Alto open source: https://www.paloaltonetworks.com/cortex/cloud/open-source-projects
9. Trivy checks: https://github.com/aquasecurity/trivy-checks
10. Trivy compromise: https://wiz.io/blog/trivy-compromised-teampcp-supply-chain-attack · https://microsoft.com/en-us/security/blog/2026/03/24/detecting-investigating-defending-against-trivy-supply-chain-compromise · https://trust.barracuda.com/security/information/trivy-supply-chain-compromise
11. AWS Config: https://docs.aws.amazon.com/config/latest/developerguide/conformancepack-sample-templates.html · https://aws.amazon.com/config/pricing
12. Azure Policy regulatory compliance: https://learn.microsoft.com/azure/governance/policy/concepts/regulatory-compliance
13. Fitness functions and agents: https://developertoolkit.ai/en/workflows/test/fitness-functions/ · https://www.qodo.ai/?p=22542 · https://pkg.go.dev/github.com/alexei-led/archfit@v1.1.1
14. Backstage and Spotify: https://backstage.io/docs/next/ai/generated-index · https://backstage.spotify.com/docs/plugins/soundcheck/mcp-actions.md · https://backstage.spotify.com/
15. Port: https://techcrunch.com/2025/12/11/port-raises-100m-at-800m-valuation-to-take-on-spotifys-backstage · https://www.port.io/pricing · https://port.io/glossary/context-lake
16. Cortex: https://docs.cortex.io/solutions/ai-readiness · https://cortex.io/products/cortex-mcp · https://bnnbloomberg.ca/business/company-news/2024/09/04/sequoia-backs-developer-startup-cortex-at-470-million-valuation · https://techstackipo.com/stack/cortex-io
17. OpsLevel: https://docs.opslevel.com/docs/mcp · https://stackpick.net/pricing/opslevel/
18. AGENTS.md (fetched 2026-10-31): https://agents.md/ · Agentic AI Foundation: https://openai.com/blog/agentic-ai-foundation
19. Agent Skills spec (fetched 2026-10-31): https://agentskills.io/specification · adoption: https://www.noqta.tn/en/news/skill-md-open-standard-30-ai-coding-tools-adoption-2026
20. Cursor: https://cursor.com/docs/context/rules · https://cursor.com/docs/plugins · https://forum.cursor.com/t/cursor-2-6-team-marketplaces-for-plugins/153484/9 · https://forum.cursor.com/t/alwaysapply-silently-ignored-in-rules-from-team-plugins/157484
21. Claude Code plugin marketplaces (secondary): https://dev.to/akaranjkar08/claude-code-plugin-marketplace-2026-package-and-publish-50nd
22. skills.sh review: https://rywalker.com/research/skills-sh
23. ToxicSkills summaries: https://clawdocs.org/security/skill-verification · https://agentguard.gopluslabs.io/advisor/AGS-2026-0006
24. Tessl: https://tessl.io/registry/tessl-labs/spec-driven-development/skills · https://codemyspec.com/blog/tessl-review · https://fortune.com/2024/11/14/tessl-funding-ai-software-development-platform
25. PRPM: https://forum.cursor.com/t/prpm-a-package-manager-for-cursor-rules-1800-cursor-rules-installable-via-the-cli/139557
26. Project CodeGuard: https://blogs.cisco.com/ai/announcing-new-framework-securing-ai-generated-code · https://www.oasis-open.org/2026/02/09/cisco-donates-project-codeguard-to-coalition-for-secure-ai · https://github.com/project-codeguard/rules
27. OpenSSF guide (2025-08-01): https://best.openssf.org/Security-Focused-Guide-for-AI-Code-Assistant-Instructions
28. Spec Kit: https://www.thoughtworks.com/en-au/radar/languages-and-frameworks/github-spec-kit · https://xebia.com/blog/building-software-with-spec-kit/
29. ETH Zurich AGENTS.md study: https://arxiv.org/abs/2602.11988 · https://infoq.com/news/2026/03/agents-context-file-value-review/
30. Semgrep rule metadata (contributor docs, seen via a docs mirror): https://semgrep.dev/docs/contributing/contributing-to-semgrep-rules-repository
31. Vanta: https://www.bloomberg.com/news/articles/2025-07-23/vanta-notches-4-15-billion-valuation-with-new-funding-round · https://www.vanta.com/pricing · https://www.vanta.com/resources/new-in-vanta-april-2026 · https://developer.vanta.com/docs/vanta-mcp · https://vanta.com/resources/introducing-custom-tests · https://developer.vanta.com/docs/concepts/tests.md · https://www.vendr.com/marketplace/vanta
32. Drata: https://drata.com/blog/introducing-mcp-built-for-ai · https://help.drata.com/en/articles/13401776-create-and-publish-custom-tests · https://help.drata.com/en/articles/13394042-map-tests-to-controls · https://drata.com/about/news/drata-expands-trust-management-platform-to-support-governance-of-enterprise-ai-agents · https://drata.com/about/news/drata-extends-trust-management-platform-to-continuously-monitor-and-govern-ai-agents · https://drata.com/blog/cybersecurity-roles-among-most-in-demand · https://www.vendr.com/marketplace/Drata
33. Secureframe: https://secureframe.com/blog/leading-ai-innovation-in-compliance · https://secureframe.com/blog/series-b.md
34. Sprinto: https://www.securityweek.com/sprinto-raises-20-million-for-automated-risk-and-compliance-platform/ · https://underdefense.com/blog/sprinto-pricing
35. Delve: https://www.iansresearch.com/resources/all-blogs/post/security-blog/2026/03/30/delve-allegations-expose-weak-points-in-modern-compliance · https://www.corporatecomplianceinsights.com/soc-2-broken-delve-scandal-shows/
36. Secure Controls Framework: https://securecontrolsframework.com/ · https://securecontrolsframework.com/blog/scf-release-20261 · https://securecontrolsframework.com/blog-posts/scf-2026-2-newsletter
37. FedRAMP 20x: https://fedramp.gov/2026/providers/20x/rules/security-decision-record/ · https://fedramp.gov/2026/reference/20x/a/key-security-indicators/ · https://secureframe.com/blog/fedramp-20x.md · https://diogenesclub.mycroft.io/blog/fedramp-20x-platform-readiness-oscal-export-guide
38. Open-source GRC: https://openalternative.co/alternatives/comp · https://openalternative.co/alternatives/probo · https://openalternative.co/alternatives/openlane
39. AIUC-1: https://www.compliancepoint.com/regulations/aiuc-1/ · https://www.unite.ai/sierra-earns-aiuc-1-certification-after-independent-audit/
40. WorkBoard: https://www.fenwick.com/insights/experience/fenwick-represents-workboard-in-acquisition-of-quantive · https://prod.quantive.com/resources/blog/workboard-acquires-quantive · https://www.workboard.com/resources/product-updates/mcp-server · https://workboard.com/resources/blog/2025-a-year-in-review
41. Cascade: https://cascade.app/solutions/ai-strategy-insights · https://www.tability.io/compare/platform/cascade
42. Lattice: https://lattice.com/blog/lattice-fall-winter-2025-product-release · https://techedgeai.com/lattice-bolsters-ai-driven-people-platform-with-mandala-acquisition-and-new-executive-hire/
43. Viva Goals retirement: https://learn.microsoft.com/en-us/viva/goals/goals-retirement-faq
44. Diligent: https://www.diligent.com/company/newsroom/diligent-unveils-ai-board-member-and-agentic-grc-workforce · https://www.cpapracticeadvisor.com/2026/04/23/diligent-debuts-ai-board-member-agentic-grc-workforce-at-elevate-2026/ · https://www.diligent.com/resources/blog/dci-board-ai-use-2026 · https://www.diligent.com/resources/blog/governai-secure-ai-for-boards · https://www.diligent.com/resources/guides/guide-ai-in-boardroom-policy
45. Cloverpop (fetched 2026-10-31): https://www.cloverpop.com/
46. Credo AI (fetched 2026-10-31): https://www.credo.ai/pl/ai-governance-platform-and-services · EU AI Act pack (2023): https://businesswire.com/news/home/20231010666022/en/Credo-AI-Deploys-EU-AI-Act-Readiness-offering-to-implement-AI-safety-and-trust-through-alignment-to-emerging-global-standards
47. ServiceNow AI Control Tower: https://newsroom.servicenow.com/press-releases/details/2026/ServiceNow-expands-AI-Control-Tower-to-discover-observe-govern-secure-and-measure-AI-deployed-across-any-system-in-the-enterprise/default.aspx · https://siliconangle.com/2026/10/17/servicenow-ai-control-tower-selling-a-penthouse-while-building-the-frame/
48. Decision rights: https://www.workpath.com/en/magazine/decision-rights-framework
49. EU AI Act Digital Omnibus: https://datamatters.sidley.com/2026/06/22/eu-lawmakers-reach-provisional-agreement-to-delay-key-eu-ai-act-obligations/ · https://www.gibsondunn.com/wp-content/uploads/2026/05/eu-ai-act-omnibus-agreement-postponed-high-risk-deadlines-and-other-key-changes.pdf
50. Hyleon stack research (2026-10-28): [../2026-10-28/5-stack.md](../2026-10-28/5-stack.md)
