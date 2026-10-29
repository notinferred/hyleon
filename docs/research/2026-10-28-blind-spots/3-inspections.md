# Appendix 3: What inspectors and reviewers reject, and where region data comes from (agent report, 2026-10-28)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-28-blind-spots.md](../2026-10-28-blind-spots.md).

**The brief:** what plan reviewers and inspectors expect and reject, covering:
- electrical permits and common violations
- industrial control panels and field evaluation
- water and wastewater (Ten States, NFPA 820)
- warehouses
- solar, storage and EV

A follow-up message asked for section 7: sources of code-adoption data for region profiles. This is research, not legal advice.

---

# Hyleon: what plan reviewers and inspectors expect and reject

Research only, not legal advice. Status as of 28 Oct 2026.

**Limits.** The web-search quota ran out several times. Some pages were blocked or needed a script to load, so I couldn't read them: the LA (LADBS) electrical correction list, Houston's list, the NFPA map, ICC's code text, IAEI's adoption page, UpCodes' terms and nrel.gov. Where I cite a code section with no source, it comes from my own code knowledge and was not checked again.

## 1. Electrical permit plan review
**What reviewers expect**
- **Phoenix AZ** (rev. 08/2026, 2023 NEC) [1]:
  - an engineer's seal
  - a one-line showing equipment voltage, phase, ampacity, interrupting rating (AIC) and enclosure; ground-fault protection; feeders with neutral, equipment ground and conduit; grounding of services, transformers and generators
  - load calculations for all distribution equipment
  - available fault current at the service no lower than the utility's tables, carried "to all panels, contactors, relays", and compared with each item's AIC or short-circuit current rating (SCCR); chillers and elevator controllers are named
  - panel schedules, 110.26 working space, and classified (hazardous) areas
- **Kirkland WA** (undated; cites the 2021 WA energy code) [2]:
  - for each conductor: fault current at start and end, impedance, length, conduit type, and the study date
  - the manufacturer's series-rating charts
  - arc-flash incident energy, boundary and date for each panel
  - selective coordination, and motor-starting loads on standby systems
  - an engineer's seal for services or feeders of 1,600 A or more

**Top rejection reasons**
- No load or fault-current calculations "for all panels"; disconnects not shown; egress lighting doesn't match the architectural exits (Dayton OH, 2011) [3].
- Systems left out of the submittal (fire pump, standby); no seal; wrong address; the 110.26(C)(3) door and panic-hardware rule for equipment of 800 A or more; 240.24(B) occupant access to breakers (Philadelphia, Sept 2024, still on the 2017 NEC) [4].

## 2. Common violations at inspection
- **Inspector feedback, national** (EC&M, Jan 2023) [5]: box fill; termination torque; panel directories (408.4(A)); service-to-feeder bonding; missing field labels; GFCI; 110.26 working space; work covered before inspection.
- **Maine Electricians' Board staff** (2017–18, 2017 NEC) [6]:
  - breakers oversized for conductors because the 75 °C terminal rating was ignored
  - neutral and ground not separated after a service-rated transfer switch
  - concrete-encased electrode not used
  - working space at VFDs and motor starters
  - no grounding electrode at separate structures
