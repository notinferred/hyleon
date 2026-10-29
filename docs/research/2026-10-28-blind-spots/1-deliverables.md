# Appendix 1: Required deliverables from bid specs and owner standards (agent report, 2026-10-28)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-28-blind-spots.md](../2026-10-28-blind-spots.md).

**The brief:** from public bid specifications and owner design standards, find:
- the required submittals and deliverables for each kind of job
- the format and style rules owners commonly mandate
- what this implies for Hyleon's packs

---

I found deliverable requirements in 18 public documents and extracted the full text of each. **Food and beverage and warehouses are still unknown:** the session's shared web-search limit ran out partway through, and I found no primary documents for those two sectors. Frequencies come from a water and wastewater (W/WW) sample of six owners: SPU [1], SAWS [2], MAWSS [3], EMWD [4,5], Winnipeg [6] and Sudbury [8,9]. Common means 4–6 of 6, sometimes 2–3, rare 1, not seen 0. Each building-sector row rests on a single US Department of Defense guide specification (UFGS), so how often those are required elsewhere is unknown.

## 1. Matrix

| Kind of job | Deliverable | How common | Src |
|---|---|---|---|
| Control/PLC panels (W/WW) | Scaled panel layouts and elevations + bill of materials | Common 6/6 | 1,2,3,5,6,8 |
| | Ladder schematics, power and I/O wiring, point-to-point interconnections with wire and terminal numbers | Common 6/6 | 1,2,3,5,6,8 |
| | Nameplate schedule | Sometimes 3/6 | 1,2,5 |
| | UL 508/508A listing evidence | Sometimes 3/6 | 1,2,5 |
| | Heat-dissipation data / formal heating-cooling calculation | Sometimes 3/6 / rare 1/6 | 1,2,5 / 2 |
| | Power-supply, UPS-runtime, PLC-memory and seismic calculations | Rare 1/6 | 2 |
| | SCCR calculation as a named submittal | Not seen. SCCR marking itself is a UL 508A/NEC requirement | 18 |
| Process I&C/SCADA (W/WW) | P&IDs (ISA-5.1), made by the designer | 2/2 owner design standards | 1,8 |
| | Loop drawings (ISA-5.4) | Common 4/6 | 1,4,6,8 |
| | I/O list including spares | Common 4/6 | 1,2,6,8 |
| | Control narrative (setpoints, alarms, trends) | Common 4/6 | 1,2,3,8 |
| | Instrument data sheets / on ISA-20 forms | Common 4/6 / sometimes 2/6 | 1,2,4,8 / 2,4 |
| | Calibration sheets, installation details | Common 4/6 | 1,2,3,4 |
| | Cable and conduit schedule | Sometimes 2/6 | 1,9 |
| Networks | Network architecture / communication diagram | Common 4/6 | 1,2,6,8 (also 7) |
| | Cabling layout, fibre/copper termination diagrams, network test reports | Rare 1/6 | 9 |
| Testing | Factory acceptance test (FAT) procedures and reports | Common 4/6. MAWSS uses site tests plus a 30-day acceptance test instead | 1,2,5,8; 3 |
| | Site acceptance test (SAT), loop-check and calibration forms with sign-offs | Common 6/6 | 1,2,3,4,6,8 |
| Closeout | O&M manuals; record (as-built) drawings | Common 6/6 | 1–6,8 |
| | Spare-parts list | Common 5/6 | 1,2,3,5,8 |
| | Training plan and materials | Common 4/6 | 2,3,4,8 |
| Power distribution (Div 26) | Load-flow, fault-current, coordination and arc-flash studies; PE-stamped; native model files | UFGS; elsewhere unknown | 12 |
| Building controls (Div 23 09/25) | Drawing index and legend, schematics, points schedule, sequences, valve and damper schedules, BACnet PICS, test procedure and report, draft and final as-builts | UFGS; elsewhere unknown | 14 |
| Fire alarm (Div 28) | Floor plans ≥1/8"=1', riser, point-to-point, I/O matrix; battery, voltage-drop and 25%-spare calculations on the drawings; NICET III/IV designer | UFGS; elsewhere unknown | 13 |
| Pipelines (water) | Red-lined as-builts with depths, fitting dimensions, GPS points | 1 owner | 17 |
| Facility layouts | Site and interior plans, wall elevations, instrument location plans, control-room layouts | 1 owner | 1 |
| Process piping shop drawings; F&B; warehouses | — | Unknown | — |

Section numbering varies by owner: process controls appear in Div 40 [2,3,7], Div 25 [6] or the legacy Div 16/17 [4,5].

## 2. Commonly mandated format and style rules

- **CAD formats:** 9 documents require native CAD [1,2,8,11,13,14,15,16,17], mostly AutoCAD DWG, with MicroStation or Revit allowed in UFGS [13,14]. Five require PDF alongside it [2,11,13,14,15].
  - The AutoCAD release is "latest/most current" or agreed at project start, never a fixed number [8,13,15,16].
  - DXF appears once, as an as-built format next to AutoCAD/MicroStation and PDF [13].
  - Manitoba Hydro requires drawings from other software to be translated to DWG with "100% bi-directional compatibility" [15].
