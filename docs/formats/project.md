# Project format, version 1

**Status: planned.**

A project is a folder that anyone, or any AI, can read. Nobody writes it directly. Every change is a command, sent through the app, the `hyleon` CLI or its MCP server. `store` appends the command to the log and then updates the model files from it.

Examples use the made-up sample pump station and shorten IDs to `…`.

## The folder

```
pump-station.hyleon/
  project.json       format version, project ID, name, display units, packs in use
  packs.lock.json    each pack's ID, version and sha256, and the region and owner profiles'
  log/               append-only commands: 000001.json, 000002.json, …
  model/             the projection of the log; can always be rebuilt from it
    entities.json    equipment, devices with their ports, cables, locations
    links.json       connections between ports: power, signal, data, pipe
    answers.json     answers to pack questions, with sources
    boundaries.json  proposed, accepted and rejected boundaries
    head.json        the sequence number the projection reflects, and its sha256
  revisions/
    A/manifest.json  an issued revision: the sha256 of every file and output
  blobs/sha256/      content-addressed files: photos, datasheets, issued outputs
```

- **Canonical JSON.** Files are UTF-8 with LF line endings and two-space indents. Keys are sorted, and lists of entities are sorted by ID. The same model always produces the same bytes, so diffs show only real changes.

## IDs and tags

- **`id`** is a UUIDv7. It's never reused and carries no meaning.
- **`tag`** is the human name, such as `P-1` or `VFD-1`. A check keeps tags unique; they aren't keys, so renaming one breaks nothing. Reference designations in the IEC 81346 style (`=PS1-P1`) come later.
- **References** between things always use IDs. A tag lives in one place, on its entity, so every sheet shows the same name.

## Verification status

Every element records how far it has been checked against reality (D-024):

```json
"verified": { "state": "field_verified", "by": "p:2", "on": "2026-11-03", "seq": 57 }
```

| `state` | Meaning |
|---|---|
| `design` | As designed; nobody has checked it on site |
| `field_verified` | A person checked it on site, recorded with who and when. `on` is a date for display; order comes from `seq` |
| `markup_pending` | A site markup says it differs, and the change isn't in the model yet |

The record set shows these states, so nobody mistakes a design-only element for a verified one.

## Values and their sources

Every engineering value is an object with its source:

```json
{ "v": 5500, "unit": "W", "src": { "kind": "person", "by": "p:1", "via": "app", "seq": 4 } }
```

| `src.kind` | Fields | Meaning |
|---|---|---|
| `person` | `by`, `via` (`app` or `elicitation`), `seq` | A person gave it |
| `standard` | `code`, `clause`, `seq`, and `via` when a region profile supplied it (`region:<id>@<version>`) | Taken from a code or standard; the clause is named, never quoted |
| `catalog` | `entry`, `field`, `seq` | Taken from a catalog entry at a stated version |
| `calculated` | `calc`, `inputs`, `seq` | Worked out by a named, versioned calculation from the listed inputs |
| `inferred` | `agent`, `session`, `seq`, and later `confirmed: { by, seq }` | Guessed by an AI agent, and unconfirmed until `confirmed` is set |

- **Units.** Values are stored in SI units: W, V, A, m, Pa, m³/s, K. They're converted only for display.
- **Conductor sizes** are catalog values: `{ "size": "AWG 10", "area_m2": 5.26e-6 }`.

## Commands

Every change is one command:

```json
{
  "id": "0192f1c4-…",
  "seq": 43,
  "base": 42,
  "actor": { "kind": "agent", "name": "claude-code", "session": "s:7" },
  "op": "set_value",
  "target": { "entity": "0192f1a0-…", "field": "flow_duty" },
  "value": { "v": 0.0125, "unit": "m3/s" }
}
```

