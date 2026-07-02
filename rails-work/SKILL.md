---
name: rails-work
description: How I work with Rails — a growing collection of subskills covering my tooling, conventions, and workflows for Rails projects. Loads the relevant subskill on demand. Use when working in a Ruby on Rails codebase (setup, running the app, gems, Ruby versions, conventions). Currently ships use-rv (Ruby version & gem management with rv).
---

# rails-work

My personal conventions for working in Rails projects. This skill is an index — the actual guidance lives in focused subskills under `references/`, each loaded only when its topic comes up, so the base stays lean as it grows.

## How to use

When working in a Rails project, check the table below and load the subskill whose trigger fires. Follow that file's guidance fully. A subskill is part of this skill — don't announce a skill switch, just apply it.

## Subskills

| Trigger | Load | What it covers |
|---|---|---|
| Anything about the Ruby version, installing Ruby/gems, `rbenv`/`rvm`/`chruby`, running app commands, `bundle install` | [`references/use-rv.md`](./references/use-rv.md) | Use **rv** (spinel-coop/rv) for Ruby version & gem management instead of rbenv/ruby-build/rvm. Setup, command cheat-sheet, and the rbenv→rv mapping. |

_More subskills to come (conventions, testing, deploy, …). Add them as rows here._

## Rules for loading subskills

- Load only when its trigger fires — progressive disclosure keeps the base skill small.
- After loading, follow that file's instructions fully.
- If two subskills apply, load both; they're written to compose.
