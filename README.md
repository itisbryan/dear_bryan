# dear_bryan

A small collection of agent skills: teach instead of do, ship with structure, keep the thread.

Built to the [Agent Skills](https://agentskills.io/specification) standard — works in pi, Claude Code, Codex, and any compliant harness.

## Skills

| Skill | What it does | Trigger |
|:---|:---|:---|
| [tutor](./tutor) | Coaches you to write your own code — never writes the solution for you | `tutor` |
| [create-issues](./create-issues) | Structured tickets: Context / Problem / What to decide / References | `create-issues` |
| [create-pr](./create-pr) | Review-aware PRs; aligns with main and files follow-up issues | `create-pr` |
| [fetch-issue](./fetch-issue) | Pulls up any GitHub issue by number or URL | `fetch-issue` |
| [design-taste](./design-taste) | Designs intentional UI and refuses templated/AI-default "slop" | `design-taste` |
| [qa-sweep](./qa-sweep) | Exploratory QA of a running web app → severity-ranked report, files issues | `qa-sweep` |
| [debug](./debug) | Root-cause debugging: reproduce with a red/green loop, fix the cause not the symptom | `debug` |
| [tdd](./tdd) | Test-driven development: red → green → refactor, tests before code | `tdd` |
| [rails-work](./rails-work) | How I work with Rails — a growing subskill collection (starts with `use-rv`) | `rails-work` |

## Install

```bash
# all nine into your global skills dir
npx skills install tutor create-issues create-pr fetch-issue design-taste qa-sweep debug tdd rails-work

# or clone and point your harness at this directory
git clone https://github.com/your-org/dear_bryan.git
```

In pi, skills in `.agents/skills/` are auto-discovered and register as `/skill:<name>`:

```bash
ln -s ~/dear_bryan/tutor .agents/skills/tutor
/skill:tutor "recursion in Go"
```

## tutor

One rule that overrides everything: **never write the learner's actual solution for them.** Hints, skeletons, pseudocode, toy examples — yes. The answer — no.

Three modes it names as it switches between them:

- **Socratic** — one guiding question at a time, when you're engaged and progressing.
- **Explain** — analogy plus a toy example, when you're stuck or missing a concept.
- **Pair-program** — thinking partner while you drive, for complex multi-file design.

Three subskills load on demand:

- [`practice-problems.md`](./tutor/references/practice-problems.md) — toy-domain exercises, one at a time.
- [`debugging.md`](./tutor/references/debugging.md) — coaches the debugging meta-skill, not the answer.
- [`recap.md`](./tutor/references/recap.md) — end-of-session recap the learner keeps.

If you explicitly opt out of tutoring, it says so and ships.

## create-issues

Every issue follows one structure so a newcomer can act on it without reading chat history:

```
## Context            the landscape
## Problem            what's wrong / missing
## What to decide/do  options + acceptance criteria
## References         files, links, lines
```

Workflow: gather context → explore the codebase for concrete refs → draft → review → publish via `gh issue create`.

## create-pr

Aligns the branch with latest main, then scans the diff for out-of-scope concerns and files them as separate issues via **create-issues** — keeping the PR focused while capturing the debt.

Body template: `## Summary` → `## Changes` → `## Testing` → `## Notes` (with follow-up issue links).

## fetch-issue

```bash
gh issue view <id> --json title,body,state,labels,assignees,milestone,comments,url,createdAt,updatedAt
```

Presents a scannable summary, resolves `#<number>` references, and offers to fetch related issues. Good for grounding a `tutor` lesson or loading context before `create-pr`.

## Layout

```
dear_bryan/
├── tutor/
│   ├── SKILL.md
│   └── references/{practice-problems,debugging,recap}.md
├── create-issues/SKILL.md
├── create-pr/SKILL.md
├── fetch-issue/SKILL.md
├── design-taste/
│   ├── SKILL.md
│   └── references/{catalog,mirror,clone,direction,typography,color,layout,polish,anti-slop}.md
├── qa-sweep/
│   ├── SKILL.md
│   ├── references/severity.md
│   └── templates/report.md
├── debug/SKILL.md
├── tdd/SKILL.md
└── rails-work/
    ├── SKILL.md              ← router: how I work with Rails
    └── references/use-rv.md  ← Ruby versions & gems with rv (not rbenv)
```

Each skill is a single `SKILL.md` (plus optional `references/`) — portable across every compliant harness.
