# Appendix 2: Network design and verification (agent report, 2026-10-31)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-31-scope.md](../2026-10-31-scope.md).

**The brief:** Map the network design and configuration verification tools (pre-deployment checking, network digital twins, intent verification, OT network design and inventory): what each checks, where its rules come from, its AI and MCP support in 2026, prices, buyers and recent news; compare them with Hyleon's networking scope (building, plant and site networks, physical layer, OT buses); list the gaps and the decisions only Dan can make; and judge which of them act as a connecting layer rather than an authoring tool.

---

## Bottom line

- **Networking already has a "verifier for AI" market, and it moved fast in 2026.** Forward Predict (GA 2026-09-29) tests a change, including one an AI agent proposed, against a model of the live network and returns evidence so the agent can revise it [9,10]. NetBrain, IP Fabric, Batfish, Cisco Modeling Labs, Itential and NetBox Labs all shipped MCP servers between 2025-05 and 2026-10 [2,13,17,21,34,41].
- **Dan's networking analogy is real, but it lives in operations, not design.** Almost every product here models a network that already exists, from its live devices or configs. They sell to enterprise network-operations teams, quote-only, priced per device. Only Batfish (configs alone), the emulators (Cisco Modeling Labs, Packet Tracer), Apstra, NetBox Designs (preview) and Siemens SINETPLAN work before the network is built.
- **The logical layer is covered; the physical layer of site and plant networks is not.** The verifiers check routing, ACLs, firewalls, NAT and segmentation. None that I found checks cable channel length, PoE budgets, cable category against PoE type, the 24 V supply and heat of network gear inside a panel, or OT bus rules (Modbus RTU segments, PROFINET line depth). NetBox *records* PoE mode, power draw and cable length, and its docs describe no check against limits [44–46]. SINETPLAN checks PROFINET load only, for Siemens networks [49].
- **Cisco already took Packet Tracer into OT.** Packet Tracer 9.0 adds industrial switches, a PLC, an HMI, a control panel, and Modbus, PROFINET, EtherNet/IP, OPC UA and IEC 61850 [25]. Hobbyists have built MCP servers that let an agent plan, validate and deploy a Packet Tracer topology [27,28]. It is a training simulator: it produces no deliverables and cites no sources.
- **No product here gives each check or value a cited, external source.** Rules come from the vendor's own libraries (IP Fabric's 160+ intent checks, NetBrain's versioned Golden Assessment Library), from checks the user writes (Forward NQE, Batfish questions, Arista ANTA catalogs), or from compliance frameworks mapped onto checks (CIS, NIST, PCI-DSS, NIS2, IEC 62443) [13,14,17,59].

Notes:
- Dates are as published; "today" is 2026-10-31. Prices were read on 2026-10-31 unless dated.
- \* = vendor claim. (u) = unverified: from a search summary or a secondary source I could not open. (k) = from my prior knowledge, not re-checked in this session.
- The session's shared web-search budget ran out partway through. Several vendor sites (HPE, Belden, GeekWire, Cisco Community, IP Fabric docs) refused or timed out on direct fetches. Gaps are marked, not guessed.

## 1. Product map

