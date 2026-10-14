# Audit test value

Audit is evidence gathering, not a mass deletion pass. A short, simple test may protect an important boundary; an elaborate test may only mirror implementation. Review both at the behavior level.

## Inventory and classify

1. Identify the suite's actual commands, CI jobs, ownership, and tests in scope. Sample representative failures and recent edits when available. Note flaky tests, skips, test-only branches, mocks that replace the subject, and assertions that never observe a result.
2. For each candidate, name the claimed behavior and a plausible defect that should fail it. Inspect the test and production path. Compare overlapping tests for **distinct inputs, boundary conditions, states, failure modes, and integration layers**; two tests are redundant only if removing one loses no meaningful detection.
3. Categorize with evidence:
   - **Retain:** detects a plausible regression not otherwise covered.
   - **Repair:** valuable contract, but current assertion, fixture, isolation, or flakiness prevents detection.
   - **Delete:** tautology, implementation-text check, mock echo, obsolete contract, or fully duplicated detection with no distinct case.
   - **Investigate:** skip, nondeterminism, or unexplained failure whose value/cause remains uncertain.
4. Where practical, prove sensitivity with a controlled, temporary defect or mutation of the behavior (not by editing the assertion). Compare before/after detecting tests; restore source after the experiment. Mutation score alone is not a deletion criterion.

## Changes and failures

- Produce an evidence table first: test, behavior, defect it would catch, overlap, action, confidence. Request/observe scope for deletions; keep uncertain cases until investigated. Remove obsolete fixtures or helpers only when no other test uses them.
- A red test is not redundant merely because it is red. Reproduce and classify product bug, bad expectation, fixture drift, environment failure, or nondeterminism. Fix the root cause, not the indicator. Never turn a failure into a skip or weaken an assertion for CI cosmetics; disabling requires explicit user approval and a restoration condition.
- For each deletion or repair, run the affected suite, then required broader checks. State which behavior is still protected and by which assertion. Do not report a smaller test count as improved reliability without evidence of retained defect detection.

## Deliver

Provide findings ranked by risk, exact retain/repair/delete decisions, evidence for any deletion, failures still open, and checks actually run. If the audit was report-only, do not edit tests.
