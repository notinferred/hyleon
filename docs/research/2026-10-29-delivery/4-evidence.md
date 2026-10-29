# Appendix 4: Supply-chain evidence and regulation (agent report, 2026-10-29)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-29-delivery.md](../2026-10-29-delivery.md).

**The brief:** the evidence a software vendor's build and release pipeline must produce, now and for enterprise or regulated buyers: the EU Cyber Resilience Act, SBOMs for Rust, build provenance and signing, GitHub security features on private repos by plan, what enterprise buyers ask small vendors for, and secrets in CI.

---

# Build and release evidence for Hyleon: what to produce now, before EU sales, and before enterprise sales

*Research date: 2026-10-29. Legal points marked **[Interpretation]** are my reading of the sources, not legal advice. Points marked **[Vendor claim]** come from parties selling the product or service.*

## Summary

- **EU Cyber Resilience Act (CRA):** the incident and vulnerability reporting duty (Article 14) has applied since **2026-09-11**, including to products already on the EU market. The main obligations apply from **2027-12-11**. A build-pipeline compromise can count as a reportable "severe incident".
- **GitHub Free organisation, private repos:** you get Dependabot only. Secret scanning, CodeQL, branch protection, rulesets, environments and artifact attestations are not available. Attestations on private repos need GitHub Enterprise Cloud.
- **SBOMs:** neither the CRA nor US guidance requires shipping an SBOM with the binary. The CRA requires one in the technical documentation. CISA's July 2026 minimum elements now expect full transitive coverage, hashes, licences, generation context and an author signature.
- **Enterprise buyers:** most enterprise evidence can wait until the first real deal. The exceptions are cheap and should start now: a disclosure policy, a per-release evidence bundle stored outside GitHub, and least-privilege CI.

---

## 1. EU Cyber Resilience Act and the revised Product Liability Directive

