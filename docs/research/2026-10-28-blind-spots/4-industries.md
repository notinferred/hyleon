# Appendix 4: Industries map, including space design (agent report, 2026-10-28)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-28-blind-spots.md](../2026-10-28-blind-spots.md).

**The brief:** map 14 industries, including warehouses and plant layout. For each:
- typical jobs, codes and deliverables
- who designs the work, the incumbent tools, and AI entrants
- one market signal
- fit for Hyleon's engine (topology, spatial, or both)

Then list the scenarios shared across industries, and say what spatial design would require.

---

I've mapped all 14 industries. Market signals are sourced, and the AI-entrant column has gaps. The shared web-search budget ran out partway through, so "none verified" means my searches found nothing, not that nothing exists. The IEA, OSHA, ICC and UNCTAD sites blocked direct reads, so a few figures come through reputable secondary reports, labelled "via". Codes, deliverables, roles and tools come from domain knowledge with spot checks, so verify standard editions before writing packs.

## Industry table

| # | Industry | Typical jobs | Codes and standards (beyond base NEC/IEC) | Deliverables | Who designs | Incumbent tools | AI-native entrants 2024–26 | Market signal | Fit |
|---|---|---|---|---|---|---|---|---|---|
| 1 | Water & wastewater | Lift and booster pump stations, treatment trains, chemical feed, SCADA upgrades, standby power | Ten States Standards; AWWA; NFPA 820; ANSI/HI 9.8; NSF 61; IEC 62443 | P&IDs, control narratives, I/O and loop sheets, panels, Div 40 specs | Consulting engineers, utilities, integrators, UL 508A shops | AutoCAD Plant 3D, Revit, OpenFlows, BioWin, EPLAN | Transcend TDG: preliminary plant and pump-station design; added an MCP Gateway, Oct 2026 | At least $630B needed over 20 years for clean water (EPA CWNS, 2024) | Both |
| 2 | Food & beverage | CIP, batching and brewhouse, HTST pasteurizers, ammonia refrigeration, packaging lines | FSMA; USDA FSIS; PMO; 3-A; EHEDG; IIAR 2 + OSHA PSM; NFPA 660; ISA-88 | P&IDs, CIP matrices, recipes, washdown panels, sanitary layouts | Food design-builders (Haskell, Stellar, CRB), integrators, skid OEMs | AutoCAD Plant 3D, SolidWorks, EPLAN, Studio 5000/TIA Portal, FactoryTalk Batch | None verified for F&B | 42,708 plants (2022); 16.8% of US manufacturing sales (USDA ERS, 2025) | Both |
| 3 | Aquaculture, aquariums, pools | RAS loops, exhibit life support, pool filtration and chemistry, aeration, backup power | CDC MAHC + state codes; ISPSC; NSF 50; VGB Act; NEC 680; DIN 19643; AZA; 40 CFR 451 | Life-support P&IDs, turnover calcs, equipment schedules, dosing narratives, health-department plan sets | Aquatic consultants (Counsilman-Hunsaker), RAS suppliers (AKVA, Billund), MEP firms | AutoCAD/Revit, spreadsheets, vendor sizing tools | None verified | $1.9B sales, up 26% on 2018 (USDA Census of Aquaculture, 2024) | Both, led by topology |
| 4 | Controlled-environment agriculture | Climate control, fertigation dosing, LED circuits, grow-rack layout | NGMA; IFC CO₂ systems; NFPA 1 Ch. 38; UL 8800; DLC; CA Title 24 | Points lists, fertigation P&IDs, light-level (PPFD) plans, dehumidification calcs | Greenhouse builders, climate-control vendors (Priva, Ridder, Argus), MEP engineers | Vendor configurators, AutoCAD/Revit, DIALux/AGi32 | Source.ag ($17.5M, Nov 2025): operations, not design | Shrinking: Plenty filed Chapter 11 after raising about $1B (TechCrunch, 2025) | Both |
| 5 | Building automation & HVAC controls | Air-handler, VAV and plant sequences, BACnet/Niagara integration, retrofits, smoke control | ASHRAE Guideline 36, 135, 90.1; Title 24 acceptance tests; ISO 52120-1; UL 864/NFPA 92 | Points lists, sequences, control schematics, network risers, functional tests | MEP consultants, controls contractors, commissioning agents | Revit/AutoCAD, Visio, Excel, Niagara/Metasys/Desigo | PassiveLogic ($74M, Sep 2025); Augmenta ($10M, Mar 2025; electrical now, mechanical and plumbing agents planned) | EU EPBD makes building automation mandatory above 290 kW by end-2024 and above 70 kW by end-2029 (Directive 2024/1275) | Topology |
| 6 | Machine building & OEM panels | Machine panels, safety circuits, drives, PLC/HMI, UL and CE variants | UL 508A (SCCR); NFPA 79; IEC 60204-1; ISO 12100, 13849; CSA C22.2 No. 286; EU Machinery Regulation 2023/1230 (applies Jan 2027) | Schematics, panel layouts, BOM, wire lists, SCCR/FLA, risk assessment | OEM controls engineers, panel shops, integrators | EPLAN, AutoCAD Electrical, SolidWorks Electrical, E3.series, SISTEMA | EPLAN Copilot (incumbent); PLCs.ai and PLC Copilot (PLC code; seed rounds early 2026, aggregator data) | US packaging machinery $11.3B (2024), +2.2% forecast (PMMI, 2025) | Both |
| 7 | Solar, battery storage, EV charging | Commercial and utility PV, battery yards, fast-charging hubs, interconnection | NEC 690/705/706/625; NFPA 855; UL 9540A, 1741 SB; IEEE 1547/2800; IFC Ch. 12; 23 CFR 680 | One-lines, array and string layouts, voltage drop, interconnection and permit sets | EPCs, installer design teams, independent engineers | PVsyst, PVcase, RatedPower, Aurora, ETAP/SKM | Paces (siting and permitting agent, not design); none verified for design | 86 GW planned for 2026, incl. 43.4 GW solar and 24 GW batteries (EIA, 2026) | Both |
| 8 | Oil & gas, chemical | Wellpad and compressor automation, safety and burner systems, tank farms, MCCs | API RP 500/505; IEC 60079/ATEX; IEC 61511; ISA-5.1/18.2; OSHA PSM; EPA RMP | PFDs, P&IDs, HAZOP/LOPA, safety requirement specs, cause-and-effect, area-classification drawings | EPCs, owner engineering, main automation contractors | AVEVA, Hexagon Smart P&ID, AutoCAD Plant 3D, ETAP | None verified | Upstream oil investment under $500B, third annual fall; gas up 10% (IEA via EnergyNow, 2026) | Both |
| 9 | Pharma & life sciences | Process skids, water-for-injection loops, cleanroom HVAC, CIP/SIP | 21 CFR 211, Part 11; EU GMP Annex 1/11/15; GAMP 5; ASME BPE; ISO 14644 | URS/FS, traceability matrix, IQ/OQ/PQ, room data sheets, pressure and flow plans | Life-science A/E (CRB, Jacobs, Exyte), validation firms, DCS integrators | Revit, DeltaV/PCS 7, Kneat/ValGenesis | None verified | About $600B announced in 2025, only about $60B tied to specific projects (CBRE via Pharma Manufacturing, 2026) | Both |
| 10 | Data centers | UPS, busway and generators, liquid cooling, white space, commissioning levels 1–5 | TIA-942; BICSI 002; Uptime Tiers; ASHRAE TC 9.9, 90.4; NFPA 75; EN 50600 | One-lines, redundancy studies, rack and containment layouts, CFD, integrated-systems test scripts | Mission-critical A/E, hyperscaler in-house teams | Revit, ETAP, Cadence Reality, DCIM | ArchiLabs (YC F24); TestFit data-center configurator (2024) | 4.4% of US electricity in 2023, 6.7–12% by 2028 (LBNL, 2024) | Both |
| 11 | Mining | Dewatering, conveyors, process-plant control, portable substations | MSHA 30 CFR 56/57/75/77 and Part 18; CSA M421; AS/NZS 3007; GISTM | One-lines, ground-check designs, cable schedules, conveyor interlocks | EPCMs (Hatch, Worley, Ausenco), OEMs | Deswik, Datamine, Ventsim, AutoCAD, ETAP | None verified | US non-fuel mineral production $112B in 2025, +5.6% (USGS, 2026) | Both (mine planning out of scope) |
| 12 | Marine | Switchboards, propulsion drives, hybrid batteries, alarm and monitoring | Class rules (ABS, DNV, LR); SOLAS II-1; IEC 60092; 46 CFR Subchapter J; ABYC E-11; IACS E26/E27 | Class approval package (load, short-circuit, selectivity, cables), FAT/SAT | Naval architects, shipyards, electrical integrators | AVEVA Marine, Cadmatic, NAPA, ShipConstructor | None verified | Global orderbook 203.2M CGT, up 13% year on year (BIMCO via Maritime Gateway, 2026) | Both (tight 3D space) |
| 13 | Warehouses & logistics | Racking, aisles and egress, sprinklers, docks, conveyors and mobile robots, lighting | NFPA 13; IFC Ch. 32; FM Data Sheet 8-9; OSHA 1910.36/.176/.178; ANSI MH16.1; ASME B20.1 | Rack plans and elevations, high-piled storage permit plans, sprinkler calcs, rack load plaques | Material-handling integrators, rack dealers, industrial A/E, fire-protection engineers | AutoCAD, Revit, SketchUp, W2MO, FlexSim, AutoSPRINK/HydraCAD | TestFit (site level); ArchiLabs (racking, vendor claims); Fraunhofer IML WarehouseDesigner | Overbuilt; "minimal growth" in 2026 (AIA, 2026) | Spatial |
| 14 | Manufacturing plant layout | Block and equipment layouts, material flow, robot cells, safety zones | OSHA 1910 Subparts D/E/O/S; NFPA 101, 660; ISO 13855/13857; ISO 10218 | Layouts, flow diagrams, utility matrices, safety-distance drawings | Industrial engineers, industrial A/E (SSOE, Ghafari) | AutoCAD Factory Design Utilities, Tecnomatix, DELMIA, Visual Components | None verified | Manufacturing construction $170.7B a year in Aug 2026, down 19% year on year (Census via FRED) | Spatial |

