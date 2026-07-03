# dear_bryan

A small collection of agent skills: teach instead of do, ship with structure, keep the thread.

Built to the [Agent Skills](https://agentskills.io/specification) standard — works in pi, Claude Code, Codex, and any compliant harness.

## Skills

| Skill | What it does | Trigger |
|:---|:---|:---|
| [tutor](./tutor) | Coaches you to write your own code — never writes the solution for you | `tutor` |
| [brainstorm](./brainstorm) | Turns an idea into an approved design doc before any code gets written | `brainstorm` |
| [gh-workflow](./gh-workflow) | GitHub issues & PRs via `gh` — subskill collection (create-issues, create-pr, fetch-issue) | `gh-workflow` |
| [design-taste](./design-taste) | Designs intentional UI and refuses templated/AI-default "slop" | `design-taste` |
| [qa-sweep](./qa-sweep) | Exploratory QA of a running web app → severity-ranked report, files issues | `qa-sweep` |
| [debug](./debug) | Root-cause debugging: reproduce with a red/green loop, fix the cause not the symptom | `debug` |
| [tdd](./tdd) | Test-driven development: red → green → refactor, tests before code | `tdd` |
| [rails-work](./rails-work) | How I work with Rails — a growing subskill collection (starts with `use-rv`) | `rails-work` |
| [repo-study](./repo-study) | Learn from a public repo: trace how they solve it in real code, write an evidence-cited learnings doc | `repo-study` |

## Install

```bash
# install every skill from the repo into your global skills dir
npx skills add itisbryan/dear_bryan -s '*' -g -y

# or clone and point your harness at this directory
git clone https://github.com/itisbryan/dear_bryan.git
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

## gh-workflow

A subskill collection for GitHub via the `gh` CLI. The router loads one on demand:

- **create-issues** — every issue follows one structure so a newcomer can act on it without reading chat history:

  ```
  ## Context            the landscape
  ## Problem            what's wrong / missing
  ## What to decide/do  options + acceptance criteria
  ## References         files, links, lines
  ```

- **create-pr** — aligns the branch with latest main, scans the diff for out-of-scope concerns and files them as follow-up issues (via create-issues), then drafts a `## Summary → ## Changes → ## Testing → ## Notes` PR.
- **fetch-issue** — `gh issue view` with the right flags; a scannable summary that resolves `#<number>` references and offers to pull related issues.

The three compose: create-pr files follow-ups with create-issues; create-issues can pull an existing ticket with fetch-issue.

## Layout

```
dear_bryan/
├── tutor/
│   ├── SKILL.md
│   └── references/{practice-problems,debugging,recap}.md
├── gh-workflow/
│   ├── SKILL.md              ← router: GitHub issues & PRs
│   └── references/{create-issues,create-pr,fetch-issue}.md
├── design-taste/
│   ├── SKILL.md
│   └── references/{catalog,mirror,clone,direction,typography,color,layout,polish,anti-slop}.md
├── qa-sweep/
│   ├── SKILL.md
│   ├── references/severity.md
│   └── templates/report.md
├── repo-study/
│   ├── SKILL.md
│   ├── references/{general-study,mining-history}.md
│   └── templates/learnings.md
├── brainstorm/SKILL.md
├── debug/SKILL.md
├── tdd/SKILL.md
└── rails-work/
    ├── SKILL.md              ← router: how I work with Rails
    └── references/use-rv.md  ← Ruby versions & gems with rv (not rbenv)
```

Each skill is a single `SKILL.md` (plus optional `references/`) — portable across every compliant harness.
