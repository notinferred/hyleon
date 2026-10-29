# Appendix 4: Legal, professional and format constraints (agent report, 2026-10-28)

The research agent's full report, verbatim. It's research, not legal advice, and it's never edited. The synthesis is in [../2026-10-28-market.md](../2026-10-28-market.md).

**The brief:**
- code text and copyright
- PE sealing and AI guidance
- UL 508A
- file-format and data-standard licensing
- Apple's font, symbol and guidelines terms

---

# Hyleon: legal, professional and licensing constraints

As of 2026-10-28. This is a research summary, not legal advice. [n] points to the source list. Lines marked "Analysis" are my conclusions, not statements from a source.

## 1. Code text and copyright

**What the standards bodies allow**
- **NFPA (US, 2026):**
  - The free NEC access requires an account, is read-only, and can't be downloaded or printed [1].
  - Copying needs NFPA's permission, and NFPA does not allow its content on other websites [2].
  - I found no public licensing program for software; licences are negotiated case by case. ImageTrend embeds licensed NFPA content at per-user fees ($462/yr for NFPA 1) [3]. NFPA's lawsuit against UpCodes settled in 2025 with a licence whose terms weren't disclosed [4].
- **ULSE / UL 508A (US, policy dated July 2026):**
  - Excerpts need a permission form.
  - Derivative works are barred, "including … large language models and Generative AI."
  - Training AI on ULSE content, or submitting it to any AI tool, needs express permission [5].
  - A purchased copy gives a personal licence: no copying, no derivatives, no putting it into other documents [6].
  - The free "Digital View" lets anyone read the standards online, view-only [7].
- **IEC (Swiss law, Webstore terms of 2025-08-04):**
  - Purchased publications may not be used "for the development of any algorithm, digital model, software program, or AI tool," or data-mined, without express permission [8].
  - IEC databases are protected by copyright and database rights [9].

**Court decisions and legislation**
- **US, D.C. Circuit, ASTM v. Public.Resource.Org (Sept 12, 2023):**
  - Held that "non-commercial dissemination of such standards, as incorporated by reference into law, constitutes fair use."
  - NFPA was a plaintiff. Posting standards that were *not* incorporated into law was infringement [10].
  - The court's 2018 decision left open whether incorporated standards keep their copyright. It suggested that for non-binding material "a paraphrase or a summary" may be enough [11].
- **US, 3rd Circuit, ASTM v. UpCodes, 172 F.4th 253 (Apr 7, 2026):** A for-profit company posting incorporated standards for free is likely fair use. This was a preliminary ruling, and the court called the market harm "equivocal" [12].
- **US Supreme Court, Georgia v. Public.Resource.Org (Apr 27, 2020):** The rule that the law can't be copyrighted turns on whether a judge or legislator wrote the work. Privately written standards weren't addressed [13].
- **US appeals courts disagree:** the 1st and 5th Circuits (including Veeck, 2002) on one side, the 2nd and 9th (including CCC, 1994) on the other. Whether incorporated standards keep copyright is **unclear** [14].
- **US Pro Codes Act (pending):**
  - Standards bodies would keep copyright if incorporated standards are free to read online (H.R. 4009, June 13, 2025 [15]; S. 4145, March 2026 [16]).
  - An October 2026 coalition letter, signed by NFPA, says a revised House text awaits committee [17].
- **EU, Court of Justice, C-588/21 P (Mar 5, 2024):** Harmonised standards "form part of EU law," and an overriding public interest justified access to them. The case is about access to documents; it gives no licence to republish [18].

