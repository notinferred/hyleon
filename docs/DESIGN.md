# Design

How Hyleon should feel and behave. It adopts the *philosophy* of Apple's [design principles](https://developer.apple.com/design/human-interface-guidelines/design-principles) (Human Interface Guidelines, reintroduced 2026-06-08), not Apple's designs. Each principle below is paraphrased and linked rather than copied, then translated into what it means here. [TRUTHS.md](../TRUTHS.md) outranks this document.

## The eight principles, applied

### Purpose ("Make something meaningful")
Apple asks for a clear intention, focus on what matters most, and new ways to solve a problem instead of re-creating existing solutions.
- The intention is TRUTHS.md: a design people can trust before anything is built.
- We don't rebuild AutoCAD, EPLAN or KiCad. We build what they lack: questions before design, sourced limits, checks on every change.
- We stay focused: the first version is three small packs done completely, not thirty done halfway.

### Agency ("Let people do things their own way")
Apple asks to stay out of the way, give freedom to explore without forced flows, and make mistakes easy to recover from.
- Questions, design, checks and issuing happen in any order. Nothing a check finds stops the work.
- Hyleon is a normal design application first: everything can be built by hand, and an agent is optional.
- People can work with an agent, without one, or both at once.
- Everything is undoable until a revision is issued. After that, a new revision corrects it, so nothing is lost.
- The agent proposes and the person decides.

### Responsibility ("Act in people's best interest")
Apple asks for transparency about what the product does and why, care with people's data, and protection against misuse.
- Every value shows where it came from, and every question says why it's asked.
- Projects stay on the person's device unless they share them, and they are never used to train AI.
- Issuing lists everything still open, and the person acknowledges each item. The record keeps who accepted what, and why. Hyleon informs; it never overrules.

### Familiarity ("Build on what people know")
Apple asks to use known concepts, keep visuals and interactions consistent, and give clear feedback.
- Hyleon speaks the trade's language: sheets, title blocks, revision clouds, markups, tags, ISA-5.1 and IEC 60617 symbols, "issued for construction", "as-built".
- One way to select, edit and connect, across every discipline.
- Each operating system gets its own menus, shortcuts, file dialogs and conventions (Cmd on macOS, Ctrl on Windows).
- Feedback is immediate: checks rerun on every change, and the findings update in place.

### Flexibility ("Adapt to diverse contexts and needs")
Apple asks to design for everyone, preserve context across devices, support many inputs, and treat each platform with care.
- Desktop is for design, a tablet is for the site, the web is for presenting. The same project keeps the same places on each.
- Any code (NEC, IEC, others), and imperial or metric units: values are stored in SI and shown in the person's units.
- Keyboard, mouse, touch and pen all work, and every action can be reached from the keyboard.
- Meaning is never carried by color alone. Line types also differ by dash pattern and label, so a sheet printed in black and white still reads correctly.

### Simplicity ("Be clear and direct")
Apple notes that simplicity isn't minimalism: include what's needed, say it concisely, establish hierarchy.
- Engineering screens may be dense, but every element earns its place.
- Hierarchy:
  - errors come first, then warnings, then information
  - project, then area, then equipment, device and port
- Findings use plain words: what's wrong, where, why (the source), and how to fix it.
- Detail appears on demand: a question shows its "why" and its default's source when asked.

### Craft ("Care about every detail")
Apple asks for quality in every element, early prototypes and iteration, and real-world testing.
- The printed sheet is the handshake. Line weights, lettering, alignment, title blocks and revision tables meet drafting practice.
- The UI is prototyped early: the Dioxus trial (T-02) comes before anything depends on it.
- Each pack is tested on a real project, and reviewed by a licensed engineer of its discipline before release (ROADMAP phase 4).
- Packs stay current with code editions. An old edition stays selectable for projects designed under it.

### Delight ("Make it human")
Apple asks to choose the emotion to inspire, create defining moments, and not mistake delight for decoration.
- The emotion is **confidence**: nothing slipped through.
- The defining moment is issuing a clean, fully checked set: every question answered by a person, every guess confirmed, every finding resolved or knowingly accepted.
- The aesthetic is practical, not decorative.

