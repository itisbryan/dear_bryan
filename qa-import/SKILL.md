---
name: qa-import
description: Convert QA bug reports and test-case spreadsheets (.xlsx or .csv) into deterministic, versioned qa-sweep-compatible JSON while preserving source rows and raw fields. Use when asked to import, normalize, migrate, or prepare spreadsheet QA history for a QA sweep.
---

# qa-import

Turn tabular QA history into one deterministic `qa-import/v1` JSON document. The importer normalizes only explicit, well-known values and retains the source text so another agent can audit every mapping.

## Use the importer

The CLI is Rust-only at runtime; it does not require Python, Excel, or LibreOffice.

```bash
cd qa-import/scripts/tabular-to-json
cargo build --release --locked
./target/release/tabular-to-json path/to/report.xlsx > qa-context.json

# choose a sheet/header and output file
./target/release/tabular-to-json report.xlsx \
  --sheet Regression --header 5 --output qa-context.json

# intentionally reproduce the workbook author's visible list
./target/release/tabular-to-json report.xlsx --visible-only

# CSV and compact JSON are also supported
./target/release/tabular-to-json report.csv --compact
```

JSON is pretty-printed to stdout by default. `--output/-o` atomically writes a file in its destination directory, `--sheet` selects one XLSX sheet, `--header` supplies a 1-based header row, and `--compact` disables pretty printing. XLSX conversion imports meaningful rows regardless of manual or AutoFilter-hidden state by default; `--visible-only` explicitly excludes hidden rows. The CLI refuses input aliases and output symlinks rather than risking input overwrite. Run `--help` for the current interface. Errors go to stderr and return a nonzero exit.

## Workflow

1. Confirm the source is `.csv` or `.xlsx`. Never alter the workbook.
2. Run with automatic detection first. For XLSX, every sheet with a detectable QA header and all meaningful rows are imported; use `--sheet` when only one sheet is relevant or `--visible-only` to reproduce the visible list.
3. Check `source.sheets`, `summary`, and a few `records[].source` locations against the workbook.
4. Enrich `records[].module` only when the evidence supports a semantic ownership or technical-layer classification; follow **Post-import module enrichment** below.
5. Preserve the JSON beside the QA output or pass it to `qa-sweep` as prior context.
6. Treat imported records as history, not fresh findings. Reproduce candidates in the current target before confirming or filing them.

## Post-import module enrichment

The parser always emits `"module": null`. It must not guess a module from spreadsheet values or copy top-level **Function Name** metadata such as a product or feature name. Module classification is an agent-owned post-import step because it depends on meaning and project architecture.

For each record, inspect its title, steps, expected and observed behavior, platform, evidence URLs, and known project architecture. Assign the narrowest module supported by that combined evidence—for example `Backend`, `Frontend`, `Mobile`, `Infrastructure`, `AI/Chatbot`, `Integration`, or a more appropriate project-specific label. Use one consistent label for the same module throughout a document. Leave `module` as `null` when the evidence is absent, ambiguous, or only identifies a feature/product; do not mistake a feature or product name for a technical module.

For example, parser output remains unclassified:

```json
{ "title": "Checkout request returns 500", "platform": "Web", "module": null }
```

An agent may change it to `"module": "Backend"` only after the steps, URL, observed response, or known architecture support backend ownership. A Web platform or checkout feature name alone is not enough.

## Header detection

The importer scores rows by distinct recognized QA fields and chooses the earliest highest-scoring row with at least two matches. Matching ignores case, spaces, hyphens, and punctuation. Common aliases include:

| v1 field | Examples |
|:---|:---|
| `id` | ID, Bug ID, Issue ID, Ticket ID, Test Case ID |
| `title` | Title, Summary, Name, Test Case |
| `preconditions` | Pre-condition, Preconditions, Prerequisite |
| `steps_raw` / `steps` | Steps, Repro Steps, Steps to Reproduce |
| `test_data` | Test Data, Prompt, Test Data/Prompt, Input |
| `expected` | Expected, Expected Result, Expected Behavior |
| `observed` | Actual, Actual Result, Observed Result |
| `status` | Status, State, Result, Test Result |
| taxonomy/context | Severity/Impact, Priority, Platform/Device/OS, Environment/Env, Date/Tested On, Notes, Category |

If detection is wrong or intentionally sparse, pass `--header N`. A sheet without two known headers is skipped unless no sheet can be imported, which is an error.

## Conservative normalization

