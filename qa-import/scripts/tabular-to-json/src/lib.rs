mod input;
pub mod model;
mod normalize;
mod xlsx_metadata;

use input::{Cell, Table};
use model::{
    Document, Record, RecordSource, SheetMetadata, SourceMetadata, Summary, SCHEMA_VERSION,
};
use normalize::Field;
use std::collections::{BTreeMap, HashMap};
use std::path::Path;

pub struct Options<'a> {
    pub sheet: Option<&'a str>,
    pub header_row: Option<usize>,
    pub visible_only: bool,
}

pub fn convert(path: &Path, options: Options<'_>) -> Result<Document, String> {
    if options.header_row == Some(0) {
        return Err("--header is 1-based and must be at least 1".to_string());
    }
    let (format, tables) = input::load(path, options.sheet, options.visible_only)?;
    let mut records = Vec::new();
    let mut sheet_metadata = Vec::new();
    let mut errors = Vec::new();
    let mut hidden_records_excluded = 0;

    for table in tables {
        match import_table(&table, options.header_row, options.visible_only) {
            Ok((header_row, mut imported, excluded)) => {
                sheet_metadata.push(SheetMetadata {
                    name: table.sheet.clone(),
                    header_row,
                    record_count: imported.len(),
                });
                records.append(&mut imported);
                hidden_records_excluded += excluded;
            }
            Err(error) => errors.push(error),
        }
    }
    if sheet_metadata.is_empty() {
        return Err(errors.join("; "));
    }

    let mut status_counts = BTreeMap::new();
    let mut severity_counts = BTreeMap::new();
    for record in &records {
        if let Some(status) = &record.status {
            *status_counts.entry(status.clone()).or_insert(0) += 1;
        }
        if let Some(severity) = &record.severity {
            *severity_counts.entry(severity.clone()).or_insert(0) += 1;
        }
    }
    let file_name = path
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .into_owned();
    Ok(Document {
        schema_version: SCHEMA_VERSION,
        source: SourceMetadata {
            file_name,
            format,
            row_visibility: if options.visible_only {
                "visible_only"
            } else {
                "all_rows"
            },
            sheets: sheet_metadata,
        },
        summary: Summary {
            total_records: records.len(),
            hidden_records_excluded,
            status_counts,
            severity_counts,
        },
        records,
    })
}

fn import_table(
    table: &Table,
    requested_header: Option<usize>,
    visible_only: bool,
) -> Result<(usize, Vec<Record>, usize), String> {
    let header_index = if let Some(number) = requested_header {
        table
            .rows
            .iter()
            .position(|row| row.number == number)
            .ok_or_else(|| format!("header row {number} is outside {}", table_name(table)))?
    } else {
        detect_header(table).ok_or_else(|| {
            format!(
                "could not detect a QA header row in {} (need at least two known columns)",
                table_name(table)
            )
        })?
    };
    let header_row = &table.rows[header_index];
    let width = table.width.max(header_row.cells.len());
    let headers = unique_headers(header_row, width, table.column_offset);
    let fields = headers
        .iter()
        .map(|header| normalize::field_for_header(header))
        .collect::<Vec<_>>();
    let id_column_present = fields.contains(&Some(Field::Id));
    let mut records = Vec::new();
    let mut hidden_records_excluded = 0;
    for row in table.rows.iter().skip(header_index + 1) {
        if is_record_row(row, &fields, id_column_present) {
            if visible_only && row.hidden {
                hidden_records_excluded += 1;
            } else {
                records.push(build_record(table, row, &headers, &fields));
            }
        }
    }
    Ok((header_row.number, records, hidden_records_excluded))
}

fn detect_header(table: &Table) -> Option<usize> {
    table
        .rows
        .iter()
        .enumerate()
        .map(|(index, row)| {
            let values = row
                .cells
                .iter()
                .map(|cell| cell.raw.clone())
                .collect::<Vec<_>>();
            (index, normalize::header_score(&values))
        })
        .filter(|(_, score)| *score >= 2)
        .max_by_key(|(index, score)| (*score, std::cmp::Reverse(*index)))
        .map(|(index, _)| index)
}

