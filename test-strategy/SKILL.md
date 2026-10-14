---
name: test-strategy
description: Use when asked to create missing behavioral test coverage, audit weak or redundant tests, investigate a failing test that someone wants to skip, or optimize CI test time and cost. Supports "test-strategy create", "test-strategy audit", "test-strategy optimize", and "test-strategy optimize --ultra"; not a replacement for TDD while implementing ordinary code changes.
---

# Test strategy

Protect behavior, not the implementation that produced it. Test count and a green badge are not evidence that the application works. A fast CI job is not a success if it stops detecting regressions.

## Select a mode

| Request | Read | Deliver |
|---|---|---|
| `create` — build missing coverage | [references/create.md](references/create.md) | Behavior/risk/coverage map and only justified new tests |
| `audit` — find weak, stale, skipped, or redundant tests | [references/audit.md](references/audit.md) | Evidence-backed retain/repair/delete recommendations; changes only in the requested scope |
| `optimize` — reduce CI time/cost | [references/optimize.md](references/optimize.md) | Measured baseline, safe lane design, validation, and measured outcome |
| `optimize --ultra` | [references/optimize.md](references/optimize.md), **Ultra** section | Five independent approaches and a separate evidence-based verdict |

In OMP, invoke `/skill:test-strategy create`, `/skill:test-strategy audit`, `/skill:test-strategy optimize`, or `/skill:test-strategy optimize --ultra`. The `--ultra` token is an instruction to this skill, not an OMP CLI option. Other harnesses can invoke the skill by name and mode. If no mode is given, infer it from the request; if the user asks only about a failed test, use `debug` to investigate before modifying tests.

## Shared decision gate

Before adding or removing a test, answer:

1. Which **observable behavior** or invariant matters to a consumer?
2. What plausible regression would break it? Show the changed code, failure report, or contract that supports this risk.
3. Which existing test already detects that regression? Inspect assertions and setup, not just filenames or test counts.
4. What is the lowest **trustworthy** verification boundary? A unit test is not enough for behavior spanning components, persistence, or a browser.
5. Does another test add detection that current checks lack? If not, do not add it.

A test is valuable when it can fail for a realistic consumer-visible regression and pass for the correct behavior. Reject tautologies, mock echoes, bare not-throw assertions, source-text checks, wiring/copy tests, incidental defaults, and duplicates of the same path. Preserve distinct boundary, failure, precedence, and state-transition coverage even when tests look similar.

## Red builds are evidence

Investigate a failing check before changing it. Reproduce the failure, determine whether the product, test, fixture, or environment is wrong, and keep the assertion that protects valid behavior. Never comment out, skip, weaken, quarantine, or exclude a failing test merely to get CI green. Request explicit user approval before temporarily disabling a test, and name the owner, reason, and restoration condition. An unrelated failure is still a failure; report it rather than hide it. Use `debug` for root-cause reproduction and `tdd` for red-green work on new behavior.

## Scope and safety

Work from the target repository's contracts, tests, and CI configuration; do not impose a generic framework or lane layout. Unknown impact means broader verification, not optimistic exclusion. The skill makes recommendations; it does not automatically create a portable `/ak:test` command, change GitHub Actions, or provide process-level enforcement. Never claim CI savings without before/after measurements from the target repository.
