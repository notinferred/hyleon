# Appendix 6: Energy, operating cost, lifecycle cost and carbon (agent report, 2026-10-31)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-31-scope.md](../2026-10-31-scope.md).

**The brief:** Research how energy use, operating cost, lifecycle cost (TCO) and carbon are computed from a design model today (tools, data sources and their licences, accuracy, AI and MCP features, prices, buyers), then say what Hyleon could compute honestly from its own model, what it must ask the person, how estimates should be labelled, where region profiles fit, where the market has gaps, which products act as a connecting layer rather than an authoring tool (Dan's addendum), and which decisions only Dan can make.

---

## How to read this

- Every claim was read on 2026-10-31 unless another date is given.
- Marks:
  - **[V]** read on the official page or primary source this session
  - **[S]** from a secondary source (press, directory, reseller, vendor blog)
  - **[U]** unverified: from prior knowledge, or a search that found nothing. Not re-checked this session
- The session's shared web-search limit ran out part-way through. After that, only direct fetches of known official URLs were possible. Items not reached are marked [U] or "not researched". Nothing was guessed to fill a cell.
- Nothing was installed or run.
- Prices are list prices where published. "Quote" means no public price.

## Summary

1. **The arithmetic is easy; the inputs are the hard part.** Energy = input power × hours. Cost = energy × price. Carbon = energy × emission factor. Hyleon's catalog can hold the power and efficiency. It cannot know run hours, load profile, tariff or the study period. Those must be questions.
2. **No tool in this category computes operating energy, cost or carbon from a panel or pump-station design model with a source on every value.** Building energy tools model envelopes and HVAC. Electrical study tools compute losses at a snapshot. Vendor calculators are single-brand, single-device and keep no model.
3. **Open data exists for prices and carbon factors.** The US Utility Rate Database is CC0, EIA data is public domain, Eurostat's own data is CC BY 4.0, and UK carbon factors are under the Open Government Licence. Embodied-carbon data (EC3) and tariff engines (Arcadia) need paid licences for production use.
4. **Accuracy is bounded by the inputs, not the formula.** Even calibrated building models are only expected within about ±5–15% monthly (ASHRAE Guideline 14). Demand charges can be 30–70% of a commercial bill and need a load profile Hyleon will not have at design stage.
5. **The useful role for Hyleon is the join, not the engine.** It can connect catalog facts, the person's answers, regional data and AI-researched values into consequences (kWh, cost, heat, carbon, payback), show which input is missing, and hand heavy simulation to specialist tools.
6. **Several energy questions are also design-trust checks.** A pump far from its best efficiency point, a motor below the regional minimum efficiency class, or drive losses that overload the panel's cooling are design errors with an energy cost. These fit Hyleon's purpose most directly.

## 1. Building and plant energy modelling; electrical tools that report losses

| Product | What it computes | Data and licence | Accuracy limits | AI / MCP (2026) | Price | Buyers |
|---|---|---|---|---|---|---|
| **EnergyPlus** (US DOE) | Whole-building sub-hourly energy simulation. Has utility-tariff and life-cycle-cost objects [U] | Open source, BSD-3-like licence since v8.5 [S]. Latest release **26.2.0, 2026-09-30** [V] | Depends on inputs and weather files. Design models are uncalibrated | **EnergyPlus-MCP** from LBNL: 35 tools to load, validate, edit, simulate and read results; modified BSD; defaults to EnergyPlus 26.1.0; server v0.1.0 [V]. Published in SoftwareX, Dec 2025 [S]. No tariff or cost tools listed [V] | Free | Energy modellers, researchers, code-compliance consultants |
| **OpenStudio** | SDK and apps over EnergyPlus | Open source; licence text not checked [U] | As EnergyPlus | Not researched | Free | As above |
| **IES VE** | Energy, compliance, comfort, HVAC, daylight, airflow, "Decarbonization Modeling" [V] | Proprietary | Not stated | No AI on product page; VE Python Scripting exists; no MCP mentioned [V] | Quote; student version £50 [V] | Building services consultants |
| **eQUEST** | GUI over DOE-2.2 | Free | Old engine | None found | Free | Legacy energy modellers |
| | DOE-2.3 still a "release candidate" on doe2.com; last listed build dated 2017; mailing-list activity Jan 2025 [S] | | | | | |
| **Trane TRACE 3D Plus** | Loads and equipment sizing with EnergyPlus annual simulation [S] | Proprietary | Not stated | Not found | Not found | HVAC designers |
| **Carrier HAP v6** | Peak loads plus energy and operating-cost comparison of alternatives [S]. Released 2022-11-29 [S]. DOE qualification note for HAP 6.1 (May 2025): hourly energy by source "suitable for determining energy and power costs" [S] | Proprietary | Same DOE note: no automatic lighting-control modelling [S] | Not found | Free upgrade from v5 for most users [S]; list price not found | HVAC designers |
| **ETAP 2026** (announced 2026-05-21) | Load flow (with losses), short circuit, arc flash, hosting capacity; "electrical digital twin" from design to operations; 23,000+ equipment models [V] | Proprietary | Load flow gives losses at one operating point. **No annual energy-loss cost feature confirmed** | "Private" AI Copilot (plain-language queries, generates diagrams and reports) and AI AutoComplete in the one-line [V]. MCP not mentioned [V] | Quote | Power-system engineers, utilities, large industry |
| **SKM PowerTools** | Not researched (search limit) | | | | | |
| **Siemens SIMARIS design** | Dimensioning of LV distribution: short circuit, load flow, voltage drop, energy balance, power-loss calculation (2019 deck) [S] | Proprietary; free download [U] | Snapshot calculation | Not researched | Free [U] | Siemens-based designers |
| **Schneider EcoStruxure Power Design (Ecodial)** | LV installation sizing; uses high-efficiency transformer loss values by default for some voltages [S] | Proprietary; free [U] | Snapshot | Not researched | Free [U] | Schneider-based designers |

**What this group does not do:** none of them takes a pump station's parts, their catalog efficiencies and the owner's run hours, and gives annual kWh, cost and carbon per part with sources. Whole-building tools need an envelope and HVAC model. Electrical tools give losses at one load point.

## 2. Motor, pump and drive energy calculators; drive efficiency classes

| Tool | What it computes | Inputs it needs | Limits | Price | Buyers |
|---|---|---|---|---|---|
| **ABB Energy Savings Calculator** | Savings from replacing direct-on-line motors with drives in pumps and fans; "light version" of ABB's energy appraisal; focuses on LV motors of IE3 and below [S]. Older versions gave kWh, money saved, drive cost and payback [S] | Motor rating, hours, price (exact fields not confirmed) | Vendor example: two 30 kW pumps, 75% saving, 7.5-month payback. A vendor example, not a benchmark [S] | Free online [S] | Plant engineers, ABB sales |
| **Danfoss MyDrive Efficiency Calculator** (was ecoSmart, then MyDrive Energy) | IE class of the drive and IES class of the drive-plus-motor system per IEC 61800-9-2, with part-load losses [S] | Drive model, motor data, operating point | Danfoss's sample report says results are "a guide" only [S] | Free [S] | Drive specifiers |
| **Grundfos Product Center** | Life-cycle cost and energy comparison between pumps; "Optimize sizing" ranks pumps by energy, investment cost, size and LCC [S] | Load profile: "standard profile" or "full load"; custom profiles in sizing [V][S] | Default profile is applied unless changed. Energy price, hours and interest inputs not confirmed on the how-to page [V] | Free with sign-in | Pump specifiers, contractors |

**IEC 61800-9-2 (drive efficiency classes).** Current edition 2023 plus Amendment 1:2025 [S].
- A complete drive module (CDM) is classed IE0 to IE2. IE2 means 25% lower losses than the IE1 reference [S].
- A power drive system (PDS, drive plus motor) is classed IES0 to IES2, ±20% losses around IES1. The 2023 edition extends classes up to IES5 [S].
- Losses are declared at standard operating points of speed and torque, so a catalog can hold part-load losses as data [S]. The exact point set was not re-checked [U].

**Why this matters to Hyleon.** Drive losses are one number used twice. They are heat inside the panel (phase 4 already plans "heat load against cooling") and they are energy cost. A catalog entry that holds part-load losses serves both.

## 3. Cost estimating and 5D BIM

| Product | What it does | Data | AI (2026) | Price | Buyers |
|---|---|---|---|---|---|
| **RSMeans Data Online** (Gordian) | Unit-cost database with location factors | Proprietary licence | **Flash AI Estimating**, launched 2026-03-17: reads construction documents and gives a conceptual estimate in minutes [S] | Not found | Estimators, owners, public agencies |
| **Trimble Accubid** (electrical) | Electrical estimating | Pricing services, history | AI Smart Assistant answers plain-language questions on historical material pricing and estimate versions [S]. "Up to 60%" time saving is a vendor claim [S] | Quote | Electrical contractors |
| **McCormick** (electrical) | Electrical estimating | 55,000 items, 25,000 assemblies (vendor claim) [S] | Not found | From $300/month (Capterra) [S] | Electrical contractors |
| **ConEst IntelliBid** | Electrical estimating with SureCount takeoff [S] | | Not found | Not found | Electrical contractors |
| **ProEst** (Autodesk, acquired 2022) | Cloud estimating | | AI-assisted takeoff via Autodesk Construction Cloud [S] | ~$299–500+/user/month, unverified [S] | GCs, trades |
| **RIB CostX** | 2D/3D takeoff, 5D BIM, workbooks [S] | | Reviewers in 2026 say it lags AI tools; Togal.AI is a partner integration, not native [S] | Quote [S] | Quantity surveyors |
| **RIB iTWO**, **Sage Estimating**, **Navisworks Quantification / Autodesk Takeoff** | Not researched (search limit) | | | | |

**Accuracy.** Estimate accuracy depends on design maturity. AACE's estimate classes (Class 5 at concept to Class 1 at bid) are the usual way to label it [U]. Hyleon's phase 4 budget estimate could carry such a label; see decisions.

**What this group does not do:** they price materials and labour. None computes operating energy or cost.

## 4. Lifecycle cost and carbon

| Product or source | What it covers | Licence and terms | AI / API | Price | Buyers |
|---|---|---|---|---|---|
| **One Click LCA** | Embodied-carbon LCA, a "Life Cycle Costing tool" (Expert pack), "MEP Carbon tool" add-on; 20+ BIM and simulation integrations, IFC import, carbon back to BIM [V]. No operational energy features listed [V] | Proprietary database | "AI-assisted automatic data mapping", "Plausibility Checker", "AI enhancements" in Power Pack [V]. API and MCP not mentioned [V] | Quote; Business, Expert, Enterprise tiers [S] | Architects, sustainability consultants, manufacturers |
| **EC3** (Building Transparency) | Free EPD database and embodied-carbon comparison tool | **API (page modified 2026-10-20):** free tier for "exploration and testing only", 4/min, 400/h, 2,000/day; Practitioner **$5,000/yr**; Sustaining **$10,000–25,000/yr**; 50% off for government and nonprofits [V]. **ToS (2026-08-28):** free tier may not store, cache or use EC3 data in commercial or production apps; no resale or redistribution as a standalone data product on any tier; notify BT before integrating; attribution required; no clause on AI training [V] | REST API | Tool free; API as above | Designers, owners, policy (Buy Clean) |
| **openEPD** | Open schema for digital EPDs | Spec under Apache 2.0 [S] | — | Free | Software makers |
| **PEP ecopassport** | EPD programme for electrical, electronic and HVAC equipment (France); mutual recognition with EPD International signed 2025-02-13 [S] | Public lookup at register.pep-ecopassport.org [S]; bulk access and licence unknown [U] | Not found | Not found | Electrical makers, French projects |
| **CIBSE TM65** | Method to estimate embodied carbon of building-services equipment when no EPD exists [S]. TM65.4 (office HVAC), Oct 2025, amended Nov 2025 [S]. CIBSE Embodied Carbon Verification register for MEP products [S] | Paid CIBSE publication [U] | — | Not found | MEP engineers, makers |
| **ISO 15686-5:2017** | LCC framework: costs from acquisition through operation to disposal; compares alternatives; discount rate converts future cash to present [S] | Paid standard | — | — | Owners, PFI/PPP, public sector |
| **EN 15978** | Building LCA modules: A1–A3 product, A4–A5 construction, B1–B7 use (B6 operational energy), C end of life, D beyond [U] | Paid standard | — | — | LCA practitioners |

**Operational carbon factors (energy × factor):**

| Source | Coverage | Licence | Notes |
|---|---|---|---|
| **EPA eGRID** | US subregion rates in lb/MWh (CO2, CH4, N2O, CO2e) [S] | US government work, likely public domain; terms not confirmed [U] | eGRID2023 released 2025-01-15, rev1 2025-01-17 [S]; rev2 2025-06-12 [S]. eGRID2024 status not confirmed [U] |
| **UK DESNZ conversion factors 2026** | UK company reporting factors | **Open Government Licence v3.0** [S] | New method cut the data lag from two years to one; the electricity factor fell sharply, about 26% by commentators' count [S]. Shows factors move year to year |
| **Electricity Maps** | Real-time and historical grid carbon intensity | Free tier non-commercial only; commercial licence required [S] | €4,500/yr per signal quoted for carbon-free % signals; carbon-intensity price not confirmed [S] |
| **IEA emission factors** | Country factors | Paid [U] | Not researched |

**Embodied carbon for Hyleon's parts is thin.** EPDs exist mostly for building materials. Electrical and pump equipment has some PEP and EPD coverage. TM65 exists because MEP products usually lack EPDs. An embodied-carbon figure for a VFD panel would often be "unknown" or a method-based estimate.

## 5. Energy prices and tariffs

| Source | What it holds | Licence | Notes |
|---|---|---|---|
| **OpenEI Utility Rate Database (URDB)** | Rate structures for **3,700+ US utilities**. About 150 utilities, covering 70% of US load, are checked and updated each year by "rate experts" [V] | Page states **CC0** "unless otherwise noted" [V] | Maintained by the **National Laboratory of the Rockies** (NREL's new name since 2025-12-01 [S]); data collected by Illinois State University [V]. API versions 3–8; v4+ include international rates; fields for TOU energy and demand schedules (weekday and weekend), tiered demand, fixed charges, minimum charge, coincident demand, and look-back (ratchet) rules; `approved` flag means an expert verified the rate; 500 records per request [V]. Free API key [S]. Each record shows its last update [V] |
| **Arcadia Signal** (was Genability) | North American tariff library and cost engine. Models TOU (on-, partial-, off-, critical-peak), tiers, demand charges, riders, and taxes in tariff books. Taxes outside tariff books are **not** included [V] | Commercial | **Self-service $499/month: 25 tariffs and 2,500 calculations; overage $12/tariff and $0.12/calculation**; 14-day trial; enterprise by contract [V] |
| **US EIA** | Retail prices, sales, by state and sector; API v2 | **Public domain**; EIA asks for credit with the publication date, e.g. "Source: U.S. Energy Information Administration (Oct 2008)" [V] | Free API key; bulk download needs no key [S] |
| **Eurostat** `nrg_pc_205` | Non-household electricity prices by country and consumption band, every six months [S] | **CC BY 4.0** for Eurostat-owned content; data from other sources may not be reused commercially; a non-responsibility disclaimer is required when data is modified [S] | Non-household prices exclude refundable taxes and levies [S]. H2 2025 data was published May 2026 [S] |
| **ENTSO-E Transparency Platform** | European day-ahead prices | Unclear; older critique says reuse needs checking with each primary data owner [S] | Not a retail tariff |

**Tariff structure is where design-stage estimates break.**
- Demand charges bill the highest average kW in a window, usually 15 minutes, per billing period [S].
- NREL (2017) put demand charges at roughly 30–70% of commercial bills [S, could not fetch the PDF]. LBNL says "often 50% or more" [S].
- Ratchets can bill a summer peak for months afterwards. The 80% figure often quoted comes from vendor blogs [S]. URDB holds the look-back fields per rate [V].
- So a flat "price per kWh" can understate cost badly for a pump station with large motors. A demand charge needs the peak kW (which Hyleon can estimate from the design) and the tariff (which the person must supply).

## 6. Equipment efficiency standards and data

| Rule or source | What it says | Status (2026-10-31) |
|---|---|---|
| **EU Regulation 2019/1781** (motors and VSDs), amended by 2021/341 | Applied from 2021-07-01. Three-phase motors 0.75–1000 kW: IE3. 75–200 kW: IE4 from July 2023. VSDs: IE2. Motors and drives must publish efficiency at several speed and torque points [V] | In force; a review is listed as "ongoing legislative work" without dates [V] |
| **EU Regulation 547/2012** (water pumps) | Minimum Efficiency Index (MEI); MEI 0.40 removes the least efficient 40% of models [V] | In force. A revision toward an "extended product" index (pump + motor + drive) was studied in 2018–2019 [S]; **no adopted revision** shown on the Commission page [V] |
| **US DOE electric motors** (10 CFR 431.25) | Direct final rule 2023-06-01 (88 FR 36066); **compliance from 2027-06-01** [S]. Most 100–250 hp motors move to NEMA Super Premium (about IE4) [S] | Compliance date seven months away |
| **US DOE pumps** (10 CFR 431, subpart Y) | Pump Energy Index (PEI). Last standards final rule 81 FR 4368, 2016-01-26 [V]. Compliance from January 2020 [U] | A 2022 notice of data availability; no newer final rule listed [V] |
| **DOE Compliance Certification Database** | Makers' certified ratings for covered products; updated about every two weeks; bulk files [S] | Public |
| **ENERGY STAR** | data.energystar.gov via Socrata SODA API, 40–50+ datasets [S] | FY2026 funding of at least $33M was enacted 2026-01-23; FY2027 not covered; certification slowed by staff losses [S]. Low relevance to motors and drives [U] |
| **EPREL** (EU energy label database) | Public part shows label and product sheet [S]; a public API is claimed by third parties [S] | Motors and drives are under ecodesign, not energy labelling, so likely not in EPREL [U] |
| **IEC 60034-30-1 / NEMA Premium** | IE1–IE4 motor classes; NEMA Premium is roughly IE3 at 60 Hz [U] | Standard tables are copyrighted; cite, don't copy (D-017) |

**Fit with Hyleon's rules.** These are regulations with dates and clauses. They fit region profiles as cited boundaries. Note they govern placing new products on the market. A motor that already exists on site is not "non-compliant" for being IE2. A check must know a part's status (existing, in stock, to buy), which the catalog already records.

## 7. Accuracy: what limits any estimate

| Source of error | Size | Who can fix it |
|---|---|---|
| Run hours and duty cycle | Often the largest single factor; doubling hours doubles energy | The person or owner |
| Load profile (flow over time) | Changes part-load efficiency and the value of a VFD; Grundfos's own tool shows different LCC for "standard" vs "full load" [V] | The person, process engineer, SCADA history |
| Actual duty point vs design | Pumps rarely run at the design point | Commissioning data |
| Part-load efficiency | Motors and drives lose efficiency at low load; catalog data must hold part-load points | Catalog |
| Tariff structure | Demand charges 30–70% of commercial bills [S] | The person's tariff |
| Price and factor drift | The UK electricity factor moved about 26% in one year from a method change [S] | Dated sources, the watcher |
| Discount rate | Can flip which option looks best [S]; "no consensus" on choice (CIBSE) [S] | The owner |
| Whole-building models | Calibrated: monthly CV(RMSE) ≤15%, NMBE ±5%; hourly ≤30%, ±10% (ASHRAE Guideline 14; secondary sources disagree slightly) [S]. Design models are uncalibrated | Specialist modellers |

**Conclusion.** The formula is exact. The answer is only as good as hours, profile and tariff. Hyleon should show those inputs next to every result.

## 8. What Hyleon could compute honestly from its model

### From the model and catalog alone (no new questions)

| Quantity | How | Source label |
|---|---|---|
| Electrical input power of a motor at rated load | Rated output ÷ efficiency | calculated, from catalog |
| Input power at the duty point | Hydraulic power (flow × head × density × g) ÷ pump efficiency ÷ motor efficiency ÷ drive efficiency, all at that point | calculated, from catalog curves and the piping pack's duty point |
| Drive and panel losses | From part-load losses in the catalog | calculated; the same value feeds heat load |
| Continuous auxiliary loads | Control power supplies, PLC, HMI, switches, heaters, panel cooling | catalog ratings |
| Peak demand estimate | Sum of running loads in the worst duty combination (duty pumps, not standby) | calculated, from the pump station pack's duty/standby answer |
| Cable losses | I²R from conductor size, current and length | calculated; real lengths from phase 5 |
| Regulatory efficiency class checks | Catalog IE/IES class vs the region profile's minimum, only for parts marked "to buy" | standard, via the region profile |

### Only after the person answers

| Question | Why it's needed | Possible default |
|---|---|---|
| Site location and utility | Selects tariff, emission factor, currency | Region profile |
| Run hours per year, per part or per duty mode | Energy = power × hours | None, or a labelled assumption (Dan's decision) |
| Load profile or control mode (fixed speed, VFD on pressure, on level) | Part-load efficiency and VFD value | A cited vendor or standard profile, marked as assumed |
| Duty, standby, alternation | Which pumps run, for how long | The pump station pack's answer |
| Tariff: energy price, TOU periods, demand charge, fixed charge, date | Cost | URDB rate, EIA or Eurostat average marked "indicative" |
| Emission factor basis (location or market) and source | Carbon | Region profile's cited factor |
| Study period, discount rate, escalation | LCC and payback | Owner's answer |
| Maintenance and replacement costs | LCC | Person's answer |
| Purchase and install prices | LCC, payback | Phase 4 catalog prices, dated |

### Illustrative example (made-up sample values, no real site or maker)

| Value | Amount | Source |
|---|---|---|
| Pump shaft power at duty | 11.0 kW | calculated (catalog pump curve, duty point from piping pack) |
| Motor efficiency at that load | 0.92 | catalog |
| Drive efficiency at that point | 0.97 | catalog |
| Input power | 11.0 ÷ (0.92 × 0.97) = 12.3 kW | calculated |
| Drive loss | ≈ 0.37 kW, also counted in panel heat load | calculated |
| Run hours | 4,000 h/yr | person |
| Annual energy | 12.33 × 4,000 ≈ 49,300 kWh | calculated |
| Price | 0.15 per kWh, flat, dated 2026-10 | person |
| Annual energy cost | ≈ 7,400 per year, **excludes demand and fixed charges** | calculated, estimate |
| Emission factor | 0.20 kg CO2e/kWh, made-up | region profile (in a real case, cited and dated) |
| Operational carbon | ≈ 9.9 t CO2e/yr | calculated, estimate |

If run hours were missing, Hyleon would not show a cost. It would show a question: "Run hours for P-101 are unknown, so energy cost isn't computed."

### Checks that are design-trust checks with an energy consequence

These fit TRUTHS.md most directly. Each needs a cited limit or a person's answer before it's in force (truth 5). They are proposals, not rules.
- Pump duty point far from its best efficiency point.
- Motor heavily oversized for its shaft load.
- Motor or drive below the region's minimum efficiency class, for parts marked "to buy".
- Drive and panel losses exceed the enclosure's cooling capacity (already in phase 4 as heat load).
- A VFD specified on a pump with no turndown in its answered load profile.

## 9. How estimates should be labelled

Proposals, consistent with PRODUCT.md (sources, findings never block):
- **Label every result an estimate** and state its basis in one line: "at duty point, 4,000 h/yr, flat 0.15/kWh dated 2026-10, excludes demand charges".
- **Show each input with its source label** (catalog, person, standard via region profile, calculated, AI-inferred), as for any other value.
- **Date prices and factors.** Show the date next to the number. A watcher can flag stale ones.
- **Carry unconfirmed inputs forward.** If an input is AI-inferred (for example, an efficiency an agent researched and nobody approved), the result says it depends on an unconfirmed value. Whether this becomes a rule is Dan's call.
- **Name what is excluded:** demand charges, taxes, maintenance, part-load.
- **Optional range** from low and high inputs (for example, hours 3,000–5,000) instead of false precision.
- **Wording:** "estimated at", never "will cost" or "saves". No "compliant" for efficiency classes; "checked against Regulation (EU) 2019/1781, Annex I" (D-025 style).
- **Never block.** Missing inputs become questions. Energy findings are info or warnings and are listed at issue like any other.
- **Budget estimates** could carry a maturity class (such as AACE's) so readers know the expected accuracy. Dan's decision.

## 10. Where region profiles fit

| Item | Fits the region profile? | Why |
|---|---|---|
| Motor, drive and pump efficiency regulations with dates (EU 2019/1781, 547/2012; DOE 10 CFR 431) | **Yes** | They are cited rules in force in a place, like code editions |
| Energy codes adopted (ASHRAE 90.1 or IECC editions, national equivalents) | **Yes** | Same as other code adoptions; cite clauses only |
| Supply frequency and voltage | Already there (conventions) | Affects which motor rating applies |
| Currency | Yes | Convention |
| Default emission factor and its source (eGRID subregion, DESNZ, national) | **Probably**, with a date | Location-specific, published yearly, open licences |
| Links to tariff sources (URDB, national regulator, Eurostat band) | Yes, as sources | Helps the person answer |
| Actual tariff and price | **No** | Utility-, customer-class- and contract-specific; changes often. Belongs in the project as the person's answer |
| Indicative average price (EIA state, Eurostat band) | Optional, labelled "indicative" | Useful for early options; must not pass for the real tariff |

**Cadence mismatch.** Region profiles change when codes change. Prices change every few months and factors yearly. The daily watcher (D-019) could watch eGRID, DESNZ, Eurostat and URDB releases too, and propose updates for a person to approve. A separate, faster "energy data" profile is the alternative. Dan's decision.

**Privacy.** Looking up a tariff by location sends the site's location to a third party (OpenEI, Arcadia). Under truth 7, that should be the person's explicit choice.

## 11. Gaps in the market

1. **No design-model energy and cost for panels and pump stations.** Building tools cover envelopes and HVAC. Electrical tools give snapshot losses. Vendor tools are single-brand and keep no model or sources.
2. **No tool turns missing inputs into questions.** Tools apply defaults quietly (Grundfos's "standard profile" is the default unless changed [V]).
3. **Losses are computed twice.** Heat load and energy cost use the same drive losses, but tools treat them separately.
4. **Embodied carbon for electrical and pump equipment is thin.** Few EPDs; TM65 exists because of that gap.
5. **Tariffs are open in the US but complex.** URDB is CC0 and detailed, yet most tools use a flat price. Demand charges are ignored at design stage.
6. **AI produces numbers without provenance.** Gordian's Flash AI makes estimates from documents [S]; ETAP's Copilot generates reports [V]. I found no evidence that any of them marks AI-produced values separately from sourced ones. That is an absence of evidence, not proof.
7. **Options comparison rarely includes operating cost with sources.** Hyleon already plans to compare options on findings, BOM and cost (PRODUCT.md).

## 12. Products as a connecting layer (Dan's addendum)

Dan's framing: Hyleon connects things after research is done (researched knowledge + the person's model + what an AI produces) into consequences and gaps. Which products in this category position themselves that way?

| Product | What it connects | How close | What's missing compared with Hyleon's idea |
|---|---|---|---|
| **One Click LCA** | BIM quantities (20+ integrations, IFC) + EPD database → embodied carbon, written back to BIM; AI data mapping and a plausibility checker flag odd values [V] | **Closest in carbon.** Its core job is joining a model to researched data | Building materials, not equipment; no operational energy; proprietary data; no questions for missing inputs; no AI-guess marking found |
| **EC3 + openEPD** | Takeoff quantities + an open EPD format → carbon comparison | Close as a data join; openEPD is an open connector format | Data licence blocks production use on the free tier [V]; materials-centric |
| **5D BIM (CostX, iTWO, Navisworks Quantification)** | Model quantities + cost rates → estimate that updates with the model | Close for cost | No energy or operating cost; provenance per rate not confirmed |
| **Arcadia Arc / Signal** | Utility accounts and bills + tariff library → cost calculations [V] | A connecting layer, but for operations data, not design | Needs metered data or a load profile; no design model |
| **ETAP 2026** | One electrical model from design through operations; AI Copilot queries it [V] | Close on "one model over the lifecycle" | An authoring and simulation tool; one discipline; proprietary; no source labels found |
| **EnergyPlus-MCP** | An AI agent ↔ a physics engine through checked tool calls, including model validation [V] | Close on "AI changes the model only through checked steps" | One domain; no marking of AI-made values; no cost or tariff tools |
| **Gordian Flash AI** | Documents → conceptual estimate by AI [S] | The opposite: AI produces the number | Sources per value not shown |
| **Vendor calculators (ABB, Danfoss, Grundfos)** | One maker's data + a few user inputs → savings | Islands | No model, no persistence, single brand |

**What it implies for Hyleon.**
- The connecting layer for equipment-level design is empty in this category. The pieces exist (catalog efficiencies, open tariffs and factors, specialist engines, AI agents) but nothing joins them with sources.
- Hyleon's value would be the joins and the ledger, not the engines:
  - catalog facts (researched, approved or still AI-inferred)
  - plus the person's answers (hours, tariff, profile)
  - plus region data (rules, factors)
  - give consequences (kWh, cost, heat, carbon, payback) and gaps (questions, unconfirmed inputs).
- When a part changes, every consequence updates and shows why. Swap a motor, and efficiency, energy, cost, heat load and carbon change together, each with its source.
- Heavy calculation stays with specialists. Hyleon exports to EnergyPlus, ETAP or a tariff engine and records their results as sourced values, as PRODUCT.md already does for power studies.
- What an AI "gets" (researched values) and "does" (proposals) flows into these consequences visibly marked, so a cost that rests on an unconfirmed efficiency says so.

## 13. Decisions only Dan can make

Each has options; a recommendation is given where the evidence supports one. None is decided here.

1. **Does energy and operating cost belong in Hyleon?**
   - A. No; hand off entirely to specialist tools.
   - B. Minimal: per-part annual kWh and cost from catalog efficiency, person-entered hours and a flat dated price; info only.
   - C. B plus TOU and demand charges, operational carbon, and LCC in the options comparison.
   - D. C plus live tariff and carbon services.
   - *Recommendation:* B in phase 4 beside prices, because it reuses drive losses from heat load and fits "options compared on cost". C later. D only through hand-off or the person's own accounts. *Trade-off:* B understates bills where demand charges dominate, so the exclusion must be shown.
   - *Open question for Dan:* does an energy cost estimate pass the decision test in TRUTHS.md ("makes a design more trustworthy before it's built")? The efficiency checks in section 8 clearly do; the cost figure is a consequence view.
2. **Defaults for run hours and load profiles.** Always ask, or offer a cited vendor or standard profile marked as assumed?
3. **Where prices and emission factors live.** In region profiles, in a separate faster energy-data profile, or only as project answers?
4. **Which data sources to use.**
   - Open only (URDB CC0, EIA public domain, Eurostat CC BY 4.0, DESNZ OGL), with attribution.
   - Paid integrations (Arcadia from $499/month; EC3 API from $5,000/year).
   - Bring your own key and licence.
5. **Carbon.** Include it? Operational only (simple, open factors), or embodied too (thin data; would often show "unknown")? Location-based or market-based factors?
6. **LCC method.** Simple payback only, or NPV per ISO 15686-5 with the owner's discount rate, period and escalation?
7. **Unconfirmed inputs.** Should a calculated value carry a mark when any input is AI-inferred? (A proposal; it would be a new rule.)
8. **Efficiency regulations as region-profile boundaries.** Add EU 2019/1781, 547/2012 and DOE 10 CFR 431 to region profiles as cited boundaries, and check them only for parts to buy?
9. **Catalog format.** Add part-load efficiency and loss points (motor, drive per IEC 61800-9-2), pump curves, standby power and an EPD or PEP link? This changes the catalog contract.
10. **Budget accuracy class.** Label phase 4 budget estimates with a maturity class?
11. **Scope and tier.** Which module and tier hold energy and cost (D-033 scopes: an estimator may never see wiring)?
12. **Location lookups.** Allow sending the site's location to a tariff service, and only on the person's explicit choice?

## Sources

All read 2026-10-31 unless dated otherwise.

**Energy modelling and electrical tools**
- EnergyPlus releases: https://github.com/NREL/EnergyPlus/releases
- EnergyPlus-MCP: https://github.com/LBNL-ETA/EnergyPlus-MCP ; paper: https://bies.lbl.gov/publications/energyplus-mcp-model-context
- EnergyPlus licence history (secondary): https://www.energy.gov/node/1040371
- IES VE: https://www.iesve.com/software/virtual-environment
- eQUEST / DOE-2: https://doe2.com/DOE2/ ; https://onebuilding.org/archive-part2/equest-users-onebuilding.org/2025-January/003817.html
- Trane TRACE 3D Plus (secondary): https://www.techbriefs.com/component/content/article/8821-updated-energy-efficiency-modeling-software-programs
- Carrier HAP v6: https://www.carrier.com/commercial/en/nl/news/news-article/carrier-releases-hap-v6--a-major-upgrade-to-hvac-system-design-software/ ; https://hvacinsider.com/carrier-releases-hap-v6-a-major-upgrade-to-hvac-system-design-software/ ; DOE note: https://www.energy.gov/sites/default/files/2025-05/bto-hap61-1054-qs-052225.pdf
- ETAP 2026: https://etap.com/company/news/2026/05/21/etap-announces-etap-2026-powering-continuous-energy-intelligence ; https://etap.com/product-releases/etap-2026-release
- Siemens SIMARIS: https://assets.new.siemens.com/siemens/assets/api/uuid:b8b0b4b7-e9f5-4548-81d0-a7bbc3816fe4/simaris-planning-tools-introduction.pdf
- Schneider Ecodial: https://www.se.com/us/en/faqs/FA415035/

**Motor, pump and drive tools; drive classes**
- ABB: https://new.abb.com/service/service/motion/energy-efficiency-and-circularity/energy-savings-calculator ; https://new.abb.com/news/detail/114809/abb-helps-business-map-a-route-to-energy-efficiency-savings-with-new-online-calculator
- Danfoss: https://www.danfoss.com/en-gb/service-and-support/downloads/dds/mydrive-ecosmart-app/ ; https://www.danfoss.com/en/about-danfoss/news/dds/mydrive-energy-enhancing-energy-efficiency-and-project-viability ; https://www.danfoss.com/en/service-and-support/articles/dds/a-new-international-standard-for-energy-efficiency-iec61800-9/
- Grundfos: https://www.grundfos.com/us/support/how-to-guides/how-to-compare-life-cycle-costs ; https://www.grundfos.com/ae/learn/research-and-insights/lifecycle-cost-equation-for-pump-systems
- IEC 61800-9-2:2023: https://online.standard.no/en/iec-61800-9-2-2023-ed2 ; https://scc-ccn.ca/standardsdb/standards/2053186

**Cost estimating**
- Gordian Flash AI (2026-03-17): https://markets.financialcontent.com/observerreporter/article/gnwcq-2026-3-17-gordian-introduces-flash-ai-estimating-an-aipowered-solution-for-earlystage-construction-cost-estimates
- RSMeans: https://www.gordian.com/resources/why-choose-rsmeans-data-online/
- Trimble MEP AI: https://www.engineering.com/?p=149219
- McCormick: https://capterra.com/p/165314/Electrical-Estimating-Software/ ; https://www.mccormicksys.com/?p=87614
- ProEst: https://www.softwareadvice.com/construction/proest-profile/ ; https://www.mastt.com/software/construction-estimating-software
- CostX: https://www.capterra.com/p/75416/CostX/ ; https://www.capterra.com/p/75416/CostX/reviews/

**Lifecycle cost and carbon**
- One Click LCA: https://oneclicklca.com/full-feature-list ; https://www.capterra.com/p/166103/One-Click-LCA/
- EC3 API pricing: https://www.buildingtransparency.org/api-access-pricing/ ; ToS: https://www.buildingtransparency.org/api-terms-of-service
- openEPD: https://www.open-epd-forum.org/terms-of-service
- PEP ecopassport: https://www.environdec.com/news/mra-pep-ecopassport ; https://www.inies.fr/en/inies-and-its-data/pep-building-equipment/
- CIBSE TM65: https://www.cibsejournal.com/cibse-news/embodied-carbon-consultation-opens-on-current-use-of-cibse-tm65/ ; https://cibse.org/policy-insight/news/cibse-launches-tm65-uae-a-methodology-for-calculating-embodied-carbon-in-building-services
- ISO 15686-5: https://webstore.ansi.org/preview-pages/ISO/preview_ISO+15686-5-2017.pdf ; https://cibse.org/media/lz1dequj/20110511_scott-west_life-cycle-costing-for-building-services.pdf
- eGRID: https://www.epa.gov/egrid ; https://epa.gov/egrid/summary-data ; https://en.wikipedia.org/wiki/Emissions_%26_Generation_Resource_Integrated_Database
- DESNZ 2026: https://www.gov.uk/government/publications/greenhouse-gas-reporting-conversion-factors-2026 ; https://assets.publishing.service.gov.uk/media/6a2940543b15d05a7ce3202e/2026-GHG-conversion-factors-methodology-report.pdf ; https://circularecology.com/news/desnz-2026-uk-ghg-conversion-factors
- Electricity Maps: https://help.electricitymaps.com/en/articles/13169258-pricing-and-quotes ; https://electricitymaps.com/terms-of-service
- ASHRAE Guideline 14 thresholds (secondary): https://bemcyclopedia.com/wiki/Calibrated_models ; https://discovery-pp.ucl.ac.uk/10117731/3/Burman_2020_BPA_SimBuild_JainN.pdf

**Prices and tariffs**
- URDB: https://openei.org/wiki/Utility_Rate_Database ; API: https://openei.org/services/doc/rest/util_rates/?version=8
- NREL renamed: https://www.energy.gov/eere/articles/energy-department-renames-nrel-national-lab-rockies
- Arcadia Signal pricing: https://dash.genability.com/pricing ; tariff object: https://docs.arcadia.com/v2022-12-21-Signal/reference/tariff
- EIA reuse: https://www.eia.gov/about/copyrights_reuse.php ; API terms: https://www.eia.gov/opendata/terms-of-service.php
- Eurostat copyright: https://ec.europa.eu/eurostat/help/copyright-notice ; prices: https://ec.europa.eu/eurostat/statistics-explained/index.php/Electricity_price_statistics
- ENTSO-E (secondary): https://www.nworbmot.org/energy/brown-entsoe.pdf ; https://annualreport2019.entsoe.eu/transparency-regulation/
- Demand charges: https://eta-publications.lbl.gov/sites/default/files/comdemandcharge-briefing.pdf ; NREL 2017 (not fetched): https://www.nrel.gov/docs/fy17osti/68963.pdf

**Efficiency rules and data**
- EU motors and VSDs: https://energy-efficient-products.ec.europa.eu/product-list/electric-motors_en
- EU water pumps: https://energy-efficient-products.ec.europa.eu/product-list/water-pumps_en ; review status (2019): https://www.eceee.org/ecodesign/products/electric-pumps
- DOE motors: https://www.govinfo.gov/content/pkg/FR-2023-10-20/html/2023-23204.htm ; https://acim.nidec.com/motors/usmotors/-/media/USMotors/Documents/Literature/Brochures/DOE-RULES-BROCHURE-05312024.pdf
- DOE pumps: https://www.energy.gov/eere/buildings/pumps
- DOE certification database: https://www.regulations.doe.gov/certification-data/products.html
- ENERGY STAR data: https://www.energystar.gov/products/spec/energy_star_api_user_essentials_pd ; funding: https://www.utilitydive.com/news/energy-star-gets-full-2026-funding-from-congress/811087/
- EPREL: https://build-up.ec.europa.eu/en/resources-and-tools/tools/eprel-european-product-registry-energy-labelling-database

**Not researched because the search limit ran out:** SKM PowerTools, ABB DOC, RIB iTWO, Sage Estimating, Navisworks Quantification and Autodesk Takeoff, OpenStudio's current version and licence, WattTime, IEA factor licensing, the Hydraulic Institute Energy Rating programme, ÖKOBAUDAT, and the DOE pump compliance date.
