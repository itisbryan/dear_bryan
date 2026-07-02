# create-pr (subskill of gh-workflow)

Load this when the user wants to open a pull request or submit changes for review. Creates a PR with a structured, consistent description, and captures any technical debt or unrelated concerns discovered during review as separate tracked issues so nothing slips through.

## Prerequisites

- GitHub CLI (`gh`) installed and authenticated (`gh auth status`).
- You must be on a feature branch (not main/master) with changes to push.

## Workflow

### 1. Align with latest main

Before creating the PR, ensure the feature branch is up to date with the latest main to avoid merge conflicts and stale reviews.

```bash
git fetch origin main
```

Check if the branch is behind main:

```bash
git merge-base --is-ancestor origin/main HEAD && echo "Up to date" || echo "Behind main — rebase needed"
```

If behind, rebase onto latest main:

```bash
git rebase origin/main
```

Resolve any conflicts, then force-push the rebased branch.

### 2. Verify state

```bash
git status
git branch --show-current
```

Ensure all changes are committed and the branch is clean.

### 3. Push branch

```bash
git push -u origin HEAD
```

### 4. Review the diff for out-of-scope concerns

Before drafting the PR description, review the full diff to surface anything that falls outside the PR's intended scope:

```bash
git diff origin/main..HEAD
```

Look for:
- **Technical debt discovered but not addressed** — e.g. a pattern that should be refactored, a deprecation that needs follow-up, a TODO comment left behind.
- **Side-effects or ripple changes** — e.g. an API change that affects consumers not yet updated, a config value that needs rollout coordination.
- **Unrelated problems noticed in passing** — e.g. a flaky test in the same file, a missing index noticed while reading a query, inconsistent error handling in adjacent code.
- **Incomplete work** — e.g. a feature flag still in place, clean-up steps deferred to a follow-up PR.

For each valid concern **not in this PR's scope**, create a standalone issue using the `create-issues` subskill's template (Context, Problem, What to decide/do, References). Do not bloat the PR with unrelated fixes — capture them as issues instead.

```bash
gh issue create --title "<descriptive title>" --body "<issue body>" --label "<label>"
```

Reference the newly created issue numbers — they will be listed in the PR's Notes section.

### 5. Generate PR description

Use git log to review commits and generate a description:

```bash
git log origin/main..HEAD --oneline
```

### 6. PR description template

Craft the PR body in this exact structure:

```markdown
## Summary

A concise paragraph describing the high-level purpose of this PR. What problem does it solve? What feature or fix does it introduce?

## Changes

- **Bold-titled change item** — description of the change, including implementation details, design decisions, or behavioral differences.
- Each change is a bullet starting with a bold key phrase followed by an em dash and a clear explanation.
- Group related changes together logically.

## Testing

- Test suite results (e.g. `bin/rubocop` clean, all controller tests green).
- New tests added and what they cover.
- Manual testing steps performed.

## Notes

- Deployment considerations, follow-up work, or caveats.
- **Out-of-scope issues created**: link to any issues #123, #456 that were filed for concerns discovered during this PR (use `#<number>` or the full issue URL).
- Anything reviewers should be aware of.
```

### Formatting rules

- Use proper markdown headings (`##`) — never raw `\n` escape sequences.
- Bold key phrases in change bullets: `**Sortable column** — description`
- Each bullet explains both what changed and why.
- Keep the Summary to one paragraph.
- Testing section lists specific test commands run and their results.
- Notes is for deployment concerns, follow-ups, or reviewer guidance. Always include any out-of-scope issues created during the diff review here with their issue numbers.

### Example PR description with out-of-scope issues

```markdown
## Summary

Brings the picking batches list page up to the order_deliveries pattern and adds selection-based printing.

## Changes

- **Sortable "OD count" column** — sorts via a correlated `COUNT` subquery in the sort allowlist (no schema change, no counter to keep in sync).
- **Status segmented tabs** — the status quick-filter is now a `.seg` segmented control with a sliding spring indicator, plus an **urgent** tab (batches holding an urgent OD). Extracted into a reusable `SegmentedTabsComponent` and adopted on the order_deliveries list as well.
- **order_deliveries-style toolbar** — responsive mobile-card / desktop-table swap, with a **row-level loading skeleton** shown while a tab/filter fetch is in flight (the card header and column titles stay solid, only the data rows skeleton).
- **Searchable filters** — assignee and warehouse filters are now SlimSelect dropdowns (type-to-search). Refined the shared SlimSelect theme (softer panel, inset rounded option rows, search icon) and removed its default left accent bar on highlighted options.
- **Preview chips** — assignee and warehouse table cells render hover preview cards (`PreviewChipComponent`), matching order_deliveries.
- **Row highlight** — replaced the left accent bar with a tinted full-row highlight.
- **Bulk print** — row checkboxes plus a "print selected" action that opens one combined picking sheet for every OD across the selected batches. Scoped to the admin's accessible slocs.
- **TableCardComponent** — the built-in bulk-delete button is now conditional on `bulk_delete_url`, so a print-only bulk bar is possible without a stray delete control. Existing delete consumers are unchanged.

## Testing

- `bin/rubocop` clean (all files).
- Picking batches controller, SegmentedTabsComponent, TableCardComponent, and order_deliveries controller tests green.
- New tests cover: OD-count sort, urgent filter, segmented tabs rendering, kanban removal, list-variant body, selection checkboxes, and bulk_print (including sloc scoping).

## Notes

- Selection checkboxes and bulk print are a desktop-table affordance; the mobile card list stays tap-to-open.
- The SlimSelect theme changes are global (shared across the admin), and are a strict visual improvement.
- **Out-of-scope issues created**: #789 — shipments list still uses the old toolbar pattern and should be migrated to match (discovered while aligning the batches toolbar). #790 — the `urgent_od?` check runs a per-row query that should be preloaded (noted during OD-count sort work but deferred to avoid scope creep).
```

### 7. Create the PR

```bash
git log origin/main..HEAD --oneline
gh pr create --base main --title "<title>" --body "<description>"
```

Use `--web` to open in browser if requested:

```bash
gh pr create --base main --title "<title>" --body "<description>" --web
```

### 8. Confirm

Report the PR URL and any out-of-scope issue URLs to the user.
