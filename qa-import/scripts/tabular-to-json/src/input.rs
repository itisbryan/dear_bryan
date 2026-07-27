use calamine::{open_workbook_auto, Data, DataType, Reader};
use std::path::Path;

#[derive(Clone, Debug, Default)]
pub struct Cell {
    pub raw: String,
    pub date: Option<String>,
}

#[derive(Debug)]
pub struct SourceRow {
    pub number: usize,
    pub cells: Vec<Cell>,
}

#[derive(Debug)]
pub struct Table {
    pub sheet: Option<String>,
    pub column_offset: usize,
    pub width: usize,
    pub rows: Vec<SourceRow>,
}

pub fn load(path: &Path, requested_sheet: Option<&str>) -> Result<(String, Vec<Table>), String> {
    let extension = path
        .extension()
        .and_then(|value| value.to_str())
        .map(str::to_ascii_lowercase)
        .ok_or_else(|| "input file must have a .csv or .xlsx extension".to_string())?;
    match extension.as_str() {
        "csv" => {
            if requested_sheet.is_some() {
                return Err("--sheet is only valid for .xlsx input".to_string());
            }
            Ok((extension, vec![load_csv(path)?]))
        }
        "xlsx" => Ok((extension, load_xlsx(path, requested_sheet)?)),
        _ => Err(format!(
            "unsupported input format '.{extension}'; expected .csv or .xlsx"
        )),
    }
}

fn load_csv(path: &Path) -> Result<Table, String> {
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(path)
        .map_err(|error| format!("could not open CSV '{}': {error}", path.display()))?;
    let mut rows = Vec::new();
    let mut width = 0;
    for (index, result) in reader.records().enumerate() {
        let record =
            result.map_err(|error| format!("invalid CSV at record {}: {error}", index + 1))?;
        let cells = record
            .iter()
            .map(|value| Cell {
                raw: value.to_string(),
                date: None,
            })
            .collect::<Vec<_>>();
        width = width.max(cells.len());
        let number = record
            .position()
            .map_or(index + 1, |position| position.line() as usize);
        rows.push(SourceRow { number, cells });
    }
    Ok(Table {
        sheet: None,
        column_offset: 0,
        width,
        rows,
    })
}

fn load_xlsx(path: &Path, requested_sheet: Option<&str>) -> Result<Vec<Table>, String> {
    let hidden_rows = crate::xlsx_metadata::hidden_rows(path)?;
    let mut workbook = open_workbook_auto(path)
        .map_err(|error| format!("could not open workbook '{}': {error}", path.display()))?;
    let available = workbook.sheet_names().to_vec();
    let names = if let Some(name) = requested_sheet {
        if !available.iter().any(|candidate| candidate == name) {
            return Err(format!(
                "sheet '{name}' not found; available sheets: {}",
                available.join(", ")
            ));
        }
        vec![name.to_string()]
    } else {
        available
    };

    names
        .into_iter()
        .map(|name| {
            let range = workbook
                .worksheet_range(&name)
                .map_err(|error| format!("could not read sheet '{name}': {error}"))?;
            let (start_row, start_column) = range.start().unwrap_or((0, 0));
            let width = range.width();
            let hidden = hidden_rows.get(&name);
            let rows = range
                .rows()
                .enumerate()
                .filter(|(index, _)| {
                    let number = start_row as usize + index + 1;
                    hidden.is_none_or(|rows| !rows.contains(&number))
                })
                .map(|(index, row)| SourceRow {
                    number: start_row as usize + index + 1,
                    cells: row.iter().map(cell_from_excel).collect(),
                })
                .collect();
            Ok(Table {
                sheet: Some(name),
                column_offset: start_column as usize,
                width,
                rows,
            })
        })
        .collect()
}

fn cell_from_excel(data: &Data) -> Cell {
    let raw = match data {
        Data::Empty => String::new(),
        Data::Int(value) => value.to_string(),
        Data::Float(value) => format_number(*value),
        Data::String(value) | Data::DateTimeIso(value) | Data::DurationIso(value) => value.clone(),
        Data::Bool(value) => value.to_string(),
        Data::DateTime(value) => format_number(value.as_f64()),
        Data::Error(value) => value.to_string(),
    };
    let date = data
        .as_date()
        .map(|value| value.format("%Y-%m-%d").to_string());
    Cell { raw, date }
}

fn format_number(value: f64) -> String {
    if value.is_finite() && value.fract() == 0.0 {
        format!("{value:.0}")
    } else {
        value.to_string()
    }
}
