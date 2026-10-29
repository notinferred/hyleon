# Region profile format, version 1

**Status: planned** (D-019).

A region profile says what is in force in one place: which code editions apply, which local amendments change them, and the conventions drawings follow there. Picking a region saves people from researching that themselves.

A profile is data, like a pack, kept in `regions/` and versioned. It never reproduces code text (D-017). It names codes, editions and clauses, and cites where each fact comes from.

## Fields

| Field | Holds |
|---|---|
| `id`, `version` | For example `us-fl@1.4.0`; semver, like packs |
| `name` | For example "Florida, USA" |
| `verified_on`, `reviewer` | The date a person last checked every fact against its source, and who checked |
| `[[codes]]` | Each code in force: `code` (e.g. NFPA 70), `edition`, `in_force_from`, `adopted_by` (the body), and its `[[amendments]]`. Each amendment has the clause, a summary in Hyleon's own words, and a citation to the statute or administrative rule with its link |
| `edition_rule` (in each code) | Which date fixes the applicable edition: for example, the edition in force on the date plans are submitted (as in Ontario), or on the permit application date. Transition periods are listed with their end dates |
| `[[local_triggers]]` | A local authority's own thresholds and checklists, each cited. For example, an engineer's seal required for services of 1,600 A and above, or the authority's plan-review checklist with its revision date |
| `[conventions]` | Units, paper sizes (ANSI or ISO), symbol set (IEEE 315 or IEC 60617), conductor colours, label languages, supply voltages and frequency |
| `[[design_rules]]` | Notes on who may design what, for example a contractor's design limits, each with its citation. These are notes, never legal determinations |
| `[[sources]]` | Every source, with its URL and the date it was read |
| `note` | Always "Confirm with your authority having jurisdiction." |

## Where the facts come from

Each source is read by a person when a profile is first written, and watched afterwards:
- the state's or province's own rulemaking register and adoption notices, which are authoritative
- ICC's code adoption chart, which warns that not every jurisdiction reports to it
- ICC's paid Code Connect API, and General Code's eCode360 for municipal amendments, each under its licence terms
- national bodies' notices, such as the IET for BS 7671
- utility tariffs, for interconnection rules
- authorities' plan-review checklists, for local triggers

**What may be quoted:**
- Enacted local amendments are government-written law, and can be quoted (Georgia v. Public.Resource.Org, 2020).
- Privately written model codes are cited by clause, never quoted (D-017).
- Ten States Standards 2022 may be copied with credit.

The full findings are in [research](../research/2026-10-28-blind-spots/3-inspections.md).

## How a project uses it

- **Choosing.** A person chooses the region with `set_region` ([project.md](project.md)).
- **Answers.** `site-basics` answers default from the profile, with a `standard` source whose `via` names the profile. The person can change any of them.
- **Locking.** The profile's version and sha256 are locked in `packs.lock.json`, so a project keeps checking against what was in force when it was designed. Upgrading is a recorded command, made by a person.

## Updates

- **The watcher.** A watcher runs every day over each profile's sources: adoption maps, rulemaking notices, and standards bodies' edition announcements.
- **Proposals only.** It never changes a released profile. When a fact changes, it proposes a new version with the changed facts and their sources, marked unverified.
- **Release.** A person verifies the proposed version and releases it.
- **Advisories.** Projects locked to an older version are told what changed and which checks it affects. The person decides whether to upgrade.
- **Absence alarm.** If the watcher misses a run, an alarm goes off. Silence is never read as "no change" ([ARCHITECTURE.md](../ARCHITECTURE.md), absence detection).

## Example

Illustrative: the values are made up, not any real jurisdiction's.

```toml
id = "example-us-state"
version = "0.1.0"
name = "Example State, USA (illustrative)"
verified_on = "2026-10-28"
reviewer = "unassigned"
note = "Confirm with your authority having jurisdiction."

[[codes]]
code = "NFPA 70"
edition = "2023"
in_force_from = "2026-01-01"
adopted_by = "Example State Building Commission"

edition_rule = "edition in force on the permit application date"

[[codes.amendments]]
clause = "210.8"
summary = "This state delays one ground-fault protection requirement for a stated period."
citation = { title = "Example Administrative Code 1-2.3", url = "https://example.org/rule-1-2-3" }

[conventions]
units = "imperial"
paper = "ANSI"
symbols = "IEEE 315"
label_languages = ["en"]
supply = ["480Y/277 V 60 Hz", "208Y/120 V 60 Hz"]

[[sources]]
url = "https://example.org/adoption-notice"
read_on = "2026-10-28"
```