**Citing clause numbers, paraphrasing, using table values**
- US law: copyright does not cover ideas, procedures or systems (17 U.S.C. §102(b)) [19]. Facts are free to use; a compilation is protected only in its original selection and arrangement (Feist, 1991) [20].
- Analysis:
  - Citing clause numbers is ordinary citation and low risk.
  - Rewriting a requirement as your own check logic is low risk.
  - Close paraphrase of whole clauses, notes, figures or handbook commentary carries more risk.
  - Single table values (e.g. ampacities) used as inputs are arguably facts [20]. Copying whole tables risks the protected selection and arrangement, and the CCC court protected values based on expert judgment [14]. This is **unclear**.
  - The fair-use decisions cover standards incorporated into law, such as the NEC where a state adopts it. UL 508A appears in the NEC only as an example in an informational note [38], and IEC texts are not US law, so that defence is weaker for them.
  - Purchase and licence terms [5][6][8] can forbid uses that fair use would allow.

## 2. Professional engineering

- **NCEES Model Law §110.20 E (Aug 2025):**
  - "Responsible charge" means "full professional knowledge of and control over work." The licensee must have the authority to change, reject or approve the work, both in progress and final. They must know the project's scope and limits personally, be able to answer for its decisions, and take full responsibility [21].
  - The January 2024 version said "direct control and personal supervision" [22].
  - The Model Law binds only in states that adopt it.
- **NCEES Model Rules 240.20 (Aug 2025):**
  - Final drawings and calculations given to a client or a public agency must be sealed, signed and dated.
  - Unsealed working documents must say "PRELIMINARY, NOT FOR CONSTRUCTION, RECORDING PURPOSES, OR IMPLEMENTATION."
  - A licensee may seal only work under their responsible charge and within their competence, whether the work is done remotely or locally [23].
- **Guidance on AI:**
  - NSPE Position Statement 03-1774 (Sept 2023, revised Feb 2026) calls for rigorous verification, validation and transparency [24].
  - NCEES adopted Position Statement 6.10, on responsible AI use, in 2025 [25]. Idaho circulated it in January 2026 but has not made it law [26].
  - North Carolina's board guidelines (July 16, 2025) require the licensee to "be capable of reproducing the output independently from any AI program" and to verify inputs. The licensee must also keep records of inputs, methods, assessments of the AI tool, and validation [27].
  - Florida's board director wrote in 2025 that there is no AI rule yet, and that stamping without "meaningful oversight" is a legal and ethical risk [28].
- **Do contractors and panel shops need a PE? Examples:**
  - **California, B&P Code §6737.3 (in force since 2004):** Contractors may design systems they install within their licence class, and draw shop and field drawings for their own contracts. They may not design work that others install [29].
  - **Florida, §471.003(2)(h) (2025):** Electrical contractors may design what they install up to $125,000 and 600 A residential or 800 A commercial at 240 V. Section (2)(c) exempts employees designing their employer's manufactured products [30].
  - **Texas, Occupations Code §1001.057 (2003):** Manufacturers are exempt for their own products, but a PE or the board may still require sealed plans [31]. Board opinion EAOR #34 (May 21, 2014) says consultants doing engineering for industrial clients need a Texas PE seal [32].
  - Whether UL 508A shop drawings themselves need a seal: I found no primary source. It is **unclear** and depends on the state and the local inspection authority.
- **How engineering software vendors limit liability:**
  - **Autodesk's General Terms:** the customer is "solely responsible" for use and output, and for independently testing "reliability, safety, accuracy." Liability is capped at the greater of the prior year's fees or US$1,000 [33].
  - **ENERCALC:** "Only experienced and licensed professional engineers should use this software" [34].
  - **EPLAN (German law):** EPLAN is liable only where it is at fault [35]. German law bars standard terms from excluding liability for injury or gross negligence (BGB §309 no. 7) [36].
  - **ETAP:** the licence is an exhibit to AVEVA's June 2026 schedule and isn't publicly readable [37].
  - **SKM:** I didn't find its licence.

## 3. UL 508A