- **Control panels**: SCCR marking (409.22/409.110) is the most common violation, then wire-bending space (an inspector's account, 2015) [7].
- I found no ranked list from IAEI or NFPA. Mike Holt material is forum anecdote only [8].

## 3. Industrial control panels
- **Listed panel**: the inspector checks for damage, ratings against the load, field terminations and field modifications. A listed enclosure with parts added in the field counts as unlisted, and the AHJ (the approving authority) can require a field evaluation to UL 508A (UL, 2017) [9].
- **409.110 nameplate**: maker; supply voltage, phase, frequency and full-load current; a note if there are multiple disconnects; SCCR from a listing or an approved method such as UL 508A Supplement SB; whether it suits service equipment; diagram number; enclosure type [9].
- **Machinery** (Phoenix, 2011, rev. 2016) [10]: a permit is needed to install or relocate. If the 670.3 nameplate is missing, an engineer may re-plate it where available fault current is 5 kA or less. Above that, a field evaluation is required.
- **Field evaluation**:
  - Hayward CA (Apr 2023) accepts listed field evaluation bodies (FEBs) in place of lab listing. The FEB labels the equipment and reports to the city [11].
  - IAEI (2016) [12]: the AHJ approves FEBs, wants an NFPA 791 chapter 7 report, and gives written approval before labels go on. Typical faults are the wrong standard (NFPA 79 on non-machinery), missing markings, a downgraded enclosure type and unlisted Class 2 supplies.
  - Whether NFPA 791's "should" is binding is **unclear**.

## 4. Water and wastewater
- **Ten States Standards** (issued by an 11-member board of IL, IN, IA, MI, MN, MO, NY, OH, ON, PA and WI) [13]: Water Works 2022 is current. Wastewater 2026 is current but sold, so I did not review it. The 2014 edition is free.
- **Water Works 2022** [14]:
  - submit plans at least 60 days ahead; no approval until plans and specifications are complete
  - switchgear above grade and outside flood-prone areas; NEC compliance
  - standby power for average-day demand, sized for motor starting, with automatic transfer
  - VFD under-speed and over-speed alarms
  - unattended plants need an automation report, major and minor alarms, shutdown without automatic restart, quarterly alarm tests, EPA cybersecurity practice, and manual operation
- **Wastewater 2014** [15]:
  - wet wells are Class I, Division 1, Group D; the motor control centre sits outside the wet well with seals
  - alarms on backup power (power failure, high level, pump failure, intrusion) sent to a site staffed 24 hours
  - emergency pumping for full capacity
- **NFPA 820 (2024)**: its tables classify each space. Continuous ventilation at 6 or 12 air changes per hour can lower or remove a classification. Gas detection and ventilation-failure alarms need backup power [16].
  - States set their own rates. Oklahoma requires 12 changes per hour continuous or 30 intermittent (2011) [17].
  - Seattle Public Utilities' 2024 basis-of-design sheet records available fault current, spare capacity, largest motor, the classification of each space and the standby power type [18].

## 5. Warehouses
- **High-piled storage permit** (Redmond WA, Mar 2024, 2021 IFC) [19]: required for storage over 12 ft, or over 6 ft for high-hazard goods.
  - Plans must show: heights; clearance to sprinkler deflectors; aisles; commodity class with evidence, stating whether the IFC or NFPA list was used; in-rack sprinkler valves; flue spaces; fire department access doors.
  - Missing items "may result in rejection".
- **San Bernardino County Fire** (rev. 2019) [20]: rack elevations; a commodity report if the reviewer asks; rack plans uploaded.
- **San Diego** (2017) [21]: a signed commodity declaration; flue widths; fire access within 150 ft.
- **Racks**:
  - LA County (2014) [22]: a permit above 5′9″; engineered calculations from 8 ft; each configuration designed; aisles and exits shown; ASCE 7 15.5.3.
  - Fresno (Apr 2026, ASCE 7-22, MH16.1-2023) [23]: anchor details, load plaques and special inspection.
- I found no ranked rejection data. Rejections follow missing checklist items.

## 6. Solar, storage and EV
- **Utility interconnection** (PG&E Rule 21, eff. Aug 2025) [24]:
  - a single-line showing relays, current and voltage transformers, and meters, with the disconnect marked
  - inverters to UL 1741 SB / IEEE 1547-2018
  - incomplete applications get 10 business days, then 5 more, or they are withdrawn
- **Permit corrections** (Newport Beach CA, residential, 04/2026) [25]: 705.12 busbar calculations; a testing-lab letter for load-side taps; label elevations; UL 9540 listing for storage; rapid shutdown.
- **Storage, fire review** (Seattle Fire, Apr 2025) [26]: a fire permit above 20 kWh of lithium-ion; spacing, seismic bracing and deflagration venting; commissioning and decommissioning plans; above the maximum allowable quantity, a hazard mitigation analysis plus a large-scale fire test.
- **EV** (California GO-Biz, Jan 2023) [27]: accessibility; load management that stays within circuit capacity, with hard-wired overcurrent protection as backup; utility connection.
- **Automated permitting**: SolarAPP+ projects failed inspection about 37% less often in 2021 data [28]. 2022 data reportedly showed no difference [29]. **Unclear.**

## Ranked priorities for Hyleon
This is my judgement: how often each item appears across the sources, weighted by whether it blocks approval.
1. Completeness against the AHJ's own checklist
2. Fault current at every node against AIC/SCCR, with the utility source, study date and series ratings
3. Load calculations that match the panel and equipment schedules
4. A complete one-line
5. Labels and nameplates (409.110, 670.3, fault current, arc flash)
6. Working space
7. Grounding and bonding
8. Conductor ampacity against overcurrent protection and terminal ratings
9. Listing evidence, or the path to field evaluation
10. Hazardous-area classification tied to ventilation
11. Standby power sizing and selective coordination
12. Pump-station alarms and telemetry
13. High-piled storage plan
14. Rack seismic package
15. Solar and storage package

## Implications
1. Most rejections are omissions or mismatches between sheets. Generate every document from one model and check them against each other.
2. Make the fault-current chain the flagship check: utility value, then each node, then each equipment rating, then the printed labels.
3. Regions need dates and local triggers, not just editions. Philadelphia was on the 2017 NEC in 2024 [4]. Ontario applies the edition in force when the plans were submitted [36].
4. Unlisted panels are a workflow: SCCR calculation, nameplate, and a package ready for a field evaluation body.
5. Water and wastewater reviewers read reliability: alarms, standby sizing and the reasoning behind each classification.
6. Model-code text needs a licence. Cite and paraphrase the limits instead. Ten States 2022 allows copying with credit [14].

## 7. Where code-adoption data comes from
- **NFPA's NEC map** [30]: needs a script to load; I found no API.
  - NFPA (via EC&M, 1 Oct 2025): 20 states had finished moving to the 2023 NEC, and none had started on 2026 [31].
  - EC&M (Sept 2026): 19 states on 2023, six on 2026, and eight more moving to 2023 [32].
  - That suggests about seven state changes in a year, but the counts conflict: **unclear**. I found no 2024 count.
- **ICC's adoption chart** (Apr 2026, as a PDF) [33]: it warns "Not all jurisdictions notify ICC". ICC's Code Connect API is paid and needs an implementation agreement plus a licence for each title; its page says nothing about AI use [34]. ICC's developer page also lists General Code's eCode360 API for municipal codes, which is where local amendments are published [35].
- **IAEI**: its adoption page blocked me, so its currency and terms are not verified.
- **State and provincial sources** are authoritative.
  - Ontario: the 2024 code took effect 1 May 2025, and the edition is fixed at the submission date [36].
  - Alberta: the 2024 Canadian Electrical Code has been in force since 1 Apr 2025 [37].
- **International**:
  - UK: BS 7671 with Amendment 4 (2026) is current, and the Amendment 2 transition ended 15 Oct 2026 [38].
  - New Zealand: reportedly moved to AS/NZS 3000:2018 on 13 Nov 2025 (search snippet only) [39].
  - EU national deviations and Latin American NEC adopters: not verified.
- **Other providers**:
  - NFPA LiNK: a 2021 copy of its licence reportedly bars offering database services to third parties (search snippet only) [42].
  - UpCodes: I couldn't read its terms. Trade press reported a partial fair-use ruling in its case with ICC in Dec 2020 [40]; I couldn't verify the final outcome.
- **Signals a daily watcher could monitor**: state rulemaking registers, the ICC chart's date, IET notices, utility tariff advice letters, and revision dates in the footers of AHJ checklists.
- **Legal status**: Georgia v. Public.Resource.Org (27 Apr 2020, 5–4) held that officials who make law cannot hold copyright in what they write in that role [41]. So enacted local amendments can be quoted. Model codes written by private bodies (NFPA, ICC) are **unclear**; ask counsel.
- Units and drawing conventions were not researched.

## Sources
[1] Phoenix electrical checklist, 08/2026 https://www.phoenix.gov/pddsite/Documents/TRT/dsd_trt_pdf_00710.pdf
[2] Kirkland guide https://www.kirklandwa.gov/files/sharedassets/public/v/6/development-services/pdfs/building-pdfs/electrical-plan-review-submittal-guide.pdf
[3] Dayton, 2011 https://daytonohio.gov/DocumentCenter/View/118/Contractors-and-Design-Professionals-Guide-101-PDF
[4] Philadelphia, 2024 https://www.phila.gov/media/20240917152808/Electrical-Permit-and-Deficiencies-Slides-9.2024.pdf
[5] EC&M, 2023 https://www.ecmweb.com/national-electrical-code/article/21256591/the-top-10-most-commonly-cited-electrical-code-violations
[6] Maine board https://www.maine.gov/future/sites/maine.gov.pfr.professionallicensing/files/inline-files/Common-2017-National-Electrical-Code-and-Electricians-Law-Violations-Revised.pdf
[7] EC&M, 2015 https://www.ecmweb.com/maintenance-repair-operations/article/20900674/make-sure-your-industrial-control-panels-are-ul-508-or-nec-compliant
[8] Mike Holt forum (not opened) https://forums.mikeholt.com/threads/50-most-common-nec-violations.95041/page-3
[9] UL, 2017 https://code-authorities.ul.com/wp-content/uploads/sites/2/2017/07/UnderstandingICP_2017.pdf
[10] Phoenix machinery https://www.phoenix.gov/pddsite/Documents/TRT/dsd_trt_pdf_00698.pdf
[11] Hayward, 2023 https://www.hayward-ca.gov/sites/default/files/documents/006_Recognized-Field-Evaluation-Bodies050123.pdf
[12] IAEI, 2016 https://iaeimagazine.org/2016/march2016/field-evaluations-an-inspectors-point-of-view/
[13] Minnesota Dept. of Health https://www.health.state.mn.us/communities/environment/water/tenstates/standards.html
[14] Water Works 2022 https://files.dep.state.pa.us/Water/BSDW/Public_Water_Supply_Permits/2022_Recommended_Standards_for_Water_Works.pdf
[15] Wastewater 2014 https://www.health.state.mn.us/communities/environment/water/docs/tenstates/tenstatestan2014.pdf
[16] CSE Magazine, 2025 https://www.csemag.com/how-to-navigate-nfpa-820-for-wastewater-treatment-plants/
[17] Oklahoma https://www.law.cornell.edu/regulations/oklahoma/OAC-252-656-7-1
[18] Seattle Public Utilities https://seattle.gov/documents/Departments/SPU/construction-resources/standards-guidelines/DSG/2024/11FExampleBODPlanSheet.pdf
[19] Redmond https://www.redmond.gov/DocumentCenter/View/32034
[20] San Bernardino County Fire https://sbcfire.org/wp-content/uploads/sites/46/2024/01/High-Pile-Checklist.pdf
[21] San Diego https://www.sandiego.gov/sites/default/files/dsdds164.pdf
[22] LA County racks https://ladpw.org/bsd/lib/fp/Building/Structural%20and%20Technical/2014%20Code%20Version/2014%20Steel%20Storage%20Racks%20Plan%20Review%20List.pdf
[23] Fresno https://www.fresno.gov/wp-content/uploads/2026/04/STORAGE-RACKS-SUB-REQUIREMENT-4.2.26-10w1471.pdf
[24] PG&E Rule 21 https://www.pge.com/tariffs/assets/pdf/tariffbook/ELEC_RULES_21.pdf
[25] Newport Beach https://newportbeachca.gov/home/showpublisheddocument/71555/639141098075870000
[26] Seattle Fire https://www.seattle.gov/documents/Departments/Fire/Business/ESSApplicationChecklist.pdf
[27] GO-Biz https://business.ca.gov/wp-content/uploads/2019/12/GoBIZ-EVCharging-Guidebook.pdf
[28] SolarAPP+ https://help.gosolarapp.org/article/222-what-is-solarapp-impact-on-jurisdictions
[29] NREL (unreachable) https://research-hub.nrel.gov/en/publications/solarapp-performance-review-2022-data/
[30] NFPA map https://www.nfpa.org/education-and-research/electrical/nec-enforcement-maps
[31] EC&M, Nov 2025 https://www.ecmweb.com/industry-viewpoint/blog/55330015/unpacking-key-revisions-to-the-2026-nec
[32] EC&M, Sept 2026 https://www.ecmweb.com/national-electrical-code/article/55403844/wisconsin-adopts-2023-national-electrical-code
[33] ICC chart (mirror) https://www.mitek-us.com/wp-content/uploads/2026/05/Master-I-Code-Adoption-Chart.pdf
[34] ICC Code Connect https://solutions.iccsafe.org/codeconnect
[35] ICC developers https://www.iccsafe.org/developers/
[36] ESA Ontario FAQ https://esasafe.com/assets/files/esasafe/pdf/About_ESA/2024-OESC-FAQs.pdf
[37] Alberta https://www.alberta.ca/electrical-codes-and-standards
[38] IET https://electrical.theiet.org/bs-7671/
[39] WorkSafe NZ (unreachable) https://www.worksafe.govt.nz/about-us/news-and-media/important-update-changes-to-electricity-safety-regulations-for-electrical-installations/
[40] BD+C, 2021 https://www.bdcnetwork.com/codes-and-standards/news/55163898/court-ruling-gives-upcodes-upper-hand-in-fight-over-building-codes-database
[41] Georgia v. PRO https://www.supremecourt.gov/opinions/19pdf/18-1150_7m58.pdf
[42] NFPA LiNK licence copy (unreachable) https://lfportal.everettwa.gov/WebLink/0/doc/1347435/Page4.aspx
