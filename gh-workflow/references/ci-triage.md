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

## 3. Diagnose a failure — grep the failure output first

Avoid dumping or reading the whole log. `grep` is linear and normally cheap; fetching and decompressing the Actions log is usually the expensive part. Identify the failed job, fetch only failed-step output, and search that output for likely root-cause lines:


```bash
gh run view <run-id> --json jobs \
  --jq '.jobs[] | select(.conclusion == "failure") | "\(.databaseId)\t\(.name)"'

gh run view <run-id> --log-failed \
  | grep -iEn -m 20 -B3 -A6 \
      'error|fatal|exception|traceback|assert(ion)?|failed|failure|timed out|exit code|command not found|no such file|permission denied' \
  | sed -n '1,160p'

```

The first command identifies the failing job IDs. The second command surfaces matching lines with nearby context and caps the displayed result. Start with this filtered output; do not read unfiltered logs.

If the failure-only output does not identify the cause, target the specific job and keep the full log behind the same filter:

```bash
gh run view <run-id> --job <job-id> --log \
  | grep -iEn -m 20 -B3 -A6 \
      'error|fatal|exception|traceback|assert(ion)?|failed|failure|timed out|exit code|command not found|no such file|permission denied' \
  | sed -n '1,200p'
```

Read a narrow surrounding range only after a match points to a relevant step. The failing step names the command; the error above the `Process completed with exit code N` line is usually the root cause.

## 4. Reproduce locally

The failing step is a shell command in the workflow. Extract that command from the filtered failure output and run it locally to reproduce, rather than guessing from the log:

```text
filtered failure output → exact failing command → local reproduction
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