## Scenarios shared across industries (by row number)

- **Motor and VFD circuits:** all 14.
- **Networks and OT security zones:** all 14.
- **Commissioning and as-builts:** all 14.
- **Control panels (UL 508A or IEC 61439, SCCR):** 1–12.
- **Pump stations:** 1–5 and 8–12.
- **Instrument loops and I/O lists:** 1–4 and 8–12.
- **Tanks and level:** 1–4, 8, 9, 11, 12.
- **Chemical dosing:** 1–4, 8, 9.
- **Filtration and disinfection:** 1–3, 9, 11.
- **Standby power (generator, ATS, UPS):** 1, 3–5, 9, 10, 13.
- **Power studies (short-circuit, arc flash, coordination):** 1, 7, 8, 10–12.
- **HVAC, refrigeration and ventilation:** 2, 4, 5, 9, 10.
- **Hazardous-area classification:** 1, 2, 8, 9, 11, 12.
- **Safety functions and safety-instrumented systems:** 2, 6, 8, 9, 11.
- **Batch and CIP sequencing (ISA-88):** 2, 8, 9.
- **Alarm and points lists:** 1, 3, 5, 8, 10, 12.
- **On-site PV, batteries, EV or forklift charging:** 1, 7, 10, 13.
- **Lighting layouts:** 4, 5, 13, 14.
- **Equipment-room layouts with working clearances:** every row marked "Both".

