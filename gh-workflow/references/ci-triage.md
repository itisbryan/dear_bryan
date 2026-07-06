# ci-triage (subskill of gh-workflow)

Load this when the user wants to check **CI / GitHub Actions** status — "why is my PR red", "watch the build", "what failed" — and diagnose a failing run. Covers Actions runs and the checks on a PR.

## Prerequisites

`gh` authenticated. Run inside the repo, or add `--repo owner/repo`. Actions must be enabled on the repo.

## 1. See the status

For a PR (a number, URL, branch, or nothing = the PR for the current branch):

```bash
gh pr checks <ref>                    # every check + pass/fail/pending, with links
gh pr checks <ref> --watch            # live-update until they settle
```

For workflow runs directly:

```bash
gh run list --limit 15                            # recent runs across workflows
gh run list --branch <branch> --limit 10          # runs for one branch
gh run list --workflow ci.yml --status failure    # failures of a specific workflow
```

## 2. Watch an in-progress run

```bash
gh run watch <run-id>                 # blocks, streams status until done, exits non-zero if it failed
gh run watch <run-id> --exit-status   # same, explicit non-zero on failure — good for scripting
```

Get the `<run-id>` from `gh run list`, or `gh run list --json databaseId -q '.[0].databaseId'` for the latest.

## 3. Diagnose a failure — go straight to the failing logs

Don't dump the whole log. Pull only what failed:

```bash
gh run view <run-id>                  # summary: which jobs/steps failed (✗ marks them)
gh run view <run-id> --log-failed     # logs of ONLY the failed steps — start here
gh run view <run-id> --job <job-id> --log   # full log of one job when --log-failed isn't enough
```

`--log-failed` is the workhorse: it skips every green step and shows just the error output. Read it, find the actual error line (assertion, compiler error, missing dep, exit code), and trace to the cause — the failing step names the command; the error above the `Process completed with exit code N` line is what broke.

> Large logs: don't read the raw dump into context. Pipe to a search — `gh run view <run-id> --log-failed | grep -iE "error|fail|exception" -A3` — or process it, and surface only the derived cause.

## 4. Reproduce locally

The failing step is a shell command in the workflow. Run that exact command locally to reproduce, rather than guessing from the log:

```bash
gh run view <run-id> --log-failed        # note the command the failed step ran
# ...then run that command in your checkout
```

For a PR that's red, `gh pr checkout <ref>` first so you're on the same code.

## 5. Fix or re-run

- **Flaky / transient** (network, timeout, runner hiccup) — re-run without changing code:

  ```bash
  gh run rerun <run-id>                 # re-run the whole run
  gh run rerun <run-id> --failed        # re-run only the failed jobs (faster)
  ```

- **Real failure** — fix the code, push; a new run triggers automatically. Confirm with `gh pr checks <ref> --watch`.

Don't reflexively re-run a red build hoping it passes — re-run only when you have reason to believe it's transient. A real failure re-run just burns minutes and hides the bug.

## Rules

- Go to `--log-failed` first; never dump full logs into context — grep/process and surface the cause.
- The failed step is a real command — reproduce it locally instead of guessing.
- Re-run only for transient failures; fix real ones at the source.
- To watch without blocking your turn, prefer a single `gh pr checks --watch` / `gh run watch` rather than polling in a loop.