| Product | What it checks | Where its rules come from | AI and MCP (2026) | Price | Buyers |
|---|---|---|---|---|---|
| **Batfish** (open source) [1–4] | Builds a model from device configs, without device access. Checks reachability, ACL and firewall behaviour, routing and BGP, undefined or unused references, failure scenarios, and the difference between current and planned configs | Questions the user writes; the network's own computed behaviour | Official MCP server (beta), merged 2026-03-16, 23 tools; shipped as the `mcp` extra in pybatfish 2026.9.17 (2026-09-18) | Free (Apache-2.0) | Network engineers; vendors build on it |
| **Forward Enterprise / Forward Predict** [7–12] | Mathematical model of every packet path across vendors and clouds. Predict tests proposed routing, ACL, NAT, firewall and segmentation changes against the model before they are applied | User-written intent checks and NQE queries; built-in checks | Forward AI (announced 2026-01-29, in the platform from 2026-04), with MCP for third parties [7]. Predict (beta 2026-05-20, GA 2026-09-29) verifies AI-proposed changes and returns evidence | Quote only; free 30-day trial from 2026-08-31 [8] | Large enterprises: Goldman Sachs, PayPal, S&P Global, IBM, Dell* [9] |
| **IP Fabric** [13–16] | Discovers the live network, builds a snapshot model, and verifies it against intent; path lookups; drift; compliance | 160+ built-in intent checks (v7.0, 2025-02) (u) [14]; mapped to CIS, NIST, ISO 27001, PCI-DSS, HIPAA, NIS2, DORA [13] | MCP server from the appliance, opt-in, with a prompt library (2026-04-23) [13] | Quote only; per active device and cloud object, yearly [15] | Enterprises: Red Hat, Major League Baseball, Air France* [13] |
| **NetBrain** [17–20] | Live topology and paths; diagnosis; change and assessment agents; "intent-based automations" | Golden Assessment Library (versioned 26.06) plus customer automations [17] | MCP server and wider LLM support (2026-05-29, available 2026-06-01) [17]. Change and Assessment agents, Agent Studio, and a "NetOps Harness" that governs outside agents (Claude, Codex, ServiceNow) over MCP (2026-09-22/23) [18] | Quote only; per node (an IP address) [19] | About a third of the Fortune 500* [17] |
| **Cisco Modeling Labs (CML)** [21–24] | Emulates real Cisco software images so a design can be run and tested before deployment | The real device software | CML 2.10 includes an MCP server; agents create labs, connect links, push configs and run commands [21,22] | Free tier (5 nodes, u) [23]; Personal $199 (20 nodes); Personal Plus $349 (40 nodes) [21]; Enterprise quote only (~$13,000+/yr, u) [23] | Engineers, students, enterprise labs |
| **Cisco Packet Tracer 9.0** [25–28] | Simulates Cisco devices, cabling and protocols for teaching | Cisco's simulation of its own devices | No official AI or MCP. Community MCP servers exist (one has 327 stars, MIT, tested on 9.0) [27,28] | Free with a Networking Academy account (k) | Students, teachers |
| **HPE Juniper Apstra** (now "Apstra Data Center Director") [29–33] | Designs data-centre fabrics from reference designs, validates the build before commit, then checks the running fabric against the design, including the cabling map against LLDP (k) | Vendor reference designs (k) | No official Apstra MCP found. HPE added MCP to Juniper Routing Director (WAN), not Apstra (u) [32]. Community HPE MCP toolkit covers Apstra [33] | Quote only | Data-centre and AI-cluster operators |
| **Itential** [34–38] | Orchestrates changes; pre- and post-checks, blast-radius limits, rollback, approval gates in workflows [37] | The customer's workflows and policies | Open-source MCP server (2025-05-28) [34]; FlowAI with FlowAgents and FlowMCP (private preview, 2025-11-19) [35] | Published tiers, prices on request; priced by agent runs (5,000 or 10,000 a month) and workflow throughput, not by device [36] | Carriers, finance, US federal, utilities* [34] |
| **NetBox Labs** [39–46] | NetBox is the record of intended state: sites, racks, devices, cables, power, IPs. Assurance finds drift from it. Validation (announced 2026-06-11) does "intent validation, configuration analysis and resilience analysis" and pre-change checks [40]. Designs (preview, 2026-10-13) builds sites from reviewed designs and checks drift [41] | Not stated for Validation [40] | Copilot GA 2026-02-10 [39]. Platform MCP server GA on SaaS 2026-10-13, read and optionally write; agents with "always allow / needs approval / blocked" per action [41] | Open source free; Cloud and Enterprise quote only; Assurance is an add-on [42] | Enterprises, data centres, service providers |
| **Arista AVD and ANTA** (extra) [47,48] | AVD generates configs from YAML design data; ANTA runs test catalogs before and after go-live ("network ready for use") | Test catalogs the user writes | None found | Free (Apache-2.0) | Arista customers |

Recent releases and funding:

