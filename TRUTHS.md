# Truths

Hyleon (working name). This file says why the product exists and what must stay true. It doesn't change: if one of these stops being true, it's a different product. Every document, feature and decision is checked against it.

## The problem

People who design and build physical systems (power and control panels, controls, networks, piping) increasingly use AI, and the AI gets it wrong in ways nobody catches. It starts designing before it knows what it needs to know, fills the gaps with guesses and loses track of details across drawings. Image generators make pictures that look technical but can't be edited, measured or checked. The work is split across tools that don't share data, so one pump carries different names on different sheets. Software has compilers and tests that catch mistakes in seconds. Physical design has nothing like them, so mistakes surface on site, where they cost the most.

## In one sentence

Hyleon is a test suite for physical design: it asks what must be known, shows where a design leaves its limits and checks every change, so people and their AI can trust a design before anything is built.

## In one paragraph

Every kind of job (a motor circuit, a drive bus, a pipe spool, a whole panel) comes with a framework. The framework holds:

- the questions to answer first
- the limits that apply, and where each comes from
- the checks a design must pass
- the documents it must produce

The design lives in one model that people and AI agents change only through checked steps. Every value records where it came from, and anything the AI guessed stays marked until a person confirms it. Drawings, schedules and guides are generated from the model, so they always agree. The record of who decided what, and why, stays with the work from the first question to the as-built set.

## What stays true

1. **Ask before designing.** Missing information becomes a question, never an assumption.
2. **The model is the design.** Drawings, lists and guides are views of it; a picture is never the source.
3. **Every value has a source:** a person, a standard, a catalog, a calculation or an AI guess.
4. **The person decides.** The AI proposes. A person confirms its guesses, accepts its limits and signs the work.
5. **Limits need a source.** A boundary is in force only if it comes from a code, a specification or a person's answer.
6. **Nothing is hidden.** Hyleon shows every finding and every AI guess as the work happens. The person decides what to issue, and the record shows what they accepted and why.
7. **The work belongs to its owner.** It stays on their devices unless they share it, and it's never used to train AI.
8. **Open to any AI and any tool.** People bring the agent they choose, and specialist tools do what they do best.

## What Hyleon is not

- Not a picture maker: it produces designs that can be checked, not images to look at.
- Not a replacement for CAD, electronics tools or an engineer's judgment. It never stamps or certifies.
- Not a copy of the codes: it points to the clause.
- Not tied to one AI vendor.
- Not a gatekeeper: it informs professionals, it never overrules them.

## Deciding with this

Before building or choosing anything, ask:

1. Does it make a design more trustworthy before it's built?
2. Does it keep the person in charge and the model the only source?
3. Does it ask instead of assume?

If any answer is no, it doesn't belong in Hyleon, however good it is.

Source: Dan's words, 2026-10-28 (`docs/brief/2026-10-28.md`).
