---
name: debug
description: Systematic root-cause debugging — reproduce the bug with a tight red/green loop, trace it to the actual cause, and fix that, not the symptom. Use for any test failure, production bug, crash, unexpected behavior, performance problem, or build failure — especially under time pressure, or after a previous fix didn't hold.
---

# debug

Random fixes waste time and breed new bugs; quick patches hide the real problem. This skill is the discipline that prevents that: understand the bug completely before you change a line.

## The one rule that overrides everything

**No fix without a confirmed root cause and a repro that proves it.**

- If you haven't reproduced the bug and traced it to its cause, you cannot propose a fix — a guess is not a fix.
- A symptom patch (swallow the error, add a null-check where it crashed, retry the flaky call) is a failure, not a solution. It leaves the cause live and every sibling code path still broken.
- Fix the cause once, where all callers route through it — not the one path the report happened to name.

Being in a hurry is not an exception. Systematic is *faster* than thrashing through fixes that don't hold.

## The feedback loop is the work

Before reading code to build a theory, create or identify **one tight command** that goes **red on the exact symptom** and **green only when the bug is truly fixed**. A tight loop is:

- **Fast** — runs in seconds so you can iterate.
- **Deterministic** — same result every run (for flaky bugs, raise the reproduction rate first).
- **Specific** — catches *this* bug, not merely "doesn't crash."
- **Agent-runnable** — you can trigger it yourself, on demand.

Build the loop in roughly this order: a **failing test** at the seam that reaches the bug → an **HTTP/curl script** against a running dev server → a **CLI invocation** with fixture input, diffing output → a manual repro with logging as a last resort. When a clean repro is hard, spend *disproportionate* effort building the loop anyway. Guessing without a red-capable loop is the exact failure this skill exists to prevent.

## The four phases

Complete each before the next. If a later phase disproves your understanding, drop back to Phase 1 — don't push forward on a broken theory.

### Phase 1 — Reproduce & investigate

1. **Read the error literally.** Full stack trace, line numbers, file paths, codes. The message often names the cause outright — don't skim past it.
2. **Get the loop red** (see above). Reproduce the user's exact symptom on demand.
3. **Shrink the repro.** Once red, cut inputs, callers, config, and steps **one at a time**, re-running after each cut. Keep only what's load-bearing for the failure. A one-line repro points at the cause; a whole-app repro hides it.
4. **Trace the data flow.** Follow the actual value from where it's wrong back to where it first went wrong. The crash site is rarely the cause site.

### Phase 2 — Analyze patterns

- **Grep for siblings.** Find every other place that uses the same function, pattern, or assumption. If the bug is here, is it there too? This is what turns a one-path patch into a root-cause fix.
- **Understand the surrounding code and its dependencies** before deciding anything is wrong. Code that looks buggy in isolation is often correct given a contract you haven't read yet.
- Check recent changes to the suspect area — `git blame`/`git log` on the traced lines often hands you the cause.

### Phase 3 — Hypothesize & test

- Generate **3–5 plausible hypotheses** before testing any single one — first-guess tunnel vision is how you fix the wrong thing.
- Rank them by likelihood × cheapness to falsify. Test the cheapest-to-kill first.
- Each hypothesis must make a **testable prediction**: "If X is the cause, then observing/changing Y makes Z happen." No prediction → sharpen it or discard it.
- **Change one variable at a time** and re-run the loop. Binary-search the cause: disable half, see which half keeps it red, repeat. A confirmed cause is one where you can turn the bug on and off at will.

### Phase 4 — Fix & verify

- Write the **minimal fix at the root**, not a patch at the symptom. If Phase 2 found siblings, fix them in the same shared place.
- **The loop goes green.** The red repro from Phase 1 now passes — that's the proof the fix works.
- **Keep the repro as a regression test** so the bug can't come back silently.
- **Run the full suite** — the fix must not turn anything else red. A fix that trades one bug for another isn't done.
- State the root cause in one sentence when you report — if you can't, you fixed a symptom.

## Red flags — STOP and return to Phase 1

Any of these means you've left the process:

- Proposing a fix you haven't reproduced.
- "Let me just try changing this and see" — with no hypothesis and no prediction.
- Adding a null-check / try-catch / retry at the crash site without knowing why the value is bad.
- Second (or third) fix attempt because the last one didn't work.
- "It's probably X" without a loop that can prove it.
- Fixing only the path in the report while known siblings stay untouched.

## When NOT to use this skill

- The learner wants to be *coached* through debugging to build the skill themselves → use `tutor` (its `references/debugging.md` coaches the method instead of finding the bug).
- The task is triaging many unknown issues across an app, not driving one to root cause → use `qa-sweep`.
- It's a trivial, already-understood one-liner (obvious typo you can see) → just fix it; the ceremony isn't worth it.
