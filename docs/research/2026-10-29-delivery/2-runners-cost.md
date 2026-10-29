# Appendix 2: CI runners, costs and cross-compiling (agent report, 2026-10-29)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-29-delivery.md](../2026-10-29-delivery.md).

**The brief:** where CI runs and what it costs, now and at scale: GitHub-hosted runners in 2026, third-party runner providers, self-hosted runners, cross-compiling to avoid Windows and macOS runners, signing from Linux, and worked cost examples.

---

# CI runners and costs for Hyleon (researched 2026-10-29)

All pages were read on 2026-10-29 unless another date is given. Speed and price claims from runner vendors are marked **(vendor claim)**.

## Summary

- **Hyleon's planned workload costs $0 today on GitHub-hosted runners.** The three jobs in §5 (Linux CI, Windows builds, macOS releases) use $6.68 of the $12 monthly compute the Free org plan includes. The money comes from **macOS**: 74% of the bill at 10× the volume.
- **Cross-compiling Windows from Linux saves very little.** About $0.40/month now and $4/month at 10×. It works for plain `cargo` builds, but `dx bundle` does not support it.
- **macOS builds cannot legally be cross-compiled from Linux.** Apple's licence forbids using its SDK on non-Apple hardware, so the macOS build needs Apple hardware. Signing and notarizing can run on Linux with `rcodesign`, but the build itself is the expensive part.
- **GitHub's self-hosted runner fee has not started.** The $0.002/min charge was announced, then postponed, and no new date has been given. Self-hosted runners are still free.
- **Hetzner is no longer a cheap runner option.** Its 2026 price rises, and plans showing as unavailable, change the "cheap VPS runner" math.

---

## 1. GitHub-hosted runners

