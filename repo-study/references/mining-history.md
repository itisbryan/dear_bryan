# Mining history — finding the *why*

Code says what; history says why. These are the exact commands, most-used first.

## From a line of code to its design discussion

```bash
# 1. Who last touched these lines, and in what commit?
git blame -L 120,140 path/file.ext

# 2. Full evolution of a specific range (better than blame for rewritten code)
git log -L 120,140:path/file.ext

# 3. The PR that merged a commit — where the discussion lives
gh api "repos/{owner}/{repo}/commits/{sha}/pulls" --jq '.[].html_url'

# 4. Read that PR's description and review comments
gh pr view <number> --repo owner/name --comments
```

## Finding when/why a mechanism appeared

```bash
# Pickaxe: commits that added/removed this string (function name, config key, magic number)
git log -S "symbolName" --oneline

# Same but regex, e.g. for a renamed concept
git log -G "wal.?replicat" --oneline

# A file's whole life, across renames
git log --follow --oneline -- path/file.ext
```

**Shallow-clone gotcha:** `--depth 50` truncates history, so pickaxe/blame may dead-end at the clone boundary. Deepen on demand:

```bash
git fetch --deepen=500        # usually enough
git fetch --unshallow         # full history, only if needed
```

## Searching the repo's discussions

```bash
# Merged PRs about a topic
gh search prs --repo owner/name "wal replication" --merged --limit 10 \
  --json number,title,url

# Issues, including closed ones (design debates often live in closed issues)
gh search issues --repo owner/name "checkpoint corruption" --limit 10 \
  --json number,title,state,url

# Issues by label — many repos tag design decisions
gh issue list --repo owner/name --label "design" --state all --limit 20

# Release notes: how they framed the change to users
gh release list --repo owner/name --limit 10
gh release view <tag> --repo owner/name
```

GitHub Discussions need GraphQL:

```bash
gh api graphql -f query='
  query { repository(owner: "OWNER", name: "NAME") {
    discussions(first: 10, orderBy: {field: UPDATED_AT, direction: DESC}) {
      nodes { title url category { name } }
    } } }'
```

## Popular issues — the users' verdict on the design

Sorted by reactions, the issue tracker is field data on where the design actually hurts. One bounded pass:

```bash
# Top issues by 👍, open AND closed — pain doesn't expire when an issue closes
gh issue list --repo owner/name --state all --limit 20 \
  --search "sort:reactions-+1-desc" --json number,title,state,url

# Most-discussed issues — long threads mark contested design ground
gh search issues --repo owner/name --sort comments --order desc --limit 15 \
  --json number,title,state,url
```

**Depth rule: skim ~15–20 titles, deep-read at most 5** — only those touching the learning question. This is a cost-evidence pass, not the study itself; if an issue demands a deep dive, that's a new trace target, so note it and move on.

What each pattern teaches:

- **Recurring pain across many issues** → the design's real-world cost. This is your Tradeoffs section, written by their users.
- **Highly-upvoted feature, maintainers said no** → a deliberate scope decision. Read their refusal — the reasoning is often the most quotable lesson in the repo.
- **Bugs clustered in one subsystem** → fragility warning. If the adopt list touches that subsystem, flag it.
- **Popular issue with a long-lived workaround comment** → the gap between the design and what users need; check if it later became a feature (that arc = how the team learns).

## Reading the results

- A **workaround + linked issue** = a lesson about a real failure mode. Quote the issue in the doc.
- A **reverted commit** (`git log --grep="Revert"`) = an approach they tried and abandoned — often the most transferable learning in the repo.
- **Long PR review threads** on the traced code = the tradeoffs section writes itself; the objections reviewers raised are the costs.
