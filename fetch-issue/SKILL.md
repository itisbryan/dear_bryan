---
name: fetch-issue
description: Fetches a GitHub issue by its number or URL and displays its full details — title, body, state, labels, assignees, and comments. Use when asked to fetch, view, or pull up an issue by ID, number, or link.
---

# Fetch Issue

Retrieves a GitHub issue by its number or URL and displays its complete contents for review.

## Usage

### By issue number

```bash
gh issue view <number>
```

### By issue URL

```bash
gh issue view <url>
```

## Output formats

### Default (rich terminal output)

Shows title, state, labels, assignees, and body:

```bash
gh issue view <id>
```

### JSON (machine-readable, includes comments)

Retrieves structured data with title, body, state, labels, assignees, milestone, and all comments:

```bash
gh issue view <id> --json title,body,state,labels,assignees,milestone,comments,url,createdAt,updatedAt
```

Useful for extracting specific fields:

```bash
gh issue view <id> --json body --jq ".body"
```

### Display comments inline

```bash
gh issue view <id> --comments
```

### Open in browser

```bash
gh issue view <id> --web
```

## Workflow

### 1. Determine the issue ID

If the user passes a number (e.g. `#42` or `42`), use it directly. If a URL, pass it as-is to `gh issue view`.

If no ID is given, ask the user which issue to fetch.

### 2. Fetch the issue

```bash
gh issue view <id> --json title,body,state,labels,assignees,milestone,comments,url,createdAt,updatedAt
```

This returns everything needed to understand the issue context.

### 3. Summarize for the user

Present the key details in a scannable format:

```markdown
## #<number>: <title>

**State**: <open/closed> | **Labels**: <label1, label2> | **Assignee**: <username>
**Created**: <date> | **Updated**: <date>
**URL**: <link>

### Body

<issue body>

### Comments (<count>)

<comment 1>
<comment 2>
...
```

### 4. If the issue body references other issues or PRs

Resolve `#<number>` references in the body and offer to fetch those too if they seem relevant.

## Common flags

| Flag | Purpose |
|------|---------|
| `--json <fields>` | Output specific fields as JSON |
| `--jq <expression>` | Filter JSON output with a jq expression |
| `--comments` | Show comments inline with the issue |
| `--web` | Open the issue in the browser |

## Troubleshooting

### "could not find issue"

The issue number may not exist in the current repo. Verify the number and that you're in the correct repository directory.

### No comments returned

Comments are only included when using `--json` with `comments` in the field list, or `--comments` for inline display. The default `gh issue view` without flags does not show comments.

### Issue is from a different repo

Pass the full URL or use `--repo owner/repo`:

```bash
gh issue view <id> --repo owner/repo --json title,body,state
```
