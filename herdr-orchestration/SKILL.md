---
name: herdr-orchestration
description: >-
  Orchestrate parallel work across Herdr panes, worktrees, and coding agents:
  fan out N agents, wait on their lifecycle states, collect results, and clean
  up. Use when the user asks to run agents in parallel, delegate to another
  pane, review something in a side pane, split work across worktrees, or
  supervise/monitor agents inside Herdr — or when they say "herdr", "in a pane",
  "worktree per agent", "fan out", "run these side by side". Requires
  HERDR_ENV=1. Do NOT use when an invisible subagent would do — this is for work
  the user wants to SEE and steer in real panes. Do NOT use for ordinary shell
  commands you can just run yourself, and do NOT use for Orca terminals or
  handoffs (use orca-cli / orchestration).
---

# Herdr orchestration

Herdr is a terminal workspace manager that recognizes coding agents running inside its
panes. This skill is the **coordination layer**: how to fan work out to several agents,
block on their real lifecycle states, harvest what they produced, and leave the user's
layout the way you found it.

It is deliberately **not** a command reference. The `herdr` binary ships its own
version-matched guide, and duplicating it here would rot the first time Herdr updates. The
few flags named below are the load-bearing ones for coordination; confirm them — and find
everything else — in `herdr --skill`.

## Before anything else

```bash
test "${HERDR_ENV:-}" = 1 || { echo "not inside Herdr"; exit 1; }
herdr --skill
```

`herdr --skill` prints the authoritative, version-matched command guide for the exact
binary that will run your commands — IDs, lifecycle states, flags, read sources, safety
rules. **Read it before running any control command.** Everything below assumes it.

If the `HERDR_ENV` check fails, say you are not inside Herdr and stop. Never drive a Herdr
session from outside it.

Then orient with the discovery commands from that guide (`workspace list`, `pane list`,
`agent list`, `pane current --current`) and take every ID out of the JSON. Never guess an
ID, never read one off the sidebar, never carry a pane ID across a `pane move`.

## Pick the smallest primitive that works

| You want | Use |
|:---|:---|
| a command's output, right now | just run it yourself — no pane, no agent |
| a long/noisy build, test run, or server | `pane split` + `pane run` + `pane wait-output` |
| judgment, code changes, a review | `pane split` + `agent start` + `agent prompt --wait` |
| several of those at once | fan-out, below |
| the same files edited concurrently | one worktree per agent, below |

Do not reach for an agent when a pane will do, and do not reach for a pane when you can
answer the question yourself. Every extra pane is layout the user has to look at.

Budget for startup friction: a fresh agent usually opens with a trust, hooks, or
onboarding dialog before it will take any prompt. `agent start` may return
`agent_not_ready`, and the first `agent prompt` may come back `agent_blocked`. That is
normal — read the pane, surface it, continue. It is not a failure.

## Fan-out

The whole pattern, in order:

1. **Split one pane per worker.** Wide pane → `--direction right`, narrow or tall →
   `--direction down`. Always `--cwd "$PWD"` and `--no-focus` so the user keeps their
   place. Never split the same direction repeatedly into unusable slivers. Three panes
   per tab is the practical ceiling; beyond that use `tab create` and split inside the new
   tab, reading its root pane ID from the response.
2. **Start each agent with a role name**, not `worker1`: `reviewer`, `tests`, `docs`.
   The name is your handle for every later command and it must be unique among live agents.
3. **Prompt every agent without waiting**, so they run concurrently.
4. **Then wait on each one** with `herdr agent wait <name> --timeout <ms>`. This is the
   barrier, and it is why step 3 omits `--wait`: prompting with `--wait` blocks on worker
   one before worker two has even started, serializing the whole fan-out. That is the
   single most common mistake here.
5. **Read each result**, then clean up.

Each prompt must be self-contained: the worker does not see your conversation. State the
repo path, the branch or diff under review, what "done" looks like, and the shape of the
answer you want back.

## One worktree per agent

Only when workers **write** to the same repo. Read-only reviewers share a cwd fine.

`herdr worktree create --branch <name> --base <ref> --cwd <repo>` gives you an isolated
checkout as its own workspace; start the agent in a pane there. One branch per worker, and
tell each worker in its prompt which branch it owns so it does not wander.

You created those worktrees, so you clean them up — but only after the work is merged or
explicitly abandoned, and say what you are removing before you do it.

## Waiting is not optional

`--wait` settles on `idle`, `done`, or `blocked`. Treat each differently:

- **`idle` / `done`** — the turn settled. Read the output; settling is not succeeding.
- **`blocked`** — the agent hit an approval or question dialog. It is *waiting on a human
  decision*. Read the pane, surface the question to the user, and let them answer. Do not
  auto-approve, do not blind-send `esc` to make it go away.
- **`unknown`** — Herdr sees an agent but cannot classify it. It is **not** a settled
  state and `--wait` will not stop on it; if `agent get` shows it, read the pane before
  concluding anything.
- **timeout** — report it as a timeout. Never report an unfinished worker as done.

Give every wait an explicit `--timeout`. An unbounded wait on a stuck agent hangs you too.

**A settled state is not a finished job.** Read the output before you believe it. Two
failures look exactly like success from the state alone:

- The agent settled `idle` while parked on a dialog Herdr did not classify as `blocked` —
  your prompt was swallowed and never ran.
- The agent settled `done` on an error: usage limit, auth expiry, crashed tool.

So after every wait, confirm the transcript actually answers what you asked. If it does
not, read the pane, deal with what is really on screen, and re-prompt.

## Collecting results

Read with the transcript-friendly source (`recent-unwrapped`) and enough `--lines`. If
raising `--lines` stops revealing more, the agent is on the terminal's alternate screen and
that scrollback is gone — ask it to write its full answer to a temp Markdown file and reply
with only the path, then read the file. Use that as a **fallback**, not as the opening
prompt; most tasks never need it.

Synthesize for the user: what each worker concluded, where they disagree, what is still
open. Do not paste raw pane dumps into the conversation.

## Leave the place tidy

- Close only panes, tabs, workspaces, and worktrees **you** created — and say so first.
- Never `herdr server stop` from a live session. Never kill the main Herdr process.
- Keep the user's focus where it was: `--no-focus` unless they asked to be moved.
- If you leave workers running on purpose, tell the user their names and where they live.
- **If the fan-out aborts** — a worker dies, the user interrupts, you hit an error you
  cannot resolve — clean up anyway. Re-read `agent list` and `pane list` to see what
  actually survived, close the panes you created, report which workers produced nothing.
  Stranded panes and half-built worktrees are the mess this section exists to prevent.

## When this is the wrong tool

- One quick task → do it yourself. Orchestration overhead beats the work.
- Orca terminals, handoffs, "give this to another agent" → `orca-cli` / `orchestration`.
- Desktop or browser UI outside a Herdr pane → `computer-use`.
