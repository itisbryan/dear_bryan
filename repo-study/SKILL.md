---
name: repo-study
description: Study a specific public repo to learn how they solve a problem — clone it, trace the real mechanism through code and tests, mine git history for the why, and write an evidence-cited learnings doc with what to adopt. Use when asked to "learn from repo X", "how does project Y do Z", "study this codebase", "what can we steal from", or "write a doc on how they built" something.
---

# repo-study

Learn from a public repo the way a senior engineer would: read the actual code, not the marketing. The output is a learnings doc that explains how they do it, why it works, what it costs, and what's worth adopting here.

## What you need

- **A target repo** — a GitHub URL or `owner/name`.
- **A learning question** — either specific ("how does Litestream replicate the WAL?") or general ("what makes this codebase good / how is it architected?").
- Optionally an **output path** (default `./learnings/<repo>-<topic>.md`).

If the question is missing, ask once: "What do you want to learn from it?" A vague answer is fine — "everything interesting" means run the general study.

## The one rule

**Every claim cites code.** A learning backed only by the README is a rumor. Each technique in the doc points at real evidence: `path/file.ext:line` in the clone, or a GitHub permalink. If you can't find it in the code, don't put it in the doc — or mark it explicitly as unverified.

## The five phases

### 1. Frame & fetch

Restate the learning question in one sentence. Then get the code:

```bash
git clone --depth 50 <url> <scratch-dir>/<name>   # 50 commits: enough history to mine, cheap to fetch
```

**Never clone inside the user's project.** The clone is study material — put it in the system temp/scratch dir so it can't bloat the project, pollute `git status`, or get accidentally committed as a nested repo. Only the learnings doc lands in the project. For huge repos where history mining matters, `git clone --filter=blob:none <url>` beats `--depth`: full history for `log`/`blame`, file contents fetched only on checkout.

### 2. Orient

Build a map before diving:
- README, `docs/`, `ARCHITECTURE.md`, ADRs, CONTRIBUTING — what *they say* they do.
- Directory layout, entry points, the dependency manifest — what they chose to build vs. buy.
- Rough scale: `git ls-files | wc -l`, dominant language, test-to-code ratio.

Note where the docs and the layout disagree — those gaps are usually where the interesting engineering lives.

### 3. Trace

This is the core. Follow the mechanism the question asks about **end to end through real code**:

- Find the entry point and walk the call path; keep a breadcrumb list of `file:line` hops.
- **Read the tests for it** — tests are the honest spec: edge cases, invariants, and failure modes the README never mentions.
- **Mine the history for the why** — blame the tricky lines, pickaxe for when the mechanism appeared, and follow commits to their PR discussions. The exact commands (including the shallow-clone gotcha and finding reverted approaches) are in [`references/mining-history.md`](./references/mining-history.md). A workaround with a 2019 issue link is a lesson; the same code without context is noise.
- For a general "what makes this repo good" study, load [`references/general-study.md`](./references/general-study.md) — pick the 3–4 dimensions where the repo is reputed strong and trace those, instead of one mechanism.

Big repo? Fan out subagents per subsystem and keep only their cited findings.

### 4. Extract

Turn the trace into transferable lessons. For each technique, answer:
- **What** they do (the mechanism, cited).
- **Why** it works — what constraint or failure it's designed around.
- **The advantage** — what it buys them over the obvious alternative.
- **The cost** — every technique has one; a doc with no tradeoffs wasn't studied hard enough. Cross-check against the repo's most-upvoted issues (the "popular issues" pass in [`references/mining-history.md`](./references/mining-history.md)) — recurring user pain is the cost measured in the field.
- **Portability** — does it depend on their scale/stack, or does it transfer to ours?

Drop anything generic ("they write tests") — keep only what's non-obvious and actionable.

### 5. Write the doc

Fill [`templates/learnings.md`](./templates/learnings.md) and save to the output path. Lead with the TL;DR, keep the mechanism walkthrough honest to the code, and end with a concrete "what to adopt here" list ranked by effort-to-payoff. Clean up the clone when done, or tell the user where it lives if they'll want to dig further.

**Quality bar — before shipping, the doc must pass all four:**
1. Would a senior engineer learn something *non-obvious* from it?
2. Does every claim cite `file:line`, a permalink, or a PR/issue?
3. Is there at least one honest tradeoff per technique?
4. Could someone start the top "adopt" item tomorrow without re-reading the repo?

Any "no" → go back to Trace, don't pad the prose.

## Notes

- **Code over docs, tests over code comments, history over both** — when sources disagree, trust them in that order.
- Pin claims to the commit you studied (record the SHA in the doc) — repos move.
- If the user has a repo of their own in mind ("could we do this in X?"), skim that codebase too so the "adopt" section names real files, not hypotheticals.

## When NOT to use this skill

- You need API usage examples, not engineering lessons → just read the docs/fetch them.
- The user wants the repo's feature *implemented* in their project → study briefly, then switch to normal implementation (brainstorm/tdd).
- Comparing many repos to pick one → that's evaluation research (deep-research), not a single-target study.
