# Issue taxonomy (subskill of qa-sweep)

Classify every finding on two axes: **severity** (how much it hurts) and **category** (what kind of problem). Load this in Phase 4. Score honestly — an inflated report is one nobody acts on.

## Severity

### 🔴 Critical
A core feature is completely unusable, or data is lost.
- App crashes or shows a blank white page.
- Form submission silently loses the user's input.
- Login is completely broken — no one can get in.
- Checkout/payment fails at the final step.
- Uncaught exceptions that break a core page.

### 🟠 High
A key feature is broken but has a workaround, or a common path is badly degraded.
- A primary button does nothing (but reload recovers it).
- Search returns nothing for valid queries.
- Validation rejects valid input.
- A nav link 404s or lands on the wrong page.
- Critical content missing or garbled on load.

### 🟡 Medium
A feature works but is noticeably flawed; a secondary path is broken.
- Confusing error messages, or errors with no message.
- Layout breaks at common viewport sizes.
- Slow, janky, or non-obvious interactions.
- Keyboard/focus traps in a non-critical flow.
- Minor data shown incorrectly (wrong format, stale value).

### 🔵 Low
Cosmetic or trivial — worth noting, not worth blocking a release.
- Typos, grammar, leftover placeholder / lorem ipsum.
- Small spacing/alignment inconsistencies.
- Missing favicon.
- Debug/info console noise that shouldn't ship.
- Subtle contrast issues that still pass WCAG.

## Category

- **Functional** — it doesn't do what it should (broken logic, dead buttons, bad validation, wrong results).
- **Visual** — it looks wrong (layout breaks, overflow, misalignment, z-index, responsive failures).
- **Accessibility** — barriers for assistive tech (missing labels/alt, no focus states, keyboard traps, contrast failures, bad heading order).
- **Console** — errors/warnings in the browser console (uncaught exceptions, failed requests, deprecations).
- **UX** — it works but frustrates (unclear affordances, no feedback, hidden state, dead-ends, confusing flow).
- **Content** — the words/media are wrong (typos, placeholder text, broken images, stale copy).

## Scoring tips

- **Severity is about impact, category is about type** — a typo in the checkout total is Content by type but High by impact. Score both independently.
- When torn between two severities, ask "would this block the release?" Yes → the higher one.
- One issue, one category — pick the primary one; note a secondary in the description if it genuinely spans two.
