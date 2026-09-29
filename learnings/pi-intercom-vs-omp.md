# pi-intercom versus current OMP

## Conclusion

Current OMP has a comparable coordination workflow for one top-level session and its task subagents, but it is not a drop-in equivalent of `pi-intercom` for arbitrary independent top-level OMP sessions.

- Use `task` to spawn planner/worker/reviewer subagents.
- Use `hub` to list, message, await replies from, inspect, steer, revive, and stop those subagents.
- Use Agent Hub (`Alt+A`) for the human-facing roster and transcript/steering UI.
- Use `/collab` and `/join` when the requirement is remote or cross-terminal access to one shared OMP session.
- There is no documented OMP local broker/overlay that discovers unrelated top-level `omp` processes and gives them Pi-intercom-style private `send`/`ask`/`reply` messaging.

## What pi-intercom provides

The official `nicobailon/pi-intercom` README describes an extension for direct 1:1 messaging between Pi sessions on the same machine. Each enabled Pi session registers with a local broker and exposes:

- `intercom({ action: "list" })` / `status` for session discovery and presence;
- `send` for fire-and-forget delivery;
- `ask` for a blocking request whose reply is returned as the tool result;
- `reply`, `pending`, and `cancel` for response and message lifecycle handling;
- `/intercom` and `Alt+M` for a session picker/compose overlay;
- optional attachments and configurable inbound triggering/reply hints.

The extension auto-starts or connects to its local broker after installation. Its documented setup is `pi install npm:pi-intercom`, restart Pi, and optionally add the project `AGENTS.md` guidance snippet. The default config is `~/.pi/agent/intercom/config.json`; no config file is required for the default behavior. Its scope is same-machine and only Pi sessions with the extension loaded and successfully registered appear.

Sources:

