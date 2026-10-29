# Appendix 5: Pre-mortem and analogue startups (agent report, 2026-10-28)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-28-blind-spots.md](../2026-10-28-blind-spots.md).

**The brief:** an evidence-based pre-mortem, covering:
- why comparable design-software companies succeeded or failed
- common failure modes
- liability incidents
- channels that worked
- whether professionals accept AI-assisted design

---

# Hyleon pre-mortem: evidence up to 2026-10-28

**Evidence labels:** **S** = study or survey. **R** = reported case (court record, filing, credible press, or a figure the company itself published). **A** = anecdote, opinion or vendor marketing. **I** = my own inference, with no source. The shared web-search budget ran out partway through, so anything I could not check is marked *unknown*.

## Analogues

| Company | Outcome | Key reason | Lesson for Hyleon |
|---|---|---|---|
| Onshape | Raised $169M; PTC bought it for $470M (2019) [1] R | About 5,000 customers and about $10M revenue at sale. This is a trade estimate; Onshape never disclosed revenue [2] A | Even a top team moved CAD users slowly; selling to an incumbent is the likely exit |
| Upverter | Altium bought it in 2017. Plan to merge it with CircuitMaker announced 2018, dropped 2019 [3] R. upverter.com no longer resolves (checked today) | *Unknown* | Tools get absorbed after acquisition; buyers fear their data getting stranded |
| CircuitMaker / Altium 365 | CircuitMaker is free, commercial use allowed [4] R. Renesas bought Altium for about US$5.9B (2024) [5] R | *Unknown* (I: the free tier feeds a paid core product) | A free tier pays off only if it leads into a paid product |
| Flux.ai | Alive; raised $37M (Feb 2026); 1.1M users and 6.4M projects [6] R | Now pitched as an AI "hardware engineer": "You steer, Flux executes" [6] A | "The human stays in control" is now a common pitch, not an advantage |
| PlanGrid | Autodesk paid $875M (2018), expecting about $100M ARR [7][8] R. Merged into Autodesk Build; the name was retired [7][9] R | Drawings on iPads for field crews [7] R | Adoption from field crews upward works; an acquirer may retire the product |
| Fieldwire | Hilti paid about $300M (2021); 4,000+ paying companies; Hilti had invested since 2017 [10] R | Free tier (5 users, 3 projects), then $39–89 per user per month [11] R | Free entry for trades; a manufacturer invests first and buys later |
| Procore | IPO 2021 (about $11B). Revenue $4.8M in 2012, $1.32B in 2025; still losing money [12] R | Unlimited users, including subcontractors and clients; price based on yearly construction volume [13] R | Don't charge for collaborators; expect it to take a decade |
| Bluebeam | Nemetschek paid about $100M (2014); revenue $16.5M in the first half of 2014 [14] R. Moved to per-person subscriptions in 2022–23 [15] R | *Unknown* (I: PDF markup fit how people already worked) | Build on file types people already exchange |
| Hypar | Pivoted from a generative-design / text-to-BIM platform to a space-planning tool (2025) [16] R | "Very, very hard to design anything real through a chat prompt" [16] A | Chat-only design doesn't deliver |
| TestFit | Independent; 6,200+ users; offers AI-generated plans and an MCP connection [17] A | *Unknown* (I: it does one narrow job, site feasibility) | Competitors already offer MCP connections, so it is no differentiator |
| Swapp | Alive; $18.5M raised [18] R | Runs inside Revit and ArchiCAD; onboarding for each engagement [19] A | Get in through the incumbents' files |
| Kreo | Now sells only AI takeoff and estimating [20] A. An older design product is listed elsewhere [21] A | *Unknown*; date *unknown* | *Unknown* |
| AI construction and engineering startups, 2023–26 | Builder.ai went insolvent in 2025 after reports that people did the "AI" work [22] R. Built Robotics narrowed to solar piling (2023) [23] R. Constru (2023) and syncx.ai (2026) closed (database listings, unverified) [24] A | Overclaiming (Builder.ai) [22]. *Unknown* for the others | Narrow the focus; never overstate what the AI does |