- **PDF:** searchable, bookmarked, OCR'd if scanned [11]. PDF/A-1b (ISO 19005-1) is required only by Manitoba Hydro [15].
- **Tables:** Excel for points and equipment schedules [14] and for instrument and equipment lists [2]; cable schedules in a database format [1].
- **Sheet sizes:**
  - 11×17 for panel and point-to-point drawings, and for reduced submittals [1,2,4,5]
  - 22×34 [14,17]
  - ANSI D minimum for fire alarm [13]
  - 24×36 or 30×42 for record sets [16]
- **Title blocks:** owner-supplied borders [15,16,17]. Required contents:
  - integrator name and address, project manager, owner and project, title, sheet number, revision number and description, responsible person [2]
  - contract and submittal numbers [11]
  - original issue date, revision history, design phase, scale [16]
- **CAD file rules:** one sheet per DWG [15,16,17]; external references (xrefs) bound and unused content purged [15,16]; AutoCAD eTransmit packages [2].
- **Layers:** AIA layer names under the US National CAD Standard [15,16], or the owner's template [17].
- **Text and lines:**
  - Arial/ROMANS or native AutoCAD fonts only [15,16,17]
  - plotted text ≥0.10" on C-size and smaller sheets, ≥0.12" on D-size and larger [15]
  - P&ID text at least "size 8" [1]
  - line weights 0.25/0.35/0.50/0.70 mm, with no custom plot-style (CTB) files [15]
- **Drawing conventions:**
  - Schematics: ladder format with line numbers and coil-to-contact cross-references [1,5]; field wiring dashed [1]; switches drawn in their normal, de-energised state [5].
  - P&IDs read left to right [1,8], with at most 32 tags per sheet [1].
  - Loop sheets: at most 8 loops per sheet [1]; no "typical" loop sheets [6]; a loop-impedance table on each loop diagram [4].
  - Sudbury publishes template panel drawing sets: title page, index, layouts with BOM, PLC module layout, communication diagram, AC/DC distribution, and a sheet per I/O slot [10].
- **Tagging:**
  - ISA-5.1 [1,4], or owner grammars such as Sudbury's `AAA(N)-AAA-AAAA-NNNN_AAACC` [8]; no one-off tags [1]
  - field wires carry the ISA tag plus a numeric ID; PLC I/O addressed by rack/slot/point; the owner approves the wire-numbering scheme [2]
  - the same wire ID at every terminal [5]
  - owner drawing-number schemes [7,15]
  - arc-flash label names must match the one-line diagram [12]
- **Revisions:** two-digit revisions starting at 00, four-digit sheet numbers [15]; revision numbers and dates on every sheet; a resubmittal of item 18 becomes 18.1 [11].
- **Seals and stamps:**
  - PE seal on control drawings, panel layouts and PLC programs [6]
  - signed and sealed hard copies [2]
  - electronic seal on the PDF/A with every revision authenticated, and the seal box left blank in the DWG [15]
  - PE-stamped power studies [12]
  - contractor's "checked" stamp with a conformance statement [2,11]
- **Review cycles:**
  - 30 calendar days per review, and the same again for each resubmittal [11]
  - action codes A/AN/RR/NR/RA [11], or "Furnish as corrected / Revise and resubmit / Rejected" [2]
  - no fabrication or ordering before approval [1,2]
  - incomplete submittals returned unreviewed, plus a paragraph-by-paragraph exceptions list [2]
  - test procedures approved before testing, with 30 days' notice [3]
  - 30/60/90% design reviews with written comment responses [1]

## 3. Implications for Hyleon

1. **Owners ask for DWG; DXF appears in only one document [13].** This needs your decision. The options are a native DWG writer (licensed library), DXF with each owner's acceptance confirmed, or conversion on the user's side [2,15,16].
2. **Make each owner's profile data:** border and title-block fields, sheet sizes, fonts, minimum text heights, line weights, layer map, numbering, revision scheme and seal workflow. Add a validator that checks a drawing set against the profile; Manitoba Hydro already runs its own title-block validator [15].
3. **Build a tag-grammar engine that checks across documents.** It should handle ISA-5.1 and owner grammars, plus wire, terminal and rack/slot/point IDs. Each name should stay identical across P&IDs, loops, panels, I/O lists and labels [1,2,5,8,12].
4. **Core set for the W/WW I&C pack** [1–9]:
   - P&IDs and one ISA-5.4 loop sheet per loop
   - panel layout with BOM and nameplate schedule
   - ladder schematics and point-to-point diagrams
   - I/O list including spares, and ISA-20 data sheets
   - network architecture diagram and cable schedule
   - control narrative with setpoint and alarm lists
   - FAT/SAT procedures with sign-off, loop-check and 0/50/100% calibration forms