- **NEC 409.110 (US, as described by Siemens in 2013 and 2018):** Industrial control panels must be marked with a short-circuit current rating (SCCR). The rating comes either from a listed assembly or from an "approved method." An informational note names UL 508A Supplement SB as such a method [38][39].
- **The Supplement SB4 method, per Siemens's 2018 summary [39]:**
  1. Rate every component in the power circuit, using its marked value or the default in Table SB4.1.
  2. Optionally, take credit for current-limiting devices in the feeder:
     - fuses only through the let-through values in Table SB4.2;
     - circuit breakers through the maker's peak let-through curves;
     - transformers through tables or an assumed 2.1% impedance.
  3. The panel's SCCR is the lowest rating that remains.

  Table numbers may differ in the current edition.
- **What a listed panel shop must have:**
  - Training, and at least one UL 508A-qualified technical representative (MTR) per site [40].
  - A procedure document UL writes for each shop, an initial production inspection, then ongoing follow-up inspections.
  - Components that are UL Listed or Recognized and acceptable under Table SA1.1.
  - The UL mark covers only parts installed at the factory (UL FAQ, 2016) [41].
- **Software tools:**
  - UL's program page lists training and an SCCR guide but no calculator [40].
  - Eaton's Bussmann OSCAR (subscription) calculates SCCR and produces documentation for "2014 NEC and UL 508A Supplement SB."
  - Eaton's free FC2 calculates available fault current and makes NEC 110.24 labels [42].

## 4. File formats and data standards

- **DWG (US):**
  - Autodesk's proprietary format; the Open Design Alliance (ODA) reverse-engineered it.
  - Autodesk sued ODA in 2006 over the TrustedDWG watermark. The case settled in 2007, and ODA stopped writing the watermark.
  - The US trademark office refused "DWG" as a trademark (2011 to 2014, including a federal court loss for Autodesk). Yet ODA and SolidWorks dropped DWG-based names in 2010 settlements, and Autodesk still claims the mark. The trademark status is **unclear**.
  - Writing DWG files is lawful. The open-source LibreDWG library (GPLv3) writes the R2000 version [43].
- **Autodesk RealDWG:** $8,000 or €7,500 a year for up to 10,000 users. Windows desktop only; "not permissible to host a RealDWG application on a server" [44].
- **ODA membership (Wikipedia, uncited):** the Commercial tier allows up to 100 copies and no web or SaaS use. Sustaining adds web and SaaS, and Founding adds source code. I could not verify fees [45].
- **DXF:** Autodesk's DXF Reference (Feb 2011) is written for people who "write your own programs to process DXF files." The document is copyrighted and provided "as is," and it states no fee or licence for implementing the format [46].
- **PDF/A (ISO 19005):**
  - Part 1 (2005) is based on PDF 1.4 and Part 2 (2011) on PDF 1.7 [47].
  - Part 3 (2012) adds embedded files of any format. Each attachment needs a declared relationship to the document (AFRelationship) and a file type [48].
  - Part 4 (2020) is based on PDF 2.0. Its "4f" profile allows embedded files, and its "4e" profile covers engineering and 3D content [47][49].
- **PDF/E (ISO 24517-1:2008):** Little used, and work on PDF/E-2 stalled. The Library of Congress (2025) says "functionally, PDF/E is replaced by PDF/A-4e" [50].
- **DEXPI:** Specifications 2.0 and 2.0.1 (Sept 30, 2026) are published under a Creative Commons licence that allows use, sharing and adaptation. The page doesn't say which CC variant [51]. Membership terms aren't published.
- **AutomationML:**
  - The standard is IEC 62714 (e.g. part 1, 2018), so its text falls under IEC's terms [52][8].
  - The AMLEngine library is MIT-licensed [53].
  - The association's website content is licensed for private, non-commercial use only [54]. Members get early access and agree not to sue each other [55].
  - The licence for the specification documents themselves is **unclear**.
- **ETIM:** The classification is under the Open Data Commons Attribution licence 1.0, "free to use for everyone." Its JSON exchange format, ETIM xChange, is Apache 2.0 [56], and ETIM now recommends it over BMEcat [57].
- **BMEcat (Germany):** BME grants a royalty-free right to use and redistribute the specification with its copyright notice. Implementing it in software needs a separate, also royalty-free, licence from BME. The current version is 2005.2 [58][59].

