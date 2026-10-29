# Appendix 3: The desktop release pipeline (agent report, 2026-10-29)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-29-delivery.md](../2026-10-29-delivery.md).

**The brief:** how the desktop app and CLI binaries get built, signed, packaged, updated and rolled back: Dioxus 0.7 bundling, Velopack, code signing on Windows and macOS, how comparable Rust desktop apps ship, release practice, and enterprise distribution.

---

# Hyleon desktop and CLI releases: build, sign, package, update, roll back

*Research date: 2026-10-29. Docs pages without a publish date are marked "accessed 2026-10-29". Nothing here was run or tested. Where a finding comes from reading source code, that is stated.*

## Summary

- **Build on each OS's own runner.** `dx bundle` cannot cross-build desktop apps. Velopack can build Windows and Linux packages from any OS, but macOS packages only on a Mac and MSIs only on Windows. So the pipeline needs a macOS runner and a Windows runner.
- **Velopack should do packaging and signing; Dioxus should only build.** Dioxus 0.7's Windows signing signs the installer file only, not the program inside it (from reading the source). Velopack signs the app's own files, its installer and the MSI.
- **Windows signing:** Azure Artifact Signing costs $9.99/month and is the cheapest route, but only if you are eligible. Individual developers must be in the US or Canada. EV certificates no longer skip SmartScreen warnings.
- **macOS signing:** $99/year Developer ID, hardened runtime, notarytool and stapling, all in CI. Velopack's `vpk` handles this, and so does `dx`.
- **Hosting:** GitHub Releases on a *private* repo cannot serve updates to the public without putting a token in the app. Cloudflare R2 (S3-compatible, no download fees) fits better.
- **The GitHub Free plan for private repos has hard limits that shape the pipeline:**
  - $12/month of included compute, and macOS minutes cost $0.062 each.
  - 500 MB of artifact storage.
  - No deployment environments or required reviewers.
  - No artifact attestations.

---

## 1. Dioxus 0.7 bundling

**Versions.** v0.7.10 is the latest stable release (2026-07-30). v0.8.0-alpha.3 came out 2026-10-16 (GitHub releases API).

