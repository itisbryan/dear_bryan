---
name: parabol
description: Draft and submit my daily standup to Parabol from my git activity, then submit only on my explicit OK. Uses the `paracli` CLI (brew install itisbryan/tap/paracli) for all the work; this skill just drives it interactively. Use when I say "do my standup", "submit my daily report", or "post my Parabol update".
---

# parabol

A thin driver over the [`paracli`](https://github.com/itisbryan/paracli) CLI, which does everything (gather git, draft, submit to Parabol). This skill is only the interactive loop: gather → **I draft** → you approve → submit. It never posts without your explicit go.

## Requires

`paracli` on PATH: `brew install itisbryan/tap/paracli`. Token in `$PARABOL_TOKEN`. First run: `paracli init` to index your repos (edit `~/.config/paracli/config.json` to scope it). Full setup, config, and the launchd/ntfy automation live in paracli's own README / `paracli --help` — point the user there for anything beyond the interactive flow.

## Interactive flow

1. **Gather** — `paracli gather` prints my git activity across the indexed repos.
2. **Draft** — *I* write the note in this turn (I'm the drafter — no LLM subprocess needed), in the template the user's standup uses:
   - **full:** `Yesterday:` / `Today:` / `Blockers:`
   - **today-only:** a single `Today:` block (Parabol responses are free text). Git is *past* activity, so for a forward-looking Today lean on in-progress (uncommitted) work + what the user tells me; ask rather than guess.
   Collapse many commits into outcome lines. Show the draft in chat.
3. **Confirm** — ask "submit?" and **wait for an explicit send word** ("submit"/"post it"). A template or format choice is **not** approval. If the user edits, revise and re-show.
4. **Submit** — only after the explicit go, write the approved text to a temp file and:
   ```bash
   paracli list                                  # get the meetingId (needs an open standup)
   paracli submit <meetingId> <file> --yes       # --yes = the confirmed post
   ```
   `paracli submit` refuses without `--yes`; add it only after step 3. It upserts, so a typo can be fixed by resubmitting to the same meeting.

## Rules

- **The asker is me (the agent), not the CLI.** Draft → show → ask → wait for an explicit send word. Never read a template/format choice as approval. `--yes` is just the signal that the human approved.
- **Token stays in the env** — never printed in chat or committed. Redact any captured secrets.
- For scheduled/automated standups (launchd + ntfy notify-to-approve), that's `paracli run` — see paracli's README; this skill is the interactive path.
