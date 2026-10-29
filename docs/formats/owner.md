# Owner profile format, version 1

**Status: planned** (D-023).

An owner profile is a client's drawing standard, kept as data: the title block, sheets, fonts, layers, numbering, revisions and seals the owner requires. Hyleon generates every sheet to the profile, and a validator checks the whole set against it before issue.

Owners publish such standards and some check drawings against them automatically ([research](../research/2026-10-28-blind-spots/1-deliverables.md)). Profiles live in `owners/`, are versioned like packs, and are locked in each project's `packs.lock.json`.

**Three kinds of profile.** A project combines all three:
- the **packs**: what the job needs
- the **region profile**: what the law requires there ([region.md](region.md))
- the **owner profile**: how the owner wants it drawn and named

## Fields

| Field | Holds |
|---|---|
| `id`, `version`, `name`, `source` | The owner's document this profile follows: title, revision, date, link |
| `[title_block]` | The required fields: owner, project, integrator, title, sheet and revision numbers, revision history, issue date, phase, scale, contract and submittal numbers, responsible person, and the seal box. The owner's own border can be imported |
| `[sheets]` | Allowed sizes (for example ANSI B for panels, ANSI D or 24×36 for record sets) and which deliverable uses which size |
| `[text]` | Allowed fonts, and minimum plotted text height per sheet size |
| `[lines]` | Line weights and line types per purpose |
| `[layers]` | The layer map: Hyleon's purpose → the owner's layer name (US National CAD Standard, or the owner's template) |
| `[numbering]` | Grammars for tags, wires, terminals, cables, drawing numbers and I/O addresses. Each is a pattern with named parts, so the validator can explain a mismatch |
| `[revisions]` | Revision scheme (for example `00`, `01` or `A`, `B`), sheet-number format, and resubmittal numbering (for example `18.1`) |
| `[seal]` | Whether sheets need a licensee's seal or signature, and how: the box on the sheet, or an electronic signature on the PDF/A. Hyleon only reserves the space and never applies a seal |
| `[files]` | Delivery rules: one sheet per file, file naming, formats (PDF/A, DWG from phase 3, DXF, XLSX) |

## The validator

- **Runs on the whole set,** not one sheet at a time. A tag spelled differently on two sheets, a missing title-block field, or text below the minimum height are each findings.
- **Speaks the owner's language.** Every finding names the profile's rule and the owner document it comes from.
- **Never blocks.** Errors need a reason when acknowledged at issue (D-022).

## Example

Illustrative: the values are made up, not a real owner's.

```toml
id = "example-utility"
version = "0.1.0"
name = "Example Water Utility, I&C drawing standard (illustrative)"
source = { title = "Example I&C Design Standard", revision = "3", date = "2025-06-01", url = "https://example.org/ic-standard.pdf" }

[sheets]
panel_drawings = "ANSI B"
record_set = "ANSI D"

[text]
fonts = ["Arial"]
min_height_mm = { "ANSI B" = 2.5, "ANSI D" = 3.0 }

[numbering]
instrument_tag = "{area:2}-{function:ISA}-{loop:4}"
wire = "{tag}-{n:2}"
io_address = "R{rack}S{slot}P{point:2}"

[revisions]
scheme = "two-digit"
first = "00"
resubmittal = "{item}.{n}"

[seal]
required = true
method = "pdf-signature"
```