## Rules that follow

- No dialog interrupts the work for a finding. Findings collect in a panel, and issuing lists the open ones for acknowledgement.
- A region profile always shows when it was last verified, its sources, and "confirm with your authority having jurisdiction".
- **Fewer warnings, each worth reading.** A check that fires often without leading to a change is reviewed, then tuned or removed. Noise teaches people to stop reading.
- Acknowledging an error asks for a written reason. Warnings and AI guesses take one click (D-022).
- The words are "checked against clause X, edition Y", never "compliant" or "certified" (D-025).
- Every element's verification state is visible: design, checked on site, or differs from the design. A design-only element never looks verified.
- Every number shows its unit, and every value shows its source on hover or focus.
- Every AI-inferred value looks different from a confirmed one until a person confirms it.
- A message names the thing by its tag and its location (sheet and zone), not by an internal ID.
- Empty states say what to do next, for example "Answer 3 questions to start the bus design."
- Colors pass contrast checks in light and dark themes. Sheets print correctly on monochrome printers.

## Not taken from Apple

- Not Apple's visual design: no copied layouts, materials, effects or icons.
- Not Apple's fonts or symbol sets. The SF fonts are licensed only for mock-ups of software for Apple's platforms, and SF Symbols only for apps on those platforms. Hyleon also runs on Windows, Linux and the web, so it uses fonts and icons under open licences (D-014).
- Not the guidelines' text: this document paraphrases and links to it.

## The workspace (proposed 2026-10-28; Dan confirms)

Hyleon is a dynamic environment to get the work done. It's built the way Adobe builds its tools: one document in the middle, tools on one side, panels you add as modules, and workspaces that arrange them for a job. It isn't an AI app. It's a design application for environments, and AI is optional (truth 9).

```
┌ Hyleon · Greenhouse 2 ───────────── Workspace: Designer ▾ ── ⌘K ┐
│ ▢ Select  │ Physical · Logical · Sheet · Table   │ Properties     │
│ ⊕ Place   │ ┌ Bay 1 ──────────┐ ┌ Pump room ────┐ │ VFD-1  7.5 kW  │
│ ⟋ Connect │ │ ▒▒ ▒▒ ▒▒ benches│ │ P-1 ── VFD-1  │ │ Addr 1 · src ✓ │
│ ⇿ Measure │ └─────────────────┘ └──┬────────────┘ │────────────────│
│ ✎ Markup  │   2" PVC ═══════════════╛             │ Parts        ▸ │
│           │      [ Pick model · Connect power ]   │ Checks 2     ▸ │
│           │                                       │ Budget       ▸ │
└───────────┴───────────────────────────────────────┴────────────────┘
 1:100 · US NEC 2023 · Rev A draft · ⚠ 2 · ? 3            + Add panel
```

- **The document is the environment:** the site, building, room or panel being worked on, with every real part in it. The same model shows as Physical (where things are), Logical (how they connect), Sheet (the drawing) and Table.
- **Tools are the same in every discipline:** select, place a part, connect (pipe, cable, network), measure, mark up.
- **Panels are modules.** They can be added, closed or docked:
  - Parts, Properties, Systems (layers)
  - Questions, Checks, Budget
  - Sheets and print, Revisions and record, Site
  - Assistant (AI, optional)
- **Workspaces arrange panels for a scope:** Designer, Panel builder, Installer, Estimator, Reviewer, Manager. People customise and save their own. A module outside someone's scope simply isn't there (D-033).
- **Next actions sit by the selection.** Select a pump and its likely next steps appear beside it: pick a model, connect power, set the duty point. Guidance without a wizard.
- **Never lost:**
  - the status bar always shows scale, units, region, revision and what's open
  - ⌘K finds any part, sheet, panel or command
  - every change can be undone
- **AI is optional.** The Assistant panel and agents use the same tools, and their changes appear in the document as proposals. Everything works without them.
