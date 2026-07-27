use serde::Serialize;
use std::collections::BTreeMap;

pub const SCHEMA_VERSION: &str = "qa-import/v1";

#[derive(Debug, Serialize)]
pub struct Document {
    pub schema_version: &'static str,
    pub source: SourceMetadata,
    pub summary: Summary,
    pub records: Vec<Record>,
}

#[derive(Debug, Serialize)]
pub struct SourceMetadata {
    pub file_name: String,
    pub format: String,
    pub sheets: Vec<SheetMetadata>,
}

#[derive(Debug, Serialize)]
pub struct SheetMetadata {
    pub name: Option<String>,
    pub header_row: usize,
    pub record_count: usize,
}

#[derive(Debug, Serialize)]
pub struct Summary {
    pub total_records: usize,
    pub status_counts: BTreeMap<String, usize>,
    pub severity_counts: BTreeMap<String, usize>,
}

#[derive(Debug, Serialize)]
pub struct Record {
    pub source: RecordSource,
    pub id: Option<String>,
    pub title: Option<String>,
    pub preconditions: Option<String>,
    pub steps_raw: Option<String>,
    pub steps: Vec<String>,
    pub test_data: Option<String>,
    pub expected: Option<String>,
    pub observed: Option<String>,
    pub status: Option<String>,
    pub severity: Option<String>,
    pub priority: Option<String>,
    pub platform: Option<String>,
    pub environment: Option<String>,
    pub date: Option<String>,
    pub notes: Option<String>,
    pub evidence_urls: Vec<String>,
    pub category: Option<String>,
    pub confirmation: &'static str,
    pub extra_fields: BTreeMap<String, String>,
    pub raw_fields: BTreeMap<String, String>,
}

#[derive(Debug, Serialize)]
pub struct RecordSource {
    pub sheet: Option<String>,
    pub row: usize,
}
