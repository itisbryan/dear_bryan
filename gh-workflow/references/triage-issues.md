# triage-issues (subskill of gh-workflow)

Load this when the user wants to **manage existing issues** — label, assign, close, comment, link to PRs, or spin a branch off an issue. Creating issues lives in `create-issues`; viewing one lives in `fetch-issue`.

## Prerequisites

`gh` authenticated. Run inside the repo, or add `--repo owner/repo`. An issue reference is a number or URL.

## Find what needs triage

```bash
gh issue list --state open --limit 50
gh issue list --search "no:label"                      # untriaged
gh issue list --search "no:assignee is:open"           # unassigned
gh issue list --label bug --state open
gh issue list --search "sort:updated-asc"              # staleest first
```

`gh issue list --json number,title,labels,assignees` when you need to process the set programmatically rather than eyeball it.

## Label

```bash
gh label list                                          # see what exists first
gh issue edit <N> --add-label "bug,priority:high"
gh issue edit <N> --remove-label "needs-triage"
gh label create "area:api" --color 0e8a16 --description "API surface"   # only if a needed label is missing
```

Don't invent a taxonomy — reuse existing labels. Create a label only when there's a real gap, and tell the user you did.

## Assign

```bash
gh issue edit <N> --add-assignee @me
gh issue edit <N> --add-assignee <username>
gh issue edit <N> --milestone "v1.2"
```

## Comment / close

```bash
gh issue comment <N> --body "Reproduced on main@<sha>. Root cause is <...>."
gh issue close <N> --reason completed   --comment "Fixed in #123."
gh issue close <N> --reason "not planned" --comment "Out of scope — see #456."
gh issue reopen <N>
```

Always close *with a reason and a comment* pointing to the PR/issue that resolved it — a bare close leaves no trail.

## Link issues and PRs

- To auto-close an issue when a PR merges, put a closing keyword in the **PR body**: `Closes #12`, `Fixes #34` (handled in `create-pr`).
- To reference without closing, mention `#N` in a comment.
- Track dependencies by adding `Blocked by #7` / `Part of #99` to the issue body via `gh issue edit <N> --body`.

## Branch off an issue

`gh issue develop` creates a branch linked to the issue (the PR you open from it auto-links back):

```bash
gh issue develop <N> --name "fix/<slug>" --checkout      # create + switch to it
```

Then implement, and open the PR with `create-pr` — the linkage is already there.

## Bulk triage

Loop the JSON list to apply the same action across many issues. Example — label every open issue with no label as `needs-triage`:

```bash
for n in $(gh issue list --search "no:label is:open" --json number -q '.[].number'); do
  gh issue edit "$n" --add-label needs-triage
done
```

For bulk **close** or **assign**, confirm the exact set with the user first (echo the numbers + titles) — bulk mutations are hard to undo.

## Rules

- Reuse existing labels/milestones; create new ones only for a genuine gap, and say so.
- Close with a reason **and** a comment linking the resolution — never a bare close.
- Confirm the target set before any bulk close/assign; a wrong filter hits every issue.
- Use `gh issue develop` to branch off an issue so the eventual PR auto-links.