## Space and layout design

**Warehouses**
- **Aisles and egress:** OSHA 1910.176(a) requires "sufficient safe clearances" and marked aisles but sets no width. Aisle widths therefore come from the truck type and the fire code.
- **Exit routes:** 1910.36(g) requires exit access at least 28 in. wide and ceilings at least 7 ft 6 in.
- **Powered trucks:** 1910.178 covers battery-charging areas and which truck types may enter hazardous locations. It requires truck-mounted lights where general lighting is below 2 lumens per sq ft, and dock and rail-car protection.
- **Rack structure:** follows ANSI MH16.1, which the IBC adopts, and requires load plaques. Europe uses EN 15512 and EN 15620.
- **Fire protection:** NFPA 13 bases sprinkler design on commodity class, storage height, rack type, flue spaces and the clearance below sprinkler heads.
- **High-piled storage:** IFC Chapter 32 applies above 12 ft, or 6 ft for high-hazard goods. Its permit plans show storage areas, heights, aisles, flues and fire-department access doors.
- **Power and lighting:** charger circuits, conveyor motor control and energy-code lighting are the parts Hyleon's device-and-link model already covers.
- **AI entrants you named:**
  - Hypar does space planning, Finch does residential, office and high-rise, and Maket does residential only. None targets warehouses.
  - TestFit stops at the building and site (docks, freight stalls) and exports DXF, SketchUp and Revit.
  - ArchiLabs claims rack layouts checked for flues, sprinkler clearance and egress. That is vendor marketing from April 2026.
  - I couldn't identify the specialist rack-structural tools.