| Product | Date | Event |
|---|---|---|
| Batfish | 2022-09 (u) | Amazon licensed Batfish from Intentionet [5] |
| Batfish | 2026-03-16 | Official MCP server (beta) merged [2] |
| Forward | 2023-01-24 | $50M Series D, the latest round I found [12] |
| Forward | 2026-05-20 | Renamed "Forward" from Forward Networks; Forward Predict launched in beta [9] |
| Forward | 2026-09-29 | Forward Predict GA [8,10] |
| IP Fabric | 2023-06 (u) | $25M Series B led by One Peak [16] |
| IP Fabric | 2026-04-23 | MCP server [13] |
| NetBrain | 2026-02-10 | Agentic NetOps, Deep Diagnosis agent [20] |
| NetBrain | 2026-09-22 | Change and Assessment agents, NetOps Harness, Agent Studio; available early October 2026 (rollout not confirmed) [18] |
| Cisco CML | 2025-11-17 | MCP server announced on Cisco's blog [22] |
| Cisco CML | 2025-12-02 | Personal editions sold only through Cisco U [24] |
| Packet Tracer | 2025 (u) | 9.0 with industrial devices; sources disagree between 2025-07 and a 2025-10-27 build [26] |
| Apstra | 2025-07-02 | HPE closed its ~$14B purchase of Juniper [29] |
| Apstra | 2026-06 | HPE Discover: QFX switches for AI clusters managed through Data Center Director; no MCP named [30] |
| Itential | 2025-05-28 | MCP server [34] |
| Itential | 2025-11-19 | FlowAI private preview [35] |
| Itential | unknown | $25.5M across two rounds (u) [38] |
| NetBox Labs | 2025-07 | $35M Series B led by NGP Capital [43] |
| NetBox Labs | 2026-06-11 | Infrastructure Intelligence Platform: Validation, Assurance, Asset Lifecycle, Platform MCP [40] |
| NetBox Labs | 2026-10-13 | Designs (preview), Platform MCP GA, three new agents [41] |

## 2. Each product in a few lines

**Batfish.** It is the open engine of this category. It needs only configs, so it can check a design before any device exists. It supports Cisco, Juniper, Arista, Palo Alto, Fortinet, Check Point, F5, SONiC, FRR, AWS and more [4]. It names no industrial switch makers (Siemens, Hirschmann, Moxa) [4]. Rockwell's Stratix switches run Cisco IOS, so Batfish may parse them (k, untested). It models no physical layer. Its MCP server lets any agent ask "is this reachable?" or "what does this ACL allow?". It is the strongest candidate for Hyleon to call rather than rebuild (see section 6).

**Forward and Forward Predict.** This is the closest product to Hyleon's "checks every change" for AI. Forward says Predict "sits between AI-generated intent and production execution." When a change fails, it returns "evidence showing why a change should not be applied," so the agent can try again [10]. The articles describe it as informing, not enforcing [10]. Scope is routing, firewall, policy, ACL and NAT, across Cisco, Check Point, F5, Zscaler and IBM Cloud [10]. Nothing about the physical layer. Customers are large enterprises; IDC puts average benefits at $14.2M a year* [9].

**IP Fabric.** It discovers the live network and checks it against built-in intent rules and compliance frameworks [13,14]. Its MCP server is opt-in and runs on the customer's appliance, which matters to regulated buyers [13]. It is priced per active device, quote only [15].

**NetBrain.** It now sells itself as the "harness" that governs any agent acting on the network. It is both an MCP server (giving agents topology, state and paths) and, by its account, a host for outside agents [18]. Its Golden Assessment Library is a versioned, vendor-curated set of checks (26.06) [17]; it is the nearest thing in networking to a Hyleon pack, though I found no per-check source citations. Customers report per-node pricing as expensive at scale (u) [19].

**Cisco Modeling Labs and Packet Tracer.** CML runs real Cisco images, so a design behaves as it will in production. Its MCP server lets an agent build and test a lab [21,22]. Packet Tracer simulates rather than emulates, and is for teaching. Version 9.0 adds IE3400 and IE9300 industrial switches, IR1101 and IR8340 routers, an ISA3000, a PLC, a data historian, an HMI and an "Industrial Control Panel" workspace, plus Modbus, PROFINET, CIP, OPC UA, IEC 61850, PTP and PRP, and ladder logic [25]. The help page lists no PoE, no cabling beyond fibre, and no AI [25]. One community MCP server runs "plan, validate, generate, deploy" from one prompt; what its validation checks is not documented [27].

