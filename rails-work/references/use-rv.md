# use-rv (subskill of rails-work)

Load this whenever Ruby versions, gem installation, or "how do I run this" comes up in a Rails project. It replaces the old-school toolchain.

## The rule

**Use [`rv`](https://github.com/spinel-coop/rv) for Ruby version and gem management. Don't use rbenv, ruby-build, rvm, or chruby.**

`rv` (from Spinel Cooperative — the folks behind Bundler, rbenv, and Rails) is a single fast Rust tool that auto-installs and manages Ruby versions *and* project gems. It installs a Ruby in ~2 seconds and switches versions per project via `.ruby-version` / `.tool-versions`. If you catch yourself reaching for `rbenv install` or `gem install bundler`, stop — use the `rv` equivalent below.

> **Windows PowerShell:** use `rvw` instead of `rv` (`rv` is a built-in PowerShell alias for `Remove-Variable`). All commands below are identical otherwise.

## One-time setup

```bash
# install rv (macOS/Linux)
brew install rv
# other options (standalone installer, Windows, specific versions):
# see https://github.com/spinel-coop/rv#install

# shell integration — enables automatic version switching from .ruby-version
rv shell zsh    # or bash | fish | nu | powershell
# follow the printed instructions (adds a hook to your shell rc), then restart the shell
```

After this, `cd`-ing into a project with a `.ruby-version` gives you the right Ruby automatically — no manual `rbenv shell`/`rbenv local` dance.

## Everyday commands

```bash
rv ruby pin 3.4.7      # set the project's Ruby version (writes .ruby-version, auto-installs if needed)
rv clean-install       # install the project's Ruby + gems from Gemfile.lock (the "get set up" command)
rv run ruby …          # run a command/script with the correct Ruby on PATH
rv run bin/rails c     # e.g. run the Rails console with the project's Ruby
rvx rails new myapp    # run a gem CLI directly, without installing it first
rv tool install rerun  # install a gem-based CLI into its own isolated environment
```

Supported Ruby versions: **3.2, 3.3, 3.4, 4.0** (macOS 14+, Linux glibc 2.35+, Windows 10+; x86 & arm64).

## Coming from rbenv / rvm — the mapping

| Old way | With `rv` |
|---|---|
| `rbenv install 3.4.7` | just `rv ruby pin 3.4.7` — it auto-installs on pin/use |
| `rbenv local 3.4.7` (write `.ruby-version`) | `rv ruby pin 3.4.7` |
| `rbenv shell` / manual switching | automatic via `rv shell` integration + `.ruby-version` |
| `bundle install` (from lockfile) | `rv clean-install` |
| `bundle exec <cmd>` | `rv run <cmd>` (or just `<cmd>` once shell integration is active) |
| `gem install <cli>` (global tool) | `rv tool install <cli>` |
| one-off `gem exec` / `npx`-style run | `rvx <cli>` |
| `rbenv versions` / `rbenv install -l` | `rv ruby --help` for the list/install subcommands |

## Typical Rails flows

- **New app:** `rvx rails new myapp` → `cd myapp` → `rv ruby pin <version>` → `rv clean-install`.
- **Cloning an existing repo:** `cd repo` → `rv clean-install` (reads `.ruby-version` + `Gemfile.lock`, installs both) → run with `rv run bin/rails …`.
- **Bumping Ruby:** `rv ruby pin <newversion>`, then `rv clean-install` to rebuild gems against it.

## Notes

- Prefer the confirmed commands above. For the full surface (and any subcommand not listed here), run `rv --help` / `rv ruby --help` rather than guessing — don't invent flags.
- Don't add rbenv/rvm shims back into the shell rc alongside `rv`; two version managers fighting over PATH is the classic "wrong Ruby" bug.
- Commit `.ruby-version` so `rv` (and CI, and teammates) all resolve the same Ruby.
