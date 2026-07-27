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

# CSV and compact JSON are also supported
./target/release/tabular-to-json report.csv --compact
```

JSON is pretty-printed to stdout by default. `--output/-o` writes a file, `--sheet` selects one XLSX sheet, `--header` supplies a 1-based header row, and `--compact` disables pretty printing. The CLI refuses to overwrite its input. Run `--help` for the current interface. Errors go to stderr and return a nonzero exit.

## Workflow

1. Confirm the source is `.csv` or `.xlsx`. Never alter the workbook.
2. Run with automatic detection first. For XLSX, every sheet with a detectable QA header is imported; use `--sheet` when only one sheet is relevant.
3. Check `source.sheets`, `summary`, and a few `records[].source` locations against the workbook.
4. Preserve the JSON beside the QA output or pass it to `qa-sweep` as prior context.
5. Treat imported records as history, not fresh findings. Reproduce candidates in the current target before confirming or filing them.

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

- Numeric IDs become decimal strings; textual IDs are trimmed and retained.
- Excel date cells and ISO `YYYY-MM-DD`/`YYYY/MM/DD` text become ISO dates. Ambiguous localized text dates remain `null` and survive in `raw_fields`.
- Known statuses become `Fail`, `Pass`, `Feature Requested`, `Pending`, `Blocked`, `Not Run`, or `In Progress`. Unknown statuses remain trimmed source text.
- Severity aliases map only to `Critical`, `High`, `Medium`, or `Low`; unknown values are `null`.
- Priority codes such as `P0 - Urgent` become `P0`; known word priorities are canonicalized. Unknown priority is `null`.
- Common platform/environment aliases are canonicalized; unknown non-empty values remain unchanged.
- Two or more numbered reproduction lines become `steps`; `steps_raw` always retains the original text. Non-numbered text remains one step.
- Distinct HTTP(S) URLs found anywhere in a row become `evidence_urls`.
- Category is accepted only when it exactly matches the qa-sweep taxonomy. It is otherwise `null`.
- Every imported row is `unconfirmed`: spreadsheet history does not establish current behavior.

When an ID column exists, rows with a blank ID are treated as scratch/section rows and skipped. Without an ID column, any row with a recognized non-empty field is imported. Formula cells use their cached workbook value. Styled empty rows do not become records.

## v1 contract

[`references/schema-v1.json`](./references/schema-v1.json) is the normative JSON Schema. Each document has:

- `schema_version`: always `qa-import/v1`.
- `source`: portable input basename/format plus imported sheet names, 1-based header rows, and counts. No timestamp is emitted, keeping repeated conversions byte-stable.
- `summary`: deterministic total, status counts, and severity counts.
- `records`: normalized fields plus 1-based source row/sheet, `extra_fields`, and all `raw_fields`.

Blank XLSX headers receive deterministic names such as `Column O`; duplicate headers receive suffixes such as `[2]`. `extra_fields` contains non-empty unmapped columns, while `raw_fields` contains every source column represented by the parsed table. Excel display formatting, comments, and formula expressions are not fields; cached values are retained.

Do not add guessed category or confirmation values in downstream transforms. A future incompatible contract must use a new `schema_version` and schema file rather than silently changing v1.
