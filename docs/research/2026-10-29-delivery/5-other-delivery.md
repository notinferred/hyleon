# Appendix 5: Web app, API, site, packs, watcher and CLI delivery (agent report, 2026-10-29)

The research agent's full report, verbatim; never edited. Synthesis: [../2026-10-29-delivery.md](../2026-10-29-delivery.md).

**The brief:** delivery for everything that isn't the desktop app: hosting and deploying the web app and the website, delivering the API server to one host, testing against real dependencies in CI, data packs as releases, the scheduled region watcher and detecting missed runs, and shipping the CLI for customers' CI.

---

# Delivery for everything except the desktop app: research report

Researched 2026-10-29. Labels: **vendor docs** means the vendor describing its own product, **third-party** means blogs or aggregators, and **inference** means my own reasoning with no source behind it. Nothing was run or measured.

## 0. What GitHub Free with a private repo rules out

Several of these limits shape the answers below.

- **No gated environments.** On the Free plan, environment secrets, required reviewers, wait timers and deployment-branch rules work only in public repositories ([GitHub: deployments and environments](https://docs.github.com/en/actions/reference/workflows-and-actions/deployments-and-environments); [github.com/pricing](https://github.com/pricing)).
  - Every secret is therefore repo-level, and any workflow on any branch can read it.
  - Team ($4 per user per month) adds environment secrets and deployment branches for private repos. Required reviewers on private repos need Enterprise.
- **Actions budget.** Free includes 2,000 Linux minutes a month, stated as $12 of compute; Linux 2-core is $0.006 per minute and macOS is about 10 times that. Artifact and Packages storage is 500 MB shared, and the cache is 10 GB per repo ([Actions billing](https://docs.github.com/en/billing/concepts/product-billing/github-actions)).
- **Customer-facing files need a public home.**
  - Only people with read access to the repo can see its releases ([About releases](https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases)).
  - Actions in a private repo can be shared only inside the same org ([share with your org](https://docs.github.com/en/actions/how-tos/reuse-automations/share-with-your-organization)).
  - Marketplace listing requires a public repo with one root `action.yml` ([Marketplace docs](https://docs.github.com/en/enterprise-cloud@latest/actions/how-tos/create-and-publish-actions/publish-in-github-marketplace)).
- **Artifact attestations are not stored.** Storage works only for public repos or Enterprise Cloud. Private repos on other plans must set `skipWrite` ([@actions/attest README](https://cdn.jsdelivr.net/npm/@actions/attest@3.2.0/README.md)).
- **Pushes made with `GITHUB_TOKEN` start no workflows.** Only `workflow_dispatch` and `repository_dispatch` are exceptions ([GITHUB_TOKEN docs](https://docs.github.com/en/actions/concepts/security/github_token)). This matters for the watcher in §5.
- **Node 20 is gone from runners.** It was removed on 2026-09-23, so JavaScript actions must target Node 24 ([changelog](https://github.blog/changelog/2026-09-23-node-20-is-no-longer-available-in-github-actions)). Composite actions avoid this kind of churn.

## 1. Web app (Dioxus wasm) and website (Astro)

**Pages or Workers.**
- Cloudflare's Pages docs now show a banner: "Start new projects with Workers" ([Pages docs](https://developers.cloudflare.com/pages/), updated 2026-08-25). No end-of-life date has been published.
- The migration guide (updated 2026-10-12) lists what Pages lacks: Cron Triggers, Workers Logs and gradual deployments. Workers lacks custom domains outside Cloudflare zones ([migrate from Pages](https://developers.cloudflare.com/workers/static-assets/migrate-from-pages/)).
- Astro's own guide says Cloudflare recommends Workers. A static Astro site needs only `assets.directory = "./dist"` and no adapter ([Astro deploy guide](https://docs.astro.build/en/guides/deploy/cloudflare/)).
- Cloudflare acquired the Astro team on 2026-01-16, and Astro stays MIT-licensed ([press release](https://www.nasdaq.com/press-release/cloudflare-acquires-astro-accelerate-future-high-performance-web-development-2026-01)).

**Costs and limits (vendor docs).**
- Requests for static assets are free and unlimited, and storage is free ([billing](https://developers.cloudflare.com/workers/static-assets/billing-and-limitations/), 2026-04-23).
- Free plan limits: 20,000 files per version (100,000 on paid), 25 MiB per file, and 100,000 Worker invocations a day ([limits](https://developers.cloudflare.com/workers/platform/limits/), 2026-10-12). The 25 MiB file limit is the hard ceiling for the `.wasm`.
- Routes matched by `run_worker_first` count as Worker invocations, and on the free plan they return 429 once over the limit.
- SPA fallback is configured with `not_found_handling = "single-page-application"` ([SPA routing](https://developers.cloudflare.com/workers/static-assets/routing/single-page-application/)).

**Previews without pull requests.** There are three mechanisms.
1. **Versions.** `wrangler versions upload` creates a version, including its static assets, with its own Version URL. `versions deploy` then sends traffic to that exact version ([versions &amp; deployments](https://developers.cloudflare.com/workers/configuration/versions-and-deployments/), 2026-09-22). Version URLs use production bindings, which doesn't matter for a pure static Worker. This gives "build once, promote" with no extra tooling.
2. **Worker Previews.** Launched 2026-09-22 and requires Wrangler 4.135.0 or later. `wrangler preview` creates a stable URL per branch plus an immutable URL per deployment. Limits are 100 previews per Worker on Free and 500 on paid; the least recently deployed are removed first ([changelog](https://developers.cloudflare.com/changelog/post/2026-09-22-worker-previews/); [Previews](https://developers.cloudflare.com/workers/previews/)). Cloudflare recommends these over aliased Version URLs for branches ([compare workflows](https://developers.cloudflare.com/workers/previews/compare-workflows/)).
3. **Wrangler environments.** Each environment is a separate, persistent Worker.

**CI.**
- `cloudflare/wrangler-action@v4` defaults to Wrangler v4, supports `command: versions upload`, and outputs `deployment-url`. Its preview outputs need Wrangler 4.136.3 or later ([wrangler-action](https://github.com/cloudflare/wrangler-action)).
- The API token should use the "Edit Cloudflare Workers" template, scoped to the account ([GitHub Actions guide](https://developers.cloudflare.com/workers/ci-cd/external-cicd/github-actions/), 2026-10-18).

**Caching and wasm size.**
- Workers sends `Cache-Control: public, max-age=0, must-revalidate` plus an ETag by default. A `_headers` file (up to 100 rules) can override this for asset responses ([headers](https://developers.cloudflare.com/workers/static-assets/headers/)).
- Dioxus `asset!` adds content hashes to filenames, which "allow[s] you to infinitely cache the asset" ([Dioxus assets](https://dioxuslabs.com/learn/0.7/essentials/ui/assets)).
- Dioxus 0.7 (2025-09-08) added per-route WASM-Split, built with `dx bundle --experimental-wasm-split`. No official size numbers have been published ([0.7 release](https://dioxuslabs.com/blog/release-070); [router docs](https://docs.rs/dioxus-router/)).

**Alternatives.**
- **Vercel:** the Hobby plan is non-commercial only, so Hyleon would need Pro ([fair use](https://vercel.com/docs/limits/fair-use-guidelines)).
- **Netlify:** the Free plan has 300 credits a month and each production deploy costs 15, while preview deploys are free ([Netlify changelog](https://www.netlify.com/changelog/netlify-pricing-update-introducing-credit-based-plans/)). That is about 20 production deploys a month, which is tight when every commit goes straight to main.

## 2. API server delivery to one host

**Building the image.**
- Buildx `type=gha` caches image layers but not `RUN --mount=type=cache` mounts; those need `buildkit-cache-dance` ([Docker docs](https://docs.docker.com/build/ci/github-actions/cache/)). The gha cache backend is also rate-limited ([gha backend](https://docs.docker.com/build/cache/backends/gha/)).
- Inference: for Rust, it is simpler to compile on the runner with a cargo cache and `COPY` the binary into a minimal runtime image. Docker then only packages the binary.

**Registry (GHCR).**
- Container storage and bandwidth are "currently free", with at least one month's notice before any change ([Packages billing](https://docs.github.com/en/billing/concepts/product-billing/github-packages)).
- A host pulling private images needs a **classic** personal access token with `read:packages` ([container registry docs](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry)).
- A package can be made public even if its repo is private. This cannot be undone ([visibility](https://docs.github.com/en/packages/learn-github-packages/configuring-a-packages-access-control-and-visibility)).

**Deploy options.**

| Option | What it gives | Notes |
|---|---|---|
| **Kamal 2** (v2.12.0, June; year inferred as 2026) | Pulls the image, starts the new container, kamal-proxy switches traffic once `GET /up` returns 200, then stops the old container ([deploy](https://kamal-deploy.org/docs/commands/deploy/)). `--skip-push` deploys a pre-built image. `kamal rollback &lt;version&gt;` restarts an old container; old containers are pruned after 3 days ([rollback](https://kamal-deploy.org/docs/commands/rollback/)). Let's Encrypt on a single server ([proxy](https://kamal-deploy.org/docs/configuration/proxy/)). `.kamal/secrets` adapters for 1Password, Bitwarden, AWS, GCP, Doppler and others ([secrets](https://kamal-deploy.org/docs/commands/secrets/)). | Works with any language. kamal-proxy checks health only during the deploy: "Once the app is up, the proxy will stop hitting the healthcheck endpoint." |
| **Binary + systemd** | With socket activation, systemd holds the listening socket during a restart, so new connections wait in the kernel queue. In-flight connections are lost without a graceful shutdown ([Bernat, 2018](https://vincent.bernat.ch/en/blog/2018-systemd-golang-socket-activation)). | Fewest moving parts. You build rollback yourself, for example with a symlink swap. |
| **Docker Compose** | No built-in traffic swap (inference). | You would add a proxy yourself. |
| **Dokku / Coolify** | Heroku-style or UI-driven self-hosted platforms. Dokku moved health checks from `CHECKS` to `app.json` ([0.31.0](https://dokku.com/blog/2023/dokku-0.31.0)). Coolify v4 reportedly went stable in 2026 (third-party). | More platform than one service needs (inference). |
| **Fly.io** | No free tier for new orgs. 256 MB machine $2.19 a month; Managed Postgres from $38 a month ([pricing](https://docs.fly.io/about/pricing)). | Doesn't match the "one host" decision. |

**Migrations by one actor.**
- In sqlx 0.9.0 (2026-05-06), `Migrator` locks the database with Postgres advisory locks by default, and `run()` checks the checksums of already-applied migrations ([Migrator](https://docs.rs/sqlx/latest/sqlx/migrate/struct.Migrator.html); [changelog](https://raw.githubusercontent.com/launchbadge/sqlx/main/CHANGELOG.md)).
- Pattern (inference):
  - The pipeline runs a `migrate` subcommand from the **same image**, before the traffic switch.
  - Only "expand" migrations go out with a release. "Contract" migrations ship in a later release, once no code reads the old shape.
  - The app never migrates itself at boot.

**Health checks.** Point Kamal's check at a readiness endpoint: database reachable and schema at or above the version the binary needs. Have an external uptime monitor hit a liveness endpoint continuously, because Kamal stops checking after the deploy.

## 3. Testing against real dependencies in CI

**Service containers.**
- Linux runners only. Use `--health-cmd pg_isready`, and map ports when the job runs directly on the runner ([GitHub guide](https://docs.github.com/en/actions/tutorials/use-containerized-services/create-postgresql-service-containers)).
- Started once per job.

**testcontainers-rs.**
- Version 0.28.0, released 2026-08-06 ([crates.io](https://crates.io/api/v1/crates/testcontainers)). It has both async and blocking runners ([repo](https://github.com/testcontainers/testcontainers-rs)).
- It needs Docker, which on GitHub-hosted runners means Linux. Each container pays its own startup cost; I found no Rust benchmark comparing it with service containers.

**`#[sqlx::test]`.**
- Creates a new database per test on one server and applies `migrations/` automatically. It deletes the database when the test passes and keeps it when the test fails ([sqlx::test](https://docs.rs/sqlx/latest/sqlx/attr.test.html)).
- It needs a superuser `DATABASE_URL`.
- Combined with one service container, this gives per-test isolation with a single container.

**Offline mode.**
- `cargo sqlx prepare --workspace` writes `.sqlx/`. With that directory committed and `SQLX_OFFLINE=true`, builds need no database.
- `cargo sqlx prepare --check` fails CI when `.sqlx` is stale ([sqlx-cli README](https://github.com/launchbadge/sqlx/blob/main/sqlx-cli/README.md)). My understanding is that the check needs a live database, so it belongs in the job that has the service container.

## 4. Data packs as releases

**How other ecosystems do it.**

| Ecosystem | Versioning and lock | Tests | Signing and distribution |
|---|---|---|---|
| OPA bundles | `.manifest` with `revision` and `roots` | `opa test` | `.signatures.json` holds JWTs listing each file's hash. Served over HTTP with ETag, or via OCI ([bundles](https://www.openpolicyagent.org/docs/management-bundles)) |
| Semgrep | Registry | `semgrep --test` with `ruleid:` / `ok:` annotations in sibling files ([testing](https://docs.semgrep.dev/writing-rules/testing-rules)) | Registry |
| Gatekeeper | n/a | `gator verify` Suite files declaring expected violation counts and message regexes ([gator](https://open-policy-agent.github.io/gatekeeper/website/docs/gator)) | n/a |
| Kyverno | n/a | `kyverno-test.yaml` declaring expected pass, fail or skip ([test](https://kyverno.io/docs/kyverno-cli/usage/test)) | n/a |
| Falco rules | Floating major and minor tags; `required_engine_version` | n/a | OCI artifacts on ghcr.io ([CNCF](https://cncf.io/blog/2025/03/10/falcoctl-artifact-management-for-falco)) |
| Terraform | `v`-prefixed semver tags. The lock file records an `h1:` hash of the package **contents**; `init` fails on a mismatch ([lock file](https://developer.hashicorp.com/terraform/language/files/dependency-lock)) | n/a | `SHA256SUMS` plus a GPG signature is mandatory ([publishing](https://developer.hashicorp.com/terraform/registry/providers/publishing)) |
| ESLint configs | npm package; peer dependency `eslint &gt;= lowest` ([docs](https://eslint.org/docs/latest/extend/shareable-configs)) | n/a | npm |
| Kubernetes CEL | ValidatingAdmissionPolicy GA in 1.30 | Offline testers such as kat and kaptest (third-party) | n/a |

**Patterns that carry over to Hyleon (inference).**
- Each pack declares a semver version and a minimum engine version, as Falco and ESLint do.
- Lock the sha256 over canonical **contents**, not the archive. Terraform's `h1:` versus `zh:` split shows why: a contents hash stays the same however the pack is packaged.
- Keep test projects next to each pack with declared expected results, as Semgrep, gator and Kyverno do.

**Signing.**
- **minisign** uses Ed25519, and its "trusted comment" is signed too, so it can carry the pack name and version to block downgrades ([minisign](https://jedisct1.github.io/minisign/)). The Rust crate `minisign-verify` 0.3.0 (2026-09-25) has no dependencies ([crates.io](https://crates.io/api/v1/crates/minisign-verify)). cargo-binstall also verifies minisign ([SIGNING.md](https://github.com/cargo-bins/cargo-binstall/blob/main/SIGNING.md)).
- **Sigstore keyless signing** writes the org, repo and workflow names to the public Rekor log ([GitHub blog, 2021](https://github.blog/security/supply-chain-security/safeguard-container-signing-capability-actions/)). That leaks private repo names.

**Running only the packs that changed.**
- `on.push.paths` uses a two-dot diff on push. If a push has more than 1,000 commits the workflow always runs; if the diff has more than 3,000 files and the matched files aren't in the first 3,000, it doesn't run ([workflow syntax](https://docs.github.com/en/actions/reference/workflows-and-actions/workflow-syntax)).
- For per-pack granularity: one job diffs `github.event.before..github.sha` and emits a matrix of changed packs (inference).
  - Run **all** packs when the engine changes.
  - Fall back to all packs when `before` is all zeros (new branch or force push).
- Avoid unpinned third-party "changed-files" actions. `tj-actions/changed-files` was compromised in March 2025 (CVE-2025-30066, [CISA](https://www.cisa.gov/news-events/alerts/2025/03/18/supply-chain-compromise-third-party-github-action-cve-2025-30066)).

## 5. Scheduled watcher and detecting missed runs

**GitHub Actions `schedule`** ([events docs](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows)).
- Runs can be delayed under high load, especially at the start of every hour, and "some queued jobs may be dropped". It runs only on the default branch, at most every 5 minutes.
- The 60-day auto-disable is documented **for public repos only**. The docs say nothing about private repos ([disable workflows](https://docs.github.com/en/actions/how-tos/manage-workflow-runs/disable-and-enable-workflows)).
- Failure notifications go to whoever last edited the cron line.
- A July 2026 community report (public repo, new account) showed 8–14-hour delays and skipped days, with no answer from GitHub staff ([discussion #201738](https://github.com/orgs/community/discussions/201738)).

**Cloudflare Cron Triggers** ([docs](https://developers.cloudflare.com/workers/configuration/cron-triggers/), 2026-10-17).
- Times are UTC, and changes take up to 15 minutes to take effect. The dashboard keeps only the last 100 invocations. The account limit is 5 triggers on Free and 250 on paid.
- No retry policy is documented.
- An aggregator recorded 2026 incidents of delayed, missed and duplicated runs (third-party, [isdown](https://isdown.app/status/cloudflare/incidents/675529-delayed-missed-and-duplicated-workers-cron-triggers)).
- Inference: a poor home for a Rust agent that writes to a git repo.

**Cron on the API host.**
- Avoids GitHub's queue.
- Ties the watcher to the API host's uptime and puts repo-write credentials on an internet-facing server (inference).

**Missed-run alarms (dead-man's switch services).**

| Service | Free tier | Paid | Notes |
|---|---|---|---|
| [Healthchecks.io](https://healthchecks.io/pricing/) | 20 checks | Business $20 a month for 100 checks | Cron schedules with timezone, grace time, and `/start` and `/fail` signals ([docs](https://healthchecks.io/docs/configuring_checks/)). BSD-3 licensed and self-hostable ([repo](https://github.com/healthchecks/healthchecks)) |
| [Cronitor](https://cronitor.io/pricing) | 5 monitors | $2 per monitor plus $5 per user per month | |
| [Better Stack](https://betterstack.com/pricing) | 10 monitors and 10 heartbeats (as reported in search results; details from third parties) | | |

**Pattern (inference).**
- Ping `/start` when the run begins and success only after the proposal is durably recorded; send `/fail` on errors.
- Keep the alarm on a different vendor from the scheduler.
- Schedule an idempotent run twice a day at odd minutes, so one dropped run does not trigger the alarm.

## 6. Shipping the CLI for customers' CI

**dist (formerly cargo-dist)** is actively released: 0.33.1 on 2026-10-21, 0.33.0 in September (adds Azure signing for Windows), 0.32.0 in May ([releases](https://github.com/axodotdev/cargo-dist/releases)).
- It generates a `release.yml` that plans, builds, hosts, publishes and announces.
- Installers: shell, PowerShell, npm, Homebrew, MSI. Checksums default to sha256.
- **`github-releases-repo`** publishes GitHub Releases to a different repo, so a private monorepo can release into a public repo.
- `simple-download-url` (0.31 and later) points installers at a static host, but "it's up to you to actually upload the files" ([config](https://axodotdev.github.io/cargo-dist/book/reference/config.html)).
- The old docs domain no longer resolves; docs are now at axodotdev.github.io.
- uv uses dist 0.32.0 with 18 targets, attestations, and a mirror at releases.astral.sh ([dist-workspace.toml](https://github.com/astral-sh/uv/blob/main/dist-workspace.toml)).

**cargo-binstall.**
- Finds archives in GitHub Releases through URL templates, or through `[package.metadata.binstall] pkg-url` for other hosts ([SUPPORT.md](https://github.com/cargo-bins/cargo-binstall/blob/main/SUPPORT.md)).
- Optionally verifies minisign signatures from a `.sig` file next to the archive.

**How other tools ship their Actions.**
- [ruff-action](https://github.com/astral-sh/ruff-action) and [setup-uv](https://github.com/astral-sh/setup-uv) download from Astral's mirror and fall back to GitHub Releases. ruff-action automatically verifies sha256 for known versions; in setup-uv a checksum is optional. Both read the version from the project's own files.
- [setup-biome](https://github.com/biomejs/setup-biome) reads the version from the lockfile or `package.json`.
- typos is a composite action that `wget`s a pinned binary **with no checksum check** ([entrypoint.sh](https://raw.githubusercontent.com/crate-ci/typos/master/action/entrypoint.sh)).
- [cargo-deny-action](https://github.com/EmbarkStudios/cargo-deny-action) appears to be a Docker container action (inferred from its files), and container actions run only on Linux.

## What fits Hyleon now, and at scale

**Now (pre-revenue, solo, $0–low cost):**
1. **Site and web app.** Two static-asset Workers. On each push to main:
   - build, then `wrangler versions upload`;
   - smoke-test the Version URL;
   - `wrangler versions deploy` that same version.

   Add `_headers` rules so hashed asset paths are cached long-term. Free tier.
2. **API.**
   - Build the binary once per commit and wrap it in a minimal image tagged by SHA in GHCR.
   - Run the pipeline's `migrate` step, then `kamal deploy --skip-push` of that tag.
   - Kamal checks the readiness endpoint; an external monitor watches liveness.
   - Wrap deploys in a `concurrency` group so they run one at a time.
   - Trade-off: Kamal adds Docker and Ruby tooling; binary + systemd adds hand-rolled rollback.
3. **Tests.** One Postgres service container plus `#[sqlx::test]`. Commit `.sqlx` and build with `SQLX_OFFLINE`, plus a `prepare --check` job. "Ephemeral environment per change" is realised as a per-commit CI environment (API image plus Postgres plus the web build), not hosted API previews.
4. **Packs.** Manifest with an engine version range, a contents hash in the lock, a per-pack test matrix, and minisign signatures.
5. **Watcher.** GitHub Actions schedule, twice daily and idempotent, with a Healthchecks.io cron check on a separate vendor.
6. **CLI.**
   - dist publishes releases to a public repo.
   - A composite Action in its own public repo downloads a pinned version, verifies sha256 or minisign, and runs `hyleon check`.
   - A public GHCR image covers GitLab, Jenkins and other CI.

**At scale:**
- GitHub Team for environment secrets, or Enterprise for required reviewers and private attestations.
- A second host with Kamal rolling deploys, or a staging host.
- OCI distribution for marketplace packs (OPA and Falco precedent).
- Public Sigstore signing for public packs.
- A buildx registry cache or larger runners if build times grow.

**Decisions for you; I have not assumed any of these:**
- **Public repos.** Should there be public repos for releases and the Action? GitHub's rules effectively require this, unless binaries go on an R2 custom domain instead. The `r2.dev` URL is rate-limited and for development only ([R2 public buckets](https://developers.cloudflare.com/r2/buckets/public-buckets/)).
- **How the watcher "proposes" without pull requests.** Options are a branch plus an issue, or a review page in the web app. Note that pushes made with `GITHUB_TOKEN` won't trigger CI, so this needs a GitHub App token or an explicit dispatch.
- **Kamal or plain systemd.**
- **Whether to pay for Team.** Repo-level secrets are readable from any branch, including branches an agent pushes.

## Limits

- **Nothing measured.** No hands-on runs, build timings or benchmarks. I found no primary source comparing testcontainers with service containers on time.
- **Third-party only:**
  - Coolify versions.
  - Better Stack free-tier details.
  - Cloudflare cron incidents.
  - The GitHub delay report (one public-repo anecdote).
- **Inferred:** Kamal release years. Whether GHCR's "free" covers private images is my reading of the docs, which don't separate public and private.
- **Not verified:**
  - Dioxus 0.7 wasm sizes and hashing of the main `.wasm`/`.js` files.
  - Whether Workers serves `dx`'s pre-compressed `.br` files.
  - That `sqlx prepare --check` needs a live database (my understanding).
- **Silence, not a statement:** the 60-day schedule rule for private repos is absent from the docs, not explicitly excluded.
- **Prices change.** Confirm them before buying anything.
