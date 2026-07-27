use rust_xlsxwriter::{Format, Workbook};
use serde_json::Value;
use std::fs;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

static NEXT_DIR: AtomicUsize = AtomicUsize::new(0);

struct TestDir(std::path::PathBuf);

impl TestDir {
    fn new() -> Self {
        let sequence = NEXT_DIR.fetch_add(1, Ordering::Relaxed);
        let path =
            std::env::temp_dir().join(format!("qa-import-{}-{sequence}", std::process::id()));
        let _ = fs::remove_dir_all(&path);
        fs::create_dir_all(&path).unwrap();
        Self(path)
    }

    fn path(&self) -> &std::path::Path {
        &self.0
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn run(input: &std::path::Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_tabular-to-json"))
        .arg(input)
        .args(args)
        .output()
        .expect("run importer")
}

#[test]
fn imports_csv_with_detected_header_and_lossless_fields() {
    let dir = TestDir::new();
    let input = dir.path().join("bugs.csv");
    fs::write(
        &input,
        "QA export,,,,,,,,,,,,,\nTicket ID,Summary,Repro Steps,Expected Result,Actual Result,State,Impact,Priority,Device,Env,Test Date,Prompt,Evidence,Owner\nBUG-001,Unicode lỗi,\"1. Open app\n2) Submit\",Works,Broke,failed,S1 - Critical,P0 - Urgent,web,prod,2026-07-09,hello,https://example.test/proof.png,Ai\n",
    )
    .unwrap();

    let output = run(&input, &[]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["schema_version"], "qa-import/v1");
    assert_eq!(json["source"]["sheets"][0]["header_row"], 2);
    assert_eq!(json["summary"]["total_records"], 1);
    assert_eq!(json["summary"]["status_counts"]["Fail"], 1);
    let record = &json["records"][0];
    assert_eq!(record["source"]["row"], 3);
    assert_eq!(record["id"], "BUG-001");
    assert_eq!(record["title"], "Unicode lỗi");
    assert_eq!(record["steps"], serde_json::json!(["Open app", "Submit"]));
    assert_eq!(record["severity"], "Critical");
    assert_eq!(record["priority"], "P0");
    assert_eq!(record["platform"], "Web");
    assert_eq!(record["environment"], "Production");
    assert_eq!(record["date"], "2026-07-09");
    assert_eq!(record["confirmation"], "unconfirmed");
    assert!(record["category"].is_null());
    assert_eq!(record["evidence_urls"][0], "https://example.test/proof.png");
    assert_eq!(record["extra_fields"]["Owner"], "Ai");
    assert_eq!(record["raw_fields"]["Impact"], "S1 - Critical");
}

#[test]
fn imports_generated_xlsx_and_honors_sheet_header_and_output_flags() {
    let dir = TestDir::new();
    let input = dir.path().join("qa.xlsx");
    let output_path = dir.path().join("qa.json");
    let mut workbook = Workbook::new();
    workbook
        .add_worksheet()
        .set_name("Ignore")
        .unwrap()
        .write_string(0, 0, "No table")
        .unwrap();
    let sheet = workbook.add_worksheet().set_name("Regression").unwrap();
    sheet.write_string(0, 0, "Release 7").unwrap();
    let headers = [
        "Test Case ID",
        "Name",
        "Steps",
        "Expected",
        "Observed",
        "Result",
        "Severity",
        "Tested On",
        "Notes",
        "Custom",
    ];
    for (column, header) in headers.iter().enumerate() {
        sheet.write_string(2, column as u16, *header).unwrap();
    }
    let values = [
        "42",
        "Checkout",
        "1: Add\n2: Pay",
        "Receipt",
        "Timeout",
        "feature request",
        "major",
        "",
        "keep me",
        "extra",
    ];
    for (column, value) in values.iter().enumerate() {
        sheet.write_string(3, column as u16, *value).unwrap();
    }
    let date_format = Format::new().set_num_format("yyyy-mm-dd");
    sheet
        .write_number_with_format(3, 7, 46212.0, &date_format)
        .unwrap();
    workbook.save(&input).unwrap();

    let output = run(
        &input,
        &[
            "--sheet",
            "Regression",
            "--header",
            "3",
            "--output",
            output_path.to_str().unwrap(),
            "--compact",
        ],
    );
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(output.stdout.is_empty());
    let text = fs::read_to_string(output_path).unwrap();
    assert!(!text.contains("\n  \""));
    let json: Value = serde_json::from_str(&text).unwrap();
    let record = &json["records"][0];
    assert_eq!(record["source"]["sheet"], "Regression");
    assert_eq!(record["source"]["row"], 4);
    assert_eq!(record["status"], "Feature Requested");
    assert_eq!(record["severity"], "High");
    assert_eq!(record["date"], "2026-07-09");
    assert_eq!(record["notes"], "keep me");
    assert_eq!(record["extra_fields"]["Custom"], "extra");
}

#[test]
fn preserves_physical_csv_rows_after_multiline_records() {
    let dir = TestDir::new();
    let input = dir.path().join("rows.csv");
    fs::write(
        &input,
        "Title,Status,Steps\nOne,Fail,\"1. A\n2. B\"\nTwo,Pass,Done\n",
    )
    .unwrap();

    let output = run(&input, &[]);
    assert!(output.status.success());
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["records"][0]["source"]["row"], 2);
    assert_eq!(json["records"][1]["source"]["row"], 4);
}

#[test]
fn refuses_to_overwrite_the_input() {
    let dir = TestDir::new();
    let input = dir.path().join("bugs.csv");
    let original = "Title,Status\nBug,Fail\n";
    fs::write(&input, original).unwrap();

    let output = run(&input, &["--output", input.to_str().unwrap()]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("same file"));
    assert_eq!(fs::read_to_string(input).unwrap(), original);
}

#[test]
fn rejects_unsupported_input_with_nonzero_exit_and_stderr() {
    let dir = TestDir::new();
    let input = dir.path().join("bugs.txt");
    fs::write(&input, "Title").unwrap();
    let output = run(&input, &[]);
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unsupported input format"));
}