fn is_record_row(
    row: &input::SourceRow,
    fields: &[Option<Field>],
    id_column_present: bool,
) -> bool {
    let has_value = |wanted: Field| {
        fields.iter().enumerate().any(|(index, field)| {
            *field == Some(wanted)
                && row
                    .cells
                    .get(index)
                    .is_some_and(|cell| !cell.raw.trim().is_empty())
        })
    };
    if id_column_present {
        return has_value(Field::Id)
            || [
                Field::Status,
                Field::Severity,
                Field::Priority,
                Field::Category,
            ]
            .into_iter()
            .any(has_value)
            || (has_value(Field::Title)
                && [
                    Field::Preconditions,
                    Field::Steps,
                    Field::TestData,
                    Field::Expected,
                    Field::Observed,
                ]
                .into_iter()
                .any(has_value));
    }
    fields.iter().enumerate().any(|(index, field)| {
        field.is_some()
            && row
                .cells
                .get(index)
                .is_some_and(|cell| !cell.raw.trim().is_empty())
    })
}

fn build_record(
    table: &Table,
    row: &input::SourceRow,
    headers: &[String],
    fields: &[Option<Field>],
) -> Record {
    let mapped = mapped_cells(row, fields);
    let raw = |field| mapped.get(&field).map(|cell| cell.raw.as_str());
    let steps_raw = normalize::text(raw(Field::Steps));
    let mut raw_fields = BTreeMap::new();
    let mut extra_fields = BTreeMap::new();
    for (index, header) in headers.iter().enumerate() {
        let value = row
            .cells
            .get(index)
            .map_or("", |cell| cell.raw.as_str())
            .to_string();
        raw_fields.insert(header.clone(), value.clone());
        if fields[index].is_none() && !value.trim().is_empty() {
            extra_fields.insert(header.clone(), value);
        }
    }
    Record {
        source: RecordSource {
            sheet: table.sheet.clone(),
            row: row.number,
        },
        id: normalize::id(raw(Field::Id)),
        title: normalize::text(raw(Field::Title)),
        preconditions: normalize::text(raw(Field::Preconditions)),
        steps: normalize::split_steps(steps_raw.as_deref()),
        steps_raw,
        test_data: normalize::text(raw(Field::TestData)),
        expected: normalize::text(raw(Field::Expected)),
        observed: normalize::text(raw(Field::Observed)),
        status: normalize::status(raw(Field::Status)),
        severity: normalize::severity(raw(Field::Severity)),
        priority: normalize::priority(raw(Field::Priority)),
        platform: normalize::platform(raw(Field::Platform)),
        module: None,
        environment: normalize::environment(raw(Field::Environment)),
        date: normalize::date(
            raw(Field::Date),
            mapped
                .get(&Field::Date)
                .and_then(|cell| cell.date.as_deref()),
        ),
        notes: normalize::text(raw(Field::Notes)),
        evidence_urls: normalize::evidence_urls(row.cells.iter().map(|cell| cell.raw.clone())),
        category: normalize::category(raw(Field::Category)),
        confirmation: "unconfirmed",
        extra_fields,
        raw_fields,
    }
}

fn mapped_cells<'a>(
    row: &'a input::SourceRow,
    fields: &[Option<Field>],
) -> HashMap<Field, &'a Cell> {
    let mut result = HashMap::new();
    for (index, field) in fields.iter().enumerate() {
        if let (Some(field), Some(cell)) = (field, row.cells.get(index)) {
            result.entry(*field).or_insert(cell);
        }
    }
    result
}

fn unique_headers(row: &input::SourceRow, width: usize, offset: usize) -> Vec<String> {
    let mut seen: HashMap<String, usize> = HashMap::new();
    (0..width)
        .map(|index| {
            let original = row.cells.get(index).map_or("", |cell| cell.raw.as_str());
            let base = if original.trim().is_empty() {
                format!("Column {}", column_name(offset + index))
            } else {
                original.to_string()
            };
            let count = seen.entry(base.clone()).or_insert(0);
            *count += 1;
            if *count == 1 {
                base
            } else {
                format!("{base} [{}]", *count)
            }
        })
        .collect()
}

fn column_name(mut index: usize) -> String {
    let mut name = String::new();
    loop {
        name.insert(0, (b'A' + (index % 26) as u8) as char);
        if index < 26 {
            break;
        }
        index = index / 26 - 1;
    }
    name
}

fn table_name(table: &Table) -> String {
    table
        .sheet
        .as_ref()
        .map_or_else(|| "CSV input".to_string(), |name| format!("sheet '{name}'"))
}