**Included usage is now a dollar amount, not minutes** ([Product usage included](https://docs.github.com/en/billing/reference/product-usage-included); [How Actions use is measured](https://docs.github.com/en/billing/concepts/product-billing/github-actions)):

| Plan | Included compute/month | Artifact + Packages storage | Cache per repo |
|---|---|---|---|
| Free for organizations | **$12** | 500 MB | 10 GB |
| Team ($4/user/mo) | $18 | 2 GB | 10 GB |
| Enterprise Cloud ($21/user/mo) | $300 | 50 GB | 10 GB |

- The docs say the included amount "is expressed as a monetary amount, not a quantity of minutes."
- [github.com/pricing](https://github.com/pricing) still advertises "2,000 CI/CD minutes" for Free. That figure is $12 ÷ $0.006, i.e. Linux 2-core minutes.
- I could not find a changelog entry dating the switch from minutes to dollars.

**Rates per minute, in effect since the 2026-01-01 price cut** ([Actions runner pricing](https://docs.github.com/en/billing/reference/actions-runner-pricing); [changelog 2026-01-01](https://github.blog/changelog/2026-01-01-reduced-pricing-for-github-hosted-runners-usage/)):

| Runner | Price per minute |
|---|---|
| Linux 1-core (`ubuntu-slim`) | $0.002 |
| Linux 2-core x64 | $0.006 |
| Linux 2-core arm64 | $0.005 |
| Windows 2-core (x64 or arm64) | $0.010 |
| macOS 3–4 core (M1) | $0.062 |

Larger runners:
- Linux x64: 4-core $0.012, 8-core $0.022, 16-core $0.042.
- Windows: 4-core $0.022, 8-core $0.042.
- macOS: 12-core $0.077; 5-core M2 Pro `macos_xl` $0.102.
- Larger runners are only for Team and Enterprise Cloud plans ([larger runners](https://docs.github.com/en/actions/concepts/runners/larger-runners)) and always need a payment method.

**How the $12 is used up:**
- There are no OS multipliers any more. Every job draws from the same $12 at its own rate. That is **2,000 Linux minutes, 1,200 Windows minutes, or about 193 macOS minutes** a month.
- Each job is **rounded up to a whole minute**. A 40-second job bills as 1 minute, so splitting work into many small jobs costs more.
- `ubuntu-slim` (1 vCPU, 5 GB RAM) became generally available on 2026-01-22. Jobs on it are **limited to 15 minutes** ([changelog](https://github.blog/changelog/2026-01-22-1-vcpu-linux-runner-now-generally-available-in-github-actions/)).
- Since 2026-06-01, **Copilot code review on private repos also uses Actions minutes** from the same allowance ([changelog 2026-04-27](https://github.blog/changelog/2026-04-27-github-copilot-code-review-will-start-consuming-github-actions-minutes-on-june-1-2026/)).

**When the $12 runs out:**
- With no payment method on file, "usage is blocked once you consume the included compute usage."
- With a payment method, extra usage is billed.
- Budgets only *block* usage if you tick "Stop usage when budget limit is reached". Otherwise they just send alerts ([set up budgets](https://docs.github.com/en/billing/how-tos/set-up-budgets)).
- GitHub emails at 90% and 100% of included usage.

**Cache and storage:**
- The cache is 10 GB per repo by default and can be raised for $0.07/GB-month.
- Cache entries not used for 7 days are evicted ([dependency caching](https://docs.github.com/en/actions/reference/workflows-and-actions/dependency-caching)).
- Artifact storage above the included amount costs $0.25/GB-month.
- Release assets have no total-size or bandwidth limit, but each file must be under 2 GiB ([About releases](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases)).

**The self-hosted runner charge:**
- On 2025-12-16 GitHub announced a $0.002/min "cloud platform charge" for self-hosted runners, starting 2026-03-01 ([changelog](https://github.blog/changelog/2025-12-16-coming-soon-simpler-pricing-and-a-better-experience-for-github-actions/)).
- An update posted at the top of that changelog entry says: "We're postponing the announced billing change for self-hosted GitHub Actions to take time to re-evaluate our approach." It gives no new date.
- The billing docs today still list self-hosted runners as "free… does not reduce the included compute usage."
- **Status: postponed, not cancelled, and not in effect** as of 2026-10-29.

## 2. Third-party runner providers

All of these work the same way: you install the vendor's GitHub App on the org and change `runs-on:` to their label. **All prices and speed figures below are vendor claims.**

| Provider | Linux 2 vCPU | Windows | macOS | Free tier | Cache | Security (vendor claim) |
|---|---|---|---|---|---|---|
| [Blacksmith](https://www.blacksmith.sh/pricing) | $0.004 (ARM $0.0025) | $0.008 (Server 2025, public beta) | M4 6 vCPU $0.08 | 3,000 "x64 2vCPU min"/org/mo (see note) | Transparent `actions/cache`, 25 GB/repo/week free; sticky disks $0.50/GB-mo | SOC 2 Type 2; Firecracker microVMs |
| [Depot](https://depot.dev/pricing) | $0.006 | $0.008 (counts 2× against minutes) | 8 CPU $0.08 | $20/mo plan includes 2,000 min + 25 GB cache | $0.20/GB-mo beyond plan | SOC 2 Type 2 ([security](https://depot.dev/security)) |
| [Namespace](https://namespace.so/pricing) | 4 vCPU $0.004 prepaid / $0.006 overage | 4 vCPU $0.008 / $0.012 | 4 vCPU $0.04 / $0.06 | Pay-as-you-go, 30-day trial | Cache volumes, billed per GB-hour/day | No certification on pricing page; trust centre exists |
| [WarpBuild](https://www.warpbuild.com/pricing) | $0.004 (ARM $0.003) | from 4 vCPU $0.016 | M4 Pro 6 vCPU $0.08 | Unspecified credits | $0.20/GB-mo; bring-your-own-cloud $0.002/min | SOC 2 Type II |
| [Ubicloud](https://ubicloud.com/docs/about/pricing) | $0.00125 (premium $0.002; ARM $0.00125) | — | — | "$2.5/month credit ≈ 1,250 min" | — | Open source; Linux only |
| [RunsOn](https://runs-on.com/pricing/) | Your AWS bill, e.g. ~$0.0009 on spot | Yes | No | Free for non-commercial; €300/yr Starter | S3-backed | Runs in **your** AWS account |
| [Cirun](https://cirun.io/pricing) | Your cloud bill | — | — | Free for public repos; $29/mo for 3 private repos | — | Runs in your cloud account |
| BuildJet | — | — | — | — | — | **Shut down 2026-03-31** ([announcement 2026-02-06](https://buildjet.com/for-github-actions/blog/we-are-shutting-down)) |

Notes:
- **How Blacksmith's free pool is used up** ([docs](https://docs.blacksmith.sh/blacksmith-runners/overview)):
  - 1 Windows minute counts as 2 units.
  - 1 macOS 6-vCPU minute counts as 20 units.
  - 1 ARM minute counts as 0.625 units.
- **Example labels:** `blacksmith-2vcpu-ubuntu-2404`, `depot-ubuntu-24.04`, `ubicloud-standard-2`.
- **Security trade-off:** with hosted vendors (Blacksmith, Depot, Namespace, WarpBuild cloud), your code and secrets run on the vendor's machines, and the vendor's GitHub App gets org-level rights to register runners. Bring-your-own-cloud options (RunsOn, Cirun, WarpBuild BYOC, Ubicloud self-hosted) keep compute in your account but add cloud setup work.
- **Vendor risk is real.** BuildJet closed with about eight weeks' notice.

## 3. Self-hosted runners

**Hetzner in 2026:**
- Prices rose on 2026-04-01 (existing and new servers) and again on 2026-06-15 (new orders and rescales) ([Hetzner price adjustment](https://docs.hetzner.com/general/infrastructure-and-availability/price-adjustment/)).
- On the live [Regular Performance page](https://www.hetzner.com/cloud/regular-performance), EU prices excluding VAT are CPX22 (2 vCPU/4 GB) €19.99/mo and CPX32 (4 vCPU/8 GB) €35.99/mo.
- Every EU create button showed "currently unavailable", and the Cost-Optimized line shows "Currently not available" ([hetzner.com/cloud](https://www.hetzner.com/cloud/)).

**Security** ([secure use reference](https://docs.github.com/en/actions/reference/security/secure-use)):
- Self-hosted runners "do not have guarantees around running in ephemeral clean virtual machines" and "can be persistently compromised by untrusted code in a workflow."
- On private repos the risk is anyone who can open a PR.
- GitHub's advice:
  - Use just-in-time runners (one job, then removed) on a clean environment.
  - Use runner groups to limit which repos can use a runner.
  - Keep secrets and network access on the machine to a minimum.
  - Use OIDC for cloud credentials.
- A solo founder is the only one who can push to Hyleon, so the main risks are compromised dependencies and Actions, not outside contributors.

**Maintenance:**
- If you turn off auto-update, you must update the runner software within 30 days of each release, or GitHub stops sending it jobs ([self-hosted runners reference](https://docs.github.com/en/actions/reference/runners/self-hosted-runners)).
- You also own OS patches, disk cleanup, toolchain images, and a Windows licence for any Windows runner.
- **Actions Runner Controller** needs a Kubernetes cluster ([ARC](https://docs.github.com/en/actions/concepts/runners/actions-runner-controller)).
- The **Runner Scale Set Client** (public preview since 2026-02-05) lets you build autoscaling without Kubernetes ([changelog](https://github.blog/changelog/2026-02-05-github-actions-early-february-2026-updates/)).

**When self-hosting pays off (raw price only, ignoring your time):**
- **Linux:** a CPX32 at about $40/month (at roughly 1.1 USD/EUR, rate not verified) matches about 6,600 billed GitHub Linux minutes beyond the included amount. Against Blacksmith that is about 10,000 minutes; against Ubicloud about 32,000. Hyleon uses about 120 Linux minutes/month.
- **macOS:** a Scaleway Mac mini M4 costs €149/mo or €0.22/h, with a 24-hour minimum lease because of Apple's licence ([Scaleway pricing](https://scaleway.com/en/pricing/apple-silicon)). It breaks even at about 2,600 macOS minutes/month.
- The non-cost reason to self-host is that a persistent runner keeps a warm cargo `target/` directory, so builds are faster.

## 4. Cross-compiling and signing

### Windows from Linux

- **cargo-xwin** builds for `x86_64-pc-windows-msvc` and downloads the MSVC runtime and Windows SDK through xwin ([repo](https://github.com/rust-cross/cargo-xwin)).
  - Using it means you accept Microsoft's licence: "By using this software you are consented to accept the license at go.microsoft.com/fwlink/?LinkId=2086102", which is the Visual Studio Build Tools terms.
  - Those terms let you use Build Tools without a Visual Studio licence only to build OSI-licensed open-source dependencies ([Microsoft C++ blog, 2022-08-18](https://devblogs.microsoft.com/cppblog/updates-to-visual-studio-build-tools-license-for-c-and-cpp-open-source-projects/)). Proprietary code needs a VS licence, for example Community if you qualify.
  - **This is a licence question I have not verified for Hyleon.**
- **`x86_64-pc-windows-gnu`** avoids the MSVC libraries, but webview2-com's static loader "doesn't work with the `*-pc-windows-gnu` targets." You must then ship `WebView2Loader.dll` with the app ([webview2-rs](https://github.com/wravery/webview2-rs)).
- **cargo-zigbuild**'s README documents Linux and macOS targets only.
- **Dioxus does not cross-bundle.** The [Dioxus 0.7 bundle docs](https://dioxuslabs.com/learn/0.7/tutorial/bundle) say "`dx bundle` only builds desktop apps for the native platform."
  - [Issue #4076](https://github.com/DioxusLabs/dioxus/issues/4076) is still open. A windows-gnu build compiled; bundling failed. A maintainer said on 2025-06-24 that cross-bundling is not supported.
- **Tauri (also built on wry)** documents cargo-xwin cross-builds that produce **NSIS only** (MSI needs Windows). It calls the path "not tested as much" ([Tauri docs](https://v2.tauri.app/distribute/windows-installer/)).
- **Velopack can package Windows releases on Linux** with `vpk [win] pack`. macOS packages "can only be built on macOS" ([Velopack cross-compiling](https://docs.velopack.io/packaging/cross-compiling)).
- **My inference, not tested:** a Linux-only Windows pipeline would be `dx build` with a cargo-xwin linker, then `vpk [win] pack`, then signing. Whether Dioxus assets get processed correctly outside `dx bundle` needs a trial build.

### macOS from Linux

- The Xcode and Apple SDKs Agreement (EA2002, dated 06/08/2026, [PDF](https://www.apple.com/legal/sla/docs/xcode.pdf)) says:
  - §2.2 allows installing the Apple Software only "on Apple-branded computers that are owned or controlled by You."
  - §2.7 forbids using it "on any non-Apple-branded computer… or to enable others to do so."
- osxcross and zigbuild's macOS Docker images need that SDK, and wry links against AppKit and WebKit. **So the macOS build must run on Apple hardware.**

### `rcodesign` (apple-codesign 0.29.0)

[Docs](https://gregoryszorc.com/docs/apple-codesign/main/apple_codesign.html), [known issues](https://gregoryszorc.com/docs/apple-codesign/main/apple_codesign_quirks.html).

- **It can:** sign, notarize and staple `.app` bundles, DMGs and `.pkg` installers from Linux, using YubiKey, PKCS#11 or remote signing.
- **It cannot:**
  - create a DMG or write `.pkg` files;
  - sign files that are already inside a DMG;
  - recursively sign nested packages.
- Its authors warn that bundle signing "is susceptible to a lot of subtle bugs."
- An Apple Developer Program membership is required for Developer ID certificates.

### Windows signing from Linux

- Since 2023-06-01, keys for publicly trusted code-signing certificates must live in hardware or a cloud HSM, so a plain `.pfx` file is out.
- **Azure Artifact Signing** (formerly Trusted Signing) ([pricing](https://learn.microsoft.com/en-us/azure/artifact-signing/how-to-change-sku), [quickstart](https://learn.microsoft.com/en-us/azure/artifact-signing/quickstart)):
  - Basic costs **$9.99/mo** for 5,000 signatures, then $0.005 per signature.
  - It needs a paid Azure subscription.
  - Public Trust certificates are for organizations in the US, Canada, EU, UK and a few other countries. **Individual developers must be in the US or Canada.**
  - Identity validation takes 1–20 business days.
- **Linux tooling:**
  - **jsign 7.6** has a `TRUSTEDSIGNING` keystore type and also supports Azure Key Vault, AWS/GCP KMS and PKCS#11 ([jsign](https://ebourg.github.io/jsign/)).
  - **osslsigncode** signs PE, MSI and APPX files using PKCS#12 or PKCS#11 ([repo](https://github.com/mtrojnar/osslsigncode)).
  - Velopack's `--azureTrustedSignFile` "works on every OS."

## 5. Cost examples

Durations are taken as given and assumed equal on every provider (vendors claim theirs are faster).

**Scenario A (your examples):** Linux 2 min × 60 = 120 min; Windows 10 min × 10 = 100 min; macOS 20 min × 4 = 80 min.

| | GitHub Free org | Blacksmith |
|---|---|---|
| Linux | $0.72 | $0.48 (120 units) |
| Windows | $1.00 | $0.80 (200 units) |
| macOS | $4.96 | $6.40 (1,600 units) |
| **Total at list price** | $6.68, i.e. 56% of the $12 included | $7.68, i.e. 1,920 of 3,000 free units |
| **Billed** | **$0** | **$0** |

**Scenario B (10× everything):** Linux 1,200 min, Windows 1,000 min, macOS 800 min.

| Setup | Billed per month |
|---|---|
| GitHub only | $7.20 + $10.00 + $49.60 − $12 = **$54.80** |
| Blacksmith only | 19,200 units − 3,000 free = **$64.80** (its macOS is pricier per minute) |
| Linux + Windows on Blacksmith, macOS on GitHub | $0.80 + $37.60 = **$38.40** |
| Cross-compiling Windows on GitHub Linux instead of a Windows runner | saves **$4** ($10 → $6) |

**The risk of a hung job:** the default `timeout-minutes` is 360 ([workflow syntax](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax)). One stuck macOS job costs 360 × $0.062 = **$22.32**, nearly twice the monthly allowance.

---

## What fits Hyleon now vs at scale

### Now

These are recommendations. The ones that are your decision are listed separately below.

1. **Stay on GitHub-hosted runners.** The planned workload costs $0. Every alternative adds an app with access to the org, another bill, or machines to maintain, and saves under $1/month.
2. **Cut waste inside the workflows:**
   - Set `timeout-minutes` on every job.
   - Use `concurrency` with `cancel-in-progress` so superseded runs stop.
   - Use path filters so jobs only run when relevant files change.
   - Run macOS only on release tags or manual dispatch, and Windows only on `main` and tags.
   - Use `ubuntu-slim` for lint and format jobs under 15 minutes.
   - Use fewer, longer jobs, because each job rounds up to a minute.
   - Cache with rust-cache, which fits within the 10 GB repo cache.
   - Set short `retention-days` on artifacts, because the storage allowance is only 500 MB.
3. **Don't cross-compile Windows to save money.** The saving is cents, and `dx bundle` doesn't support it. Cross-compiling is worth revisiting only for other reasons, such as keeping all CI on one OS.
4. **Windows signing does not require a Windows runner.** Artifact Signing works through jsign or Velopack on any OS, if you are eligible.
5. **macOS needs a Mac.** Signing and notarizing on the same macOS runner is simplest. Moving them to Linux with `rcodesign` would save only a minute or two of macOS time.

**Decisions for you:**
- **How to cap spending.** No payment method gives a hard stop at $12, but CI is then blocked until the next month. A payment method plus a budget with "Stop usage" turned on gives a cap you choose.
- **Whether macOS builds run on pull requests or only on releases.** This is the biggest cost lever.
- **Whether Hyleon qualifies for Artifact Signing** (organization and country) and for **Visual Studio Community**, which matters if you use cargo-xwin.

### At scale

1. **macOS is the cost to watch.** Your levers are:
   - fewer and shorter macOS builds;
   - a third-party M4 runner, if its speed claims hold up in your own measurements;
   - a dedicated Mac. Scaleway breaks even at about 2,600 minutes/month. A Mac mini you own works as a self-hosted runner without loading your laptop.
2. **Linux and Windows at several thousand minutes/month:** a third-party provider cuts the cost to roughly ¼–⅔ of GitHub's (vendor prices). The trade-offs are vendor access to code and secrets, and the risk of the vendor shutting down.
3. **GitHub Team** ($4/user/month) raises the allowance to $18 and unlocks larger runners.
4. **Self-hosted Linux** only makes sense around 7,000+ minutes/month, or when warm caches matter. Use JIT/ephemeral runners. ARC only makes sense if you already run Kubernetes.
5. **Watch GitHub's postponed self-hosted fee.** If it returns at $0.002/min, it narrows the self-hosting margin, especially against Ubicloud and RunsOn, which are already about $0.001–0.002/min.

## Limits

- **Not found or not verified:**
  - Any GitHub changelog dating the switch from minutes to dollars.
  - Third-party claims that budgets default to $0 when a payment method is on file.
- **Vendor details I couldn't confirm:**
  - Blacksmith's billing granularity.
  - Which Depot plans include macOS.
  - Namespace's certifications.
  - WarpBuild's free tier.
- **Dioxus pipeline not tested:**
  - A full Dioxus 0.7 cargo-xwin + Velopack build was not tested.
  - Asset handling outside `dx bundle` is unknown.
- **Licences:**
  - The Visual Studio Build Tools and Apple licence readings are my interpretation, not legal advice.
  - The $99/year Apple Developer Program fee comes from general knowledge; I didn't re-check it.
- **Numbers that can move:**
  - EUR to USD conversion uses an unverified rate of about 1.1.
  - Hetzner availability changes over time.
  - Job durations in §5 are hypothetical.
- **Speed:** all vendor speed claims are unverified.