## Top 10 ways Hyleon fails

1. **Incumbents copy the pitch or give the feature away.** EPLAN calls its Copilot "a co-thinking partner, not … a decision-maker", which is Hyleon's own positioning [25] A. EPLAN also showed AI-generated panel layouts, a use case it calls conceptual [25][26] A. Eaton's fault-current calculator and Rockwell's SCCR tool are free [27][28] R. Since 2019, AutoCAD subscriptions include several industry toolsets [29] R; I believe Electrical is one of them but could not confirm it. *Mitigation:* own one job incumbents do badly, such as a sourced SCCR / NEC 409 record for UL 508A shops. Stay vendor-neutral and export into EPLAN and AutoCAD rather than trying to replace them.

2. **Locked out of the file flow.** DWG is proprietary. Autodesk sued the Open Design Alliance in 2006, and licenses its DWG library only to products that don't compete with it [30] R. Swapp got in by working inside Revit [19]. *Mitigation:* from day one, support DXF/DWG (through the Open Design Alliance), PDF, and BOM and wire-list CSV files; publish Hyleon's model format.

3. **Content packs cost more than they earn.** NFPA, ASTM and ASHRAE sued Public.Resource.Org from 2013 to 2023 for posting their standards online for free. Fair use covered only standards incorporated into law [31] R. ICC now resells an AI plan checker built on its own code data [32] R, so the owners of the content also control channels. Cost of licensed-engineer (PE) review: *unknown*. *Mitigation:* cite clause numbers instead of reproducing text; license content where needed; ship one pack first; pay reviewers and publish versioned corrections.

4. **Professionals don't trust the outputs.**
   - Autodesk's 2025 survey of 5,594 people: trust in AI fell 11 points to 65% [33] S.
   - A 2026 ServiceTitan survey of 1,017 US contractors: 64% of electrical contractors use AI. Across all trades, only 11% trust it highly and 58% trust it little or not at all; accuracy is their top concern [34] S.
   - RICS 2025, 2,200+ construction professionals: 45% report no AI use in their organisation [35] S.

   *Mitigation:* keep the rule-based checks separate from the AI; show the source and edition for every value; publish worked validation cases.

5. **An engineer stamps output nobody reviewed.** In NSPE ethics case 24-2, an engineer sealed AI-drafted design documents after a quick look. They had misaligned dimensions and were missing required safety features. The board ruled the engineer failed to keep "responsible charge" [36] R. The 1978 Hartford Civic Center roof collapse was blamed on "unprecedented use of and trust in computer analysis", with no independent review [37] R. *Mitigation:* show changes side by side, require sign-off per decision, offer an independent-check mode; never apply a seal automatically.

6. **"Never blocks" turns into "always ignored".** NASA lost the Mars Climate Orbiter ($327.6M) to a units error that navigators had noticed. Their concerns were dismissed because they hadn't filled out the right form [38] R. Logs of overridden warnings could also become evidence against customers (I). *Mitigation:* few warnings, each worth reading; severity levels; a required reason to override a critical check; customers control how long records are kept (needs legal advice).

7. **Liability reaches the vendor.** A tribunal held Air Canada "fully responsible" for its chatbot's wrong answer [39] R. From 9 December 2026, EU law treats all software, including AI, as a product. It covers injury, property and data harm to individuals, but not harm to companies [40] R. *Mitigation:* see the liability lessons below.

8. **Revenue grows slower than the money lasts.** Procore had $4.8M in revenue after ten years [12]. Onshape had about $10M after seven years and $169M raised [1][2]. Sales-cycle length for panel shops and integrators: *unknown*. *Mitigation:* a self-serve Pro plan with a free tier; delay enterprise sales; sell pack reviews or training for early cash.

