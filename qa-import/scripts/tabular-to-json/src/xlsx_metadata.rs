use quick_xml::events::{BytesStart, Event};
use quick_xml::Reader;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use zip::ZipArchive;

pub fn hidden_rows(path: &Path) -> Result<HashMap<String, HashSet<usize>>, String> {
    let file = File::open(path)
        .map_err(|error| format!("could not open XLSX '{}': {error}", path.display()))?;
    let mut archive = ZipArchive::new(file)
        .map_err(|error| format!("could not read XLSX '{}': {error}", path.display()))?;
    let workbook = read_entry(&mut archive, "xl/workbook.xml")?;
    let relationships = read_entry(&mut archive, "xl/_rels/workbook.xml.rels")?;
    let targets = relationship_targets(&relationships)?;
    let mut result = HashMap::new();

    for (sheet, relationship) in workbook_sheets(&workbook)? {
        let target = targets
            .get(&relationship)
            .ok_or_else(|| format!("XLSX relationship '{relationship}' is missing"))?;
        let xml = read_entry(&mut archive, &resolve_workbook_target(target))?;
        result.insert(sheet, worksheet_hidden_rows(&xml)?);
    }
    Ok(result)
}

fn read_entry(archive: &mut ZipArchive<File>, name: &str) -> Result<Vec<u8>, String> {
    let mut entry = archive
        .by_name(name)
        .map_err(|error| format!("could not read XLSX entry '{name}': {error}"))?;
    let mut bytes = Vec::new();
    entry
        .read_to_end(&mut bytes)
        .map_err(|error| format!("could not read XLSX entry '{name}': {error}"))?;
    Ok(bytes)
}

fn workbook_sheets(xml: &[u8]) -> Result<Vec<(String, String)>, String> {
    let mut reader = Reader::from_reader(xml);
    let mut sheets = Vec::new();
    loop {
        match reader.read_event() {
            Ok(Event::Empty(element)) | Ok(Event::Start(element))
                if element.local_name().as_ref() == b"sheet" =>
            {
                let name = attribute(&reader, &element, b"name")?
                    .ok_or_else(|| "XLSX sheet is missing its name".to_string())?;
                let relationship = attribute(&reader, &element, b"id")?
                    .ok_or_else(|| format!("XLSX sheet '{name}' is missing its relationship"))?;
                sheets.push((name, relationship));
            }
            Ok(Event::Eof) => return Ok(sheets),
            Ok(_) => {}
            Err(error) => return Err(format!("could not parse XLSX workbook metadata: {error}")),
        }
    }
}

fn relationship_targets(xml: &[u8]) -> Result<HashMap<String, String>, String> {
    let mut reader = Reader::from_reader(xml);
    let mut targets = HashMap::new();
    loop {
        match reader.read_event() {
            Ok(Event::Empty(element)) | Ok(Event::Start(element))
                if element.local_name().as_ref() == b"Relationship" =>
            {
                if let (Some(id), Some(target)) = (
                    attribute(&reader, &element, b"Id")?,
                    attribute(&reader, &element, b"Target")?,
                ) {
                    targets.insert(id, target);
                }
            }
            Ok(Event::Eof) => return Ok(targets),
            Ok(_) => {}
            Err(error) => {
                return Err(format!(
                    "could not parse XLSX workbook relationships: {error}"
                ))
            }
        }
    }
}

fn worksheet_hidden_rows(xml: &[u8]) -> Result<HashSet<usize>, String> {
    let mut reader = Reader::from_reader(xml);
    let mut rows = HashSet::new();
    let mut next_number = 1;
    let mut current: Option<(bool, Option<usize>, usize)> = None;
    loop {
        match reader.read_event() {
            Ok(Event::Start(element)) if element.local_name().as_ref() == b"row" => {
                let hidden = row_is_hidden(&reader, &element)?;
                let number = row_number(&reader, &element)?;
                current = Some((hidden, number, next_number));
            }
            Ok(Event::Empty(element)) if element.local_name().as_ref() == b"row" => {
                let hidden = row_is_hidden(&reader, &element)?;
                let number = row_number(&reader, &element)?.unwrap_or(next_number);
                finish_row(&mut rows, &mut next_number, hidden, number);
            }
            Ok(Event::Start(element)) | Ok(Event::Empty(element))
                if element.local_name().as_ref() == b"c" =>
            {
                if let Some((_, number @ None, _)) = current.as_mut() {
                    if let Some(reference) = attribute(&reader, &element, b"r")? {
                        *number = cell_row_number(&reference);
                    }
                }
            }
            Ok(Event::End(element)) if element.local_name().as_ref() == b"row" => {
                if let Some((hidden, number, fallback)) = current.take() {
                    finish_row(
                        &mut rows,
                        &mut next_number,
                        hidden,
                        number.unwrap_or(fallback),
                    );
                }
            }
            Ok(Event::Eof) => return Ok(rows),
            Ok(_) => {}
            Err(error) => return Err(format!("could not parse XLSX row metadata: {error}")),
        }
    }
}

fn row_is_hidden(reader: &Reader<&[u8]>, element: &BytesStart<'_>) -> Result<bool, String> {
    Ok(attribute(reader, element, b"hidden")?
        .is_some_and(|value| matches!(value.as_str(), "1" | "true")))
}

fn row_number(reader: &Reader<&[u8]>, element: &BytesStart<'_>) -> Result<Option<usize>, String> {
    attribute(reader, element, b"r")?
        .map(|value| {
            value
                .parse::<usize>()
                .map_err(|error| format!("invalid XLSX row number: {error}"))
                .and_then(|number| {
                    (number > 0)
                        .then_some(number)
                        .ok_or_else(|| "invalid XLSX row number: must be at least 1".to_string())
                })
        })
        .transpose()
}

fn cell_row_number(reference: &str) -> Option<usize> {
    let digits = reference.trim_start_matches(|character: char| character.is_ascii_alphabetic());
    (!digits.is_empty() && digits.chars().all(|character| character.is_ascii_digit()))
        .then(|| digits.parse().ok())
        .flatten()
        .filter(|number| *number > 0)
}

fn finish_row(rows: &mut HashSet<usize>, next_number: &mut usize, hidden: bool, number: usize) {
    if hidden {
        rows.insert(number);
    }
    *next_number = number.saturating_add(1);
}

fn attribute(
    reader: &Reader<&[u8]>,
    element: &BytesStart<'_>,
    wanted: &[u8],
) -> Result<Option<String>, String> {
    for result in element.attributes() {
        let attribute = result.map_err(|error| format!("invalid XLSX attribute: {error}"))?;
        if attribute.key.local_name().as_ref() == wanted {
            return attribute
                .decode_and_unescape_value(reader)
                .map(|value| Some(value.into_owned()))
                .map_err(|error| format!("invalid XLSX attribute value: {error}"));
        }
    }
    Ok(None)
}

fn resolve_workbook_target(target: &str) -> String {
    let joined = if target.starts_with('/') {
        target.trim_start_matches('/').to_string()
    } else {
        format!("xl/{target}")
    };
    let mut parts = Vec::new();
    for part in joined.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(part),
        }
    }
    parts.join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hidden_rows_infer_missing_numbers_from_cells_or_document_order() {
        let xml = br#"<worksheet><sheetData>
            <row r="2"><c r="A2"/></row>
            <row hidden="1"><c r="C5"/></row>
            <row hidden="true"><c/></row>
        </sheetData></worksheet>"#;

        assert_eq!(worksheet_hidden_rows(xml).unwrap(), HashSet::from([5, 6]));
    }
}
