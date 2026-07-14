---
name: paperwork
description: Create and render polished structured documents with the Paperwork Ruby CLI: turn a spec.json into HTML, PDF, and PNG; use deterministic --json results for agents; diagnose build or Playwright render failures. Use when asked to create an API guide, SOP, manual, report, or other polished single-file document with Paperwork.
---

# paperwork

Use Paperwork to turn a structured `spec.json` into a polished HTML document, PDF, and PNG screenshot. Establish the document’s content and structure first, then encode it faithfully in the spec. The spec—not generated artifacts—is the source of truth.

## Rules

1. **Automate through JSON.** Run `paperwork --json ...`; consume its one-line JSON `Result`, never checkmarks or prose.
2. **Regenerate, never patch output.** Do not hand-edit generated `.html`, `.pdf`, or `.png` files.
3. **Respect the workflow.** Ruby builds HTML; Node/Playwright renders Mermaid, PDF, and PNG. PDF/PNG require the browser leg.
4. **A run succeeds only when exit status is zero and `ok` is `true`.** On failure report `error`, all completed `stages`, and only artifact paths actually present in `artifacts`.
5. **Missing images are intentional placeholders by default.** Do not fail or rewrite the spec solely because `image.src` is absent on disk unless the user requires the real image.

## Setup

Paperwork needs Ruby 3+, Node, Playwright/Chromium, and either an installed `paperwork` command or a Paperwork checkout.

```bash
# Once, inside a Paperwork checkout:
bundle install && npm install

# Pick exactly one command for the rest of the session:
PAPERWORK=paperwork             # installed gem
# PAPERWORK=bin/paperwork       # from the Paperwork checkout
"$PAPERWORK" --help
```

Do not assume a `--preflight` command exists. Check `--help` when upgrading Paperwork and only add that step after the installed version implements it.

## Choose the right mode

| Need | Use | Why |
|---|---|---|
| Plan or revise document content | Edit `spec.json` | The spec remains the source of truth. |
| Fast HTML-only visual iteration in a Paperwork checkout | `Builder.from_files(...).write(...)` | Avoids Node/Chromium; PDF and Mermaid browser verification do not run. |
| Deliverable HTML + PDF + PNG | `"$PAPERWORK" --json spec.json out-dir` | Runs full production and returns the agent-readable `Result`. |

For a substantive new document whose structure is not agreed, use the collection’s `brainstorm` skill before authoring the spec.

## Create the spec

Start from [`references/minimal-spec.json`](references/minimal-spec.json), then use only the conventions the document needs:

- Text beginning `Method:` becomes an endpoint card.
- Heading `Phần ...` (or `Part ...` for English locale) becomes a divider page.
- Vietnamese `Lưu ý`, `Ghi chú`, `Khuyến nghị`, `Cảnh báo`, or English `Note`, `Warning`, `Tip`, `Danger` headings become callouts.
- `image.src` missing on disk renders a labelled placeholder.
- `mermaid` is rendered in the browser.
- Tables use `columns`, `rows`, and optional relative `widths`.

Use `"locale": "en"` for neutral English labels. Use `"boldTerms": []` to disable Paperwork’s default bold-term list. Keep image paths relative to the spec file. Do not add endpoint-card syntax to a report or SOP simply because it is supported.

Validate JSON before the browser run:

```bash
ruby -rjson -e 'JSON.parse(File.read(ARGV.fetch(0)))' path/to/spec.json
```

For an HTML-only loop from a Paperwork checkout:

```bash
PAPERWORK_ROOT=/absolute/path/to/paperwork
ruby -I"$PAPERWORK_ROOT/lib" -rpaperwork -e \
  'Paperwork::Builder.from_files(ARGV[0], File.join(ENV.fetch("PAPERWORK_ROOT"), "assets/mermaid.min.js")).write(ARGV[1])' \
  path/to/spec.json path/to/preview.html
```

## Produce and parse a full result

Use a deliberate, empty-or-disposable output directory. A spec named `guide.json` writes `guide.html`, `guide.pdf`, and `guide.png` into that directory, replacing same-named artifacts from an earlier run.

```bash
out_dir=build/paperwork-guide
mkdir -p "$out_dir"
"$PAPERWORK" --json path/to/guide.json "$out_dir" > "$out_dir/result.json"
status=$?

jq '{ok, artifacts, error, stages}' "$out_dir/result.json"
exit "$status"
```

[`references/result-success.json`](references/result-success.json) and [`references/result-render-failure.json`](references/result-render-failure.json) show the result shape.

- `ok: true` means every stage succeeded.
- `artifacts` contains only files actually written—never infer paths yourself.
- `stages` contains build and render facts such as section counts and Mermaid diagnostics.
- `error` is `null` on success; otherwise it identifies the owning stage and message.

For a person at a terminal, omit `--json`. Agents and CI should retain `result.json` with the job logs.

## Verify and report

On success:

1. Confirm exit status `0` and `ok: true`.
2. Report the absolute paths listed in `artifacts`.
3. Inspect `stages` for Mermaid counts/errors; every expected diagram must render and errors must be zero.
4. Open the PDF or PNG when print layout, tables, or diagrams matter.

On failure:

1. Preserve `result.json` and state `error.stage` and `error.message`.
2. Report any artifact paths that appear in `artifacts`; a rendered HTML file is diagnostic evidence, not a completed document.
3. Fix the owning problem, then rerun full production—do not patch the partial artifact.

| Failure stage | Typical cause | Response |
|---|---|---|
| `input` | Missing requested spec | Correct the path or obtain the spec. |
| `build` | Invalid JSON, invalid spec data, or HTML build I/O | Run the JSON validation command; read the returned message and fix the source spec. |
| `render` | Missing Node/Playwright or Mermaid/browser render failure | Run `npm install` in the Paperwork checkout when dependencies are absent; inspect Mermaid diagnostics and fix the source diagram. |

A render-failure result may retain HTML but omit PDF/PNG. See the failure reference for the expected agent response.

## Quality bar

- Content is approved, accurate, and represented in the spec.
- JSON validates; referenced image paths are intentional.
- Full production returns exit `0`, `ok: true`, and HTML/PDF/PNG entries in `artifacts`.
- Mermaid diagnostics indicate all expected diagrams rendered with zero errors.
- A visual spot-check confirms readable print layout with no obvious clipping.

## Scope

This skill uses Paperwork; it does not change its templates, renderer, or CLI architecture. For implementation changes, work in the Paperwork repository and follow its `CLAUDE.md`.