9. **Per-seat pricing clashes with per-project billing.** Procore prices by construction volume with unlimited users [13]. Inductive Automation sells its Ignition software per server and markets it as "unlimited licensing" [41] R. Fieldwire charges per seat after a free tier [11]. How integrators prefer to pay: *unknown*. *Mitigation:* free seats for reviewers (PEs, inspectors, end clients); test per-project or per-panel pricing alongside seats.

10. **Too broad for one founder, and buyers fear abandonment.** Hypar and Built Robotics both narrowed to one job [16][23] R; that this is why they survived is not proven. Autodesk ended its EAGLE PCB software on 7 June 2026 and moved users to Fusion [42] R. Evidence on solo founders is mixed. Paul Graham calls it a top mistake [43] A. A study by Greenberg and Mollick found solo-founded for-profit ventures about 2.5 times as likely to survive (Kickstarter sample) [44] S. *Mitigation:* one domain (panels) until customers keep paying; open export and source-code escrow; a network of reviewers.

## Professional acceptance, 2024–2026

- **Engineers:** ASCE policy statement 573 (2024) says AI "cannot serve as a replacement" for a licensed engineer's judgement [45] R. NSPE's position statement 03-1774 (revised February 2026) calls for "rigorous verification & validation" [46] R.
- **Electricians and contractors:** they use AI heavily but trust it little [34] S. Trust figures for electricians alone: *unknown*.
- **Inspectors:** the code body ICC partnered with Archistar, an AI plan-check company, after testing it in 11 cities (July 2025) [32] R. LA County ran a pilot of AI zoning pre-checks for rebuilding after the fires [47] R. Surveys of inspectors: *unknown*.
- **Software developers, as a proxy:** 46% distrust the accuracy of AI tools and 33% trust it [48] S.

## Channel lessons

- **Integrator programs work.** Inductive Automation built a network of 5,500+ integrator firms using free trials and free training (30,000+ people credentialed). It says 69% of the Fortune 100 use its software [49] R.
- **Manufacturers give tools away to sell hardware.** Examples: Eaton and Rockwell [27][28], AutomationDirect's free PLC software [50] R, and EasyEDA from PCB maker JLCPCB [51] R. Rittal and EPLAN belong to the same group [52] R, and Hilti bought Fieldwire [10]. These firms are free competitors and likely buyers. A vendor-neutral record that hands BOMs to distributors may be the way to partner with them (I). Schneider and ABB tools: not checked.
- **CSIA (control-system integrators association):** "hundreds of members". Vendor members pay for directory listing, sponsorship, media and podcast access [53] R. CSIA certification covers project, quality and cybersecurity practices [54] R. Results for vendors: *unknown*.
- **Trade shows:** incumbents launch their AI features there [26]. Return on investment: *unknown*.
- **Community:** Flux reached 1.1M users with a free browser tool [6]. Onshape has been free for education since 2016 [1].

## Liability lessons

- **Software errors become disasters when nobody checks independently.** The Sleipner A platform (1991): the NASTRAN analysis underestimated shear stress by 47%, causing a loss of about $700M. A later, careful analysis predicted failure at 62 m depth; it actually failed at 65 m [55] R.
- **Disclaimers between businesses usually hold for financial loss, even when the vendor knew about the bug.** In *Mortenson v. Timberline* (Washington Supreme Court, 2000), estimating software caused a $1.95M underbid. A 1993 internal memo had described the bug, and a fixed version had gone to some customers. The court still enforced the license's limit on damages [56][57] R. Liability for personal injury generally can't be disclaimed [57] A.
- **Data can count as a "product".** US courts treated Jeppesen's aeronautical charts as products. *Winter v. G.P. Putnam's Sons* (9th Cir. 1991) contrasted charts with books and suggested software could also count. I cite these from memory and could not re-check them. Hyleon's sourced packs look more like charts than books (I).
- **What the AI says is the company's statement** [39], and the engineer who seals the drawing stays responsible [36].
- **What vendors did afterwards:** the responses I found are contractual: limits on liability in licenses [57], and end-of-support disclaimers. Bluebeam says using unsupported Revu 20 is "at the customer's own risk" [58] R. Changes by software vendors after Sleipner or Hartford: *unknown*.
- **For Hyleon:**
  - A license that limits liability.
  - Notify *all* users when a defect is known.
  - Versioned packs with published corrections.
  - Say "checked against clause X, edition Y", never "compliant".
  - Professional liability (E&O) and product-liability insurance; cost *unknown*.
  - Record who decided what, and what the AI proposed.

