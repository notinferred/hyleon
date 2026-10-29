# Pack format, version 1

**Status: planned.**

A pack is the framework for one kind of job: the questions to answer first, the limits that apply, the checks a design must pass and the documents it must produce. Packs are TOML files (D-005) in `packs/`, and anyone can read them.

**Packs keep Hyleon dynamic.** A new kind of job needs only a new pack, unless it needs a calculation Hyleon doesn't have yet. Calculations are versioned Rust code in `crates/calc`.

## Sections

| Section | Holds |
|---|---|
| `[pack]` | `id`, `version` (semver), `title`, `discipline`, `scope` (the entity type one instance applies to, or `project`), `uses` (packs it builds on), `status` (`draft`, `reviewed` or `released`), `reviewed_by`, `edition` (the code editions it was written for), `tests` (a folder of small test projects, each with the findings it must produce, which the pack is checked against whenever it loads or changes) |
| `[[questions]]` | `id`, `ask`, `why`, `kind` (`choice`, `bool`, `number`, `text`, `catalog`), `options` (for `choice`), `catalog` and `allow_none` (for `catalog`), `unit`, `default` (with its source), `required`, `when` (a CEL condition), `sets` (the model field the answer fills) |
| `[[boundaries]]` | `id`, `says`, `rule` (CEL), `source`, `when`. A boundary is in force only once its source exists: a code the project selected, a specification, or a person's answer |
| `[[checks]]` | `id`, `severity` (`error`, `warning`, `info`), then either `rule` (CEL) or `calc` (a built-in calculation and its inputs), plus `says`, `fix` and `source`. No severity stops the work; at issue, open findings are acknowledged |
| `[[deliverables]]` | The sheets and schedules the job must produce |

- **Messages.** `says` and `fix` may contain `{…}` placeholders, which are evaluated as CEL.
- **Sources** use the same kinds as model values ([project.md](project.md)). In a pack, a `person` source names the question whose answer supplies the limit.
- **Code text.** A pack never quotes code text, figures, notes or whole tables: it names the code and the clause, and states the check in its own words.
- **Table values.** When a check needs a value from a code's table, the pack asks the person to enter it from their own copy, naming the table, until Hyleon is licensed to use that table (D-017).

## Rules and conditions: CEL