- [pi-intercom README — overview and install](https://github.com/nicobailon/pi-intercom#readme)
- [pi-intercom README — tool reference and config](https://github.com/nicobailon/pi-intercom#tool-reference)
- [pi-intercom source — extension entry point](https://github.com/nicobailon/pi-intercom/blob/main/index.ts)
- [pi-intercom source — broker client](https://github.com/nicobailon/pi-intercom/blob/main/broker/client.ts)
- [pi-intercom source — broker lifecycle](https://github.com/nicobailon/pi-intercom/blob/main/broker/spawn.ts)

## What current OMP provides

### 1. Task subagents plus `hub` messaging

OMP's `task` tool creates child agent sessions. With `async.enabled=true`, ordinary non-blocking tasks become background jobs and deliver their completion into the parent. The task workflow supports a batch `{ context, tasks[] }` shape, named agents, isolated worktrees when requested, and lifecycle states including running, idle, parked, and aborted.

The `hub` tool is the agent-facing coordination surface. Its messaging operations are:

```json
{"op":"list"}
{"op":"send","to":"worker","message":"..."}
{"op":"send","to":"worker","message":"...","await":true}
{"op":"inbox"}
{"op":"wait","from":"worker"}
```

`send` is non-blocking. `await: true` waits for the next message from that peer, which is the closest OMP analogue to Pi-intercom's blocking `ask`. A running recipient receives a non-interrupting incoming message; a parked recipient can be revived by direct send. `hub` also covers job polling/cancellation and long-running-process supervision.

The peer model is session-scoped: the documented implementation uses a process-global `IrcBus` and `AgentRegistry`, and `isIrcEnabled` is derived from subagent/task availability. The documented top-level flow is therefore parent ↔ task-agent coordination, not a machine-wide directory of unrelated OMP top-level processes.

Sources:

- [OMP `hub` tool documentation](https://github.com/can1357/oh-my-pi/blob/main/docs/tools/hub.md)
- [OMP Agent Hub documentation](https://github.com/can1357/oh-my-pi/blob/main/docs/agent-hub.md)
- [OMP task-agent discovery and lifecycle](https://github.com/can1357/oh-my-pi/blob/main/docs/task-agent-discovery.md)
- [OMP hub messaging implementation](https://github.com/can1357/oh-my-pi/blob/main/packages/coding-agent/src/tools/hub/messaging.ts)

### 2. Agent Hub UI

Agent Hub is the human-facing view for OMP's subagent workflow. `Alt+A` opens it by default. It shows the roster, status, parent/child lineage, activity, usage, transcripts, and unread messages. From there the operator can focus/steer an agent, revive parked agents, or kill running agents.

This covers the supervision part of the Pi-intercom workflow, but it supervises OMP task agents attached to the current session; it is not a standalone peer-session inbox.

Source: [OMP Agent Hub documentation](https://github.com/can1357/oh-my-pi/blob/main/docs/agent-hub.md)

### 3. `/collab` for independent OMP instances

OMP's cross-instance feature is `/collab`, with `/join <link>` for a guest. A full-control link lets a guest prompt/interrupt the host and use the host's Agent Hub; a view-only link permits observation. The protocol is host-authoritative and encrypted through a relay. Guests do not become direct peers: they join a replica/control view of the host session.

Therefore `/collab` is useful for sharing or remotely operating one session, not for Pi-intercom-style private 1:1 messages between two independent OMP sessions. It normally uses the hosted relay (`wss://my.omp.sh`) and needs no extra local broker configuration. The OMP docs state that the production relay is hosted rather than distributed for self-hosting; a local relay exists for protocol development.

Source: [OMP live session sharing](https://github.com/can1357/oh-my-pi/blob/main/docs/collab.md)

### 4. Visible pane orchestration in this repository

This repository also contains `herdr-orchestration/SKILL.md`. It describes a visible planner/worker/reviewer workflow across Herdr panes, with optional worktrees and explicit lifecycle waits. It requires `HERDR_ENV=1` and is a separate, user-visible orchestration layer; it is not an OMP-native intercom broker.

Source: `herdr-orchestration/SKILL.md`, especially “Before anything else”, “Pick the smallest primitive that works”, and “Fan-out”.

## Capability comparison

| Capability | pi-intercom | OMP today |
|---|---|---|
| Discover unrelated active sessions | Yes, broker-backed Pi session list | Not documented for arbitrary top-level OMP processes; `hub` lists addressable agents in the current OMP session/roster |
| Direct 1:1 notification | `send` | `hub` `send` to a task-agent peer |
| Blocking question/reply | `ask` + `reply` | `hub` `send` with `await:true`, or `hub` `wait` for a peer message |
| Incoming message trigger | Configurable (`always`, `replies`, `never`) | Running task recipients get incoming messages; parked task agents can be revived by direct send |
| Human picker overlay | `/intercom`, `Alt+M` | Agent Hub `Alt+A` for current-session subagents; no equivalent arbitrary-session intercom picker found |
| Session supervision | Presence/status plus messages | Agent Hub roster, transcripts, steering, revive, kill |
| Cross-terminal/cross-machine use | Same-machine broker only | `/collab` supports shared-session guests through a relay; not peer-to-peer messaging |
| Attachments | Protocol supports file/snippet/context attachments | Use shared `local://` artifacts, `agent://` outputs, or normal tool paths; no Pi-intercom attachment API documented |
| Durable message log | Messages stored in Pi session history | OMP messages are injected into OMP session/subagent history; `hub` is not a separate durable intercom inbox |

## Configuration decision for this workstation

No additional configuration is required for the normal OMP subagent workflow. The active OMP configuration was checked with `omp config get`:

- `async.enabled = true` — background task execution is enabled.
- `task.batch = true` — the `{ context, tasks[] }` multi-agent form is enabled.
- `irc.timeoutMs = 120000` — `hub` message waits and `await:true` default to a 120-second timeout.
- `task.maxConcurrency = 32` — up to 32 concurrent subagents, subject to other runtime limits.
- `task.maxRecursionDepth = 2` — subagents may spawn within the configured depth limit.
- `launch.enabled = true` — shared long-running process supervision is enabled.

The global config contains a `modelRoles.task` mapping, so task agents have an explicit model role. No `irc` block is present in `~/.omp/agent/config.yml`; that is fine because the documented default timeout is active. The current repository does not need an intercom extension install.

Optional configuration only:

```yaml
# ~/.omp/agent/config.yml
irc:
  timeoutMs: 300000  # if five-minute blocking peer waits are preferred
```

Only change this if long waits are a real workflow requirement. Do not set it to `0` casually: that disables the timeout and can leave an `await:true` call waiting indefinitely.

## Recommended operating pattern

For the closest OMP equivalent to a Pi planner/worker/reviewer setup:

1. Ask the main OMP session to spawn named `planner`, `worker`, and `reviewer` task agents, preferably with a shared `context` and explicit file ownership.
2. Open Agent Hub with `Alt+A` to watch progress and inspect transcripts.
3. Let workers send completion or escalation messages through `hub`; use `await:true` when the sender cannot proceed without the response.
4. Reuse an idle/parked agent through `hub` rather than spawning a fresh follow-up; direct messaging can revive parked agents.
5. Use isolated worktrees for concurrent edits to overlapping files.
6. Use `/collab` only when another person or terminal needs access to the same host session.
7. Use Herdr only when visible panes/worktrees are specifically desired and `HERDR_ENV=1` is available.

## Bottom line

If the desired workflow is “one OMP session delegates to workers and receives questions/results,” OMP already supports it and this workstation is configured for it. If the desired workflow is “open two independent OMP sessions and privately message either one by name from the other,” current OMP does not expose the same documented local-broker intercom feature; use a parent task-agent tree, `/collab` for shared-session access, or Herdr for visible pane orchestration depending on the goal.

## Accessed sources

- `https://github.com/nicobailon/pi-intercom` and its README/source files
- `https://github.com/can1357/oh-my-pi` OMP documentation and source files
- Local OMP documentation: `omp://tools/hub.md`, `omp://agent-hub.md`, `omp://collab.md`, `omp://task-agent-discovery.md`
- Local repository files: `README.md`, `herdr-orchestration/SKILL.md`, `~/.omp/agent/config.yml`
