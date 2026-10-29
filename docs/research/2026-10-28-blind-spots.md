# Research record: blind spots (2026-10-28, second round)

This record is never edited; a later round gets a new file. It follows [2026-10-28-market.md](2026-10-28-market.md). Dan asked for it to find the gaps neither he nor Hyleon's agents could see.

## How this was done

Five research agents worked in parallel. Their full reports are kept verbatim in [2026-10-28-blind-spots/](2026-10-28-blind-spots/):

1. [Required deliverables](2026-10-28-blind-spots/1-deliverables.md): 18 public bid specifications and owner standards, mostly water and wastewater.
2. [Practitioner pains](2026-10-28-blind-spots/2-pains.md): reviews, forums and studies.
3. [What reviewers reject](2026-10-28-blind-spots/3-inspections.md): plan-review checklists, violations, field evaluation, Ten States, warehouses, solar. Also covers where region data can come from.
4. [Industries](2026-10-28-blind-spots/4-industries.md): 14 industries, including warehouses and plant layout.
5. [Pre-mortem](2026-10-28-blind-spots/5-premortem.md): analogues, failure modes, liability, channels, trust.

**Limits:**
- Search budgets ran out.
- Reddit, G2, Capterra, PLCtalk and some authority pages blocked automated reading.
- Food and beverage and warehouse owner standards weren't found.
- Some code citations come from the agents' memory and are marked as such.

## The blind spots

1. **Owners dictate the output.**
   - Of the 18 documents, 9 require native CAD, mostly DWG; DXF appears once.
   - Owners impose their own title blocks, fonts, minimum text heights, line weights, layer names, sheet sizes, numbering and tag grammars, revision schemes and seal workflows.
   - One owner runs its own validator against its standard.
   - Source: appendix 1.
2. **Reviewers reject omissions and mismatches.**
   - The top reason is incompleteness against the authority's own checklist.
   - Next is fault current at every node compared with each device's interrupting or short-circuit rating, with the utility source and the study date. NEC 409.22(B) requires that date.
   - Missing SCCR marking is the most common control-panel violation.
   - Source: appendix 3.
3. **Intelligence is lost at handoff, and drawings drift from what was built.**
   - A DXF/DWG export from EPLAN drops the logic.
   - Some firms strip the intelligence on purpose.
   - Owners bear about two-thirds of the cost of data that doesn't move between systems (NIST).
   - OSHA cites missing or incorrect P&IDs.
   - Source: appendix 2.
4. **Projects run through a lifecycle:**
   - 30/60/90% design reviews with written responses
   - submittal action codes and numbered resubmittals, with no fabrication before approval
   - factory and site acceptance tests with sign-offs
   - draft as-builts, then a record set
   - final payment tied to the record documents in 3 of the documents

   Source: appendix 1.
5. **The same base content serves most industries.**
   - Motor and VFD circuits, panels, loops and I/O, networks, pump stations, tanks and level, dosing, standby power and commissioning cover 12 of the 14 industries.
   - Warehouses and plant layout need a spatial engine first.
   - Packs must stack: base, then industry, then jurisdiction, then standard edition.
   - Source: appendix 4.
6. **Regions need dates, not only editions.**
   - Ontario fixes the edition at the date plans are submitted, and Philadelphia was still on NEC 2017 in 2024.
   - Adoption data comes from ICC's chart (a PDF, admittedly incomplete), ICC's paid Code Connect API, eCode360 for municipal amendments, and state registers.
   - Enacted amendments may be quoted (Georgia v. Public.Resource.Org); privately written model codes may not.
   - Ten States 2022 may be copied with credit.
   - Source: appendix 3.
7. **How Hyleon could fail** (appendix 5):
   - Incumbents copy the pitch: EPLAN calls its Copilot "a co-thinking partner, not … a decision-maker".
   - Free calculators already exist (Eaton, Rockwell).
   - Hyleon gets locked out of DWG.
   - Content costs more than it earns.
   - Professionals don't trust it: only 11% of contractors trust AI highly.
   - An engineer stamps output nobody reviewed (NSPE case 24-2).
   - Warnings get ignored (Mars Climate Orbiter).
   - The vendor becomes liable: the EU treats software as a product from 9 December 2026.
   - Revenue comes slowly: Procore made $4.8M after ten years.
   - Per-seat pricing clashes with per-project billing.
   - One founder spreads too wide, and buyers fear abandonment (EAGLE's end of life).

## What the findings confirm

- **One model generating every document.** Most rejections are mismatches between sheets.
- **A source on every value, and "it points to the clause".**
- **Hyleon never certifies; the person decides.** That matches NSPE case 24-2 and ASCE policy 573.
- **An open format and vendor-neutral exports.** That answers the fear of abandonment.
- **Region profiles with sources and dates.**

## Proposals for Dan (not decided)

**A. The outputs owners demand**
- **Owner profiles:** title blocks, fonts, text heights, line weights, layers, sheet sizes, numbering, tag grammars, revision schemes and seal workflow, plus a validator.
- **A tag-grammar engine** that keeps names identical across every document.
- **When DWG arrives:** a decision for Dan.
- **Handoff choices:** the full model, a deliverable with the intelligence stripped out, DWG or DXF with mapped attributes, and CSV or JSON.

**B. The first job, and focus**
- Support every industry, but start with the shared base packs.
- **Flagship check: the fault-current chain.** The utility value, then each node, then each device's rating, then the printed labels, with the study date.
- **The unlisted-panel workflow:** SCCR, nameplate, and a package ready for a field evaluation body.
- **Water and wastewater content:** Ten States, NFPA 820, alarms and standby power.
- **A spatial engine** as its own phase, before warehouses and layouts.
- **Region profiles carry effective-date rules,** such as the edition in force on the submission date.

**C. Lifecycle and records**
- **Submittal stages:** 30/60/90 reviews, action codes, resubmittals, acceptance-test forms, draft and final record sets.
- **A verification status on each element:** design intent, field-verified (by whom and when), or markup pending.
- **Field markups flow back into the model.**
- **Record-set label:** the person chooses "record drawings" or "as-built".
- **Sealing:** the licensee's own electronic seal or signature on the PDF, which Hyleon never applies itself.
- **Power studies:** one-line data exported to SKM, ETAP and EasyPower.

**D. Trust and liability**
- **Acknowledging an error requires a reason;** for a warning, a reason stays optional.
- **Fewer warnings,** each worth reading.
- **The wording rule:** "checked against clause X, edition Y", never "compliant".
- **Pack defects:** every affected user is told, with a versioned correction.
- **For counsel:** EU product liability for software (from 9 December 2026), and E&O and product insurance.
- **Enterprise:** open export and source-code escrow.
- **Pricing:**
  - free seats for reviewers (engineers, inspectors, clients)
  - test per-project pricing alongside per-seat
- **A count of what Hyleon caught,** kept on the user's device.
