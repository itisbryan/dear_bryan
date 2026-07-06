# review-pr (subskill of gh-workflow)

Load this when the user wants to **review** a pull request from the CLI (someone else's PR, or a bot/AI PR), or **respond to review feedback** on their own PR. This is the read-and-react side; opening a PR lives in `create-pr`.

## Prerequisites

`gh` authenticated. Run inside the repo, or target another with `--repo owner/repo`. A PR reference is a number (`123`), a URL, or a branch — `gh pr view` resolves all three.

## A. Reviewing someone's PR

### 1. Understand it before judging

```bash
gh pr view <ref>                          # title, body, author, state
gh pr diff <ref>                          # the actual change
gh pr checks <ref>                         # CI status — don't review a red PR's logic first, see why it's red
gh pr view <ref> --json files,additions,deletions,reviewDecision
```

Read the diff fully. A review that only reacts to the description misses what the code actually does. For a large PR, `gh pr diff <ref> --name-only` to scope, then read the risky files closely.

### 2. Check it out locally if it needs running

```bash
gh pr checkout <ref>     # puts you on the PR's branch
```

Do this when correctness can't be judged by eye — run the tests, exercise the flow, reproduce the bug it claims to fix. Skip for trivial diffs.

### 3. Decide the verdict

- **Approve** — correct, in scope, tests cover it.
- **Request changes** — a real defect, a correctness/security risk, or missing tests on non-trivial logic. Name the specific blocker.
- **Comment** — questions or non-blocking suggestions; no verdict.

Review the *change*, not the whole file. Flag out-of-scope concerns as separate follow-up issues (`create-issues`) rather than blocking the PR on them.

### 4. Post the review

Body-only verdict:

```bash
gh pr review <ref> --approve  --body "LGTM — verified tests pass locally."
gh pr review <ref> --request-changes --body "Blocking: <specific issue>. See inline."
gh pr review <ref> --comment  --body "Question about the retry logic below."
```

**Inline comments** aren't supported by `gh pr review` directly — use the API to attach line comments, then submit:

```bash
# One inline comment on a specific file/line (RIGHT side of the diff):
gh api repos/{owner}/{repo}/pulls/<N>/comments -f body="This can NPE if list is empty" \
  -f commit_id="$(gh pr view <ref> --json headRefOid -q .headRefOid)" \
  -f path="src/foo.ts" -F line=42 -f side=RIGHT
```

For several inline comments plus a verdict in one submission, POST to `pulls/<N>/reviews` with a `comments[]` array and an `event` of `APPROVE`/`REQUEST_CHANGES`/`COMMENT`. Batch them so the author gets one notification, not ten.

## B. Responding to feedback on your own PR

### 1. Read the threads

```bash
gh pr view <ref> --comments                                   # issue-level discussion
gh api repos/{owner}/{repo}/pulls/<N>/comments -q '.[] | {path,line,user:.user.login,body}'   # inline review comments
```

### 2. Address, then reply

Make the code changes, push, then reply to each thread so reviewers see it's handled — don't silently push. Reply to a specific inline comment:

```bash
gh api repos/{owner}/{repo}/pulls/<N>/comments/<COMMENT_ID>/replies -f body="Fixed in <sha> — extracted the guard."
```

For a comment you disagree with, reply with the reasoning rather than ignoring it. Re-request review once all threads are addressed:

```bash
gh pr edit <ref> --add-reviewer <username>
```

## Rules

- Read the full diff (and run it when non-trivial) before any verdict — no drive-by approvals.
- One review submission with batched inline comments, not a stream of separate ones.
- Block only on real defects; route scope creep to follow-up issues.
- On your own PR: reply to every thread you've addressed, then re-request review.
