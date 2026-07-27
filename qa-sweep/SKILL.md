---
name: qa-sweep
description: Systematic exploratory QA of a running web app — navigate it, exercise the flows, capture evidence, and produce a severity-ranked bug report. Optionally files each finding as a structured issue via gh-workflow. Use when asked to QA, dogfood, smoke-test, or "find bugs in" a web app, page, or feature before shipping.
---

# qa-sweep

A structured exploratory QA pass over a running web app. You drive the app like a skeptical user, capture proof of every issue, classify it, and hand back a ranked report — then optionally file the findings as issues.

## What you need

- **A browser/automation toolset** — the ability to navigate to a URL, snapshot the page (DOM/accessibility tree), click and type, read the console, scroll, and take screenshots. In Claude Code this is the Playwright MCP browser tools; in other harnesses it's whatever browser control is available. This skill is written against those *capabilities*, not any one tool's names.
- **From the user:** a **target URL**, a **scope** (which areas/flows, or "full site"), and optionally an **output dir** (default `./qa-sweep-output/`).
- **Optional prior QA context:** a `qa-import/v1` JSON document produced by [`qa-import`](../qa-import/SKILL.md).

If any is missing, ask once, then proceed with sensible defaults.

## Safety first — this skill drives a live app

Exploratory QA clicks and submits forms on a real site, so it can cause real side effects. Before anything else:

- **Target staging/test by default.** Only run against production with the user's explicit go-ahead — confirm the environment before the first navigation.
- **Non-destructive by default.** Never submit real purchases, delete data, spam signups, send messages, or trigger emails/webhooks. Use obvious throwaway test data (`qa-sweep test`).
- **Ask before any irreversible or outward-facing action** (payments, deletions, anything that notifies real people). When unsure whether a submit mutates real state, don't — note it as "needs manual testing" instead.

## The one rule

**Every finding needs reproducible evidence.** A bug report without a screenshot, the exact steps, and the observed-vs-expected is a rumor, not a finding. If you can't reproduce it, say so and mark it unconfirmed — never pad the report with guesses.

## The five phases

### 1. Plan

Set up `{output_dir}/screenshots/` and sketch a rough map of what to test from the scope:
- Landing/home, then navigation (header, footer, sidebar).
- Key user flows (sign up, login, search, create, checkout — whatever the app does).
- Forms and interactive elements.
- Edge cases: empty states, error pages, 404s, invalid input, back-button, reload mid-flow.

Keep the plan lightweight — a checklist, not a spec. Adapt it as you discover the app.

If prior `qa-import/v1` JSON is provided, load its `source`, `summary`, and `records` before exploring. Use historical Fail rows as candidate regression checks and their source locations/raw fields for traceability. Group and sequence related checks by non-null `module` when that improves coverage planning; keep null-module records in an unclassified group rather than guessing. A module is routing/planning context, not evidence that the historical row is a bug. Imported rows are still `unconfirmed` until reproduced against this target. Pass rows describe previously observed coverage, not proof that the flow passes now; retest relevant flows. Feature Requested rows are product requests, not confirmed current bugs, and stay out of findings unless current behavior independently violates an explicit expectation. Do not invent a category for `null` imports—classify only after observing the current app.

### 2. Explore

For each page/flow in the plan:
1. **Navigate** to it.
2. **Snapshot** the DOM/accessibility tree to understand structure and find interactive refs.
3. **Clear and read the console** — do this after every navigation and every significant interaction. Silent JS errors are high-value findings.
4. **Look at it** — screenshot (annotated if your tool supports numbering interactive elements) and assess layout, broken elements, and obvious a11y problems.
5. **Exercise it systematically** — click buttons/links, fill and submit forms (valid, invalid, and empty), tab through with the keyboard, scroll long content, hit back/reload.
6. **After each interaction**, re-check the console and look for visual regressions or dead-ends.

Note anything off *as you go* — don't trust memory to Phase 3.

### 3. Collect evidence

For each candidate issue, capture while it's fresh:
- A **screenshot** saved to `screenshots/` with a descriptive name.
- The **exact steps** to reproduce (URL + clicks + input).
- **Observed vs. expected** behavior.
- Any **console output** verbatim.
- A candidate **module** only when the current evidence and known architecture support one.

### 4. Classify

Score each issue against [`references/severity.md`](./references/severity.md):
- **Severity:** Critical / High / Medium / Low.
- **Category:** Functional / Visual / Accessibility / Console / UX / Content.
- **Module (optional):** the narrowest evidence-supported ownership/layer label, using project-consistent names such as Backend, Frontend, Mobile, Infrastructure, AI/Chatbot, or Integration. Leave it null/omitted when uncertain, and do not confuse a feature/product name with a technical module.

Module is an independent planning and routing dimension. It does not prove that behavior is a bug and must not determine confirmation, severity, or category. Be honest—inflating severity or ownership confidence trains the reader to ignore you.

### 5. Report

1. De-duplicate — merge the same bug seen in multiple places into one finding.
2. Assign final severity + category; sort Critical → Low.
3. Fill [`templates/report.md`](./templates/report.md): executive-summary counts, overall assessment, then one entry per finding with its evidence and evidence-supported module when known.
4. Save to `{output_dir}/report.md`.

### 6. File the findings (optional)

Offer to turn findings into tracked issues: "Found 7 issues — want me to file the Critical/High ones?" If yes, hand each to **gh-workflow** (its `create-issues` subskill), mapping fields into the Context / Problem / What to decide-do / References format (the reproduction steps become Problem; the screenshot paths and URLs become References). Keep the report as the human-readable summary; issues are the actionable backlog. Skip trivial/Low items unless asked.

## Notes

- **Breadth first, then depth.** One good pass over everything beats an exhaustive audit of the landing page. Note areas that need deeper testing rather than blocking on them.
- Grounding an issue in an existing ticket? Pull it with **gh-workflow** (its `fetch-issue` subskill) before filing a duplicate.
- Imported QA JSON is prior context, never evidence by itself. A report finding still needs current reproduction and evidence under **The one rule**.

## When NOT to use this skill

- The user wants automated regression tests written → use a test framework / the `tdd` skill, not exploratory QA.
- A single specific bug is already known and needs fixing → just fix it.
- A pure security assessment → use `security-review`; this sweep flags obvious issues but isn't a pentest.
