# Session handoff (subskill of tutor)

Load this reference (via `read`) when the learner wants to **pause and resume later** rather than end — they're stopping mid-thread and a fresh session (with no memory of this one) will continue. The handoff is a resume file for the *next tutor*, not a memory aid for the learner. It exists so the next session picks up at the exact next step instead of re-diagnosing from scratch.

## When to load

- The learner says "let's pause", "pick this up next time", "call it a day but I'm not done", or "hand this off".
- You're stopping mid-goal — the learning goal is not yet achieved and you want to resume it later.

Contrast with `recap.md`: recap is for a *finished* goal and is written for the learner to re-read. A handoff is for an *unfinished* goal and is written for the next tutor to act on. If the goal is done, use recap instead.

## What the handoff must contain

Produce a single markdown block, written so a tutor who has never seen this session can resume in one read. Use exactly these sections:

```
## Session handoff — <date or topic>

### Goal (in progress)
<the one-sentence learning goal; note it is not yet achieved>

### Where we stopped
<the exact next micro-step — what the learner was about to attempt or answer when we paused>

### Mode to resume in
<Socratic / explain / pair — and one line on why, so the next tutor doesn't reset to default and lose momentum>

### Covered so far
- <concept> — landed / partially landed, in the learner's own framing

### Still shaky / open question
- <the concept or misconception to watch for>
- <any question the learner was mid-answering — quote their last attempt so the next tutor responds to their reasoning, not a cold start>

### Tried and rejected
- <approaches or hints already given that didn't work — so the next tutor doesn't repeat them>

### First move next session
<the single opening question or step the next tutor should lead with>
```

## Rules

- **Actionable, not descriptive.** Every section serves the next tutor's first five minutes. If a line doesn't change what the next session does, cut it.
- **Preserve the learner's exact words and last attempt.** The next tutor must respond to their reasoning, not restart. This is the difference between resuming and re-teaching.
- **Name what's been tried.** "Tried and rejected" is what stops the next session from re-walking a dead end — the most valuable part of a handoff.
- **Honest, not flattering.** Same as recap: mark partial understanding as partial. Inflated state makes the next session start too far ahead.
- **No solution code.** A handoff carries state and direction, never the answer. Same guardrail as the one rule.
- **Concise.** A handoff longer than the remaining work is a failure mode. One line per bullet.

## Delivery

Write the handoff block to `HANDOFF-<topic-slug>.md` in this skill's own directory (same folder as `SKILL.md`) — e.g. `HANDOFF-recursion.md`, `HANDOFF-sql-joins.md`. The per-topic name keeps two paused threads from overwriting each other. Overwrite an existing file only if it's the same topic. This is how the next session finds it: session-flow step 0 scans for `HANDOFF-*.md` on startup and resumes from the matching one. Also show the block to the learner so they can see the state, and tell them plainly: "Saved — next session I'll read this and pick up exactly here. You don't need to paste anything." Then stop; do not keep tutoring.

(Manual fallback: if for any reason you can't write the file, show the block and tell the learner to paste it into the start of their next session.)
