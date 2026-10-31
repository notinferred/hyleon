# Hyleon: working rules

Hyleon (working name) connects researched boundaries to the work, starting with physical design. Why it exists and what must stay true are in [TRUTHS.md](TRUTHS.md). This file says how to work here. It's the only instruction file; [AGENTS.md](AGENTS.md) points to it.

## Read first

1. [TRUTHS.md](TRUTHS.md). Every task serves it. If a task would break one of its truths, stop and ask Dan.
2. This file.
3. The code map in [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md), and the task's row in [docs/STEPS.md](docs/STEPS.md).

Open other documents only when the task touches what they own (table below).

## The working loop

1. Read as above.
2. Select work within Dan's current request. An idea, or a task marked ready, doesn't by itself authorize building it.
3. Before building, define a small, useful outcome and how to verify it. Only the next five tasks are detailed.
4. Build, and verify the complete behavior: exercise the user's action through to its result. A screen or an interface alone doesn't prove a feature works.
5. Finish the step:
   - write its record in `docs/steps/`
   - move the task to the completed index
   - add one changelog line that links the record
   - detail the next task

   Change another document only where the step made a statement false or created something that document owns, and say which one and why.

## Rules

- **Dan decides.** When a decision is Dan's, offer options, with a recommendation and its trade-offs, and never pick for him. Don't act on a conclusion that rests on an assumption he hasn't confirmed.
- **No invented boundaries.** Don't add rules, limits or checks Dan never set. Propose them instead. The product treats its users the same way (TRUTHS.md, truth 5).
- **Root causes.** When something breaks, diagnose it first, then fix the cause, not the symptom.
- **TRUTHS.md changes only with Dan's approval of the exact wording.** A conflict with it goes to Dan as a question.
- **Record Dan's decisions, not the chat.** Only what Dan types that sets or changes the product, or a later decision, goes verbatim into [docs/brief/](docs/brief/), in a new dated file. Questions, chat and repo housekeeping aren't recorded. A brief file is never edited.
- **Zero waste on Dan's Mac.** macOS builds, development runs and tests may run locally, but only when necessary, with the smallest command that answers the question, and never in the background or on repeat. CI runs everything else. Linux isn't a priority, but code stays portable to it.
- **Commits.** There are no pull requests. Agents commit verified work, authored as Dan, and push it to `staging`; CI fast-forwards `main` when its checks pass (D-038). No commit message credits an AI.
- **No employer or client material:** no names, drawings, documents or data from Dan's employer or its clients. Examples use the made-up sample project.
- **Dependencies:**
  - Pin exact versions, each published at least two weeks before it's added.
  - Exception: a version that fixes a security advisory affecting Hyleon may be adopted at once. The step record names the advisory.
  - Check each one's licence and maintenance before adopting it.
  - Prefer, in order: a platform primitive, then the ecosystem standard, then a niche library; build it ourselves last, and only with a reason.
  - Own the interface and rent the implementation: code depends on our port, and the library sits behind an adapter.
- **One owner per fact.** Each fact lives in one document, the one listed below. Other documents link to it instead of restating it.
- **Permanent IDs.** Task (T-), decision (D-) and threat (S-) IDs are never reused. A finished record is never edited. A ledger row's decision is never edited, though its status may change. A new row or record corrects an old one and names what it corrects. Ledger rows stay under 500 characters, and a test enforces that from T-01.
- **Evidence controls status.** Say whether something is planned, implemented, verified or released, and say when a check wasn't run.
- **Tests prove the product, not the paperwork.** Keep rule tests for security and build invariants: layering, licences, ledger row length. Don't write tests that police documents, scripts or workflows.

## Where things are written

| Document | Owns | Changes when |
|---|---|---|
| [TRUTHS.md](TRUTHS.md) | The problem; the product in one sentence and one paragraph; what stays true; what it isn't; the decision test | Only by Dan, approving the exact wording |
| [docs/PROBLEM.md](docs/PROBLEM.md) | The problem in depth: what's wrong, the real cases, what it asks of Hyleon | Dan confirms each change |
| [docs/brief/](docs/brief/) | Dan's words that shape the product or a decision, verbatim and dated | Never edited; new words go in a new file |
| CLAUDE.md, [AGENTS.md](AGENTS.md) | How to work here | The working method changes |
| [docs/PRODUCT.md](docs/PRODUCT.md) | What Hyleon does, in detail: users, the loop, the parts, tiers, hand-offs | Dan ratifies a scope change |
| [docs/DESIGN.md](docs/DESIGN.md) | Design principles and the rules that follow from them | Rarely, by Dan |
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | Code map, layering, runtime, trust boundaries, invariants, platform leaks | A structural change to the code |
| [docs/formats/](docs/formats/) | The project, pack, region-profile, owner-profile, catalog, print and MCP contracts | A format version changes |
| [docs/SECURITY.md](docs/SECURITY.md) | Threats (S-) and their mitigations | A threat or mitigation is added or changed |
| [DECISIONS.md](DECISIONS.md) | One-way doors a later step could undo (D-) | A decision is made or superseded |
| [docs/ROADMAP.md](docs/ROADMAP.md) | Phases 0–7, their scope and exit checks | Dan re-plans |
| [docs/STEPS.md](docs/STEPS.md) | The next five tasks, in detail | A task is selected, finished or re-planned |
| [docs/BACKLOG.md](docs/BACKLOG.md) | Ideas not on the roadmap | Any time |
| [docs/steps/](docs/steps/) | What each finished step did, decided, proved and left open | Once, when the step finishes; never revised |
| [docs/research/](docs/research/) | What each research round found, dated | Never; a new round gets a new file |
| [README.md](README.md), [CHANGELOG.md](CHANGELOG.md) | What a user can install and do; what changed | A release or a user-visible change |