**What spatial support requires of the engine**
1. A building shell (column grid, clear height, doors) and the ability to import DWG, DXF, PDF or IFC drawings as backgrounds.
2. Placed objects with footprint, height and orientation, and clearance envelopes that are constraints in their own right, each tied to its source clause.
3. Parametric arrays (rack bays, PV rows, server rows) and layout generators that score their options.
4. Zones whose properties trigger rules, such as commodity class and storage height, hazardous-area extent or cleanroom grade.
5. Path analysis for egress width, travel distance and aisle continuity.
6. Routes that feed lengths back to the device model, for voltage-drop and hydraulic checks.
7. Elevations and sections, and dimensioned DXF, PDF and IFC output.
8. A decision on specialist calculations (rack structure, sprinkler hydraulics): integrate them, export to other tools, or build them.

## Implications: support all industries vs order of content

1. **Write shared base packs first.** Motor and VFD, panel, loop and I/O, network, pump station, tank and level, dosing, standby power, and commissioning cover most of rows 1–12. Water and F&B both start with those same packs, so choosing between them changes little early on.
2. **The engine roadmap limits the order.** Row 5 and the device-and-link side of rows 1–3 and 6 can ship before spatial support exists. Rows 13–14 cannot, and rows 4, 7 and 10 would be weak without it. Warehouses also share little content with rows 1–12.
3. **Packs need to stack.** One pump-station scenario has to resolve to Ten States, the pool code (MAHC), HI 9.8 or marine class rules depending on the job. That means layers: base, industry, jurisdiction and standard edition. Otherwise "all industries" means many duplicate packs.
4. **Quoting limits may be restricted.** This is my inference and needs a licensing check. Public law (CFR, OSHA, EU directives) can be quoted, but NFPA, UL, ISA, IEC and ASHRAE values may only be citable by clause. That affects which packs can ship with numbers in them.
5. **Regulated deliverables need engine features.** Pharma needs Part 11 audit trails and a validated tool, marine needs class approval packages, and solar and warehouses need engineer-stamped permit sets. These belong in the engine, not in packs.
6. **AI competition is concentrated.** The entrants I could verify cluster in building design and controls, PLC code and water preliminary design. I found none for panel, one-line or loop design in process industries, but the search gap may explain that.
7. **Demand points two ways.** Data centers, solar and battery storage, and water needs are growing. Manufacturing construction, warehouses and vertical farms are shrinking. Ordering by demand favours data centers and solar; ordering by reachable buyers (integrators) favours water and F&B. That's your call; I haven't assumed one.

