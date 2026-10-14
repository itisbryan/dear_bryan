# Optimize test time and cost

Optimize a specific repository with evidence; never paste a universal path-filtered CI workflow into an unknown project. Faster wall time can increase billed runner-minutes when jobs are parallelized.

## Establish a baseline

1. Read the workflows, required status checks/branch protection, test commands, dependency graph, ownership, and recent run history. Preserve the current pass/fail baseline, including flaky retries.
2. Measure representative runs on comparable hardware/cache state: per-job wall time, billed runner-minutes (sum of charged job durations), p50/p95, setup time, retries/flaky failures, and total end-to-end feedback time. Note concurrency and billing rounding. If run data is unavailable, describe how to collect it; do not claim an improvement.
3. Rank actual bottlenecks. Consider caching/setup, redundant invocations, parallelism, fixture/database cost, and test isolation before making selection more aggressive. Weigh maintenance complexity against saved minutes.

## Design safe lanes

- Identify fast required checks and tests with known dependencies. Map **changed paths → transitive affected behavior → tests**; paths alone are not a dependency graph. Include reverse dependencies and shared entry points (APIs, schemas, cross-package types, generated code).
- A localized change may run targeted tests plus fast required checks *only after* impact mapping has been validated. Unknown impact, shared infrastructure, dependencies/lockfiles, migrations, test harness, generated artifacts, or CI configuration take the conservative broad/full route. Never skip a required status check or hide a failing job.
- Preserve a full-suite safety net on a schedule and for release/stable builds. On PRs, run the full suite until the selection policy has demonstrated that it does not miss historical failures; rollout may compare selection against full runs in shadow mode.
- Build representative change fixtures, including a local component, a shared library/API, a schema or migration, a lockfile/test harness change, and an unknown path. For each, verify the selected lane includes every test known to detect its plausible regression and selects full coverage for uncertainty. Replay recent PRs/commits and compare selected vs full failures; a missed failure invalidates the map.
- If changing CI, ensure required check names still report for every PR and cannot appear green when selected work is missing or failed. Keep cancellation, concurrency, and cache behavior consistent with the repo's merge rules. Record a rollback path to full coverage.

## Rollout and report

First fix low-risk overhead, then trial any selective lanes behind the broad check, then remove redundant per-PR runs only after the shadow comparison is clean. Re-measure with comparable runs: wall time, billed minutes, p95, reruns, and missed-failure rate. If the safety or cost evidence is incomplete, leave broad coverage enabled and report the limitation. Periodic full-suite failures must be triaged, not accepted as the price of speed.

## Ultra: five approaches, independent verdict

Use only when explicitly requested with `--ultra`; this analysis costs extra agent work. When independent workers are available, dispatch five isolated proposals with the same measured baseline and constraints, without sharing drafts between them. Require **materially different** approaches (for example: setup caching, fixture reuse, test-level parallelism, job consolidation, validated impact lanes), each with expected benefit, required measurement, safety risks, and rollback. Reject five renamed versions of one idea.

Send the five proposals to a **different verifier** that has not authored them. Compare against the measured bottleneck, defect-detection risk, runner cost, complexity, and reversibility; reject unsupported savings claims and any plan that skips checks on unknown impact. Select the strongest supported approach or say that no change is justified. If independent workers or a separate verifier are unavailable, do not label a single-agent second pass an ultra verdict; report the limitation and provide the standard `optimize` analysis instead. Execute a CI change only if the user requested implementation and the target repo provides the measurements and workflow configuration needed to verify it.
