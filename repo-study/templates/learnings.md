# Learnings: {repo} — {topic}

> Studied at commit `{sha}` on {date}. Question: {the learning question, one sentence}

## TL;DR

{3–5 sentences: the core insight, the biggest advantage, the main tradeoff, and whether it transfers to us.}

## How they do it

{The mechanism walkthrough, in the order the code executes. Every step cites `file:line` or a permalink.}

1. **{Step}** — {what happens} (`path/file.ext:123`)
2. …

{What the tests revealed that the code alone didn't: edge cases, invariants, failure modes.}

## Why it works — their advantages

| Technique | Advantage over the obvious alternative | Evidence |
|:---|:---|:---|
| {technique} | {what it buys them} | `file:line` / PR #{n} |

{Context from history: the constraint or incident that shaped the design, with issue/PR links.}

## Tradeoffs & costs

- {What this design gives up, complicates, or assumes — every technique has a price.}

## What to adopt here

Ranked by effort-to-payoff:

1. **{Lesson}** — {concrete first step in *our* codebase, naming real files if known}. Effort: {S/M/L}.
2. …

**Not worth adopting:** {techniques that depend on their scale/stack, and why.}

## Open questions

- {What couldn't be verified from the code, or needs a maintainer's answer.}

## Sources

- Repo: {url} @ `{sha}`
- Key files: {list}
- PRs/issues/discussions read: {links}