**Formats** ([0.7 bundle docs](https://dioxuslabs.com/learn/0.7/tutorial/bundle), accessed 2026-10-29):

| OS | Formats |
|---|---|
| macOS | `.app`, `.dmg` |
| Windows | `.msi` (WiX), NSIS `.exe` |
| Linux | AppImage, deb, rpm |

The docs say: "`dx bundle` only builds desktop apps for the native platform and architecture". They recommend a CI matrix.

**How it is built.** By v0.7.10 the CLI has its own bundler (`packages/cli/src/bundler/{macos,windows,updater}.rs`) instead of the tauri-bundler it used before. It downloads WiX 3.14 (`candle`/`light`) and NSIS 3.11 (from reading the source).

**macOS signing is built in and driven by environment variables** (from reading `macos.rs` at v0.7.10):
- `APPLE_CERTIFICATE` and `APPLE_CERTIFICATE_PASSWORD` set up a temporary keychain.
- `APPLE_ID`/`APPLE_PASSWORD`/`APPLE_TEAM_ID`, or `APPLE_API_KEY`/`APPLE_API_ISSUER`/`APPLE_API_KEY_PATH`, trigger notarization and stapling.
- Missing credentials are not an error: you get an unsigned `.app`.

Two bugs fixed on 2026-05-04 ([#5527](https://github.com/DioxusLabs/dioxus/pull/5527), 0.7 backport [#5528](https://github.com/DioxusLabs/dioxus/pull/5528)):
- `hardened_runtime` defaulted to `false`, which breaks notarization.
- The ticket was stapled to the `.zip` instead of the `.app`.

[#6009](https://github.com/DioxusLabs/dioxus/pull/6009) (2026-10-25, on main only) retries `stapler staple`. Its write-up reports that stapling failed in 13 of 40 macOS x86_64 runs on GitHub's `macos-15-intel` runners in another project's release workflow.

**Windows signing.** Signing uses `certificate_thumbprint` (signtool) or a custom `sign_command`. In v0.7.10, `try_sign_windows` is called only on the finished `.msi` or NSIS `.exe`, not on the app's own `.exe` inside it (from reading the source; not tested).

**Other Windows issues:**
- The MSI is hard-coded to install for all users (per-machine). [PR #5799](https://github.com/DioxusLabs/dioxus/pull/5799), which adds a per-user option, is still open.
- [#5723](https://github.com/DioxusLabs/dioxus/issues/5723): bundles that include resource folders fail with `PermissionDenied` on Windows in 0.7.9 and 0.7.10. The fix is on main but not in the 0.7.x releases.

**Fitting Dioxus to Velopack.** `vpk pack` needs a folder: the app directory on Windows, or a `.app` on macOS. Velopack "copies it and uses it as-is" if the folder name ends in `.app`, and deep-signs everything by default ([Velopack macOS](https://docs.velopack.io/packaging/operating-systems/macos), [signing](https://docs.velopack.io/packaging/signing)).

`dx build --release` already writes a `.app` on macOS and an `app/` folder on Windows under `target/dx/...` (from reading `build/request.rs`; check the exact path during the trial). So Hyleon can feed the `dx build` output straight to `vpk`, without `dx bundle`'s installers or signing. That also avoids #5723.

## 2. Velopack

**Status and licence.**
- First 1.x release: 1.0.1 (2026-05-26). Current stable: 1.2.161 (2026-09-29). Pre-release: 1.3.39 (2026-10-17).
- The `velopack` crate is MIT-licensed (crates.io).
- `vpk` is a .NET tool, built for net8.0, net9.0 and net10.0. The docs advise using the same `vpk` version as the library your app uses ([GitHub Actions page](https://docs.velopack.io/distributing/github-actions)).

**What `vpk pack` produces** ([overview](https://docs.velopack.io/packaging/overview), [installer](https://docs.velopack.io/packaging/installer)):
- Common files: `-full.nupkg`, `-delta.nupkg` (only when a previous release is present), `-Portable.zip`, `releases.{channel}.json`, `assets.{channel}.json`.
- Windows: a one-click `Setup.exe` that installs per user to `%LocalAppData%`. With `--msi`, also an MSI that can install per user or per machine (`--instLocation`).
- macOS: a signed `.pkg`.
- Linux: an AppImage.
- Versions must be 3-part SemVer2. The MSI needs a separate `x.x.x.x` `--msiVersion`.

**Building for other OSes** ([cross-compiling](https://docs.velopack.io/packaging/cross-compiling)):
- `vpk [win]` and `vpk [linux]` work from any OS.
- macOS packages need a Mac ("hard dependencies on … codesign, xcrun, productbuild").
- MSIs build only on Windows.
- `--signParams` needs `signtool`, which runs only on Windows.
- `--azureTrustedSignFile` works on every OS only from 1.3, which is still a pre-release.
- Other certificates can go through `--signTemplate` with a tool such as jsign.

**Channels** ([channels](https://docs.velopack.io/packaging/channels), [switching](https://docs.velopack.io/integrating/switching-channels)):
- The default channel is just the OS name (`win`, `osx`, `linux`).
- Every OS-and-architecture combination needs its own channel, for example `win-x64-stable` or `osx-arm64-beta`.
- The channel is fixed when the package is built. At runtime, `ExplicitChannel` overrides it, usually together with `AllowVersionDowngrade`.

**Delta updates** ([deltas](https://docs.velopack.io/packaging/deltas)):
- Built with Zstandard, file by file. No single file may exceed 2 GB.
- In CI, `vpk download` fetches the previous release first so a delta can be made.
- The client falls back to the full package after 10 deltas (`MaximumDeltasBeforeFallback`, [docs.rs 1.2.161](https://docs.rs/velopack/latest/velopack/struct.UpdateOptions.html)).

**Hosting** ([self-hosting](https://docs.velopack.io/distributing/self-hosting)):
- `vpk upload` and `vpk download` support S3 (`--endpoint` for S3-compatible stores such as R2), Azure Blob, GitHub, Gitea and a local folder.
- Plain HTTP is download-only; you upload with your own tool.
- `--keepMaxReleases` permanently deletes older releases.
- The Rust crate's update sources include `HttpSource`, `FileSource` (a local or network folder), `GithubSource` and `NoneSource` ([docs.rs](https://docs.rs/velopack/latest/velopack/sources/index.html)).

**Rollback** ([specific version](https://docs.velopack.io/integrating/specific-version)):
- `AllowVersionDowngrade` lets the client move to a *lower* version that is now the latest in the feed. Installing an exact version is also possible.
- Startup auto-apply "will silently do nothing" for downgrades, so the app must apply them explicitly.
- Deltas are not used when downgrading.
- The target version must still be in the feed.
- This behaviour has to be in client code *before* you need it.

**Staged rollout** is only available in **Velopack Flow**, the hosted service. Each install has a random ID, the service hashes it, and the install gets the update if it falls inside the percentage slider ([tiered rollout](https://docs.velopack.io/distributing/Flow/tiered-rollout)). Flow has a free tier with limits. Paid prices are not listed in the docs (vendor). Self-hosted Velopack has no percentage rollout.

**Other notes:**
- `--framework webview2` installs the WebView2 runtime first ([bootstrapping](https://docs.velopack.io/packaging/bootstrapping)).
- The App Sandbox entitlement is not supported on macOS.
- The feed lists a SHA256 for each package. I found **no signature over the feed itself**. By contrast, Tauri's updater *requires* a minisign signature, and electron-updater checks code signatures on Windows ([electron-builder](https://github.com/electron-userland/electron-builder/blob/master/website/docs/features/auto-update.md)).

## 3. Code signing

### Windows

Microsoft compares the options in [Code signing options](https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/code-signing-options) (ms.date 2026-08-29).

**Azure Artifact Signing (formerly Trusted Signing)** became generally available in January 2026 ([DevClass, 2026-01-14](https://www.devclass.com/security/2026/01/14/code-signing-windows-apps-may-be-easier-and-more-secure-with-new-azure-artifact-service/4079554)).

- **Price:**
  - Basic: $9.99/month for 5,000 signatures.
  - Premium: $99.99/month for 100,000 signatures.
  - Billing starts when the account is created and is not prorated.
  - A free or trial Azure subscription is not accepted ([FAQ](https://learn.microsoft.com/en-us/azure/artifact-signing/faq), updated 2026-10-27).
- **Who can use it:**
  - The [quickstart](https://learn.microsoft.com/en-us/azure/artifact-signing/quickstart) (updated 2026-10-08) lists organizations in the US, Canada, EU, UK, Australia, New Zealand, Japan, South Korea, Singapore, Switzerland, Norway and Israel.
  - Individual developers must be in the US or Canada. For an individual, the certificate shows the person's legal name.
  - The 2026-08-29 options page lists a narrower set: US, Canada, EU and UK.
  - Earlier Microsoft pages required three years of business history for organizations. The current quickstart does not mention it. These sources conflict.
  - Identity checks take 1–20 business days.
- **Certificates** ([certificate management](https://learn.microsoft.com/en-us/azure/artifact-signing/concept-certificate-management), 2026-09-30):
  - Renewed daily and valid for 72 hours, so timestamping is required (`http://timestamp.acs.microsoft.com`). DevClass said 24 hours; Microsoft's docs say 72.
  - Keys are held in FIPS 140-3 Level 3 hardware security modules (HSMs).
  - No EV certificates, now or planned.
  - Its certificate authorities are "scheduled for removal from the Common CA Database" but stay on the Windows Platform Trust List. Signatures are trusted by Windows, but they don't qualify for the Microsoft Store's MSI/EXE route.
- **CI:** [azure/artifact-signing-action](https://github.com/Azure/artifact-signing-action) runs only on Windows runners and supports sign-in with GitHub's OIDC tokens. Alternatively, use Velopack's `--azureTrustedSignFile`.

**OV and EV certificates:**
- Since June 2023 the private key must be on an HSM or hardware token.
- OV costs about $150–300/year; EV costs $400+/year.
- Since 2024, EV no longer skips SmartScreen.
- Certificates issued from 2026-03-01 last at most 460 days ([CA/B Forum CSC-31](https://cabforum.org/2025/11/17/ballot-csc-31-maximum-validity-reduction)).

**SmartScreen** ([SmartScreen reputation](https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/smartscreen-reputation), 2026-05-04):
- New signed files still show warnings until they build reputation. This "can take several weeks and hundreds of clean installs."
- Reputation carries over between versions only when they are signed with the same identity.
- On Windows 11, Smart App Control blocks unsigned files outright.

### macOS

- **Apple Developer Program:** $99/year. An individual account shows the personal name as seller. An organization account needs a D-U-N-S number and a legal entity ([enroll](https://developer.apple.com/programs/enroll/)).
- **Notarization requirements:** a Developer ID certificate, the hardened runtime and a secure timestamp ([Apple](https://developer.apple.com/documentation/security/notarizing-macos-software-before-distribution)).
- **Tools:** use `notarytool`; `altool` uploads have been refused since 2023-11-01.
- **Stapling:** works on an app, a disk image or a flat `.pkg`. "You can't staple to [a ZIP]" and "it's not currently possible to staple tickets to [standalone binaries]" ([customizing workflow](https://developer.apple.com/documentation/security/customizing-the-notarization-workflow)). A bare `hyleon` CLI binary therefore depends on Gatekeeper checking online.
- **In CI:**
  - Zed uses an App Store Connect API key (`APPLE_NOTARIZATION_KEY/_KEY_ID/_ISSUER_ID`).
  - Velopack uses `--signAppIdentity`, `--signInstallIdentity`, `--notaryProfile` and `--keychain`.
- **rcodesign** (signing from Linux): the last release is 0.29.0 (2024-11-29). The repo gets dependency bumps through 2026. It is usable, but Velopack's macOS packaging needs a Mac anyway, so it would only save cost on the CLI.

## 4. How comparable Rust desktop apps ship

| App | Trigger and channels | Runners | Signing | Updates |
|---|---|---|---|---|
| **Zed** ([release.yml](https://github.com/zed-industries/zed/blob/main/.github/workflows/release.yml), updated 2026-10-26) | Tags `v*`. The channel (stable or preview, preview tags end in `-pre`) is fixed at build time from `crates/zed/RELEASE_CHANNEL`. Nightly cron `0 2,14 * * *` | Namespace macOS/Linux; self-hosted Windows | macOS: `codesign --options runtime` plus notarization with an API key. Windows: Azure Trusted Signing | Draft GitHub release → upload → asset check → preview publishes automatically (except the first of each minor version) → `after_release` refreshes `cloud.zed.dev` and publishes to winget. Nightlies go to the bucket `zed-nightly-host` |
| **Warp** ([create_release.yml](https://github.com/warpdotdev/warp/blob/main/.github/workflows/create_release.yml), AGPL, updated 2026-10-28) | Channels `dev` (nightly, deploys automatically), `preview` and `stable` (weekly, deployed manually) in `release_configurations.json` | `macos-26-xlarge`, `windows-latest-large`, Namespace Linux | Windows: `azure/artifact-signing-action` v2.0.0, pinned to a commit SHA. macOS: notarization | GitHub Releases plus Google Cloud Storage with a CDN (`immutable` cache headers). A pointer file, `channel_versions.json`, decides what is live (details below) |
| **Lapce** ([release.yml](https://github.com/lapce/lapce/blob/master/.github/workflows/release.yml)) | Nightly cron plus tags | `windows-latest`, `ubuntu-latest`, `macos-15` | macOS: certificate import plus notarization. Windows MSI (WiX): I found no signing step | GitHub Releases. The nightly release is deleted and recreated each time |
| **Tauri apps** ([tauri-action](https://github.com/tauri-apps/tauri-action), [updater](https://v2.tauri.app/plugin/updater/)) | Matrix: `macos-latest` ×2 targets, `ubuntu-22.04`, `windows-latest` | Per OS | Update signature is mandatory: "This cannot be disabled" | Static `latest.json` or a dynamic server (returns 204 when there is no update). Downgrades via `version_comparator`. No built-in staged rollout |

**Warp's pointer file.** Its fields include `version`, `version_for_new_users`, `update_by`, `soft_cutoff` and `is_rollback`. The client refuses to move to an older version unless `is_rollback` is true (`app/src/autoupdate/mod.rs`). Its `delete_release` workflow refuses to delete a release that is currently deployed.

**Common pattern:**
- Every app builds natively on each OS.
- Zed and Warp build separately per channel.
- "Promotion" happens *within* a channel: a draft becomes published, or a pointer file is edited.

## 5. Release practice

- **Versioning.** Keep one SemVer version in the workspace `Cargo.toml`. A tag `vX.Y.Z` triggers the release, and the workflow checks that the tag matches the version (Zed's `determine-release-channel` does this). Work out the MSI's 4-part version from the SemVer version.
- **Changelog.** Commit-based tools suit a workflow without pull requests, for example git-cliff (v2.14.2, 2026-09-18). Zed builds its notes from pull-request text, which Hyleon doesn't have. Velopack embeds Markdown notes with `--releaseNotes`. I found no Rust API for reading them; the docs only show C#.
- **Build once, promote.** Velopack's `releases.{channel}.json` works as a pointer file:
  1. Upload the packages to a staging location.
  2. Test the exact files.
  3. Publish the feed file last. That step is the release.
- **Pulling a bad update.** electron-builder warns that to withdraw a staged release "you **must** increment the version number higher than your broken release". So fixing forward is the default rollback for desktop apps. A true downgrade needs client support that already shipped (Velopack `AllowVersionDowngrade`, Warp `is_rollback`).
- **Kill switches and staged rollout.** These must be in the client before you need them. electron-builder uses `stagingPercentage` in its feed file. Velopack has rollout only in Flow. A self-hosted alternative (my suggestion, untested): the app reads a small `updates.json` (paused, percentage, blocked versions) and hashes its install ID before downloading.
- **Rehearse rollback.** Practise N → N+1 → back to N regularly on an internal channel.
- **Reproducibility.** These are practices Velopack lists for its own repos, from its [supply-chain post](https://docs.velopack.io/blog/2026/05/31/supply-chain-attacks) (2026-05-31):
  - Commit lockfiles and build with `--locked`.
  - Pin the Rust toolchain.
  - Pin `dioxus-cli` and `vpk` versions.
  - Pin actions to commit SHAs.
  - Run cargo-deny.

  Signing makes byte-identical rebuilds impossible, so record SHA-256 hashes of every shipped file.
- **GitHub Free constraints for a private org repo** ([billing](https://docs.github.com/en/billing/concepts/product-billing/github-actions), [environments](https://docs.github.com/en/actions/managing-workflow-runs-and-deployments/managing-deployments/managing-environments-for-deployment), [attestations](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/use-artifact-attestations)):
  - $12 of included compute per month. Rates: Linux $0.006/min, Windows $0.010/min, macOS $0.062/min, so the allowance covers only about 190 macOS minutes.
  - 500 MB of artifact storage.
  - **No** environments, environment secrets or required reviewers.
  - **No** artifact attestations.

## 6. Enterprise distribution

- **Windows MSI.** `vpk pack --msi --instLocation PerMachine` supports silent installs (`msiexec /i … /qn VELOPACK_INSTALLDIR=…`) and Group Policy deployment. It must be built on Windows. Velopack uses its own fork of WiX v7 and says WiX's Open Source Maintenance Fee "does not apply" (vendor claim). How Velopack self-updates a *per-machine* install is not documented.
- **Common enterprise pattern.** Managed installs turn self-update off and IT pushes new MSIs. Example: 1Password's `MANAGED_UPDATE=1`, which admins report is sometimes unreliable ([community](https://www.1password.community/1password-at-work-58/expected-behaviour-of-the-newish-managed-update-1-msi-deployment-parameter-22227)).
- **MSIX.** The Microsoft Store re-signs MSIX packages for free, but Velopack does not produce MSIX.
- **macOS for MDM.** Velopack's `.pkg` is signed with Developer ID Installer and adds a `postinstall` script that launches the app. Intune's managed LOB route requires a Developer ID Installer–signed `.pkg` containing a single app that installs to `/Applications`. Intune notes that packages with scripts may not work as managed LOB apps ([Intune LOB](https://learn.microsoft.com/en-us/mem/intune/apps/lob-apps-macos)). Whether Velopack's `.pkg` passes needs a test.
- **Offline and air-gapped.** Point the app at an internal mirror with `FileSource` or `HttpSource`, or use `NoneSource` to disable updates. WebView2 needs an offline plan on machines that don't have it.
- **Turning off updates by policy.** VS Code is a model: an `UpdateMode` policy set through ADMX templates and stored in the registry under `Software\Policies\Microsoft\VSCode`, a macOS `.mobileconfig` profile, and `/etc/vscode/policy.json` on Linux ([VS Code policies](https://code.visualstudio.com/docs/enterprise/policies)).

---

## What fits Hyleon now vs at scale

### Proposed pipeline now

1. **`ci.yml` on every push to main.** Linux only: fmt, clippy, tests, web build. Optionally, a weekly unsigned macOS/Windows build to catch platform breakage without spending macOS minutes on every commit.
2. **`release.yml` on tag `v*`.**
   - **prepare** (Linux): check the tag against `Cargo.toml`; generate notes with git-cliff.
   - **macos** (`macos-15`, arm64):
     1. `dx build --release`.
     2. `vpk download s3 --endpoint &lt;R2&gt;` to enable deltas.
     3. `vpk pack` with signing, notary and keychain flags, channel `osx-arm64-stable`.
     4. Sign the CLI and notarize it as a zip.
     5. Check with `codesign --verify --deep --strict`, `spctl -a`, `stapler validate`.
     6. Upload to an R2 `staging/vX.Y.Z/` folder.
   - **windows** (`windows-2025`):
     1. `dx build --release`.
     2. `vpk pack` with `--azureTrustedSignFile` (OIDC sign-in), `--framework webview2`, channel `win-x64-stable`.
     3. Sign `hyleon.exe`.
     4. Check signatures with `Get-AuthenticodeSignature`.
     5. Install `Setup.exe --silent` on the runner and run a self-test of the installed app (this tests the exact shipped files).
     6. Upload to R2 staging.
   - Upload straight to R2 rather than passing files between jobs as GitHub artifacts, because of the 500 MB storage cap.
3. **`promote.yml`, started by hand with version and channel as inputs.** This replaces the required-reviewer gate that Free can't provide. It copies staging to live: packages first, feed file last. The feed gets a short cache lifetime; versioned files get `immutable`. Rollback means running promote again with the previous feed (for clients with downgrade enabled), or shipping N+1.

### Built into the client from v1

- `VelopackApp::build().run()` as the first line of `main`.
- Update checks off the UI thread (the crate has no async feature).
- `AllowVersionDowngrade` handling.
- A remote `updates.json` kill switch.
- A policy check (environment variable, registry, plist) that switches to `NoneSource`.
- An internal `canary` channel, reached via `ExplicitChannel`, to rehearse rollbacks.

### Rough fixed costs

| Item | Cost |
|---|---|
| Apple Developer Program | $99/year |
| Windows signing | $9.99/month (Artifact Signing, if eligible), or OV ~$150–300/year plus a cloud HSM |
| R2 hosting | about $0 under 10 GB |
| CI | macOS minutes dominate. My estimate: a few dollars per release |

### Trade-offs to settle

- **Velopack version:** stay on 1.2.x (stable; Windows signing must run on a Windows runner) rather than 1.3 (pre-release; signs from any OS).
- **macOS architecture:** arm64 only (half the macOS minutes) or also x86_64. Universal binaries with Velopack are unverified.
- **MSI:** ship it now, or wait until the first enterprise customer.

### At scale

- Add per-OS `beta` channels and a Warp-style pointer file with an `is_rollback` flag.
- Add percentage rollout, through Velopack Flow (paid prices not listed in its docs) or an app-side gate.
- Ship a per-machine MSI with self-update off by policy, plus ADMX and `.mobileconfig` templates.
- Offer an offline bundle for air-gapped customers: installer plus a feed mirror.
- Move to GitHub Team for environments and required reviewers. Artifact attestations on private repos need Enterprise Cloud; signing provenance with your own key is the alternative.
- Use larger runners if build time starts to matter.

### Decisions only you can make

- Which legal entity and country signing goes under. Individuals outside the US and Canada cannot use Artifact Signing.
- Whether to support Intel Macs.
- Whether the CLI also ships inside the app, as Zed and Warp do, or only as separate archives.

## Limits

- Nothing was built or run. The Dioxus and Velopack behaviour above comes from docs and source reading (v0.7.10, Velopack 1.2.161).
- **Unverified:**
  - The exact `dx build` output paths.
  - Whether `vpk` staples notarization tickets.
  - Velopack self-update on per-machine MSI installs.
  - Velopack universal macOS builds.
  - A Rust API for reading release notes.
  - How the Velopack `.pkg` behaves under Intune.
- Artifact Signing eligibility pages conflict (country lists, the three-year history rule). Confirm in the Azure portal before relying on it.
- Velopack Flow pricing and the GitHub per-release cost are not published or are my estimates.
- How Warp's and Zed's update servers behave is inferred from client code and workflow files.
- I found no evidence that Velopack signs its update feed. That should be checked before enterprise sales.
