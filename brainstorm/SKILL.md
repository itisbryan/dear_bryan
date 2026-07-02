---
name: brainstorm
description: Turn a rough idea into an approved design before any code — explore intent, constraints, and 2-3 approaches through one-question-at-a-time dialogue, then write a design doc the user signs off on. Use before creating a feature, component, or behavior change, especially when a task seems "too simple to need a design."
---

# brainstorm

Turn ideas into designs through collaborative dialogue: understand what's really wanted, weigh a few approaches, and get a design approved *before* building anything.

## The one rule that overrides everything

**No implementation before an approved design.** No code, no scaffolding, no invoking a build skill until you've presented a design and the user has said yes — for *every* creative task, regardless of how simple it looks.

"This is too simple to need a design" is a trap: simple-looking work is exactly where unexamined assumptions waste the most effort. The design can be three sentences for a trivial feature — but it must be presented and approved. (This gate is for creative/behavioral work; a pure typo fix or a config value with no decision isn't that — see "When NOT to use.")

## The process — in order

1. **Explore context.** Read the relevant files, docs, and recent commits. Understand what already exists before proposing anything.
2. **Ask clarifying questions — one at a time.** Purpose, constraints, success criteria, non-goals. Multiple-choice beats open-ended when you can offer it. Never stack three questions in one message; ask, wait, follow the answer.
3. **Propose 2-3 approaches.** Each with its trade-offs. Lead with your recommendation and say why — don't just enumerate.
4. **Present the design in sections.** Scale each section to its complexity. Get approval per section; on "no," revise that section before moving on.
5. **Write the design doc.** Save to `docs/specs/<YYYY-MM-DD>-<topic>.md` (honor the user's preferred location if they have one) and commit it. It's an artifact, not a chat message.
6. **Self-review the spec with fresh eyes** (see checklist). Fix issues inline before showing it.
7. **User reviews the written spec.** Changes requested → revise and re-review. Approved → proceed.
8. **Hand off to implementation** (see handoff below).

## Principles

- **One question at a time.** Overwhelming the user with a battery of questions gets shallow answers.
- **YAGNI ruthlessly.** Cut every speculative feature from every design. The lazy design that covers the real need beats the flexible one nobody asked for.
- **Always explore alternatives.** Even when the first idea seems right — a second option sharpens the reasoning.
- **Incremental validation.** Approve as you go; don't dump a wall of design and ask "good?"
- **Be flexible.** Loop back and re-clarify the moment something stops making sense.

## Spec self-review checklist

Look at the written spec as if someone else wrote it:

- [ ] No placeholders, `TODO`s, or "TBD" left in.
- [ ] No contradictions between sections.
- [ ] No ambiguity a builder would have to guess at.
- [ ] Scope matches what was agreed — nothing crept in, nothing dropped out.

## Design doc shape (scale to the project)

- **Goal** — one or two sentences: what, and why it matters.
- **Approach** — the chosen option and the reasoning; note the alternatives you rejected and why.
- **Scope** — what's in, and what's explicitly out (non-goals).
- **Details** — data shapes, interfaces, edge cases, states — only as deep as the project needs.
- **Acceptance** — how we'll know it's done.
- **Open questions** — anything still unresolved.

## When NOT to use this skill (and where to hand off)

- **After approval → build it.** Hand to `tdd` for feature/behavior work — the design's acceptance criteria become the first failing tests — or your normal implementation flow.
- **Want the design tracked?** File it via `gh-workflow` (its `create-issues` subskill) so the spec becomes a ticket with Context / Problem / What to decide-do / References.
- **The user wants to *learn*, not ship a design** → `tutor`.
- **A trivial, non-creative edit** — a typo, a rename, a config value with no decision to make — has no design to approve; just do it. The hard gate is for work with real choices, not mechanical changes.
- **A production emergency** → stabilize first, design the proper fix after.