## 5. Apple

- **SF Pro and the other SF fonts (licence on Apple's font page):**
  - "SOLELY FOR CREATING MOCK-UPS OF USER INTERFACES" for software on Apple's operating systems.
  - No mock-ups for other operating systems, no embedding, no "website content" [60].
  - Naming the system font in CSS, without shipping the font file, isn't addressed. It is **unclear**.
- **SF Symbols:** Under §2.10 of the Xcode and Apple SDKs Agreement (EA2002, 06/08/2026), the symbols may be used only for "developing Applications for Apple-branded products that run on the system for which the image was provided." Never in app icons, logos or trademarks [61].
- **Human Interface Guidelines text:**
  - Apple's website terms (Nov 20, 2009) bar copying or republishing without written approval, and say nothing about links [62].
  - Copyright doesn't protect ideas [19], so paraphrasing the guidance in your own words and linking to it is likely fine (analysis).
  - Apple's trademark guidelines allow Apple product names in phrases like "runs on" or "compatible with" [63].

## Implications for the product

1. **Packs should hold pointers and your own logic only.** Store clause IDs, edition, jurisdiction and checks written in your own words. Include no standard text, figures, notes or whole tables, and link to NFPA's and ULSE's free readers. This stays safe even if the Pro Codes Act passes.
2. **Get licences before encoding IEC or UL rules, and ask NFPA as well.** IEC's and ULSE's terms forbid building software or AI from their documents without permission, whatever fair use allows. For tables, either license them or have users enter values from their own copies.
3. **Keep standards text out of the AI.** No standards text in prompts, retrieval indexes or fine-tuning, and block users from uploading standards PDFs into AI features.
4. **Support responsible charge, and never certify.**
   - Mark unsealed sheets "PRELIMINARY, NOT FOR CONSTRUCTION…" by default.
   - Make checks deterministic, and log inputs, rule versions, AI suggestions and human approvals so a PE can reproduce the result.
   - If a licensee seals a drawing, they apply their own seal.
   - Show state-by-state notes, not legal determinations.
5. **UL 508A:** Present Hyleon as an aid for the SB calculation and its documentation. Component ratings come from UL's Product iQ database or maker data, and the shop's MTR makes the decision. Name "UL 508A" but never use UL marks.
6. **Licence terms (EULA):** Follow the industry pattern: Hyleon is a tool, not a professional service; the user verifies results; liability is capped. Have counsel adapt it for each market, since German law limits what can be excluded.
7. **File formats:**
   - Write DXF natively.
   - DWG in the web app needs an ODA tier that covers SaaS, because RealDWG can't run on servers. Use "DWG" only to describe the file format, not in product names.
   - Embed source data (DXF, AutomationML, DEXPI XML, check reports) in PDF/A-3 or PDF/A-4f, and skip PDF/E.
   - Credit ETIM and DEXPI as their licences require, and get BME's implementation licence for BMEcat.
8. **Apple:**
   - Don't ship SF Pro or SF Symbols in the web or non-Apple builds, and never use them in icons.
   - Native Apple builds may use them through the system's own interfaces.
   - Use open-licensed fonts and icons elsewhere, and paraphrase and link the guidelines rather than copying them.

**Not verified:** the web-search budget ran out partway through, and some sites blocked automated fetching. Unconfirmed items:
- ODA fees
- ETAP and SKM licence text
- the full NEC 409.110 nameplate list
- the full text of NCEES Position Statement 6.10
- current UL 508A table numbers

I read Autodesk's and NFPA's terms through search-index excerpts, not the live pages.

## Sources
1. NFPA free access FAQ: https://www.nfpa.org/customer-support/how-can-i-access-nfpa-codes-and-standards-for-free
2. NFPA Terms of Use: https://www.nfpa.org/customer-support/terms-of-use
3. ImageTrend NFPA licensing: https://www.imagetrend.com/nfpa-codes/
4. NFPA–UpCodes settlement: https://www.nfpa.org/about-nfpa/press-room/news-releases/2025/nfpa-announces-settlement-of-litigation-against-upcodes ; https://internationalfireandsafetyjournal.com/nfpa-and-upcodes-settle-copyright-lawsuit-with-licensing-agreement/
5. ULSE copyright policy (Jul 2026): https://ulse.org/wp-content/uploads/2026/07/ULSE-External-Copyright-Policy-Jul-2026.pdf
6. ShopULStandards terms: https://shopulstandards.com/TermsService.aspx
7. ULSE Digital View: https://ulse.org/wp-content/uploads/2025/05/UL_Standards__Engagement_Digital_View_Functionality_Overview.pdf
8. IEC Webstore terms: https://webstore.iec.ch/en/terms-conditions
9. IEC copyright: https://webstore.iec.ch/en/copyright
10. ASTM v. PRO (D.C. Cir. 2023): https://law.resource.org/pub/us/cfr/regulations.gov.astm.appeal2/D.C.%20Cir.%2022-07063%20dckt%20_000%20filed%202023-09-12.2.pdf
11. ASTM v. PRO (D.C. Cir. 2018): https://caselaw.findlaw.com/court/us-dc-circuit/1943651.html
12. ASTM v. UpCodes (3d Cir. 2026): https://www.copyright.gov/fair-use/summaries/ASTM-v-UpCodes-Inc-172-F-4th-253-3d-Cir-2026.pdf ; date: https://www.lawnext.com/2026/04/third-circuit-rules-that-upcodes-publication-of-incorporated-building-standards-is-likely-fair-use.html
13. Georgia v. PRO: https://www.law.cornell.edu/supremecourt/text/18-1150
14. CRS R47656 (Sept 26, 2023): https://www.everycrsreport.com/files/2023-09-26_R47656_06eea6d3ebb0acb711bfcf09a27e744b1579e96e.html
15. H.R. 4009: https://www.govinfo.gov/content/pkg/BILLS-119hr4009ih/html/BILLS-119hr4009ih.htm
16. S. 4145: https://www.govinfo.gov/metadata/pkg/BILLS-119s4145is/mods.xml
17. Coalition letter (Oct 2026): https://www.ashrae.org/file%20library/about/government%20affairs/public%20policy%20resources/hjc-pro-codes-act-coalition-october-2026-final.pdf
18. CJEU press release 41/24: https://law.resource.org/pub/eu/docket/2024-03-05.CJEU%20Press%20Relase.pdf
19. 17 U.S.C. §102: https://www.law.cornell.edu/uscode/text/17/102
20. Feist v. Rural: https://www.law.cornell.edu/supremecourt/text/499/340
21. NCEES Model Law (Aug 2025): https://ncees.org/wp-content/uploads/2025/09/Model-Law_August-2025_web.pdf
22. NCEES Model Law (Jan 2024): https://ncees.org/wp-content/uploads/2024/01/Model_Law_January-2024_web.pdf
23. NCEES Model Rules (Aug 2025): https://ncees.org/wp-content/uploads/2025/09/Model_Rules_August-2025_web.pdf
24. NSPE PS 03-1774: https://nspe.org/nspe-advocacy/explore-issues/professional-policies-and-position-statements/artificial-intelligence
25. NCEES 2025 annual report: https://ncees.org/wp-content/uploads/2026/03/2025-annual-report_access.pdf
26. Idaho board AI letter: https://dopl.idaho.gov/wp-content/uploads/2026/01/IPELS-AI-Statement.pdf
27. NC board AI guidelines: https://www.ncbels.org/wp-content/uploads/2025/07/Guidelines-on-the-Use-of-Artificial-Intelligence-in-Engineering-and-Surveying-Services-FINAL.pdf
28. FBPE "Regulation in the AI Era": https://fbpe.org/?p=57954
29. Cal. B&P §6737.3: https://california.public.law/codes/business_and_professions_code_section_6737.3
30. Fla. Stat. §471.003: https://www.flsenate.gov/Laws/Statutes/2025/471.003
31. Tex. Occ. Code §1001.057: https://texas.public.law/statutes/tex._occ._code_section_1001.057
32. TBPELS EAOR #34: https://www.pels.texas.gov/nm/EAOR34response.pdf
33. Autodesk General Terms: https://www.autodesk.com/company/terms-of-use/en/general-terms
34. ENERCALC licence: https://media.enercalc.com/Earth_Help/i_license_agreement.htm
35. EPLAN Cloud terms (08-2022): https://www.eplan.com/content/dam/eplan/corporate/legal/terms-of-use/eplan-cloud-terms-of-use-08-2022-de-en.pdf
36. BGB (English): https://www.gesetze-im-internet.de/englisch_bgb/englisch_bgb.html
37. AVEVA ETAP schedule (Jun 2026): https://www.aveva.com/content/dam/aveva/documents/legal/policies/AVEVA-Software-Schedule-ETAP-Products-v1-1-June-2026.pdf
38. Siemens SCCR manual (2013): https://docs.rs-online.com/0f0e/A700000014361036.pdf
39. Siemens SCCR white paper (2018): https://assets.new.siemens.com/siemens/assets/api/uuid:e38c994f-b2da-4846-ba00-f8e4733dfbde/ulsccr-whitepaper-icp-us.pdf
40. UL panel shop program: https://www.ul.com/services/industrial-control-panels-and-panel-shop-program
41. UL ICP FAQs (2016): https://code-authorities.ul.com/wp-content/uploads/sites/2/2016/10/ICP-FAQs-101716.pdf
42. Eaton SCCR tools: https://automotive.eaton.com/in/en-us/products/electrical-circuit-protection/fuses/sccr-made-easy.html
43. DWG history (Wikipedia): https://en.wikipedia.org/wiki/.dwg
44. RealDWG licensing: https://www.techsoft3d.com/products/realdwg/
45. ODA (Wikipedia): https://en.wikipedia.org/wiki/Open_Design_Alliance
46. Autodesk DXF Reference (2011): https://images.autodesk.com/adsk/files/autocad_2012_pdf_dxf-reference_enu.pdf
47. LoC PDF/A family: https://www.loc.gov/preservation/digital/formats/fdd/fdd000318.shtml
48. LoC PDF/A-3: https://www.loc.gov/preservation/digital/formats/fdd/fdd000360.shtml
49. LoC PDF/A-4: https://www.loc.gov/preservation/digital/formats/fdd/fdd000532.shtml
50. LoC PDF/E-1: https://www.loc.gov/preservation/digital/formats/fdd/fdd000495.shtml
51. DEXPI specifications: https://dexpi.org/specifications/ ; https://dexpi.org/
52. AutomationML overview: https://www.automationml.org/about-automationml/
53. AMLEngine (MIT): https://github.com/AutomationML/AMLEngine2.1
54. AutomationML legal notice: https://www.automationml.org/legal-notice/
55. AutomationML membership: https://www.automationml.org/organisation/join-us/
56. ETIM licence: https://www.etim-international.com/classification/license-info/
57. ETIM exchange formats: https://www.etim-international.com/classification/exchange-format/
58. BMEcat legal notice: https://www.bme.de/services/bmecat/rechtlicher-hinweis/
59. BMEcat overview: https://www.bme.de/initiativen/bmecat/
60. Apple fonts: https://developer.apple.com/fonts/
61. Xcode and Apple SDKs Agreement: https://www.apple.com/legal/sla/docs/xcode.pdf
62. Apple website terms: https://www.apple.com/legal/internet-services/terms/site.html
63. Apple trademark guidelines: https://www.apple.com/legal/intellectual-property/guidelinesfor3rdparties.html