5. **Deliver schedules as editable XLSX/CSV, not only as PDF** [1,2,14].
6. **Calculation packages Hyleon could own:** panel heat and cooling, power-supply and UPS runtime, loop impedance, conduit fill, and fire-alarm battery, voltage-drop and spare-capacity checks [2,4,13]. An SCCR worksheet (UL 508A Supplement SB) would support the code requirement [18].
   - Power studies are run in SKM, ETAP or EasyPower, and the native model files are required [12]. Hyleon should export one-line data to those tools rather than replace them.
7. **Model the project lifecycle:** 30/60/90 design, shop drawing, resubmittal, FAT, SAT, draft as-built, then record set, with intake of field red-lines [1,3,11,14]. In three documents, final payment or acceptance depends on the record documents [1,6,17].
8. **Before building F&B and warehouse packs,** get real customer standards and confirm DXF acceptance with target owners.

## Sources (★ = the 10 key documents)

1. ★ Seattle Public Utilities, DSG Ch. 10 I&C (SCADA), Feb 2024 — https://www.seattle.gov/documents/Departments/SPU/Engineering/DSG/2024/10IC-SCADA-FinalRedacted.pdf
2. ★ San Antonio Water System, PLC Replacement Ph. 4, Addendum 1 (Jun 2023), §40 61 00 — https://apps.saws.org/Business_Center/ContractSol/IFB%5C4226%5CAddendum%201%20BBP4%20(sealed)%20-%20Final_POSTED.pdf
3. ★ Mobile Area Water & Sewer System, Stickney WTP, Addendum 2 (Jan 2026), §40 61 13 — https://www.mawss.com/uploads/bids/2401551-mawss-stickney-wtp-solids-upgrades-addendum-no.-2-with-attachments.pdf
4. ★ Eastern Municipal Water District, §17005 I&C Components (2016) — https://content.emwd.org/sites/default/files/migrate-documents/17005_rev2016.pdf
5. Eastern Municipal Water District, §16950 Custom Control Panels (2016) — https://content.emwd.org/sites/default/files/migrate-documents/16950_rev1.pdf
6. ★ City of Winnipeg, Bid 983-2015 pumping station, Div 25 — https://legacy.winnipeg.ca/finance/findata/matmgt/documents/2015/983-2015/983-2015_NMS_Specifications_Division-25.pdf
7. City of Winnipeg, Bid 385-2023 NEWPCC DCS Migration, Addendum 1 (Jan 2024) — https://legacy.winnipeg.ca/finance/findata/matmgt/documents/2023/385-2023/385-2023_ADDENDUM_1/385-2023_Addendum_1.pdf
8. ★ City of Greater Sudbury, SCADA/C&I Design Standards Manual v1.0 (Apr 2018) — https://www.greatersudbury.ca/do-business/infrastructure-and-city-construction/engineering-services/scada-design-standards/manual-and-pdfs/scada-controls-and-instrumentation-systems-design-standards-manual-april-2018-docx/
9. Sudbury, Appendix B-1 Matrix of Design Phases and Deliverables — https://www.greatersudbury.ca/do-business/infrastructure-and-city-construction/engineering-services/scada-design-standards/manual-and-pdfs/appendix-b-1-matrix-of-design-phases-and-deliverables-xlsx/
10. Sudbury, appendix/template index — https://www.greatersudbury.ca/do-business/infrastructure-and-city-construction/engineering-services/scada-design-standards/manual-and-pdfs/
11. ★ UFGS 01 33 00 Submittal Procedures (Aug 2026) — https://www.wbdg.org/FFC/DOD/UFGS/UFGS%2001%2033%2000.pdf
12. ★ UFGS 26 05 73 Power System Studies (Aug 2023) — https://www.wbdg.org/FFC/DOD/UFGS/UFGS%2026%2005%2073.pdf
13. UFGS 28 31 76 Interior Fire Alarm and Mass Notification (Aug 2020) — https://www.wbdg.org/FFC/DOD/UFGS/UFGS%2028%2031%2076.pdf
14. UFGS 23 09 00 Instrumentation and Control for HVAC (Aug 2024) — https://www.wbdg.org/FFC/DOD/UFGS/UFGS%2023%2009%2000.pdf
15. ★ Manitoba Hydro, Minimum Standards for Drawings (Jan 2016) — https://www.hydro.mb.ca/docs/corporate/2016-01-07-minimum-standards-for-drawings-for-fabricators-manufactures-and-construction-companies.pdf
16. ★ Duke University, CAD Standards Guideline v2.2 (Jul 2017) — https://facilities.duke.edu/sites/default/files/Duke%20CAD%20Standards%20Guideline_v2.2.pdf
17. SCV Water, CAD Standards Manual (Jun 2025) — https://www.yourscvwater.com/sites/default/files/SCVWA/engineering/SCV-Water-CAD-Manual-Revised-061225.pdf
18. IAEI Magazine, "How to Determine SCCR" (2012; secondary source) — https://iaeimagazine.org/2012/march2012/how-to-determine-short-circuit-current-ratings/

The fetch tool could not read these PDFs, so I extracted their text with the PDF reader built into macOS. Nothing was installed and I wrote no files.