**Apstra (HPE Juniper Data Center Director).** It designs a data-centre fabric from a reference design, checks the build for errors before commit, and then checks the running fabric against that design, including expected cabling against LLDP (k). This is "intent-based networking" in its original sense. It covers data-centre fabrics only. I found no official MCP server for it [30,32,33]. HPE's product pages timed out on fetch [31].

**Itential.** It is an orchestration layer, not a verifier. Its MCP server separates "AI-generated intent" from execution: every agent request passes through workflows with pre- and post-checks, blast-radius limits, rollback and approval gates [37]. It publishes tiers priced by agent runs, not devices [36]. That model, and its claim that FlowAI is "the core product, not a paid add-on," may be useful for Hyleon's own pricing [36].

**NetBox Labs.** NetBox is the most widely used open record of networks and data centres. Its model already covers what Hyleon cares about at the physical layer:
- interfaces with PoE mode (supplying or powered) and PoE type [44]
- power ports with maximum and allocated draw, summed downstream [45]
- cables with type, colour, length and termination profile [46]

Its docs describe no check of PoE budgets, cable length or power capacity against a limit [44–46]. The company is moving from record to verification: Validation (2026-06-11) and Designs with drift checks (2026-10-13) [40,41]. Its agents use per-action "always allow / needs approval / blocked" settings [41]. Hyleon's backlog already plans a NetBox import [docs/BACKLOG.md].

## 3. OT and industrial network tools

| Product | What it does | Rules from | AI / MCP | Price | Recent |
|---|---|---|---|---|---|
| **Siemens SINETPLAN** [49,50] | Plans PROFINET networks before commissioning. Simulates load from the topology and nodes, cyclic PROFINET IO plus TCP/IP traffic; flags overloaded ports, burst limits and reserve; reports for plant acceptance | Siemens' own model of PROFINET traffic | None found | Free trial; paid version (price not found) | V2.0 on 2019-04-01 added AutomationML import from ECAD and TIA Portal V15.1 import [49]. Current version not found (u) |
| **Siemens PRONETA** [51] | At commissioning: scans the PROFINET network, compares the real installation with a reference, tests I/O wiring without a PLC | Siemens device data | None found | Basic free; Professional licensed | Professional V2.0 adds an SBOM configurator (date not stated) [51] |
| **PI design guidelines** [52] | Not software. The PROFINET Design Guideline sets topology, line depth and cyclic load limits | PROFIBUS & PROFINET International | n/a | Free download | Design Guideline v1.59, 2025-01 (u) [52] |
| **Belden / Hirschmann Industrial HiVision** [53–55] | Monitors and configures industrial networks: discovery, topology, multi-device config; v8.6 added TSN configuration (u) | Device data | None found | Licensed by node count (64 to 8,192 nodes, reseller listings) (u) | 8.6.00 in 2025-05; 8.7 in 2026 per one secondary source (u) [54]. Belden Horizon is its cloud OT remote-access platform (u) [55] |
| **Nozomi Networks** [56–59] | Passive OT and IoT asset inventory, traffic and threat detection. Compass (2026-09-16) adds an OT system of record with planned, approved and documented changes, and compliance evidence for NERC CIP, IEC 62443, NIS2, TSA [59] | Its own detections and threat intelligence; compliance frameworks | Vantage IQ, a private AI assistant, 2026-01-15; advisory wording; no MCP [58] | Quote only | Mitsubishi Electric bought it: announced 2025-09-09, closed 2026-01-28; price not stated [57] |
| **Claroty** [60–63] | Passive CPS asset inventory and exposure management. With Frenos (2026-07-09), builds a digital twin from Claroty's inventory and has an AI adversary test which vulnerabilities could disrupt production [62] | Its own detections; threat intelligence | "Claire" AI agent, described as being built, human-in-the-loop; no MCP stated [61] | Quote only | Axios Pro (2026-05-07) reported Claroty is eyeing an IPO [63] |

What these mean for Hyleon:
- **SINETPLAN is the only design-time OT network checker I found.** It covers one protocol, one vendor, and network load only. It does not check cabling, power or other buses.
- **PRONETA is the commissioning half.** It compares what was built with what was planned, which is Hyleon's "Record" step for PROFINET.
- **Claroty and Nozomi know what is on a plant network today.** They are security tools. Their inventory could seed an existing-site model, the way the backlog plans for NetBox.

