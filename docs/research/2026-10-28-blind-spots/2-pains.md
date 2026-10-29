# Appendix 2: Practitioner pains with current tools (agent report, 2026-10-28)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-28-blind-spots.md](../2026-10-28-blind-spots.md).

**The brief:** what practitioners find painful about designing and documenting site systems today, covering:
- consistency, compliance, revisions and as-builts
- handoffs, learning curve, cost and licensing
- field use and symbol standards
- switching triggers and what people would pay

---

# What practitioners find painful about designing and documenting site systems (as of 2026-10-28)

**Coverage limits.** I couldn't reach Reddit at all, because it blocks my crawler. G2, Capterra, PLCtalk, Eng-Tips, Control.com and Mike Holt's forum refused my requests ("403" errors), and the session's search budget ran out partway through. I read these directly: TrustRadius reviews (through its dev host; the links below go to the public pages), the Inductive Automation (IA) and MrPLC forums, government documents and studies. Quotes marked *(snippet)* come only from search-result excerpts of pages I couldn't open. Evidence on piping and networks is thin, and I found nothing on facility layouts or AutoCAD Plant 3D reviews. Many forum quotes are old, so I give the year.

## Ranked pains

**1. Drawings drift away from what was built, and owners pay for it.** *Studies and regulator data, plus one anecdote.*
- FMI/PlanGrid 2018 (600 respondents, vendor-sponsored): 22% of rework came from poor project information. People spent ["five-and-a-half hours per week"](https://www.constructiondive.com/news/industry-could-be-overspending-177b-per-year-study-finds/529450/) looking for project data.
- OSHA refinery inspections, 2007–11: process safety information was the second most-cited element of OSHA's process safety rules (17.4% of citations), and ["P&IDs missing / incorrect"](https://www.csb.gov/UserFiles/file/Barab%20%28OSHA%29%20PowerPoint.pdf) drew 37 citations. The Construction Industry Institute (2001) puts field rework on heavy industrial projects above 3% of construction cost, with roots ["in the design and project definition phases"](https://construction-institute.org/the-field-rework-index-early-warning-for-field-rework-and-cost-growth).
- NIST 2004: owners and operators carried about two-thirds of a $15.8B-a-year cost from systems that don't exchange data, ["about $10.5 billion … during operations and maintenance"](https://www.enr.com/articles/30495-study-of-data-roadblocks-costs-tags-capital-facilities-industry-with-15-8-billion-loss-8-9-2004). I found little on revision control itself. One Revit reviewer: ["No way to print specific issuances."](https://www.trustradius.com/products/revit/reviews)

**2. The "intelligent" data is lost, or deliberately withheld, at handoff.** *Vendor documentation and anecdote.*
- EPLAN's own help says that in a DXF/DWG export ["the logic information of the diagram pages can't be transferred"](https://www.eplan.help/en-us/infoportal/content/Plattform/2.9/Content/htm/xdxfgui_k_grundlagen.htm).
- Some firms strip it on purpose: ["Submitting exported simplified drawings to clients ensures that your intellectual property is protected"](https://help.elecdes.com/EBASE_USER/exportdwgs.htm).
- Customers dictate the tool: ["If you're a machine tool supplier to GM, you are expected to have EPlan P8"](https://mrplc.com/forums/topic/13720-any-users-of-eplan-p8/) (2008).

**3. License cost and complexity.** *Anecdotes and one collective letter.*
- EPLAN: ["License structure is not easy"; "License management and upgrade are difficult"](https://www.trustradius.com/products/eplan/reviews) (2024).
- Revit: ["they have a monopoly so you don't have much of a choice"](https://www.trustradius.com/products/revit/reviews) (2022), and "I am forced to pay $400 a month" (2024). An open letter from architecture firms in 2020 said costs rose ["up to 70 per cent and beyond"](https://www.dezeen.com/2020/07/28/autodesk-revit-bim-software-criticism/) by the end of 2019.
- On AutoCAD Electrical: ["I'll never look back into it as long as it remains subscription-based"](https://www.plctalk.net/threads/what-tools-and-how-are-you-guys-designing-your-control-panels.128265/) *(snippet)*.

**4. Setup and relearning: libraries, house standards, occasional users.** *Anecdotes.*
- ["spending months just to build library before actual drafting can start"](https://mrplc.com/forums/topic/13720-any-users-of-eplan-p8/) (2008). Another poster said intelligent tools mean you ["spend loads of time implementing their drawing standards into it"](https://mrplc.com/forums/topic/13720-any-users-of-eplan-p8/page/2/), and he preferred plain AutoCAD (2013).
- A small integrator who owns AutoCAD Electrical: ["eveytime I do it's a new learning curve"](https://forum.inductiveautomation.com/t/electrical-drawing-software/79881). He wants something "easy to learn but yet has the power for me to create new symbols" (2023).
- On EPLAN: "The learning curve is steep and there are many options that are not well documented" ([Capterra](https://www.capterra.com/reviews/152595/Electric-P8), *snippet*).

**5. Hidden project databases drift or stall, so people keep shadow spreadsheets.** *Anecdotes.*
- Revit users export panel schedules ["to excel … in order to appease plan-checkers"](https://forums.autodesk.com/t5/revit-ideas/panel-schedule-demand-load-calculations/idi-p/8521479) *(snippet; a 2023 comment reportedly says it's still unresolved)*.
- When AutoCAD Electrical reassigns wire numbers, the usual fix is to delete its scratch database and rebuild it ([Autodesk forum](https://forums.autodesk.com/t5/autocad-electrical-forum/wd-jumpers-and-wire-numbers-not-propagating/m-p/6919247), *snippet*). A SOLIDWORKS Electrical user on slowdowns: ["We aren't sure where these performance issues come from"](https://3dswym.3dexperience.3ds.com/question/solidworks-user-forum/solidworks-electrical-slowing-down_bT5Kb-YeRAqRZTU0TQwsDQ) (2018).

**6. Compliance means proving and formatting, not only calculating.** *Anecdote and a code requirement.*
- In the Revit case the numbers were right, but the layout wasn't what plan reviewers expect.
- NEC 409.22(B) requires the available fault current at a control panel, and the date it was calculated, to be documented ([Siemens](https://assets.new.siemens.com/siemens/assets/api/uuid:e38c994f-b2da-4846-ba00-f8e4733dfbde/ulsccr-whitepaper-icp-us.pdf), *snippet*).
- I found no direct demand for automated code-checking as such.

**7. Manual numbering and cross-references in tools that don't track them.** *Anecdotes, partly old.*
- What felt slow in AutoCAD: ["numbering of wire lables etc. - when done manually"](https://mrplc.com/forums/topic/13720-any-users-of-eplan-p8/) (2008).
- In 2023, BricsCAD was suggested as a cheap option with no automatic wire labeling, and Visio is still used for electrical and network diagrams ([IA](https://forum.inductiveautomation.com/t/electrical-drawing-software/79881)). A Visio reviewer: ["Icon sets can be difficult to find for non-general use"](https://www.trustradius.com/products/visio/reviews).

**8. Field, remote and offline work.** *Thin anecdotes.*
- Lucid is ["limited and weak when accessed offline"](https://www.trustradius.com/products/lucidchart/reviews) (2025).
- Bluebeam: ["Subscription base creates issues for remote users"](https://www.trustradius.com/products/bluebeam-revu/reviews) (2022).
- A need for use in air-gapped plants is plausible, but I found no evidence for it.

**9. Network documentation for plant control systems is now a security duty.** *Government guidance.*
- EPA (2024): ["A complete and up-to-date logical network diagram is essential to cyber disaster recovery"](https://www.epa.gov/system/files/documents/2024-07/maintain-updated-documentation.pdf).
- It advises contract clauses that oblige integrators "to support these documentation efforts."

**Not evidenced:** friction between IEC and ANSI symbols. The only item I found is an EPLAN parts library that ["contains only European brands"](https://mrplc.com/forums/topic/13720-any-users-of-eplan-p8/) (2008).

## Switching triggers and blockers

**What makes people switch:**
- **A customer mandate,** as with GM requiring EPLAN.
- **Project scale:** "on a large scale project automatic referencing wins hands down" and "it really saved us a lot of time and errors" (2008).
- **Automation:** one EPLAN shop generates about 70% of its drawings with a custom tool plus EPLAN Cogineer (TrustRadius 2024).
- **Subscription conversions,** after which holdouts freeze on old versions. A 1-out-of-10 Bluebeam review complains about the renewal.
- **A more modern architecture:** one integrator prefers SOLIDWORKS Electrical over AutoCAD Electrical for this reason (IA 2023).

**What stops them:**
- Building libraries and house standards.
- Migration time: "a full migration … is going to take us multiple years" (Capterra, *snippet*).
- Training cost: ["Eplan training cost a lot!!!"](https://mrplc.com/forums/topic/13720-any-users-of-eplan-p8/page/2/)
- "the cost and fears" (2010).
- Market lock-in, as in the Revit "monopoly" quote.
- Clients requiring a particular file format.

## Pricing and willingness to pay
- **Bluebeam (official prices):** [$260, $330 or $440 per user per year; Max is $590 at an introductory price](https://www.bluebeam.com/pricing/).
- **EPLAN:** prices aren't published ("You have to request a quote"). One poster estimated about £5K (2023). Another called it "horrifically expensive" ([Control.com](https://control.com/forums/threads/autocad-electrical.31289/post-31289), *snippet*).
- **Siemens Radica (cloud electrical CAD):** "subscription based and not cheap" (2023).
- **Revit:** one reviewer pays about $400 a month.
- **Cheap substitutes are in use:** Visio, BricsCAD, the free QElectroTech, and PCSchematic, said to cost about a fifth of EPLAN ([Control.com](https://control.com/forums/threads/electrical-design-software.23115/page-3), *snippet*).
- **Owners already pay to reconcile drawings:** integrators sell field-verified as-built updates with change logs as a service ([ISE Controls](https://isecontrols.com/?p=52589)).
- I found no survey-grade data on willingness to pay.

## Implications for Hyleon
1. **Store a verification status on each object:** design intent, field-verified (by whom and when), or redline pending. That supports issuing "record drawings"; a lawyer's advice relayed on a forum was to ["NEVER use the term 'as built'"](https://www.electrical-contractor.net/forums/ubbthreads.php/topics/128928.html) (2004). *Not covered yet:* bringing field redlines back into the model.
2. **Treat handoff as a feature.** Offer the full model, a deliverable with the intelligence stripped out to protect IP, DWG/DXF with mapped attributes, and open CSV/JSON for owners' maintenance and ERP systems. Owners carry most of the cost of data that doesn't transfer, so they might specify Hyleon outputs.
3. **Let packs carry presentation rules as well as limits:** schedule layouts that plan reviewers expect, owner title blocks, and tag and numbering conventions.
4. **Remove setup for occasional users.** Packs could ship with libraries and standards, importing a house standard could be a first-class task, and guided questions could serve people who draw a few panels a year. Agent mode has to beat configurators like Cogineer, not just hand drafting.
5. **Keep one inspectable model with no hidden second database.** Regenerate outputs the same way every time and show the source of every check. Confirm the need for offline use and local files before building them.
6. **Licensing that never traps data:** simple seats, permanent read and export access to your own projects, and a cheap review seat. Bluebeam sets a price anchor at $260–590 a year.
7. **Gaps to decide on:**
   - importing plant networks and IP/VLAN schedules (the EPA push);
   - syncing PLC and SCADA tags (one user asked about ["electrical drawings in Ignition Vision/Perspective windows"](https://forum.inductiveautomation.com/t/electrical-drawing-software/79881));
   - change-management records that link each change to sheets, as OSHA's process safety rules expect;
   - BOM and ERP export;
   - facility layouts, where I found no evidence.
8. **Don't sell on headline rework figures.**
   - FMI/Autodesk ([14% of avoidable rework, $1.8T in total](https://www.constructiondive.com/news/contractors-lost-18-trillion-globally-in-2020-due-to-bad-data-new-report/606939/)) and GIRI ([21% of turnover](https://www.designingbuildings.co.uk/wiki/The_Get_It_Right_Initiative)) are broad survey estimates, and the FMI one is vendor-sponsored.
   - A 2026 study of one contractor's actual costs found rework of [0.38–0.76% of contract value](https://www.asce.org/publications-and-news/civil-engineering-source/article/2026/01/22/how-much-does-field-rework-in-construction-actually-cost), though it also found rework was underreported by 300%.
   - Build in measurement of the issues Hyleon catches and the revisions it avoids.

## Sources
- **Studies and regulators:** [FMI/PlanGrid 2018](https://www.constructiondive.com/news/industry-could-be-overspending-177b-per-year-study-finds/529450/) · [FMI/Autodesk 2021](https://www.constructiondive.com/news/contractors-lost-18-trillion-globally-in-2020-due-to-bad-data-new-report/606939/) · [NIST GCR 04-867](https://www.nist.gov/publications/cost-analysis-inadequate-interoperability-us-capital-facilities-industry) and [ENR summary](https://www.enr.com/articles/30495-study-of-data-roadblocks-costs-tags-capital-facilities-industry-with-15-8-billion-loss-8-9-2004) · [OSHA inspection program slides, 2012](https://www.csb.gov/UserFiles/file/Barab%20%28OSHA%29%20PowerPoint.pdf) · [CII RS153-1](https://construction-institute.org/the-field-rework-index-early-warning-for-field-rework-and-cost-growth) · [ASCE/Love 2026](https://www.asce.org/publications-and-news/civil-engineering-source/article/2026/01/22/how-much-does-field-rework-in-construction-actually-cost) · [GIRI](https://www.designingbuildings.co.uk/wiki/The_Get_It_Right_Initiative) · [EPA 2024](https://www.epa.gov/system/files/documents/2024-07/maintain-updated-documentation.pdf)
- **Reviews:** TrustRadius [EPLAN](https://www.trustradius.com/products/eplan/reviews), [Revit](https://www.trustradius.com/products/revit/reviews), [Bluebeam](https://www.trustradius.com/products/bluebeam-revu/reviews), [Lucidchart](https://www.trustradius.com/products/lucidchart/reviews), [Visio](https://www.trustradius.com/products/visio/reviews) · [Capterra EPLAN](https://www.capterra.com/reviews/152595/Electric-P8) *(snippet)*
- **Forums:** [Inductive Automation](https://forum.inductiveautomation.com/t/electrical-drawing-software/79881) · MrPLC [page 1](https://mrplc.com/forums/topic/13720-any-users-of-eplan-p8/) and [page 2](https://mrplc.com/forums/topic/13720-any-users-of-eplan-p8/page/2/) · [electrical-contractor.net](https://www.electrical-contractor.net/forums/ubbthreads.php/topics/128928.html) · [3DEXPERIENCE](https://3dswym.3dexperience.3ds.com/question/solidworks-user-forum/solidworks-electrical-slowing-down_bT5Kb-YeRAqRZTU0TQwsDQ) · *snippets only:* [PLCtalk](https://www.plctalk.net/threads/what-tools-and-how-are-you-guys-designing-your-control-panels.128265/), Control.com [thread 1](https://control.com/forums/threads/autocad-electrical.31289/post-31289) and [thread 2](https://control.com/forums/threads/electrical-design-software.23115/page-3), [Revit Ideas](https://forums.autodesk.com/t5/revit-ideas/panel-schedule-demand-load-calculations/idi-p/8521479), [AutoCAD Electrical forum](https://forums.autodesk.com/t5/autocad-electrical-forum/wd-jumpers-and-wire-numbers-not-propagating/m-p/6919247)
- **Vendor and news:** [EPLAN help](https://www.eplan.help/en-us/infoportal/content/Plattform/2.9/Content/htm/xdxfgui_k_grundlagen.htm) · [Elecdes](https://help.elecdes.com/EBASE_USER/exportdwgs.htm) · [Siemens SCCR whitepaper](https://assets.new.siemens.com/siemens/assets/api/uuid:e38c994f-b2da-4846-ba00-f8e4733dfbde/ulsccr-whitepaper-icp-us.pdf) · [Bluebeam pricing](https://www.bluebeam.com/pricing/) · [Dezeen 2020](https://www.dezeen.com/2020/07/28/autodesk-revit-bim-software-criticism/) · [ISE Controls](https://isecontrols.com/?p=52589)

The biggest gaps are Reddit and G2/Capterra in full, plus anything on piping and facility layouts. Two ways to close them: a second pass once the search budget refills, or you paste in threads to analyze.