## Sources

- **Market:** [EPA CWNS](https://epa.gov/newsreleases/new-epa-survey-highlights-wastewater-infrastructure-needs-protect-waterbodies) · [USDA ERS](https://www.ers.usda.gov/topics/food-markets-prices/processing-marketing/food-and-beverage-manufacturing) · [USDA NASS](https://nass.usda.gov/Statistics_by_State/Utah/Publications/News_Releases/2024/UT-Census-of-Aquaculture-12162024.pdf) · [TechCrunch](https://techcrunch.com/2025/03/24/vertical-farming-company-plenty-files-for-bankruptcy-after-raising-nearly-1b) · [EPBD Art. 13](https://gesetze.legal/eu/rl_2024_1275_eu/13) · [PMMI via MMH](https://www.mmh.com/article/2025_packaging_state_of_the_industry_u.s_expands_canada_maintains_strong_base) · [EIA](https://www.eia.gov/todayinenergy/detail.php?id=67205) · [IEA via EnergyNow](https://energynow.ca/2026/05/natural-gas-spending-to-hit-10-year-high-in-2026-as-oil-investment-falls-iea-says/) · [CBRE](https://www.pharmamanufacturing.com/facilities/article/55359868/big-pharmas-600b-us-investment-plans-are-short-on-details-cbre) · [LBNL](https://energyanalysis.lbl.gov/publications/2024-lbnl-data-center-energy-usage-report) · [USGS](https://www.usgs.gov/news/national-news-release/value-us-mineral-production-rose-last-year-driven-precious-metals-prices) · [BIMCO](https://www.maritimegateway.com/global-shipyard-orderbook-rises-13-as-china-faces-record-delivery-challenge/) · [AIA](https://www.aia.org/about-aia/press/nonresidential-construction-spending-face-prolonged-weakness-through-2027) · [Census/FRED](https://fred.stlouisfed.org/series/TLMFGCONS)
- **OSHA:** [1910.36](https://www.law.cornell.edu/cfr/text/29/1910.36) · [1910.176](https://www.law.cornell.edu/cfr/text/29/1910.176) · [1910.178](https://www.law.cornell.edu/cfr/text/29/1910.178)
- **Entrants:** [Transcend](https://www.wateronline.com/doc/transcend-gives-enterprise-ai-two-ways-to-work-with-deterministic-infrastructure-engineering-0001) · [Augmenta](https://www.constructiondive.com/press-release/20250312-augmenta-secures-10-million-usd-in-funding-to-pioneer-ai-driven-building-1) · [PassiveLogic](https://www.utahbusiness.com/press-releases/2025/09/18/passivelogic-raises-74-million-series-c-funding-generative-autonomy-utah/) · [ArchiLabs](https://ycombinator.com/companies/archilabs) · [ArchiLabs racking](https://archilabs.ai/posts/ai-cad-for-warehouse-racking-and-storage-system-layouts) · [TestFit industrial](https://www.testfit.io/typologies/industrial) · [TestFit data centers](https://testfit.io/blog/2024-testfit-year-in-review) · [Hypar 2.0](https://aecmag.com/features/hypar-2-0/) · [Finch](https://www.finch3d.com) · [Maket](https://www.maket.ai) · [Fraunhofer IML](https://www.iml.fraunhofer.de/en/projects/2025/warehouse-design-software.html) · [W2MO](https://www.logivations.com) · [EPLAN Copilot](https://www.eplan.com/de-en/about-us/innovations/ai-driven-industrial-automation/eplan-copilot/) · [PLCs.ai](https://www.caplight.com/company/plcs) · [PLC Copilot](https://www.caplight.com/company/plccopilot) · [Source.ag](https://www.newaginternational.com/?p=3290) · [Paces](https://www.paces.com)

I briefly wrote a helper script to the scratchpad to read a USDA PDF. It didn't work, and I deleted it; no other files were written.