## 4. Physical-layer and low-voltage tools (adjacent)

These are not verifiers, but they serve the people who design building and site networks.

- **System Surveyor** [64]. Site surveys and low-voltage design on floor plans, with device catalogs and a bill of materials. AI note-taker and a camera advisor (DORI). Its page names no cabling or PoE checks. Buyers: security and low-voltage integrators.
- **D-Tools** [65]. Proposals, design and project management for AV, security and low-voltage integrators. Cloud from $99 a month; System Integrator from $150 per user a month. No design checks or AI described. D-Tools merged with Portal.io on 2026-08-25.

Neither checks a PoE budget, a cable length or a panel's power. That matches section 1: no one checks the physical layer of small networks against cited limits.

## 5. What the research says about AI plus a network verifier

- GPT-4 alone made "egregious errors in topology, syntax and semantics" in router configs. With a verifier giving **localized** feedback, automated prompts outnumbered human prompts 10 to 1 for one task and 6 to 1 for another. Humans were still needed [6].
- The same group found that whole-network counterexamples confused the model and made it oscillate. Feedback about one router or one policy worked better (u, from a search summary of [67]).
- Later work uses the same loop: an agent proposes, a containerized verifier checks, a machine-readable report goes back, and the loop stops when the candidate passes (u) [66].

For Hyleon, this supports findings that name one element, one rule and one fix, rather than a whole-design verdict.

## 6. Overlap with Hyleon

| Hyleon item | Who already does it | Overlap |
|---|---|---|
| Phase 6 S1: "network reachability (VLANs, routes, firewall rules)" [docs/ROADMAP.md] | Batfish (free), Forward, IP Fabric, NetBrain, Apstra | **Full.** This is the core of this category |
| Check every change an agent proposes, return findings, don't block | Forward Predict, Batfish MCP, NetBrain Harness | **Strong in shape**, for logical network changes only |
| MCP access for any agent | Batfish, CML, IP Fabric, NetBrain, Itential, NetBox | **Common** in networking, unlike in electrical or piping design |
| Device catalog with real ports and settings (the "Packet Tracer" idea) | Packet Tracer 9.0 (now with OT devices), CML; NetBox device types and NetBox Data Exchange [40] | **Partial**: training simulators and records; no deliverables |
| NetBox import [docs/BACKLOG.md] | NetBox's own model holds PoE, power draw and cables | **Complementary**: NetBox records them; Hyleon would check them |
| Phase 6 S1: Modbus poll timing, bus load | SINETPLAN for PROFINET only | **Small**: one vendor, one protocol |
| Physical layer: cabling, ports, PoE budgets, OT buses, network gear in panels | Nobody found | **None** |
| Questions before design | Nobody found; all start from configs or live devices | **None** |
| Every value and boundary with a cited source | Nobody found; rules are vendor libraries, user-written checks, or framework mappings | **None** |
| Inform, never block | Forward Predict informs [10]. Itential, NetBrain and NetBox agents offer approval gates and "blocked" actions [37,41] | **Mixed**: network buyers expect gates as an option |

## 7. Gaps

1. **Physical-layer checks for site and plant networks, before build.** Examples of what nobody found checks, against a cited source:
   - PoE: each port's powered-device class against the switch's total PoE budget, and against the cable category
   - copper channel length; fibre type against the optics
   - the 24 V supply and heat of DIN-rail switches inside a panel
   - Modbus RTU segments: termination, biasing, device count and length
   - PROFINET line depth and load, outside Siemens' tool
   - EtherNet/IP packet load against the switch

   I have not checked which clauses would source these (for example TIA-568, IEEE 802.3bt, the PI and ODVA guidelines, and the NEC's PoE ampacity table). Hyleon must cite clauses, not copy them (PRODUCT.md, Boundaries).
2. **Networks as part of a cross-discipline job.** A switch sits in a panel, draws power, makes heat and serves a pump drive's bus. No network tool sees the panel; no panel tool checks the network.
3. **Design before anything exists, for small teams.** The verifiers need live devices or configs and sell to enterprise operations teams, quote-only. The free options (Batfish, Packet Tracer, CML Free, NetBox open source) cover logic, teaching or records, not physical checks.
4. **Sources per boundary.** No product attaches a code, clause or catalog source to each limit it checks.
5. **Questions first.** No product asks what must be known before a network is designed.

## 8. Connecting layer: who already plays it, and how close they come

Dan's addendum: Hyleon "connects things after research has been done ... connect the dots on what AI does or gets." The question is which products in this category position themselves as a connecting layer (researched knowledge + the user's model + what an AI produces → consequences and gaps) rather than an authoring tool.