## Sources
1. [Onshape – Wikipedia](https://en.wikipedia.org/wiki/Onshape)
2. [GfxSpeak on PTC/Onshape](https://gfxspeak.com/newswatch/says-look-growth/)
3. [Upverter – Wikipedia](https://en.wikipedia.org/wiki/Upverter)
4. [Altium CircuitMaker](https://www.altium.com/circuitmaker)
5. [Renesas completes Altium acquisition](https://www.altium.com/company/newsroom/press-releases/renesas-completes-acquisition-altium)
6. [Flux: $37M](https://www.flux.ai/blog/we-raised-37m-to-take-the-hard-out-of-hardware)
7. [PlanGrid – Wikipedia](https://en.wikipedia.org/wiki/PlanGrid)
8. [BD+C: Autodesk–PlanGrid](https://www.bdcnetwork.com/building-technology/news/55162211/autodesk-is-spending-115-billion-to-acquire-two-construction-tech-providers)
9. [PlanGrid into Autodesk Build](https://groundbreakcarolinas.com/?p=16182)
10. [ENR: Hilti–Fieldwire](https://enr.com/articles/53015-hilti-acquires-startup-fieldwire-in-300m-deal)
11. [Fieldwire pricing](https://www.fieldwire.com/pricing/)
12. [Procore – Wikipedia](https://en.wikipedia.org/wiki/Procore)
13. [Procore pricing](https://www.procore.com/pricing)
14. [Nemetschek–Bluebeam](https://schnitgercorp.com/2014/10/03/nemetschek-acquires-bluebeam-for-100-million/)
15. [Bluebeam licensing change](https://www.arkance.us/blog/bluebeam-end-of-perpetual-licenses-and-other-important-updates)
16. [AEC Magazine: Hypar 2.0](https://aecmag.com/features/hypar-2-0/)
17. [TestFit](https://www.testfit.io/)
18. [Swapp Series A](https://swapp.ai/news/swapp-raises-11-5-million-series-a-to-help-architects-with-ai-platform/)
19. [Swapp](https://swapp.ai/)
20. [Kreo](https://www.kreo.net/)
21. [Kreo on Autodesk Marketplace](https://marketplace.autodesk.com/publisher-profile/ab8359b1-4028-41bf-9143-dda73eac0501)
22. [Builder.ai – Wikipedia](https://en.wikipedia.org/wiki/Builder.ai)
23. [Built Robotics – Wikipedia](https://en.wikipedia.org/wiki/Built_Robotics)
24. Start-Up Nation Finder: [Constru](https://finder.startupnationcentral.org/company_page/constru), [syncx.ai](https://finder.startupnationcentral.org/company_page/syncx-ai)
25. [EPLAN Copilot](https://www.eplan.com/us-en/about-us/innovations/ai-driven-industrial-automation/eplan-copilot/)
26. [Computer&Automation: EPLAN AI](https://www.computer-automation.de/engineering-en/industrial-ai-at-sps---eplan--ai-reduces-the-planning-time-.htm?thema=engineering)
27. [Eaton SCCR tools](https://www.eaton.com/sccr)
28. [Rockwell free SCCR tool](https://www.rockwellautomation.com/en-ca/company/news/press-releases/Selection-Tool-Helps-Panel-Builders-Meet-UL-Electrical-Standards.html)
29. [AutoCAD – Wikipedia](https://en.wikipedia.org/wiki/AutoCAD)
30. [.dwg – Wikipedia](https://en.wikipedia.org/wiki/.dwg)
31. [Public.Resource.Org – Wikipedia](https://en.wikipedia.org/wiki/Public.Resource.Org)
32. [ICC–Archistar](https://www.iccsafe.org/about/periodicals-and-newsroom/international-code-council-collaborates-with-archistar-to-modernize-permitting-and-accelerate-housing-development)
33. [Autodesk State of Design & Make 2025](https://adsknews.autodesk.com/en/news/2025-state-of-design-and-make/)
34. [ServiceTitan 2026 trades survey](https://distributionstrategy.com/2026/09/contractors-ramp-up-ai-use-but-profit-gains-lag/)
35. [RICS AI in Construction 2025](https://www.rics.org/news-insights/artificial-intelligence-in-construction-report)
36. [NSPE BER Case 24-2](https://www.nspe.org/career-growth/ethics/board-ethical-review-cases/use-artificial-intelligence-engineering-practice)
37. [Hartford Civic Center – Wikipedia](https://en.wikipedia.org/wiki/Hartford_Civic_Center)
38. [Mars Climate Orbiter – Wikipedia](https://en.wikipedia.org/wiki/Mars_Climate_Orbiter)
39. [Moffatt v. Air Canada – Wikipedia](https://en.wikipedia.org/wiki/Moffatt_v._Air_Canada)
40. [EC: defective-products liability](https://single-market-economy.ec.europa.eu/single-market/goods/free-movement-sectors/liability-defective-products_en)
41. [Ignition pricing](https://inductiveautomation.com/pricing/ignition)
42. [EAGLE – Wikipedia](https://en.wikipedia.org/wiki/EAGLE_(program))
43. [Graham: 18 Mistakes](https://paulgraham.com/startupmistakes.html)
44. [NYU Stern: solo founders](https://www.stern.nyu.edu/experience-stern/faculty-research/venture-investors-take-note-solo-founders-outperform-teams)
45. [ASCE PS 573](https://www.asce.org/advocacy/policy-statements/ps573---artificial-intelligence-and-engineering-responsibility)
46. [NSPE PS 03-1774](https://nspe.org/nspe-advocacy/explore-issues/professional-policies-and-position-statements/artificial-intelligence)
47. [LA County eCheck](https://recovery.lacounty.gov/2025/07/15/la-county-launches-echeck-ai-pilot-as-part-of-express-lane-for-faster-rebuilding/)
48. [Stack Overflow 2025: AI](https://survey.stackoverflow.co/2025/ai)
49. [Inductive Automation](https://inductiveautomation.com/about/)
50. [AutomationDirect CLICK](https://www.automationdirect.com/clickplcs)
51. [EasyEDA – Wikipedia](https://en.wikipedia.org/wiki/EasyEDA)
52. [Friedhelm Loh Group – Wikipedia](https://en.wikipedia.org/wiki/Friedhelm_Loh_Group)
53. [CSIA](https://www.controlsys.org/about)
54. [CSIA certification](https://www.controlsys.org/certification)
55. [Sleipner A (UMN)](https://www-users.cse.umn.edu/~arnold/disasters/sleipner.html)
56. [Mortenson v. Timberline (FindLaw)](https://caselaw.findlaw.com/wa-supreme-court/1409490.html)
57. [ENR 2001: liability disclaimers](https://www.enr.com/articles/29869-liability-disclaimers-lead-way-as-digital-age-issues-weigh-in)
58. [Bluebeam Revu 20 end of life](https://support.bluebeam.com/revu/resources/revu-20-eol.html)

**Still unknown:**
- Number of UL 508A panel shops.
- How integrators want to pay, and how long their sales cycles are.
- Return on trade shows.
- Surveys of inspectors.
- How customers reacted to Bluebeam's subscription switch.
- The outcome of ICC's lawsuit against UpCodes.

Sources [8], [24], [26] and [27] were read only through search-result summaries; I could not open the pages themselves.
