# Appendix 2: Segments and market size (agent report, 2026-10-28)

The research agent's full report, verbatim. It's never edited. The synthesis is in [../2026-10-28-market.md](../2026-10-28-market.md).

**The brief:**
- Gather sourced counts for the candidate segments: integrators, UL 508A panel shops, electrical contractors, small water and wastewater, aquaculture, CEA, F&B and labs.
- Find software-spend benchmarks.
- Build a bottom-up market-size framework from those inputs.

---

I gathered counts for all six segments and built the model from them. Every count comes from a primary file or API (Census, BLS, EPA, USDA, Eurostat) except the integrator, CSIA and UL 508A counts, which come only from trade press or vendor pages. The web-search allowance ran out for part of the session, so I went on by fetching agency data directly; the gaps that remain are marked "unknown". I wrote no files, though the fetch tool saved a few PDFs to its own cache on its own.

## 1. Sourced counts

| Segment | Count | Unit | Year | Source |
|---|---|---|---|---|
| CSIA members | 500+ in 35 countries | member companies, global | 2026 | [12](https://www.telit.com/partners/csia/) (partner page; CSIA's own site just says "hundreds") |
| Control system integrators, North America | ~3,000 (estimate, no method given) | firms | 2024 | [11](https://www.controldesign.com/management/vertical-industries/article/55130175/delivering-system-integration) |
| Integrator profile (CSIA–JP Morgan survey) | 52% under $10M revenue; 50% serve food & beverage, 29% water/wastewater | % of integrators | 2023 | [11](https://www.controldesign.com/management/vertical-industries/article/55130175/delivering-system-integration) |
| Integrators, US only | unknown | – | – | – |
| Engineering services (NAICS 541330) | 45,692 (17,106 with 5–499 employees) | firms | 2022 | [2](https://www2.census.gov/programs-surveys/susb/tables/2022/us_state_6digitnaics_2022.txt) |
| Engineering services (NAICS 541330) | 63,233 | establishments | 2023 | [1](https://www2.census.gov/programs-surveys/cbp/datasets/2023/cbp23us.zip) |
| UL 508A certified shops, US | 4,058 | shops | 2025 | [13](https://istprecision.com/ist-precision-achieves-ul508a-certification-now-accepting-build-to-print-electrical-control-panel-projects/) (vendor blog; UL's own figure unknown) |
| UL 508A listings in UL Product iQ (US / Canada) | ~5,600 / ~5,350 | listed shops | undated | [14](https://www.oempanels.com/ul508a-third-party-safety-approval) (vendor page) |
| Relay & industrial control manufacturing (NAICS 335314, stand-in for panel builders) | 661 (724 establishments) | firms | 2022 | [4](https://www2.census.gov/programs-surveys/economic-census/data/2022/sector31/EC2231BASIC.zip) |
| Germany, NACE 27.12 (includes switchgear and panel building) | 1,228 | enterprises | 2024 | [10](https://ec.europa.eu/eurostat/databrowser/view/sbs_ovw_act/default/table) |
| EU-27, NACE 27.12 | ~6,500 | enterprises | 2024 | [10](https://ec.europa.eu/eurostat/databrowser/view/sbs_ovw_act/default/table) |
| ZVEI or VDMA count of panel builders | unknown | – | – | – |
| Eplan customers | 77,400 | customers, global | FY2025 | [15](https://www.eplan.com/us-en/about-us/eplan/) |
| Electrical contractors (NAICS 238210) | 79,611 (31,302 with 5–499 employees) | firms | 2022 | [2](https://www2.census.gov/programs-surveys/susb/tables/2022/us_state_6digitnaics_2022.txt) |
| Electrical contractors (NAICS 238210) | 83,342 | establishments | 2023 | [1](https://www2.census.gov/programs-surveys/cbp/datasets/2023/cbp23us.zip) |
| Electrical contractor revenue from firms whose main business is control-systems installation | $13.1B of $249.2B (5.3%) | revenue | 2022 | [3](https://www2.census.gov/programs-surveys/economic-census/data/2022/sector23/EC2223KOB.zip) |
| Commercial/industrial share of electrical contractor work | unknown | – | – | – |
| NECA member firms | unknown (sources conflict) | – | – | – |
| Electricians (SOC 47-2111) | 757,220, of whom 541,390 (71.5%) work at electrical contractors | people | May 2025 | [5](https://api.bls.gov/publicAPI/v2/timeseries/data/OEUN000000000000047211101), [5b](https://api.bls.gov/publicAPI/v2/timeseries/data/OEUN000000023821047211101) |
| EU-27 electrical installation (NACE 43.21) | 447,071 (Germany 42,628) | enterprises | 2024 | [10](https://ec.europa.eu/eurostat/databrowser/view/sbs_ovw_act/default/table) |
| EU-27 engineering (NACE 71.12) | 698,228 (Germany 78,818) | enterprises | 2024 | [10](https://ec.europa.eu/eurostat/databrowser/view/sbs_ovw_act/default/table) |
| Active community water systems | 49,342 in total. By people served: ≤500: 26,488; 501–3,300: 13,257; 3,301–10,000: 4,996; 10,001–100,000: 4,114; over 100,000: 487 | systems | 2026 | [6](https://echodata.epa.gov/echo/sdw_rest_services.get_systems?output=JSON&p_systyp=CWS&p_act=Y) (I grouped the EPA records by size, pulled 2026-10-28) |
| Publicly owned wastewater treatment works | 17,544 | facilities | 2022 | [7](https://www.epa.gov/cwns) |
| Aquaculture farms | 3,453 (467 use recirculating systems) | farms | 2023 | [8](https://www.nass.usda.gov/Publications/AgCensus/2022/Online_Resources/Aquaculture/AQUA.txt) |
| Greenhouse vegetables and herbs grown under cover | 11,465 (1,401 with ≥10,000 sq ft) | farms | 2022 | [9](https://www.nass.usda.gov/Publications/AgCensus/2022/Full_Report/Volume_1,_Chapter_1_US/usv1.txt) |
| Vertical farms | unknown | – | – | – |
| Food (311) and beverage (312) plants | 43,707 (4,150 with ≥100 employees) | establishments | 2023 | [1](https://www2.census.gov/programs-surveys/cbp/datasets/2023/cbp23us.zip) |
| Testing labs (541380) / R&D labs (541715) | 7,463 / 12,929 | establishments | 2023 | [1](https://www2.census.gov/programs-surveys/cbp/datasets/2023/cbp23us.zip) |
| AutoCAD (includes the Electrical toolset) / AutoCAD LT | $2,095 / $540 | per user per year | Oct 2026 | [16](https://www.autodesk.com/solutions/autocad-subscription-faq) (Autodesk blocked direct fetch; price confirmed by [16b](https://www.renderahouse.com/blog/autocad-pricing)) |
| Bluebeam Basics / Core / Complete / Max | $260 / $330 / $440 / $590 | per user per year | 2026 | [17](https://www.bluebeam.com/pricing/) |
| EPLAN Electric P8 | unknown (price by quote only) | – | – | – |
| Software purchases, electrical contractors | $574M in total, which works out to $7,273 per firm, $593 per employee, 0.23% of revenue | $/year | 2022 | [3](https://www2.census.gov/programs-surveys/economic-census/data/2022/sector23/EC2223BASIC.zip) |
| Software purchases, NAICS 335314 | $27.9M in total, which works out to $42,139 per firm, $877 per employee | $/year | 2022 | [4](https://www2.census.gov/programs-surveys/economic-census/data/2022/sector31/EC2231BASIC.zip) |
| Survey of integrator software spend | unknown | – | – | – |

Census establishment and firm counts include only businesses with paid employees.

## 2. Bottom-up framework (US)

**Formula:** ARR = (sum over segments of N × f × k) × a × p

- **N** = sourced count of firms
- **f** = share of those firms that fit
- **k** = seats per firm that buys
- **a** = adoption rate
- **p** = price per seat per year
- **(A)** marks an assumption. Anything without (A) is sourced.

| Segment | N (sourced) | f, low/mid/high | k, low/mid/high |
|---|---|---|---|
| S1 Integrators | 3,000 | 100% | 2/4/6 (A) |
| S2 UL 508A shops | 4,058 × 0.6 = 2,435 (A: 40% are already counted in S1 or S3) | 100% | 1/2/3 (A) |
| S3 Electrical contractors, 5–499 employees | 31,302 | 5.3% (Census controls revenue share, used as a stand-in) / 15% (A) / 25% (A) | 1/2/3 (A) |
| S4 Engineering firms, 5–499 employees | 17,106 | 10% / 20% / 30% (A) | 1/2/3 (A) |
| S5 Food & beverage plants with ≥100 employees | 4,150 | 100% | 1/1/2 (A) |
| S6 Water systems serving over 10,000 people | 4,114 + 487 = 4,601 | 100% | 1 (A) |
| S7 Recirculating aquaculture farms + greenhouse farms ≥10,000 sq ft | 467 + 1,401 = 1,868 | 100% | 1 (A) |

Adoption **a** is 2% / 5% / 10% (A). Price **p** is $600 / $1,200 / $2,095 (A), set against Bluebeam Max ($590) and AutoCAD ($2,095).

**Arithmetic (seats = N × f × k):**

| | Low | Mid | High |
|---|---|---|---|
| S1 | 3,000×2 = 6,000 | 3,000×4 = 12,000 | 3,000×6 = 18,000 |
| S2 | 2,435×1 = 2,435 | 2,435×2 = 4,870 | 2,435×3 = 7,304 |
| S3 | 31,302×5.3%×1 = 1,659 | 31,302×15%×2 = 9,391 | 31,302×25%×3 = 23,477 |
| S4 | 17,106×10%×1 = 1,711 | 17,106×20%×2 = 6,842 | 17,106×30%×3 = 15,395 |
| S5 | 4,150 | 4,150 | 8,300 |
| S6 | 4,601 | 4,601 | 4,601 |
| S7 | 1,868 | 1,868 | 1,868 |
| **Seats that could be served** | **22,423** | **43,722** | **78,945** |
| Value if every one bought (seats × p) | ×$600 = $13.5M | ×$1,200 = $52.5M | ×$2,095 = $165.4M |
| Paid seats (seats × a) | ×2% = 448 | ×5% = 2,186 | ×10% = 7,895 |
| **ARR** | **$0.27M** | **$2.62M** | **$16.5M** |

**Checks against the Census spending data (2022):**
- In the mid case an electrical contractor would pay 2 × $1,200 = $2,400 a year. That is about 33% of what an average electrical contractor spends on all software ($7,273).
- In the mid case an integrator would pay $4,800 a year. That is about 11% of the average NAICS 335314 firm's software spend ($42,139).

**EU:** The same formula applies. Sourced counts exist for NACE 43.21, 71.12 and 27.12. However, EU integrator counts and panel-shop certification counts are unknown. Across all specialised construction (NACE F43), 67% of enterprises have 0–1 persons (2023, [10](https://ec.europa.eu/eurostat/databrowser/view/sbs_sc_ovw/default/table)). Because of these gaps I did not compute EU scenarios.

## 3. Best first market

- **Integrators plus UL 508A panel shops.** There are about 3,000 North American integrators [11, 2024] and about 4,058 US UL 508A shops [13, 2025]. That is a small, countable group that can be reached through CSIA, which has 500+ members [12, 2026]. Most are small: 52% of integrators have under $10M revenue [11, 2023], which suits per-seat Pro pricing.
- **These firms spend more on software per head.** Control manufacturers spend $877 per employee per year on software, against $593 for electrical contractors [3, 4, 2022].
- **Water/wastewater and food & beverage are where integrators already work.** 29% of integrators serve water/wastewater and 50% serve food & beverage [11, 2023]. EPA counts 39,745 community water systems serving 3,300 people or fewer [6, 2026] and 17,544 wastewater treatment works [7, 2022]. That is a large number of small, repeated projects, mostly done by outside integrators and engineers, which suits generated drawing sets.
- **Electrical contractors are the largest group but a weaker first target.** There are 79,611 firms [2, 2022] and 757,220 electricians [5, 2025]. But only 5.3% of electrical contractor revenue comes from control-systems work, and average total software spend is $7,273 per firm [3, 2022]. They fit better as a second wave.
- **Engineering firms, aquaculture, greenhouse farms and labs come later.** Engineering firms are numerous (17,106 with 5–499 employees [2, 2022]), but how many do electrical or controls design is unknown. Recirculating aquaculture (467 farms [8, 2023]) and greenhouse farms of ≥10,000 sq ft (1,401 [9, 2022]) are too small to lead with; they could become vertical templates instead.

The mid-case result depends heavily on four assumptions you haven't confirmed: the price point, seats per firm, the 40% panel-shop overlap, and the fit shares. Those are worth settling first.

## 4. Sources

1. Census County Business Patterns 2023: https://www2.census.gov/programs-surveys/cbp/datasets/2023/cbp23us.zip
2. Census Statistics of US Businesses 2022: https://www2.census.gov/programs-surveys/susb/tables/2022/us_state_6digitnaics_2022.txt
3. Census 2022 Economic Census, construction (basic and kind-of-business files): https://www2.census.gov/programs-surveys/economic-census/data/2022/sector23/
4. Census 2022 Economic Census, manufacturing: https://www2.census.gov/programs-surveys/economic-census/data/2022/sector31/EC2231BASIC.zip
5. BLS OEWS May 2025, through the BLS API: https://api.bls.gov/publicAPI/v2/timeseries/data/OEUN000000000000047211101 and https://api.bls.gov/publicAPI/v2/timeseries/data/OEUN000000023821047211101
6. EPA ECHO drinking-water service: https://echodata.epa.gov/echo/sdw_rest_services.get_systems?output=JSON&p_systyp=CWS&p_act=Y
7. EPA Clean Watersheds Needs Survey 2022: https://www.epa.gov/cwns
8. USDA 2023 Census of Aquaculture: https://www.nass.usda.gov/Publications/AgCensus/2022/Online_Resources/Aquaculture/AQUA.txt
9. USDA 2022 Census of Agriculture, Table 39: https://www.nass.usda.gov/Publications/AgCensus/2022/Full_Report/Volume_1,_Chapter_1_US/usv1.txt
10. Eurostat business statistics (sbs_ovw_act, sbs_sc_ovw): https://ec.europa.eu/eurostat/databrowser/view/sbs_ovw_act/default/table
11. Control Design, Aug 1 2024: https://www.controldesign.com/management/vertical-industries/article/55130175/delivering-system-integration
12. Telit, CSIA partner page: https://www.telit.com/partners/csia/
13. IST Precision, June 23 2025: https://istprecision.com/ist-precision-achieves-ul508a-certification-now-accepting-build-to-print-electrical-control-panel-projects/
14. OEM Panels: https://www.oempanels.com/ul508a-third-party-safety-approval
15. Eplan: https://www.eplan.com/us-en/about-us/eplan/
16. Autodesk subscription FAQ: https://www.autodesk.com/solutions/autocad-subscription-faq; confirming price page: https://www.renderahouse.com/blog/autocad-pricing
17. Bluebeam pricing: https://www.bluebeam.com/pricing/