| Product | Knowledge it brings | The user's model | What AI produces | Consequences returned | How close |
|---|---|---|---|---|---|
| **Forward Predict** | Exact computed behaviour of every packet; user intent checks | A twin of the live network | A proposed change | Evidence of what breaks, so the agent can revise [10] | **Closest in shape.** But the knowledge is computed from the network itself, not researched external sources |
| **NetBrain NetOps Harness** | Golden Assessment Library (vendor-researched, versioned); customer runbooks ("Agent Skills") | Live topology, paths, state | Actions from its own or outside agents over MCP | Diagnosis, assessment; governed actions [17,18] | **Close.** It explicitly calls itself the layer that grounds and governs any agent. Its library is the nearest match to packs |
| **IP Fabric** | 160+ intent checks; compliance frameworks | Discovered snapshot | Questions or plans from agents over MCP | Gaps between intended and actual state [13] | **Medium.** It connects state to checks; AI is a client |
| **NetBox Labs** | Device-type catalog (NDX); Validation engines | The intended-state record and designs | Agent edits through branching and approvals | Drift, pre-change safety, single points of failure [40,41] | **Medium and rising.** It owns the record and is adding checks |
| **Itential** | None of its own; the customer's workflows | None; it calls other systems | Agent requests | Pass or fail of governed workflows [37] | **Connects actions, not knowledge** |
| **Claroty + Frenos** | Threat intelligence; exploitability | Inventory-built twin of the plant | An AI adversary's attack paths | Which flaws could stop production [62] | **Close, in security only** |
| **Batfish** | None; it is an engine | Configs | Answers to agents' questions | Reachability and policy results | **A component** that connecting layers use |

What this implies for Hyleon (for Dan to weigh, not decided):
- **The connecting-layer idea is proven in networking.** Enterprises pay for "check what the AI proposes against a model, and show the consequences." Forward and NetBrain both shipped it in 2026.
- **The winners hold a deep model of one domain.** Forward computes every packet; Apstra knows its fabrics. Their checks are strong because the model is exact. A general "any work" layer would face such a specialist in each domain.
- **What none of them does is attach researched, cited boundaries.** Their knowledge is computed behaviour, vendor libraries or user-written checks. Hyleon's packs (questions, sourced limits, checks) are the part nobody here has. That is a distinct asset in any domain, but on its own it would be thinner than a specialist's model.
- **A connecting layer can call specialists instead of competing.** Hyleon could run Batfish for reachability and record its result as a "calculated" value with its version and inputs. That fits PRODUCT.md's hand-off table and truth 8. NetBox could be the network record that Hyleon checks.
- **Physical-layer network design is open,** and it is where Hyleon's existing model (places, parts, ports, connections, panels) already reaches.

## 9. Decisions only Dan can make

1. **Logical network verification (VLANs, routes, firewall rules; ROADMAP Phase 6 S1).**
   - A. Leave it out. Point to Batfish, Forward and others.
   - B. Call Batfish from packs, and record its results as sourced, calculated values. (My recommendation: it reuses a free, maintained engine with an MCP server. Trade-off: a Java service to run, and Batfish names no industrial switch makers.)
   - C. Build Hyleon's own. Trade-off: years of work that Batfish already did.
2. **Is the physical layer of site and plant networks a first pack family?** Cabling, PoE, network gear in panels, OT buses.
   - A. Yes, within the existing order of content (the Modbus drive-bus pack is already in the first slice).
   - B. Later, after panels.
   - C. No.
