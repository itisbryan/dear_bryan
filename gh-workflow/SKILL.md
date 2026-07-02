---
name: gh-workflow
description: GitHub workflows via the gh CLI — a collection of subskills for issues and pull requests. Loads the right subskill on demand. Use when asked to create or open an issue, ticket, bug report, or decision/spike; open a pull request or submit changes for review; or fetch, view, or pull up an issue by number or URL.
---

# gh-workflow

How I work with GitHub issues and pull requests through the `gh` CLI. This skill is an index — the actual guidance lives in focused subskills under `references/`, each loaded only when its topic comes up, so the base stays lean.

## Prerequisites

GitHub CLI (`gh`) installed and authenticated (`gh auth status`). Run inside the target repository (or pass `--repo owner/repo`).

## How to use

Match the request to the table below and load that subskill's file. Follow its guidance fully — a subskill is part of this skill, not a separate one, so don't announce a skill switch, just apply it.

## Subskills

| Trigger | Load | What it covers |
|---|---|---|
| Create/open an issue, ticket, bug report, or decision/spike ticket | [`references/create-issues.md`](./references/create-issues.md) | Structured issues with a Context / Problem / What to decide-do / References template, then `gh issue create`. |
| Open a pull request, submit changes for review | [`references/create-pr.md`](./references/create-pr.md) | Align with main, review the diff for out-of-scope concerns, draft a Summary/Changes/Testing/Notes PR, file follow-up issues. |
| Fetch, view, or pull up an issue by number or URL | [`references/fetch-issue.md`](./references/fetch-issue.md) | `gh issue view` with the right flags; scannable summary; resolve `#N` references. |

## Rules for loading subskills

- Load only when its trigger fires — progressive disclosure keeps the base skill small.
- The subskills compose: `create-pr` files follow-ups with `create-issues`; `create-issues` can pull an existing ticket with `fetch-issue`. When a flow spans two, load both.
