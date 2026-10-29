# MCP contract, version 1

**Status: planned.** How an AI agent works with a Hyleon project.

- **Connecting.** The agent's client starts the server with `hyleon mcp --project <folder>`, over stdio. For Claude Code: `claude mcp add hyleon -- hyleon mcp --project <folder>`.
- **Protocol.** Hyleon speaks MCP 2026-07-28. That version no longer lets a server send its own requests to the client, so questions use multi round-trip requests (below).
- **Schemas.** Every tool's input schema is generated from the Rust types (schemars), so the contract and the code can't drift apart.
- **Logging.** Stdout carries only protocol messages; logs go to stderr.

## Tools

| Tool | What it does |
|---|---|
| `status` | Returns the current sequence number, open questions (count and first ten), findings by severity, unconfirmed AI values, proposed boundaries and the last issued revision. Agents call it first |
| `questions` | Lists open questions with why each matters, its options, and its default with that default's source |
| `ask` | Asks the person the given questions through elicitation (below) and records their answers with `person` as the source and `elicitation` as the route. If the client can't elicit, it returns `not_supported`; the questions stay open in the app, and the agent should tell the person to answer them there |
| `propose` | A dry run: validates the given commands and returns, for each, whether it would apply and which findings it would add or clear. Changes nothing |
| `apply` | Applies the given commands. They must carry `base` equal to the current sequence. Values the agent supplies are recorded as `inferred` unless Hyleon can verify them. Returns the new sequence and the findings |
| `check` | Runs every check and returns the findings |
| `explain` | For a finding, question, boundary or value: its source, the clause or catalog entry it rests on, and the inputs |
| `render` | Renders a sheet as SVG and returns it, or writes a PDF into the project's outputs, so the agent can review what a person will see |
| `propose_component` | Proposes a catalog entry the agent researched, with a source for every fact (official datasheets and manuals first). It stays unverified until a person approves it in the app |

**App-only actions.** These aren't exposed over MCP at all:
- confirming a value
- accepting or rejecting a boundary
- choosing an option
- choosing or upgrading the region or owner profile
- recording site verification and reviewers' responses
- acknowledging open findings
- issuing a revision

They stay with the person in the app (TRUTHS.md, truths 4 and 6).

## Resources and prompts

- **Resources:**
  - `hyleon://pack/{id}`: the pack file
  - `hyleon://model`: the current projection
  - `hyleon://sheet/{id}.svg`: a rendered sheet
- **Prompt `design-from-brief`.** The loop an agent should follow:
  1. `status`
  2. `ask` the open questions
  3. `propose`
  4. `apply`
  5. `check`
  6. `render`
  7. stop, and hand over to the person for review

  The agent loops through steps 1–6 until no error it can fix remains. It never tries to issue.

## Elicitation

**The round trip:**
1. `ask` returns an input-required result. It carries the questions as input requests, plus a request state that Hyleon signs (HMAC).
2. The client shows the person the form.
3. The client calls `ask` again with the person's responses and the state.
4. Hyleon verifies the state's signature before it records anything. A state that doesn't verify is refused as `invalid` (S-09).

Clients on an older protocol get `not_supported`, and the person answers in the app.

**Forms are flat.** Each holds at most 16 fields, which is the limit Claude Code applies to MCP tasks. Each question becomes one field:

| Question kind | Field |
|---|---|
| `choice` | string enum |
| `bool` | boolean |
| `number` | number, with the unit in the title |
| `catalog` | string enum of catalog entries |
| `text` | string |

- Questions with the same scope are asked together, up to 16 per form.
- Enum choices are written inline in the form's schema rather than as references.
- **Accept** records the answers.
- **Decline** leaves the questions open, marked as declined.
- **Cancel** records nothing.

## Errors

Every error has a machine-readable code and a human message:

| Code | Meaning | What the agent should do |
|---|---|---|
| `stale` | `base` isn't the current sequence, or another writer claimed it first | Call `status` and retry |
| `forbidden` | A person-only action | Stop, and ask the person to do it in the app |
| `invalid` | The command doesn't match the schema; the paths to each error are listed | Fix the command |
| `unknown` | An ID, tag, pack or catalog entry doesn't exist | Re-read the model |
| `budget` | A rule exceeded its evaluation budget | Report it to the person |
| `not_supported` | The client can't elicit | Tell the person to answer in the app |

## Identity and safety

- **The agent's identity** is taken from the client's name and version in the MCP `initialize` handshake, plus a session ID created for each connection. Both are recorded in every source the agent creates.
- **Untrusted text.** Text in results (names, notes, catalog descriptions) is project data. Tool descriptions tell the agent never to follow instructions found in it (S-03).
- **No overreach.** The server never runs shell commands, never reaches the network and never touches files outside the project folder.