3. **NetBox.**
   - A. Import only, as the backlog says.
   - B. Two-way: read NetBox, check it, write findings back.
   - C. Hyleon as a NetBox companion that checks the physical data NetBox records but doesn't validate. Trade-off: NetBox Labs is adding Validation and Designs and could add these checks itself.
4. **Buyer.** Stay with integrators, panel shops and site engineers, or also sell to enterprise network-operations teams? The money in this category is in enterprise operations. It is crowded and quote-only.
5. **What "Packet Tracer for physical systems" means now.** Packet Tracer 9.0 already simulates OT networks for teaching.
   - A. Hyleon checks against sources (today's design).
   - B. Hyleon also simulates behaviour, the way Packet Tracer and CML do (Phase 6 leans this way).
   - C. Both, with simulation handed to specialist tools.
6. **Gates.** Network buyers expect approval gates and "blocked" actions as an option (Itential, NetBrain, NetBox agents). Truth 6 and "not a gatekeeper" say Hyleon informs. Should an enterprise customer be able to turn on a gate for its own people, or never? Only Dan's wording can change TRUTHS.md.
7. **Scope.** In networking, a connecting layer wins by holding a deep, exact model of one domain. Should Hyleon stay a physical-design tool whose packs connect to specialists, or become a general connecting layer for "any kind of work that involves AI"? This report covers one category and cannot settle that alone.

## 10. Sources

1. https://github.com/batfish/batfish
2. https://github.com/batfish/pybatfish/pull/970
3. https://pypi.org/project/pybatfish/
4. https://batfish.readthedocs.io/en/latest/supported_devices.html
5. https://www.geekwire.com/tag/batfish/ (search summary only; the page refused fetch) (u)
6. https://arxiv.org/abs/2307.04945 (Mondal, Tang, Beckett, Millstein, Varghese; 2023-07-11)
7. https://www.sdxcentral.com/news/forward-networks-adds-agentic-ai-tool-to-augment-its-network-digital-twin-platform/
8. https://www.forwardnetworks.com/press-releases/
9. https://www.forwardnetworks.com/blog/press-release/forward-launches-forward-predict-to-unlock-autonomous-networking/
10. https://itbrief.co.uk/story/forward-launches-ai-tool-to-test-network-changes-first (2026-10-08)
11. https://www.forwardnetworks.com/blog/2019/11/19/forward-networks-unveils-new-in-app-network-query-engine-nqe-checks/ ; https://forwardnetworks.com/wp-content/uploads/2021/11/TwoPagerDatasheet-2021UpdateA-V03.pdf (search summary) (u)
12. https://techcrunch.com/2023/01/24/forward-networks-raises-50m-for-digital-twin-technology-to-help-model-manage-and-secure-complex-enterprise-networks
13. https://itbrief.co.uk/story/ip-fabric-launches-mcp-server-for-network-operations (2026-04-23)
14. https://siliconangle.com/2025/02/10/ip-fabric-releases-new-7-0-update-enhanced-cloud-security-insights/ (search summary) (u)
15. https://ipfabric.io/pricing/
16. https://sifted.eu/articles/czech-startup-ip-fabric-news (search summary) (u)
17. https://www.netbrain.com/news/from-network-automation-to-agentic-netops/ (2026-05-29)
18. https://vmblog.com/news/netbrain-launches-a-full-team-of-governed-agentic-netops-agents-that-safely-act-on-the-live-network/ (search summary, 2026-09-22) (u)
19. https://www.netbrain.com/docs/ie101/help/netbrain-license-overview.htm ; reviews on https://www.peerspot.com/products/netbrain/archived_reviews (search summary) (u)
20. https://www.businesswire.com/news/home/20260210263958/en/NetBrain-Delivers-New-AI-Capabilities-to-Accelerate-its-Vision-for-Agentic-NetOps
21. https://developer.cisco.com/modeling-labs/
22. https://blogs.cisco.com/learning/speak-your-lab-into-existence-with-ai-driven-cisco-modeling-labs-and-mcp (2025-11-17)
23. https://www.netpilot.io/blog/cisco-cml-cost (third party; updated 2026-10-30) (u)
24. https://learningnetworkstore.cisco.com/cisco-modeling-labs-personal
25. https://tutorials.ptnetacad.net/help/default/whatsNew.htm
26. https://community.cisco.com/t5/online-tools-and-resources/cisco-packet-tracer-9-0-new-features/td-p/5285503 (search summary; refused fetch) (u)
27. https://github.com/Mats2208/MCP-Packet-Tracer
28. https://github.com/caixax/packet-tracer-mcp (search summary) (u)
29. https://en.wikipedia.org/wiki/Juniper_Networks
30. https://nand-research.com/hpe-discover-2026-recap/ (2026-06-22)
31. https://www.hpe.com/us/en/products/networking/data-center-director.html (timed out on fetch)
32. https://nand-research.com/hpe-connectivity-and-compute-announcements-mwc-2026/ (search summary) (u)
33. https://glama.ai/mcp/servers/secure-ssid/hpe-networking-mcp (community; search summary) (u)
34. https://www.itential.com/news/itential-launches-mcp-server-to-orchestrate-ai-agents-with-guardrails-across-enterprise-infrastructure (2025-05-28)
35. https://www.itential.com/news/itential-unveils-flowai-delivering-agentic-orchestration-for-infrastructure-operations/ (2025-11-19)
36. https://www.itential.com/pricing/
37. https://www.itential.com/cloud-platform/itential-mcp-server
38. https://www.everydev.ai/developers/itential (search summary) (u)
39. https://netboxlabs.com/news/
40. https://netboxlabs.com/news/infrastructure-intelligence-platform/ (2026-06-11)
41. https://netboxlabs.com/blog/netbox-evolve-2026-platform-announcements-nvidia-grafana (2026-10-13)
42. https://netboxlabs.com/pricing/
43. https://netboxlabs.com/news/netbox-labs-raises-35m-series-b/
44. https://netboxlabs.com/docs/netbox/models/dcim/interface/
45. https://netboxlabs.com/docs/netbox/models/dcim/powerport/
46. https://netboxlabs.com/docs/netbox/models/dcim/cable/
47. https://anta.arista.com/stable/
48. https://avd.arista.com/stable/ (redirects to version 6.4)
49. https://press.siemens.com/global/en/pressrelease/software-update-planning-and-simulation-tool (2019-04-01)
50. https://www.siemens.com/en-us/products/profinet/sinetplan/ (search summary; fetch returned other content) (u)
51. https://www.siemens.com/global/en/products/automation/industrial-communication/profinet/proneta.html
52. https://se.profibus.com/fileadmin/media/downloadsection/installation_guide/PROFINET_Design_Guideline_8062_V159_Jan25.pdf (search summary; not opened) (u)
53. https://www.belden.com/products/industrial-networking-cybersecurity/software-solutions/network-management-software/industrial-hivision-network-management-software (search summary; site refused fetch) (u)
54. https://softprom.com/belden-it-ot-infrastructure-security-innovations (secondary) (u)
55. https://statementdog.com/analysis/BDC/earnings_calls/317346 (Belden Q2 2026 call transcript; secondary) (u)
56. https://www.nozominetworks.com/press-releases
57. https://www.nozominetworks.com/press-release/nozomi-networks-enters-next-phase-of-growth-as-mitsubishi-electric-completes-acquisition (2026-01-28)
58. https://www.nozominetworks.com/press-release/nozomi-networks-unleashes-vantage-iq-the-worlds-first-private-ot-iot-cybersecurity-assistant (2026-01-15)
59. https://www.nozominetworks.com/press-release/nozomi-networks-introduces-nozomi-compass-to-modernize-the-everyday-work-of-ot-asset-and-service-management (2026-09-16)
60. https://claroty.com/press-releases
61. https://claroty.com/claire
62. https://claroty.com/press-releases/claroty-and-frenos-partner-to-reduce-downtime-risk-for-industrial-environments (2026-07-09)
63. https://www.axios.com/pro/enterprise-software-deals/2026/05/07/claroty-ipo-ot-security-shakeout-enterprise (headline only; paywalled) (u)
64. https://www.systemsurveyor.com/
65. https://www.d-tools.com/
66. https://arxiv.org/pdf/2512.17371 (GraphCue; search summary) (u)
67. https://conferences.sigcomm.org/hotnets/2023/papers/hotnets23_mondal.pdf (CoSynth; search summary) (u)