- Non-empty textual IDs are preserved exactly, including surrounding characters, leading zeroes, and values too large for machine integers. Numeric XLSX IDs use the spreadsheet reader's numeric value and render integer-like values as decimal strings.
- Excel date cells and ISO `YYYY-MM-DD`/`YYYY/MM/DD` text become ISO dates. Ambiguous localized text dates remain `null` and survive in `raw_fields`.
- Known statuses become `Fail`, `Pass`, `Feature Requested`, `Pending`, `Blocked`, `Not Run`, or `In Progress`. Unknown statuses remain trimmed source text.
- Severity aliases map only to `Critical`, `High`, `Medium`, or `Low`; unknown values are `null`.
- Priority codes such as `P0 - Urgent` become `P0`; known word priorities are canonicalized. Unknown priority is `null`.
- Common platform/environment aliases are canonicalized; unknown non-empty values remain unchanged.
- `module` is always emitted as `null`; parser normalization never infers it from a column, top-level Function Name metadata, platform, URL, or feature name.
- Two or more numbered reproduction lines become `steps`; `steps_raw` always retains the original text. Non-numbered text remains one step.
- Valid HTTP(S) URI candidates with a host become `evidence_urls`. A candidate must begin at the start of a field, after whitespace, or immediately after an opening wrapper that itself follows whitespace or field start; it ends only at whitespace. Matching is case-insensitive. A closing wrapper is removed only when it matches that external opening wrapper and is the final token character; balanced parentheses within URL content remain intact. Separate multiple evidence URLs with whitespace or newlines. Terminal URI punctuation, commas, semicolons, and later schemes within one non-whitespace token remain URI data: ambiguous punctuation-concatenated text is retained as one candidate or rejected, never destructively split.
- Evidence is parser-canonicalized and stably deduplicated. Unicode paths are percent-encoded and internationalized hosts use their ASCII form; valid `%HH` escapes and IPv6 authority brackets are preserved. Before parsing, candidates containing raw backslashes, ASCII controls/spaces, malformed percent escapes, or multiple literal fragment delimiters (`#`) are rejected rather than rewritten. Candidates are also skipped unless the serialization satisfies the RFC 3986 ASCII character and percent-escape contract, rejecting unescaped pipes and brackets outside an IPv6 authority.
- Category is accepted only when it exactly matches the qa-sweep taxonomy. It is otherwise `null`.
- Every imported row is `unconfirmed`: spreadsheet history does not establish current behavior.

Without an ID column, any row with a recognized non-empty field is imported. With an ID column, a blank-ID row is still imported when it has an explicit row-level status or taxonomy marker (status, severity, priority, or category), or when it has a title plus substantive QA content (preconditions, steps, test data, expected, or observed). Requiring this combination retains independent unclassified QA records while avoiding comment, continuation, and section rows. Fully empty rows, blank separators, and styled cells without values do not become records. Meaningful hidden XLSX rows remain eligible by default because hiding/filtering is presentation state, not deletion; `--visible-only` excludes them intentionally. Formula cells use their cached workbook value.

## v1 contract

[`references/schema-v1.json`](./references/schema-v1.json) is the normative JSON Schema. Each document has:

- `schema_version`: always `qa-import/v1`.
- `source`: portable input basename/format, explicit `row_visibility` (`all_rows` or `visible_only`), plus imported sheet names, 1-based header rows, and counts. No timestamp is emitted, keeping repeated conversions byte-stable.
- `summary`: deterministic total, `hidden_records_excluded`, status counts, and severity counts.
- `records`: normalized fields plus required nullable `module`, 1-based source row/sheet, `extra_fields`, and all `raw_fields`. `module` accepts any string so agents can use project-appropriate labels.

Blank XLSX headers receive deterministic names such as `Column O`; duplicate headers receive suffixes such as `[2]`. `extra_fields` contains non-empty unmapped columns, while `raw_fields` contains every source column represented by the parsed table. Excel display formatting, comments, and formula expressions are not fields; cached values are retained.

SpreadsheetML row numbers are optional. For visibility matching, rows without `row@r` use a cell reference when present and otherwise infer the next 1-based index from document order; omission alone never invalidates a row.

Do not add guessed category or confirmation values in downstream transforms. A future incompatible contract must use a new `schema_version` and schema file rather than silently changing v1.

Migration note: v1 records now require nullable `module`; parser-produced records initialize it to `null`. Existing v1 producers and consumers with fixed record shapes must add or accept this field. v1 source metadata also requires `row_visibility`, and v1 summary metadata requires `hidden_records_excluded`. The default XLSX count can increase because hidden meaningful records are no longer silently discarded.