**Operations in version 1:**
- `add_entity`, `remove_entity`, `set_tag`, `set_value`
- `connect`, `disconnect`
- `answer`, `propose_boundary`, `create_option`
- person-only, through the app: `accept_boundary`, `reject_boundary`, `confirm_value`, `choose_option`, `issue_revision` (which carries the person's acknowledgements), `set_region`, `set_owner`, `set_verification`, `record_review`

**How a command applies, in order:**
1. **Parse.** The command is checked against the schema. If it fails, it's refused, with the path of each error.
2. **Authorize.** A person-only operation from anyone but a person in the app is refused. An `answer` that came through elicitation is recorded as the person's.
3. **Stamp the source.**
   - A value from an agent becomes `inferred`.
   - The exception is a value Hyleon can verify. A `catalog` value must match the entry it names. A `calculated` value is recomputed by Hyleon.
   - An agent can never claim `person`.
4. **Fence.** `base` must equal the current sequence number. If it doesn't, the command is refused as `stale`, and the writer re-reads and retries.
5. **Claim.** `log/<seq>.json` is written with a create-if-absent write. If the file already exists, another writer won, and the command is refused as `stale`. This one atomic step is what lets the app and an agent write the same project safely.
6. **Project.** The model files and `head.json` are updated.
7. **Check.** Checks run and the findings are returned. They never undo the command.

**Idempotent.** A command whose ID is already in the log does nothing, and returns the original result.

## Revisions

- **Issuing** is person-only, and happens in the app. Nothing prevents it.
  - Hyleon lists every open error, warning and unconfirmed `inferred` value.
  - The person acknowledges each one. A reason is required for an error and optional for the rest (D-022).
  - Accepting an inferred value at issue records the person as confirming it.
- **Purpose and label.** Each issued revision records its purpose (D-024):
  - a design review stage (30%, 60%, 90%)
  - a submittal
  - for construction
  - the record set, labelled "record drawings" or "as-built" as the person chooses
- **Reviews.** A reviewer's response is recorded against the revision with `record_review`: the action code (for example "approved", "approved as noted", "revise and resubmit") and the comments. A resubmittal is a new revision that names the one it answers, numbered as the owner profile says (for example `18.1`).
- It writes `revisions/<letter>/manifest.json` with:
  - the sequence number
  - the sha256 of each model file and of `packs.lock.json`
  - the sha256 of each output (PDF, XLSX), stored in `blobs/`
  - the acknowledgements: each open item, who acknowledged it, and why
  - the purpose, the label and, once received, the review response
- **The change list** is generated from the commands between two issued revisions, grouped by tag.
- **An issued revision never changes.** A correction is a new revision.

## Edits made outside Hyleon

`head.json` holds the sha256 of the projection. If the files under `model/` don't match it on load:
- the projection is rebuilt from the log
- the difference is reported as a finding, for example "model/entities.json was edited outside Hyleon: 2 changes"
- the person can turn the difference into commands, recorded with `person` as the source, or discard it

## Format versions

- `project.json` holds `"format": 1`.
- **Opening an older format.** A newer Hyleon migrates it forward with one migrator per version, and records the migration as a command.
- **Opening a newer format.** An older Hyleon refuses it, and says which version it needs.
- **Compacting the log** into a snapshot happens only at an issued revision, and comes later.

## Example: the sample pump station

`project.json`:

```json
{
  "format": 1,
  "id": "0192f19c-…",
  "name": "Sample pump station",
  "units": "metric",
  "region": "example-iec@0.1.0",
  "owner": "example-utility@0.1.0",
  "packs": ["modbus-drive-bus@0.1.0", "motor-branch-circuit@0.1.0", "pump-discharge-spool@0.1.0", "site-basics@0.1.0"]
}
```

Part of `model/entities.json`, a pump and its drive:

```json
[
  {
    "id": "0192f1a0-…01",
    "type": "pump",
    "tag": "P-1",
    "fields": {
      "rated_power": { "v": 5500, "unit": "W", "src": { "kind": "person", "by": "p:1", "via": "app", "seq": 4 } },
      "flow_duty": { "v": 0.0125, "unit": "m3/s", "src": { "kind": "inferred", "agent": "claude-code", "session": "s:7", "seq": 43 } }
    },
    "ports": [
      { "name": "power", "kind": "power-3ph" },
      { "name": "suction", "kind": "pipe" },
      { "name": "discharge", "kind": "pipe" }
    ]
  },
  {
    "id": "0192f1a0-…02",
    "type": "vfd",
    "tag": "VFD-1",
    "catalog": "vfd-generic-7k5@1",
    "fields": {
      "modbus_address": { "v": 1, "src": { "kind": "person", "by": "p:1", "via": "elicitation", "seq": 12 } }
    },
    "ports": [
      { "name": "line", "kind": "power-3ph" },
      { "name": "motor", "kind": "power-3ph" },
      { "name": "rs485", "kind": "rs485" }
    ]
  }
]
```

Part of `model/links.json`:

```json
[
  { "id": "0192f1b0-…10", "kind": "power", "from": { "entity": "0192f1a0-…02", "port": "motor" }, "to": { "entity": "0192f1a0-…01", "port": "power" } },
  { "id": "0192f1b0-…11", "kind": "data", "bus": "0192f1a0-…30", "from": { "entity": "0192f1a0-…05", "port": "rs485" }, "to": { "entity": "0192f1a0-…02", "port": "rs485" } }
]
```

Part of `model/answers.json`:

```json
[
  {
    "pack": "modbus-drive-bus@0.1.0",
    "question": "framing",
    "scope": "0192f1a0-…30",
    "value": "19200 8E1",
    "src": { "kind": "person", "by": "p:1", "via": "elicitation", "seq": 6 }
  }
]
```

In this example, `flow_duty` on P-1 is still an AI guess. When revision A is issued, Hyleon lists it, and the person confirms it, replaces it or accepts it as it stands. The manifest records which.
