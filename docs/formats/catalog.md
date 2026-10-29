# Component catalog, version 1

**Status: planned** (D-031, D-032).

The catalog is the record of real parts: what exists on site, what is in stock, and what was picked for the design. Every fact in it cites its source.

## Entries

| Field | Holds |
|---|---|
| `id`, `version`, `maker`, `model`, `part_number` | The exact identity of the part |
| `kind` | For example `vfd`, `breaker`, `pump`, `flowmeter`, `gateway`, `fitting`. Custom kinds are allowed |
| `ports` | Interfaces, each with its type and ratings (for example `power-3ph`, `rs485`, `4-20mA`, a 2-inch BSP pipe) |
| `ratings` | Electrical, mechanical and environmental: SCCR with its specified protective device, enclosure type, temperature range |
| `dimensions` | Size, weight and required clearances, used by layouts in phase 5 |
| `settings` | The device's settings schema: for each parameter, its number, its meaning in Hyleon's own words, range, default and unit, and the firmware versions it applies to |
| `compatibility` | What the part can connect to or replace: protocols, signal types, sizes |
| `documents` | Links to the maker's datasheet and manual, with the date each was read. A file is attached only when a person adds their own copy |
| `sources`, `status` | A source for every fact. Status is `draft` (unverified), `approved` (with who and when), or `retired` |

## In a project

- **What each part is in the project.** It's logged as existing on site, in stock, or picked to be bought, with its quantity.
- **Who picked it.** A person picks it by hand, or an agent proposes it; either way it's recorded.
- **Swapping.** Replacing a part checks compatibility: ports, protocols, ratings, size, and the panel's short-circuit rating. Hyleon lists everything the swap changes (wiring, settings, drawings, parts list, checks), and the person decides.

## Adding what's missing

- **By hand.** Anyone can add a custom component in the app, in the same format.
- **Researched by an agent.**
  - The agent looks for the part, starting with official sources: the maker's datasheets, manuals and listings.
  - It proposes an entry with a source for every fact, marked unverified.
  - A person approves the entry before it's used as verified data.
- **Sharing.** Approved entries can be shared to a common catalog from phase 3.

## Device settings

- Each device in a project holds its settings, following its catalog schema.
- **Checks across devices.** For example: a bus's framing matches on every device, addresses are unique, and an analog input's mode matches the transmitter wired to it.
- **Exports:** parameter sheets, and commissioning checklists.
- **The installed state.** Firmware and software versions are logged as installed.

## What's copied

- Facts (ratings, parameter numbers and ranges) are recorded with citations.
- Makers' text isn't copied, and manuals are linked, not redistributed.