**Dates.** These are verified against the Official Journal text of [Regulation (EU) 2024/2847](http://publications.europa.eu/resource/celex/32024R2847), Article 71:
- The regulation applies from 2027-12-11.
- Article 14 (reporting) applies from 2026-09-11.
- Chapter IV (notified bodies) applies from 2026-06-11.

Article 69(3) applies Article 14 to products placed on the market *before* 2027-12-11 as well. The Commission says ENISA's Single Reporting Platform has been "operational as of 11 September 2026" ([Commission CRA reporting page, updated 2026-09-11](https://digital-strategy.ec.europa.eu/en/policies/cra-reporting)).

**Who it applies to.** A "manufacturer" markets a product "whether for payment, monetisation or free of charge". "Making available" means supply "in the course of a commercial activity" (Art. 3).
- **Unfinished software:** it may be made available for testing for a limited period if it carries a visible sign saying it does not comply (Art. 4(3)).
- **SaaS:** it is outside the CRA unless it is a "remote data processing solution" without which the product cannot perform a function (recitals 11–12).
- **Open source:** free and open-source software supplied outside commercial activity is out of scope. Lighter "steward" rules apply in some cases. Neither helps a proprietary product.
- **[Interpretation]** Hyleon's desktop app, CLI and on-premises builds are in scope. A standalone web app probably is not. The future API server would be in scope if the desktop app depends on it. Free betas given to EU users may count as "making available" unless they are clearly labelled under Art. 4(3).

**Reporting (Art. 14).** For actively exploited vulnerabilities and for severe incidents:
- an early warning within 24 hours;
- a notification within 72 hours;
- a final report within 14 days of a fix (for vulnerabilities) or within one month (for incidents).

An incident is "severe" if it "has led or is capable of leading to the introduction or execution of malicious code in a product" (Art. 14(5)(b)). **[Interpretation]** A compromised CI pipeline or signing key is therefore a reportable event. Impacted users must also be informed (Art. 14(8)).

Microenterprises and small enterprises are exempt from *fines* only for missing the 24-hour early-warning deadline (Art. 64(10)). The duty itself still applies.

**Process requirements that create pipeline evidence (Annex I Part II, Art. 13):**
- **SBOM:** "in a commonly used and machine-readable format covering at the very least the top-level dependencies". It belongs in the technical documentation (Annex VII). Giving it to users is optional; Annex II only says to state where it is *if* you choose to share it. Authorities can request it.
- **Fixing vulnerabilities:** remediate "without delay". Ship security updates separately from feature updates "where technically feasible". Distribute updates securely, automatically where applicable, and free of charge.
- **Disclosure:** have a coordinated vulnerability disclosure (CVD) policy and a contact address for reports. Publish advisories after fixes.
- **Support period:** at least 5 years, or the expected use time if shorter. Users must be told the end date.
- **Retention:** each security update must stay available for at least 10 years. Technical documentation must be kept for 10 years or the support period, whichever is longer (Art. 13(8), (9), (13)).
- **Latest version only:** Art. 13(10) allows fixing only the latest version, provided earlier users get it free of charge.
- **Component vulnerabilities:** if you find a vulnerability in a dependency, report it to that component's maintainer (Art. 13(6)).

**Conformity class.** Products not listed in Annex III or IV use self-assessment (internal control, "module A"; recital 90). The categories are clarified in [Implementing Regulation (EU) 2025/2392](https://eur-lex.europa.eu/eli/reg_impl/2025/2392/oj/eng) (adopted 2025-11-28; core functionality decides the class). **[Interpretation]** A design-testing tool is not an identity, network, OS or security product, so it falls in the default class: self-assessment, an EU declaration of conformity, and a CE mark on the declaration or the accompanying website (Art. 30).

**Penalties.** Up to €15M or 2.5% of worldwide turnover for breaches of Annex I or Articles 13–14 (Art. 64(2)).

**Still pending:**
- The Commission's non-binding guidance [C(2026) 5252](https://digital-strategy.ec.europa.eu/en/library/commission-publishes-new-guidance-support-timely-cyber-resilience-act-implementation) was published 2026-07-27, with SME examples.
- No implementing act fixing the SBOM format has been adopted.
- No harmonised standards were cited in the Official Journal as of mid-2026. The EN 40000 vulnerability-handling part (prEN 40000-1-3) was due around 2026-10-31 ([secondary source, 2026-07-30](https://www.cyberresilienceact.eu/news/cra-standardisation-deadlines-pushed-back-two-months.html)).
- The Digital Omnibus proposal (2025-11-19) for a single reporting entry point has not been adopted.

**Product Liability Directive.** Source: [Directive (EU) 2024/2853](http://publications.europa.eu/resource/celex/32024L2853).
- Applies to products placed on the market after 2026-12-09, and software is a "product".
- **Limits:** compensation is for natural persons only. Damage to property used exclusively for professional purposes is excluded, as is destruction of data used for professional purposes (recital 62, Art. 6).
- **Updates:** manufacturers cannot escape liability where the defect is "a lack of software updates or upgrades necessary to maintain safety" while they control updates.
- **Evidence:** courts can order disclosure of evidence. A failure to disclose triggers a presumption that the product is defective (Arts. 9–10).
- **[Interpretation]** For a business-to-business tool the exposure is narrower than for consumer software. The practical point is that release records and update history may become court evidence.

## 2. SBOMs for Rust

**US policy:**
- [OMB M-26-05](https://www.whitehouse.gov/wp-content/uploads/2026/01/M-26-05-Adopting-a-Risk-based-Approach-to-Software-and-Hardware-Security.pdf) (2026-01-23) rescinded M-22-18 and M-23-16. The federal "Common Form" self-attestation is now optional, and agencies decide for themselves whether to require SBOMs ([Mayer Brown, 2026-02](https://www.mayerbrown.com/en/insights/publications/2026/02/omb-rescinds-biden-era-software-security-memoranda)).
- NIST's SSDF v1.2 (SP 800-218r1) is still a draft from 2025-12-17. v1.1 is the final version ([NIST](https://csrc.nist.gov/projects/ssdf/publications)).

**CISA 2026 minimum elements.** [CISA's 2026 Minimum Elements](https://www.cisa.gov/resources-tools/resources/2026-minimum-elements-software-bill-materials-sbom) (2026-07-29) replace the 2021 NTIA document and are co-signed by BSI, ANSSI and others. They are non-binding ("do not create new requirements").
- **New fields:** SBOM author signature, tool name and version, format name and version, generation context ("before build / build / after build"), component hash and algorithm, and component licence.
- **Coverage:** "all components… including transitive dependencies. There is no minimum depth."
- **Frequency:** a new SBOM for every build or release.
- **Unknowns:** state explicitly what is unknown or withheld.
- **Distribution:** "can accompany installation" or be served through a version-specific URL or API, with access controls allowed. Shipping with the binary is one option, not a requirement.

**Germany's BSI TR-03183-2** (v2.x) sets floors of CycloneDX ≥ 1.6 or SPDX ≥ 3.0.1 ([secondary summary](https://fossa.com/blog/germany-bsi-sbom-guidelines/)). It is a likely reference for EU buyers but is not binding EU law.

**Tools:**

| Tool | Status | Notes |
|---|---|---|
| [cargo-cyclonedx](https://github.com/CycloneDX/cyclonedx-rust-cargo) 0.5.9 (2026-03-19) | Maintained | Uses Cargo metadata directly; can emit one SBOM per binary and per target. **Outputs only CycloneDX 1.3–1.5 (default 1.3)**, below BSI's 1.6 floor. |
| [syft](https://github.com/anchore/syft) v1.55.0 (2026-10-19) | Maintained | Reads `Cargo.lock` and binaries built with cargo-auditable. CycloneDX default on main is 1.7. |
| [cargo-auditable](https://github.com/rust-secure-code/cargo-auditable) v0.7.7 (2026-10-02) | Maintained | Embeds the dependency tree in the binary in under 4 kB. Read by cargo-audit, trivy, grype, syft and osv-scanner. |
| cargo-sbom v0.10.0 (2025-06-17) | Slower release cadence | SPDX and CycloneDX output. |
| Cargo `-Zsbom` | Nightly only | A [2026 Rust project goal](https://rust-lang.github.io/goals/2026/stabilize-cargo-sbom.html) aims to stabilise it. |

**Pitfalls ([Interpretation]):**
- Cargo-based tools list `-sys` crates but not the version of C code bundled inside them.
- Dioxus desktop relies on the system webview (WebView2 or WKWebView), which is not in `Cargo.lock`. Declare it as a dependency or as a known unknown.
- macOS and Windows builds need separate SBOMs, generated per target.

## 3. Build provenance, signing and reproducibility

**SLSA.** The current version is [SLSA v1.2](https://slsa.dev/blog/2025/11/announce-slsa-v1.2) (2025-11-24). It adds a Source track and is backwards compatible with v1.1.
- **Build L1:** provenance exists but can be forged.
- **Build L2:** a hosted platform signs the provenance.
- **Build L3:** builds are isolated from each other and the signing material is kept away from user build steps ([build track](https://slsa.dev/spec/v1.2/build-track-basics)).

**GitHub artifact attestations** (`actions/attest-build-provenance`) give SLSA Build L2 on their own, and L3 when used with reusable workflows ([GitHub docs](https://docs.github.com/en/actions/concepts/security/artifact-attestations)). On availability, GitHub says: "If you are on a Free, Pro, or Team plan, artifact attestations are only available for public repositories. To use artifact attestations in private or internal repositories, you must be on a GitHub Enterprise Cloud plan" (docs source text, last changed 2025-09-03). Private-repo attestations use GitHub's own Sigstore instance, which has no public transparency log. Consumers verify with `gh attestation verify`.

**Alternatives on the Free plan:**
- **Public Sigstore, keyless:** the Fulcio certificate embeds the source repository URI, the commit and the workflow path ([Fulcio OIDs](https://github.com/sigstore/fulcio/blob/main/docs/oid-info.md)) in the *public* Rekor log. That leaks the private repo's name. `slsa-github-generator` refuses private repos unless you set `private-repository: true`, and its last release was v2.1.0 (2025-02).
- **cosign with a cloud KMS key:** supported for AWS, GCP, Azure and Vault ([Sigstore docs](https://docs.sigstore.dev/cosign/key_management/overview/)). This signs a checksum manifest without a public log. It does not produce SLSA provenance issued by the platform.

**Platform code signing is needed separately for user trust.**
- **Windows:** since 2023-06-01, CA/B Forum rules require code-signing keys to be generated and held in a hardware module ([CA/B Forum](https://cabforum.org/working-groups/code-signing/requirements/)). Certificates are capped at 460 days from 2026-03-01 ([ballot CSC-31](https://cabforum.org/2025/11/17/ballot-csc-31-maximum-validity-reduction)). Azure Artifact Signing (formerly Trusted Signing) became generally available in the US, Canada and Europe in January 2026 ([DevClass, 2026-01-14](https://devclass.com/2026/01/14/code-signing-windows-apps-may-be-easier-and-more-secure-with-new-azure-artifact-service/); secondary).
- **macOS:** distribution requires a Developer ID certificate and notarization.

**Reproducible builds.** Rust is "often already reproducible by default" when the toolchain version matches and paths are normalised. Use `--locked`, and rust-embed's `deterministic-timestamps` feature where it applies ([reproducible-builds.org](https://reproducible-builds.org/docs/rust/)). Known pitfalls:
- `trim-paths` (RFC 3127) still appears nightly-only. I could not confirm that it has been stabilised.
- On MSVC, `/Brepro` helps but is not enough on its own ([rb-general, 2024-12](https://lists.reproducible-builds.org/pipermail/rb-general/2024-December/003592.html); [normalizer docs](https://msvcpp-normalize-pe.readthedocs.io/en/latest/_sources/technical-details.rst.txt)).
- **[Interpretation]** Code signatures are embedded in the binary on macOS and Windows, so the realistic goal is reproducing the *unsigned* binary.

## 4. GitHub security features for private repos, by plan

Availability comes from GitHub docs source text (fetched 2026-10-29). Prices: Team $4/user/month and Enterprise $21/user/month ([pricing page](https://github.com/pricing)). Secret Protection is $19 and Code Security $30 per active committer per month, and both can be bought on Team since [2025-04-01](https://github.blog/changelog/2025-04-01-github-advanced-security-is-here-for-github-team-organizations).

| Private-repo feature | Free org | Team | Needs |
|---|---|---|---|
| Dependency graph, Dependabot alerts, security and version updates | Yes | Yes | — |
| Secret scanning and push protection | No | With Secret Protection | $19/committer |
| CodeQL (Rust generally available since [2025-10-14](https://github.blog/changelog/2025-10-14-codeql-scanning-rust-and-c-c-without-builds-is-now-generally-available)), dependency review | No | With Code Security | $30/committer |
| Protected branches, rulesets, CODEOWNERS | No | Yes | — |
| Environments, environment secrets, deployment branches | No | Yes | — |
| Environment required reviewers and wait timers | No | No | Enterprise Cloud |
| Artifact attestations | No | No | Enterprise Cloud |
| Private vulnerability reporting | No (public repos only) | No | — |
| Audit log | 180 days; Git events 7 days; no streaming for standalone orgs | Same | Enterprise |

Two related details:
- Dependabot's `cooldown` setting (default 3 days) applies to version updates only, **not security updates**. That fits Hyleon's "at least 2 weeks old" policy while letting security fixes through.
- Actions retention for private repos is at most 400 days. From 2026-10-01 that limit also covers workflow runs and checks ([GitHub changelog, 2026-07-17](https://github.blog/changelog/2026-07-17-actions-retention-will-cover-checks-workflow-runs-and-statuses/)). **CI history is not a durable evidence store.**

## 5. What enterprise buyers ask small vendors for

- **Questionnaires.** The Shared Assessments SIG (2026 edition with Lite/Core presets) is licensed, not free. CSA's CAIQ v4 and the STAR Level 1 self-assessment are free ([secondary summaries](https://complydog.com/blog/what-is-the-caiq-questionnaire)). Expect questions on SDLC, change management, vulnerability management, incident response, SBOMs and access control.
- **SOC 2 Type II.**
  - SOC 2 covers a *service organisation's* controls. Vendors of on-premises or desktop software often say SOC 2 does not apply to the installed product (e.g. [IBM support](https://www.ibm.com/support/pages/node/7247643)). **[Vendor claim]**
  - It becomes relevant once Hyleon hosts the web app or API with customer data.
  - The AICPA sets no minimum observation period. Three months is the common floor and six months is typical for a first report. Audit fees are commonly quoted at roughly $12k–$100k ([secondary](https://soc2auditors.org/guides/how-long-does-soc-2-take-for-a-10-person-startup/)). **[Vendor claim]**
- **Change-management evidence.** Buyers want proof of who changed what, who approved it and who released it. On a Free private repo none of this can be *enforced*: there is no branch protection, ruleset or environment gate. A solo founder cannot provide two-person review in any case. The realistic evidence is: releases built only by CI from signed tags, a per-release manifest, and an immutable archive.
- **Vulnerability disclosure.** A `security.txt` file ([RFC 9116](https://www.rfc-editor.org/rfc/rfc9116)) plus a CVD policy. This is required by the CRA anyway.
- **Source-code escrow.** Typical for on-premises deals in regulated sectors. Release conditions are the usual point of contention ([Lowenstein](https://www.lowenstein.com/news-insights/publications/articles/source-code-escrow-agreements-are-reaching-for-the-cloud-kesslen-satlin)). Defer until a contract requires it. Reproducible, documented builds make escrow verifiable.

## 6. Secrets in CI

- **OIDC.** OIDC federation is available on all plans ([GitHub docs](https://docs.github.com/en/actions/concepts/security/openid-connect)) and removes long-lived cloud keys. Without environments on Free private repos, cloud trust policies must be scoped by the `sub` claim to the repo and the tag ref (`repo:ORG/REPO:ref:refs/tags/v*`), not to an environment.
- **Least privilege.** Organisations created after 2023-02-02 default to a read-only `GITHUB_TOKEN` ([changelog](https://github.blog/changelog/2023-02-02-github-actions-updating-the-default-github_token-permissions-to-read-only/)). Set `permissions: contents: read` at workflow level and raise it per job. Attestations need `id-token: write` and `attestations: write`.
- **Signing keys.** Keep them in a cloud KMS or HSM-backed signing service reached through OIDC, never as exported key files in repository secrets.
- **Secret scanning on Free.** Use the gitleaks CLI (MIT licence). The gitleaks GitHub Action v2 needs a free licence key for organisations.

---

## What Hyleon needs: now, before EU sales, before enterprise

**Now (cheap, and reduces risk regardless of market):**
1. **Per-release evidence bundle.** Each release produces binaries, SHA-256 checksums, a CycloneDX SBOM per target with full transitive dependencies, cargo-auditable embedding, cargo-deny output, the toolchain version, the commit SHA and the run URL. Copy it to storage outside GitHub with retention of 10 years or more (for example object-locked cloud storage). *Trade-off:* small storage cost and a little CI time.
2. **Workflow permissions.** Least-privilege `permissions:` in every workflow. Confirm the organisation default is read-only.
3. **Free scanning.** Turn on Dependabot alerts and security updates. Add a daily scheduled cargo-deny run so new advisories surface between pushes. Add gitleaks as a pre-commit hook and in CI.
4. **Security-fix exception.** Write an explicit exception to the "at least 2 weeks old" pin rule for security fixes, since the CRA requires remediation "without delay".
5. **Disclosure.** A CVD policy, a security contact and `security.txt`.
6. **Signing.** Windows signing through an HSM-backed service and macOS Developer ID plus notarization, both reached from CI through OIDC.

**Before any EU sale or EU beta (Article 14 already applies):**
- Register on ENISA's reporting platform.
- Write an incident runbook for the 24h/72h/14-day deadlines, including CI or signing-key compromise.
- Set up a way to notify users.
- Label free betas as Art. 4(3) test software, or treat them as on the market.

**By 2027-12-11, for EU products:**
- A documented risk assessment and Annex VII technical documentation (simplified form for micro/small enterprises once the Commission adopts it).
- A declared support period of 5 years or more.
- A path to ship security-only updates.
- A secure update channel.
- Self-assessment, EU declaration of conformity and CE mark (default class; confirm against 2025/2392).
- Consider generating CycloneDX 1.6 or later (syft), because cargo-cyclonedx stops at 1.5.

**Before enterprise:**
- Prepare a CAIQ answer pack.
- Upgrade to Team ($4/month) for rulesets, protected tags and environments.
- Add Secret Protection or Code Security if buyers ask for GitHub-native evidence.
- Move to Enterprise Cloud ($21/user/month) when a buyer wants verifiable provenance from a private repo (SLSA Build L2/L3) or audit-log streaming.
- Start the SOC 2 Type II clock 6–12 months before a hosted product is sold.
- Arrange escrow per contract.
- Set reproducible unsigned builds as a target.

**Decisions only you can make (options, not assumptions):**
- **(a) Provenance now:** stay on Free and sign checksums with cosign and a KMS key; move to Enterprise Cloud for GitHub attestations; or defer provenance entirely.
- **(b) Hosting the evidence archive:** which provider.
- **(c) Support-period length:** 5 years, or longer to match how long customers keep a design tool.
- **(d) Free beta to EU users before 2027:** whether to offer one.
- **(e) CRA classification:** whether to get a lawyer to confirm the classification and the PLD exposure, given what "physical design" covers.

## Limits

- I could not load EUR-Lex HTML directly; the CRA and PLD were read from the EU Publications Office XHTML of the same Official Journal texts.
- I did not read the Commission's C(2026) 5252 guidance annex itself, only its press page and a secondary summary.
- The status of harmonised standards, the SBOM implementing act and the Digital Omnibus after July 2026 is unconfirmed.
- BSI TR-03183-2 field requirements come from a secondary summary.
- Azure Artifact Signing pricing could not be retrieved.
- SOC 2 timelines and costs, SIG and CAIQ details, and escrow practice are vendor or secondary sources.
- I could not confirm whether `trim-paths` has been stabilised.
- I did not verify GitHub's immutable-releases plan gating or current Actions minute pricing; GitHub postponed its self-hosted runner fee and I found no later status.
- None of this is legal advice. Whether CRA classification, "making available" and PLD apply to Hyleon's specific product needs counsel. Export controls or sector rules for Hyleon's customers were out of scope.
