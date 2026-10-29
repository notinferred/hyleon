# The problem, in depth

Our understanding of Dan's problem, written down from his words ([brief/](brief/)) so he can correct it.
- [TRUTHS.md](../TRUTHS.md) holds the short version. This document holds the detail and the real cases behind it.
- Dan confirms every change to it.
- Client and employer details are left out.

## What's wrong today

1. **A system lives in words.**
   - An AI agent works only with words. A person can describe a system in prose or Markdown, but nobody can *see* it: its real components, the custom ones, and how everything connects.
   - A description isn't a professional document.
2. **Nothing models real components that AI can work with.**
   - Networking has Cisco's Packet Tracer: real devices, real ports and real settings, which a student or an AI can design with, document and hand over.
   - Physical systems have nothing like it across industries, whether at the scale of a whole factory or of a single pump skid.
3. **Each industry has its own tools, but real projects cross industries.**
   - A greenhouse is structure, climate, irrigation, power, controls and network at once.
   - Industry-specific software doesn't connect these, so the design is split and nobody sees the whole.
4. **Designing from one perspective misses things.**
   - A design seen only technically overlooks what matters to the people who install, operate, maintain and pay for it.
   - The flaws are found later, on site (case B).
5. **Documentation is a separate, slow phase.**
   - The design is finished, but the work can't start until professional documents have been made by hand (case B).
6. **AI can't reference what already exists.**
   - Datasheets, manuals and past drawings, large and small, aren't attached to the components they describe.
   - So neither people nor AI can cite them while designing.
7. **Budgets are guesses.**
   - Without a real model there's no reliable count of the components needed, the ones still missing, pipe and cable lengths from real distances, or the installation effort.
   - Dan places this later in the roadmap.

## The two real cases

- **Case A: a pump system's documents.**
  - One site's control-system documents, made with six tools that don't share data.
  - They contain nine mismatches between documents, rules buried in notes, and open items nobody tracked.
  - Anonymized details: [research/2026-10-28-market.md](research/2026-10-28-market.md), "The starting evidence".
- **Case B: a greenhouse.**
  - An engineer designed it from a technical perspective, and it has many flaws because much was overlooked.
  - The team then designed its system with AI. Installation took a long time, because the documentation had to be produced afterwards.
  - Source: Dan's account, 2026-10-28.

## What this asks of Hyleon

Proposed. Dan confirms each item before it moves into [PRODUCT.md](PRODUCT.md) and [ROADMAP.md](ROADMAP.md).

- **Real components, including custom ones.**
  - Anyone can extend the component library: define a component's ports, properties, ratings, symbol and documents once, and use it anywhere.
  - If it exists on site, it can exist in the model.
  - *Adopted 2026-10-28 (D-031): the catalog records what's on site, in stock and picked, and agents research missing parts from official sources for a person to approve.*
- **Questions from every perspective.**
  - A pack asks the designer's technical questions, and also the installer's, the operator's, the maintainer's, safety's, cost's, and those of the owner's own process (for example, the grower's).
  - Hyleon shows which perspectives were never asked.
- **Space and dimensions.** Positions and sizes of bays, rooms, floors and aisles, so that distances, clearances and runs are real. *Adopted 2026-10-28 as roadmap phase 5 (D-021).*
- **Quantities and budgets** (later): counts of every component and of the missing ones, pipe and cable lengths from real distances, and installation effort, priced from catalogs.
- **Documents attached to components.** Datasheets, manuals, photos and past drawings that people and agents can open and cite.
- **Documentation as a by-product.** The professional set is ready when the design is, so installation can start as soon as design ends.
- **Any scale.** From one skid to a whole site, navigable by system and area.
- **Compatibility across standards.** Projects mix measuring systems and standards: metric and imperial, ISO and ASTM pipe, BSP and NPT threads, 50 and 60 Hz, IEC and ANSI symbols. Hyleon would find where they conflict, and list the adapters and conversions needed before anything ships to site. In scope: it's a family of checks plus the region and owner profiles, not a separate product. The sample project comes in both variants to prove it.

## Already part of the plan

- One model across disciplines: [TRUTHS.md](../TRUTHS.md), truth 2.
- Packs of questions, limits, checks and deliverables: [formats/pack.md](formats/pack.md).
- A source on every value: truth 3.
- Professional vector output generated from the model: [PRODUCT.md](PRODUCT.md), "The five parts".
- Working by hand or with an agent: [PRODUCT.md](PRODUCT.md), "Working by hand, or with an AI agent".
