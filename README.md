<div align="center">

# 🌟 dear_bryan

### A thoughtfully crafted collection of agent skills — for tutoring, shipping, and keeping the thread.

[![Skills](https://img.shields.io/badge/skills-4-8A2BE2?style=for-the-badge)](./)
[![Agent Skills](https://img.shields.io/badge/Agent%20Skills-standard-FF6B35?style=for-the-badge)](https://agentskills.io/specification)
[![License](https://img.shields.io/badge/license-MIT-22C55E?style=for-the-badge)](./)
[![Pi](https://img.shields.io/badge/works%20with-pi-6366F1?style=for-the-badge)](https://github.com/earendil-works/pi-coding-agent)

*Teach me to fish 🎣 · Ship with structure 📦 · Never lose context 🧵*

</div>

---

## ✨ What's inside

Four skills that make an AI coding agent **teach instead of do**, **ship with structure**, and **keep the thread** across a session.

```mermaid
flowchart LR
    subgraph Family["dear_bryan skill family"]
        direction TB
        T["🎓 tutor<br/>learn, don't just do"]
        CI["📝 create-issues<br/>structured tickets"]
        CP["🔀 create-pr<br/>reviewed pull requests"]
        FI["🔍 fetch-issue<br/>pull up any issue"]
    end

    T -.->|"discovers out-of-scope"| CI
    CP -.->|"files follow-ups"| CI
    CP -.->|"reviews context"| FI
    FI -.->|"feeds a lesson"| T

    style T fill:#EEF2FF,stroke:#6366F1,stroke-width:2px,color:#1E1B4B
    style CI fill:#FFF7ED,stroke:#F97316,stroke-width:2px,color:#431407
    style CP fill:#FDF4FF,stroke:#A855F7,stroke-width:2px,color:#3B0764
    style FI fill:#ECFEFF,stroke:#06B6D4,stroke-width:2px,color:#083344
```

| Skill | One-liner | Trigger |
|:---|:---|:---|
| **[tutor](./tutor)** | A hybrid adaptive coding tutor — coaches you to write your own code, never writes it for you | `tutor` |
| **[create-issues](./create-issues)** | Structured tickets with Context / Problem / What to decide / References | `create-issues` |
| **[create-pr](./create-pr)** | Review-aware PRs that align with main and file follow-up issues for out-of-scope concerns | `create-pr` |
| **[fetch-issue](./fetch-issue)** | Pull up any GitHub issue by number or URL with full details and comments | `fetch-issue` |

---

## 🚀 Install

These follow the [Agent Skills](https://agentskills.io/specification) standard, so they work in **pi**, **Claude Code**, **OpenAI Codex**, and any compliant harness.

```bash
# install all four into your global skills dir
npx skills install tutor create-issues create-pr fetch-issue

# or clone and link locally
git clone https://github.com/your-org/dear_bryan.git
# then point your harness at this directory
```

<details>
<summary><b>🧪 In pi specifically</b></summary>

```bash
# per-project: skills in .agents/skills/ are auto-discovered
ln -s ~/dear_bryan/tutor .agents/skills/tutor

# or load on demand
pi --skill ~/dear_bryan/tutor

# invoke once installed
/skill:tutor "recursion in Go"
```

Skills register as `/skill:<name>` commands in pi. The `description` in each `SKILL.md` frontmatter is what the agent matches against — only that snippet lives in context until the skill is invoked (progressive disclosure).

</details>

---

## 🧑‍🎓 tutor — the centerpiece

A personal coding tutor with **one rule that overrides everything** and **three modes** it adapts between.

> **The one rule:** *Never write the learner's actual solution for them.*
> Toy examples, skeletons, pseudocode, one-line hints — yes. The answer — never.

```mermaid
flowchart TD
    Start(["learner asks for help"]) --> Diagnose["1. Diagnose what they know"]
    Diagnose --> Agree["2. Agree on one learning goal"]
    Agree --> Mode{"pick a mode, name it aloud"}

    Mode -->|"engaged & progressing"| S["🟣 Socratic\nask one guiding question"]
    Mode -->|"stuck / confused / missing concept"| E["🔵 Explain\nanalogy + toy example"]
    Mode -->|"complex, multi-file design"| P["🟢 Pair-program\nthinking partner, you drive"]

    S --> Review["4. Review their code\nfind what's right first"]
    E --> Review
    P --> Review

    Review --> Check["5. Check understanding\nmake them explain it back"]
    Check --> Practice{"concept landed?"}
    Practice -->|yes| Sub["load a subskill on demand"]
    Practice -->|no| Mode
    Sub --> Wrap(["session recap"])

    style S fill:#F3E8FF,stroke:#A855F7,color:#3B0764
    style E fill:#DBEAFE,stroke:#3B82F6,color:#1E3A8A
    style P fill:#D1FAE5,stroke:#10B981,color:#064E3B
    style Start fill:#FEF3C7,stroke:#F59E0B,color:#78350F
    style Wrap fill:#FEF3C7,stroke:#F59E0B,color:#78350F
```

### Three formats for diagrams

The tutor teaches with pictures, not just prose — and picks the lightest format that fits:

| Format | Use it for |
|:---|:---|
| **Mermaid** | Static structure — flowcharts, sequence, state, architecture |
| **ASCII** | Quick terminal sketches, Mermaid fallback |
| **HTML / SVG** | Interactive & animated — step-through algorithms, clickable state machines, growing call stacks |

### Subskills (loaded on demand)

The tutor carries three focused reference docs that load via `read` only when triggered — keeping the base skill lean.

| Trigger | Loads | What it does |
|:---|:---|
| concept just landed, learner wants practice | [`references/practice-problems.md`](./tutor/references/practice-problems.md) | Generate a toy-domain exercise, one at a time, solution withheld |
| stuck on a bug or failing test | [`references/debugging.md`](./tutor/references/debugging.md) | Coach the debugging meta-skill (read error → reproduce → bisect → hypothesize) — not the answer |
| session wrapping up | [`references/recap.md`](./tutor/references/recap.md) | Produce a written recap the learner keeps: goal, takeaways, still-shaky, next steps |

<details>
<summary><b>📖 See the anti-patterns it refuses</b></summary>

The tutor holds the line, politely:

- **"Can you just write it for me?"** → "Writing it for you is the one thing that won't help you learn. Let me ask about the part blocking you instead."
- **"Just fix this function."** → "I'll help you fix it yourself. What input breaks it, and what do you expect?"
- **"Give me the solution."** → Reoffers the modes: a hint, a concept explanation, or a pair walk-through where *you* drive.

If the learner explicitly opts out of tutoring, it respects that — says so plainly, switches off, and ships.

</details>

---

## 📝 create-issues

Every issue follows one exact structure so a newcomer can act on it without reading chat history.

```mermaid
flowchart LR
    C["## Context<br/>the landscape"] --> P["## Problem<br/>what's wrong / missing"]
    P --> W["## What to decide/do<br/>options + acceptance criteria"]
    W --> R["## References<br/>files, links, lines"]

    style C fill:#EEF2FF,stroke:#6366F1,color:#1E1B4B
    style P fill:#FEF2F2,stroke:#EF4444,color:#450A0A
    style W fill:#FFF7ED,stroke:#F97316,color:#431407
    style R fill:#ECFEFF,stroke:#06B6D4,color:#083344
```

**Workflow:** gather context → explore the codebase for concrete refs → draft → review with you → publish via `gh issue create`.

<details>
<summary><b>🧾 Example issue (bug report)</b></summary>

```markdown
## Context
The picking batches list page uses a `TableCardComponent` for bulk actions.
Its delete button always renders when any row is selected.

## Problem
Clicking delete on a print-only workflow triggers a 404 — no `bulk_delete_url`.
Users see a broken button implying functionality that doesn't exist.

Steps to reproduce:
1. Go to picking batches list.
2. Select rows via checkbox.
3. Observe the "Delete selected" button.
4. Click it — 404.

## What to decide/do
- [ ] Add a `bulk_delete_url` prop to `TableCardComponent`.
- [ ] Only render the delete button when the prop is present.
- [ ] Verify existing delete consumers still show it.
- [ ] Add a component test asserting absence when the prop is missing.

## References
- `app/components/table_card_component.rb` — bulk action bar rendering
- Related PR: https://github.com/org/repo/pull/1234
```

</details>

---

## 🔀 create-pr

A PR skill that's **review-aware**: it aligns the branch with latest main, then **scans the diff for out-of-scope concerns and files them as separate issues** so nothing slips through.

```mermaid
flowchart TD
    A["1. Align with latest main<br/>fetch + rebase"] --> B["2. Verify clean state"]
    B --> C["3. Push branch"]
    C --> D{"4. Review the diff"}
    D -->|"in scope"| E["5. Draft PR description"]
    D -->|"out of scope"| F["create follow-up issue 📝"]
    F --> E
    E --> G["6. Summary / Changes / Testing / Notes"]
    G --> H["link follow-up issues in Notes 🔗"]
    H --> I["7. gh pr create"]

    style D fill:#FEF3C7,stroke:#F59E0B,color:#78350F
    style F fill:#FFF7ED,stroke:#F97316,color:#431407
    style I fill:#D1FAE5,stroke:#10B981,color:#064E3B
```

**PR body template:** `## Summary` → `## Changes` (bold-keyed bullets) → `## Testing` → `## Notes` (with out-of-scope issue links).

It deliberately hands off to **create-issues** for anything discovered but not in this PR's scope — keeping the PR focused while capturing the debt.

---

## 🔍 fetch-issue

Pulls a GitHub issue by number or URL and displays its full anatomy.

```bash
gh issue view <id> --json title,body,state,labels,assignees,milestone,comments,url,createdAt,updatedAt
```

Presents a scannable summary, resolves `#<number>` references in the body, and offers to fetch related issues too. Great for grounding a lesson in `tutor` or loading context before `create-pr`.

---

<div align="center">

## 🧬 How they fit together

```mermaid
flowchart TD
    User(["you"])

    User -->|"teach me"| T[tutor]
    User -->|"open a ticket"| CI[create-issues]
    User -->|"ship a PR"| CP[create-pr]
    User -->|"what's issue #42"| FI[fetch-issue]

    CP -->|"out-of-scope concerns"| CI
    CP -->|"review context"| FI
    FI -->|"feed a lesson"| T
    T -->|"wrap up with"| Recap["session recap"]

    style User fill:#FEF3C7,stroke:#F59E0B,stroke-width:2px,color:#78350F
    style T fill:#EEF2FF,stroke:#6366F1,color:#1E1B4B
    style CI fill:#FFF7ED,stroke:#F97316,color:#431407
    style CP fill:#FDF4FF,stroke:#A855F7,color:#3B0764
    style FI fill:#ECFEFF,stroke:#06B6D4,color:#083344
    style Recap fill:#FEF3C7,stroke:#F59E0B,color:#78350F
```

</div>

---

## 📁 Layout

```
dear_bryan/
├── README.md                 ← you are here
├── .gitignore                ← ignores .pi/ runtime state
├── tutor/
│   ├── SKILL.md              ← the core skill (3 modes, diagrams, one rule)
│   └── references/
│       ├── practice-problems.md   ← toy-domain exercises
│       ├── debugging.md           ← debugging meta-skill coaching
│       └── recap.md               ← end-of-session artifact
├── create-issues/
│   └── SKILL.md              ← Context/Problem/Decide/References template
├── create-pr/
│   └── SKILL.md              ← review-aware PRs with follow-up issues
└── fetch-issue/
    └── SKILL.md              ← gh issue view, structured display
```

---

<div align="center">

**Made with care for agents that care back.** 💜

*Each skill is a single `SKILL.md` (plus optional `references/`) following the [Agent Skills](https://agentskills.io/specification) standard — portable across every compliant harness.*

[▲ back to top](#-dear_bryan)

</div>
