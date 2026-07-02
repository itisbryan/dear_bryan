---
name: tdd
description: Test-driven development — write the failing test first, watch it fail, write the minimal code to pass, then refactor. Enforces red-green-refactor for new features, bug fixes, refactors, and behavior changes. Use when building or changing behavior and you want the tests to drive the design, not trail behind it.
---

# tdd

Write the test first. Watch it fail. Write the minimal code to make it pass. Then clean up. The test comes before the code — always — because a test you never watched fail proves nothing.

## The one rule that overrides everything

**No production code without a failing test first.**

- Wrote the code before the test? Delete it and start over from the test. Not "keep it as reference," not "adapt it while I write the test," not "just look at it" — delete means delete. Implement fresh, driven by the test.
- If you didn't *watch the test fail*, you don't know it tests the right thing — it might pass for the wrong reason, or test nothing at all.

"Skip TDD just this once" is a rationalization, not an exception. The one place to genuinely pause and ask the user first: throwaway prototypes, generated code, and config files.

## Red → Green → Refactor

### RED — write one failing test

One minimal test showing the behavior you want. Requirements:
- **One behavior per test.** An "and" in the test name means split it.
- **Name describes the behavior, not the implementation** — `retries_failed_operation_three_times`, not `test_retry_works`.
- **Exercise real code, not mocks.** Mock only what's genuinely unavoidable (network, clock, paid API). A test that only checks a mock tests nothing.

```
test "retries a failing operation up to 3 times, then succeeds":
    attempts = 0
    operation = () => { attempts++; if attempts < 3 throw Error("fail"); return "ok" }

    result = retry(operation)

    assert result == "ok"
    assert attempts == 3
```

Clear name, real behavior, one thing. (Vague name + assert-on-a-mock = the anti-test.)

### Verify RED — watch it fail (mandatory, never skip)

Run *just this test*. Confirm:
- It **fails** — it doesn't error from a typo or import mistake.
- It fails for the **expected reason**: the feature is missing, not the test is broken.

Passes immediately? You're testing behavior that already exists — fix the test. Errors instead of fails? Fix the error and re-run until it fails *correctly*.

### GREEN — the minimal code to pass

Write the simplest thing that makes the test pass. Nothing more — no extra features, no logging, no "while I'm here" improvements to nearby code.

**Cheating is not just allowed in GREEN, it's encouraged:** hardcode the return, copy-paste, duplicate, skip edge cases. The next RED test will force the generalization. Getting to green fast keeps the loop tight; elegance is the refactor step's job.

### REFACTOR — clean up with the tests green

Now improve the code — remove the duplication, extract, rename, generalize the hardcoded value — **re-running the tests after each change** so they stay green. Refactor only what the passing tests already cover; new behavior needs a new RED test first. If a refactor turns something red, you changed behavior — revert and drive it with a test.

## Verification checklist (before you move on)

- [ ] The test was written before the code.
- [ ] You watched it fail, for the right reason.
- [ ] The code is the minimum that passes — nothing speculative.
- [ ] All tests (not just the new one) are green.
- [ ] One behavior per test; names describe behavior.

## Red flags — STOP and start over

- Writing production code with no failing test in front of it.
- "I'll add the tests after." (Then they test what you wrote, not what you meant.)
- Test passed the first time you ran it — and you moved on anyway.
- Adding logic in GREEN the current test doesn't require.
- Writing five tests before making the first one pass — go one at a time.
- Refactoring while a test is red.

## When NOT to use this skill

- The learner wants to *learn* TDD by doing it themselves → use `tutor`, which coaches rather than writes.
- You're chasing an existing bug to its cause → use `debug`; its Phase 1 (a red repro at the seam) *is* the failing test — start there, then this cycle takes over for the fix.
- Throwaway spike / generated code / config → ask the user before skipping; otherwise TDD still applies.
