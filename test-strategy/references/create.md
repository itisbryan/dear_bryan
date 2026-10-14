# Create missing coverage

Use for an existing behavior with a coverage gap, or before changing behavior when a red regression test is needed. Follow the project's test conventions and keep the smallest set of tests that would catch distinct failures.

## Reconnaissance

1. Read the requested contract, change/failure report, relevant code, existing tests, and test commands. Identify callers and cross-component boundaries; a nearby test file is not proof of coverage.
2. Write a short map for each risk: **behavior → plausible wrong outcome → existing detecting assertion (or none) → proposed check**. Include unhappy paths and boundaries only when their failure would matter to a consumer.
3. Choose the lowest trustworthy level: a pure rule at unit level; a cross-module or database invariant at integration level; a user flow requiring browser/server at end-to-end level. Prefer a cheaper level only if it actually observes the failure. Reuse existing fixtures/helpers where they do not erase the behavior under test.
4. For each proposed test, name a realistic bug it would fail on (wrong return, stale state, missed rollback, broken API response, etc.). If no such bug can be named, reconsider adding it.

## Build and verify

- When changing code, write the focused test before the fix and observe failure for the expected behavior, not for setup/import errors. When adding coverage around *already-correct* behavior, a passing test can be legitimate; demonstrate its sensitivity by temporarily perturbing the implementation or by identifying a concrete known regression it would catch. Do not commit the perturbation.
- Exercise real code and assert outputs, state, errors, side effects, or transitions visible to a consumer. Mock only unavoidable external boundaries, not the behavior under test. Avoid copying production logic into expected values.
- Keep tests deterministic and independent. Cover distinct conditions rather than multiple rows that take the same path. Prefer an existing meaningful test over a new duplicate; modify it only if the strengthened assertion preserves its existing contract.
- Run the focused test and relevant impacted checks. In a repository without a validated impact-selection policy, run its required broad suite before declaring the change complete. A local smoke of the changed application path is still needed; tests alone do not establish runtime behavior.

## Deliver

Report the behavior/risk map, tests retained or added, one plausible regression each added test detects, commands actually run, and any uncovered risk. If existing coverage suffices, say **no new test** and show the detecting assertion rather than writing a test for appearance's sake.