Rules and conditions are written in [CEL](https://cel.dev), the Common Expression Language.

- **What a rule sees:**
  - `project`
  - `site` (the answers to `site-basics`)
  - `answers`
  - the entity the pack is scoped to (`bus`, `motor`, `spool`), with its fields, ports, links and neighbours
- **What a rule can do:** CEL has no loops, no recursion and no I/O. Its macros (`all`, `exists`, `filter`, `map`) are bounded by the size of their input.
- **Bounds Hyleon adds.** The CEL library (the `cel` crate) has no execution budget and no type checker, so Hyleon bounds every rule itself.
  - **When the pack loads:**
    - every rule and condition is parsed
    - macros may nest at most two deep
    - every rule is dry-run against the pack's test projects (`tests`, below)

    A rule that fails any of these is refused with its file, line and column.
  - **When a rule runs:** the collections it sees are capped in size, and a rule that hits the cap produces an `info` finding naming it.

  No rule can make Hyleon hang.

## Versions and review

- **A change to questions, boundaries or checks is a new version.**
- **Projects lock the versions they use** in `packs.lock.json`, with each pack's sha256. Upgrading a project to a newer pack version is a recorded command, so old projects keep checking against the edition they were designed under.
- **Release needs review.** A pack can be `released` only with a named reviewer (a licensed engineer of its discipline) and the code editions it covers. Until then it's `draft`, and Hyleon says so on every finding it produces.

## Layers

So that "every industry" doesn't mean a duplicate pack for every industry (D-021), packs stack in layers:

1. **Base pack:** one kind of job, such as a pump station or a motor branch circuit.
2. **Industry overlay:** what water, food or marine adds to it.
3. **Region profile:** the codes and editions in force ([region.md](region.md)).
4. **Edition:** the exact versions of the standards the checks follow.

- An overlay can add questions, boundaries and checks, or tighten a default.
- It never silently removes anything from the layer below.
- When two layers disagree, the disagreement is a finding, shown with both sources.
- Base packs are written first, because they serve most industries at once (D-021).

## The four packs of the first version

**Illustrative.** The clause references below are to be verified, and each pack reviewed, before it leaves `draft`.

### `site-basics@0.1.0`: questions every project answers once

Scope: `project`. It has questions and no checks of its own. When the project has a region profile ([region.md](region.md)), these answers default from it, with the profile as their source. The person can change any of them. Its questions:
- the governing code and edition: NEC 2023 with UL 508A, IEC 60204-1 with IEC 60364, or another
- supply voltage, phases and frequency
- the earthing system: TN-S, TN-C-S, TT, or IT / ungrounded
- the available fault current at the panel
- the maximum ambient temperature
- the display units

It has one conditional question: **which ground-fault detection device is installed?** It's asked only when the earthing system is IT / ungrounded (`when = "site.earthing == 'IT'"`), with the source NEC 250.21(B) or IEC 60364-4-41.

### `modbus-drive-bus@0.1.0`: controls

```toml
[pack]
id = "modbus-drive-bus"
version = "0.1.0"
title = "Modbus RTU drive bus"
discipline = "controls"
scope = "bus"
uses = ["site-basics@0.1.0"]
status = "draft"

[[questions]]
id = "framing"
ask = "What speed and framing does this bus use?"
why = "Every device on one bus must use the same speed, parity and stop bits, or it won't answer."
kind = "choice"
options = ["19200 8E1", "9600 8E1", "38400 8N1", "Other"]
default = { value = "19200 8E1", source = { kind = "standard", code = "Modbus over Serial Line V1.02", clause = "Data signaling rates; character format" } }
required = true
sets = "bus.framing"

[[questions]]
id = "gateway"
ask = "Which gateway connects this bus to the network, if any?"
why = "The gateway is one end of the bus, so it decides where a termination goes."
kind = "catalog"
catalog = "modbus-gateway"
allow_none = true
required = true
sets = "bus.gateway"

[[checks]]
id = "address-unique"
severity = "error"
rule = "bus.devices.all(d, bus.devices.filter(e, e.address == d.address).size() == 1)"
says = "Two devices on {bus.tag} share a Modbus address."
fix = "Give each device on the bus its own address."
source = { kind = "standard", code = "Modbus over Serial Line V1.02", clause = "Addressing rules" }

[[checks]]
id = "address-range"
severity = "error"
rule = "bus.devices.all(d, d.address >= 1 && d.address <= 247)"
says = "A device on {bus.tag} has an address outside 1–247."
fix = "Use an address from 1 to 247."
source = { kind = "standard", code = "Modbus over Serial Line V1.02", clause = "Addressing rules" }

[[checks]]
id = "framing-matches"
severity = "error"
rule = "bus.devices.all(d, d.framing == bus.framing)"
says = "A device on {bus.tag} isn't set to the bus framing ({bus.framing})."
fix = "Set every device's serial parameters to the bus framing."
source = { kind = "person", question = "framing" }

[[checks]]
id = "terminated-both-ends"
severity = "error"
rule = "bus.terminations.size() == 2"
says = "{bus.tag} has {bus.terminations.size()} terminations; it needs exactly two, one at each end."
fix = "Fit a termination at the first and the last device on the bus, and nowhere else."
source = { kind = "standard", code = "Modbus over Serial Line V1.02", clause = "Line termination" }

[[checks]]
id = "unit-loads"
severity = "warning"
rule = "bus.unit_loads <= 32.0"
says = "{bus.tag} carries {bus.unit_loads} unit loads; RS-485 drivers are specified for 32 without a repeater."
fix = "Split the bus, add a repeater, or use lower-load transceivers."
source = { kind = "standard", code = "TIA-485-A", clause = "Unit loads" }

[[deliverables]]
id = "bus-wiring-sheet"
sheet = "bus-wiring"

[[deliverables]]
id = "register-list"
schedule = "modbus-registers"
```

### `motor-branch-circuit@0.1.0`: power

Scope: `motor` (one motor and its drive or starter). Uses `site-basics`.

**Questions:**
- the motor's rated power and nameplate full-load current
- driven by a VFD? If yes, which drive (from the catalog)
- conductor material and insulation temperature rating
- the number of current-carrying conductors bundled together
- the run length
- **Table values from the person's own copy of the code** (D-017):
  - the motor's full-load current from the code's motor table (NEC Table 430.250)
  - the conductor's ampacity from the ampacity table (NEC Table 310.16), with its correction factors

**Checks:**

| Check | Severity | Rule or calculation | Source |
|---|---|---|---|
| `drive-sccr` | error | The drive's short-circuit rating with its specified protective device ≥ the site's available fault current | Catalog (the drive maker's rating); NEC 409.22; UL 508A Supplement SB |
| `protective-device-as-specified` | error | The branch protective device is the type and maximum rating the drive maker specifies | Catalog |
| `conductor-ampacity` (NEC) | error | `calc = "nec.motor_branch_conductor@1"`: derated ampacity ≥ 125% of the drive's rated input current, or 125% of the table full-load current for a motor without a drive. The table values are the person's, entered above | NEC 430.122(A), 430.22, 430.6(A)(1) |
| `conductor-ampacity` (IEC) | warning | This version can't size conductors under IEC 60364-5-52 yet. The finding says so, and asks for a sized value recorded with `person` as its source. A gap is stated, never skipped | — |
| `voltage-drop` | warning | Drop over the run ≤ the limit the person sets; default 3%, sourced to the code's informational guidance | NEC 210.19(A) informational note |

**Deliverables:** one-line diagram, branch power schematic, load table.

### `pump-discharge-spool@0.1.0`: piping

Scope: `spool` (one pump's discharge spool). Uses `site-basics`.

**Questions:**
- the pipe standard and material: PVC-U to ISO 1452, PN16; PVC to ASTM D1785, Schedule 40; or another
- the thread standard for threaded joints: BSP (ISO 228 / ISO 7) or NPT (ASME B1.20.1)
- the design pressure and design flow. The flow defaults to the pump's duty point, with that value's source
- the maximum velocity allowed. There's no default: it comes from the person's specification or the pipe maker, and Hyleon doesn't invent it
- whether the company requires an isolation valve on each side of every pump
- whether it requires a check valve on every pump discharge

**Checks:**

| Check | Severity | Rule or calculation | Source |
|---|---|---|---|
| `thread-standard` | error | Every threaded joint uses the answered thread standard | The person's answer |
| `pressure-class` | error | Every component's pressure rating, derated for temperature (`calc = "pvc.pressure_derating@1"`), ≥ the design pressure | Catalog; ISO 1452 |
| `velocity` | warning | Design flow ÷ bore area ≤ the answered limit | The person's answer |
| `isolation-valves` | warning | When required: an isolation valve on each side of the pump | The person's answer (company practice) |
| `check-valve` | warning | When required: a check valve on the discharge | The person's answer (company practice) |
| `flowmeter-straight-run` | warning | The straight lengths before and after a flowmeter are at least what its maker requires, in multiples of DN | Catalog (the meter) |

**Deliverables:** spool sheet, P&ID excerpt, BOM.

## Checks that belong to no pack

Built into `checks`, they always run.

**The fault-current chain is the flagship (D-021):**
- It starts from the utility's available fault current, with its source and date.
- It follows each node down to every device's interrupting rating or SCCR, and on to the printed labels and nameplates.
- It records the study date, which NEC 409.22(B) requires.
- Impedances and let-through values come from catalogs, or from the person's entry from their own copy of the code (D-017).
- Changing any link reruns the whole chain, so a label never disagrees with the calculation behind it.

The other built-in checks:
- **Tags unique.** No two entities share a tag.
- **References resolve.** Every sheet cross-reference, link and catalog entry points at something that exists.
- **Complete across disciplines.** A pump needs both a power feed and pipe connections.

  So if the same pump is entered twice under two names, once in the power pack and once in the piping pack, it shows up as two incomplete pumps: "P-1 has no power feed" and "PUMP-1 is connected to no pipe", with the hint "If these are the same pump, merge them." This is the check that catches one pump carrying two names.
