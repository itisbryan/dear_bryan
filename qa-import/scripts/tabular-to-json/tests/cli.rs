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
    assert_eq!(json["source"]["row_visibility"], "all_rows");
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
fn retains_meaningful_blank_id_rows_and_exact_text_ids() {
    let dir = TestDir::new();
    let input = dir.path().join("ids.csv");
    fs::write(
        &input,
        "ID,Title,Status,Notes\n,Unassigned bug,Fail,keep\n,Continuation row,,detail\n,,,\n\" 000123 \",Leading zero,Pass,\n123456789012345678901234567890,Large ID,Fail,\n",
    )
    .unwrap();

    let output = run(&input, &[]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["summary"]["total_records"], 3);
    assert!(json["records"][0]["id"].is_null());
    assert_eq!(json["records"][0]["title"], "Unassigned bug");
    assert_eq!(json["records"][1]["id"], " 000123 ");
    assert_eq!(json["records"][2]["id"], "123456789012345678901234567890");
}

#[test]
fn csv_retains_unclassified_blank_id_rows_with_substantive_qa_content() {
    let dir = TestDir::new();
    let input = dir.path().join("unclassified.csv");
    fs::write(
        &input,
        "ID,Title,Steps,Expected,Observed,Status,Notes\n,Checkout fails,Open checkout,Checkout opens,Spinner remains,,\n,Continuation only,,,,,more context\n,,,,,,\n",
    )
    .unwrap();

    let output = run(&input, &[]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["summary"]["total_records"], 1);
    assert!(json["records"][0]["id"].is_null());
    assert!(json["records"][0]["status"].is_null());
    assert_eq!(json["records"][0]["title"], "Checkout fails");
    assert_eq!(json["records"][0]["steps_raw"], "Open checkout");
    assert_eq!(json["records"][0]["expected"], "Checkout opens");
    assert_eq!(json["records"][0]["observed"], "Spinner remains");
}

#[test]
fn preserves_nested_and_punctuation_concatenated_urls_as_single_candidates() {
    let dir = TestDir::new();
    let input = dir.path().join("nested-urls.csv");
    fs::write(
        &input,
        "Title,Status,Evidence\nBug,Fail,\"(https://outer.test/go?targets=https://inner.test/a,https://inner.test/b;https://inner.test/c) [https://outer.test/proxy/https://inner.test/a,https://inner.test/b;https://inner.test/c/end] https://outer.test/page#targets=https://inner.test/a,https://inner.test/b;https://inner.test/c prefix(https://ignored.test/no) https://first.test/a,https://second.test/b\"\n",
    )
    .unwrap();

    let output = run(&input, &[]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        json["records"][0]["evidence_urls"],
        serde_json::json!([
            "https://outer.test/go?targets=https://inner.test/a,https://inner.test/b;https://inner.test/c",
            "https://outer.test/proxy/https://inner.test/a,https://inner.test/b;https://inner.test/c/end",
            "https://outer.test/page#targets=https://inner.test/a,https://inner.test/b;https://inner.test/c",
            "https://first.test/a,https://second.test/b"
        ])
    );
}

#[test]
fn emits_canonical_rfc3986_http_uris_and_skips_malformed_candidates() {
    let dir = TestDir::new();
    let input = dir.path().join("validated-urls.csv");
    fs::write(
        &input,
        "Title,Status,Evidence\nBug,Fail,\"<https://[2001:db8::1]/proof> https://例え.テスト/雪?q=✓ https://valid.test/a%2Fb?q=%7E https://bad.test/%ZZ https://bad.test/% https://bad.test/a|b https://bad.test/a[b https://bad.test/a]b\"\n",
    )
    .unwrap();

    let output = run(&input, &[]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        json["records"][0]["evidence_urls"],
        serde_json::json!([
            "https://[2001:db8::1]/proof",
            "https://xn--r8jz45g.xn--zckzah/%E9%9B%AA?q=%E2%9C%93",
            "https://valid.test/a%2Fb?q=%7E"
        ])
    );
}

#[test]
fn preserves_uri_punctuation_and_rejects_whatwg_rewrites() {
    let dir = TestDir::new();
    let input = dir.path().join("uri-edges.csv");
    fs::write(
        &input,
        "Title,Status,Evidence\nBug,Fail,\"(https://wrap.test/proof) [https://wrap.test/other] https://terminal.test/comma, https://terminal.test/semicolon; https://terminal.test/colon: https://terminal.test/bang! https://terminal.test/question? https://terminal.test/period. https://outer.test/go?next=https://inner.test/a; https://outer.test/go?next=https://inner.test/a? https://balanced.test/a(b) (https://en.wikipedia.org/wiki/Function_(mathematics)) https://bad.test/a#one#two https://bad.test\\rewritten\"\n",
    )
    .unwrap();

    let output = run(&input, &[]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(
        json["records"][0]["evidence_urls"],
        serde_json::json!([
            "https://wrap.test/proof",
            "https://wrap.test/other",
            "https://terminal.test/comma,",
            "https://terminal.test/semicolon;",
            "https://terminal.test/colon:",
            "https://terminal.test/bang!",
            "https://terminal.test/question?",
            "https://terminal.test/period.",
            "https://outer.test/go?next=https://inner.test/a;",
            "https://outer.test/go?next=https://inner.test/a?",
            "https://balanced.test/a(b)",
            "https://en.wikipedia.org/wiki/Function_(mathematics)"
        ])
    );
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
fn xlsx_retains_blank_ids_skips_styled_empty_rows_and_renders_numeric_ids() {
    let dir = TestDir::new();
    let input = dir.path().join("ids.xlsx");
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    sheet.write_string(0, 0, "ID").unwrap();
    sheet.write_string(0, 1, "Title").unwrap();
    sheet.write_string(0, 2, "Status").unwrap();
    sheet.write_string(1, 1, "Blank ID bug").unwrap();
    sheet.write_string(1, 2, "Fail").unwrap();
    let styled = Format::new().set_background_color("#FFFF00");
    sheet.write_blank(2, 0, &styled).unwrap();
    sheet.write_blank(2, 1, &styled).unwrap();
    sheet.write_number(3, 0, 9_007_199_254_740_992.0).unwrap();
    sheet.write_string(3, 1, "Numeric ID").unwrap();
    sheet.write_string(3, 2, "Pass").unwrap();
    sheet.write_string(4, 0, "000007").unwrap();
    sheet.write_string(4, 1, "Text ID").unwrap();
    sheet.write_string(4, 2, "Pass").unwrap();
    workbook.save(&input).unwrap();

    let output = run(&input, &[]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["summary"]["total_records"], 3);
    assert!(json["records"][0]["id"].is_null());
    assert_eq!(json["records"][1]["id"], "9007199254740992");
    assert_eq!(json["records"][2]["id"], "000007");
}

#[test]
fn xlsx_visibility_is_lossless_by_default_and_explicit_when_filtered() {
    let dir = TestDir::new();
    let input = dir.path().join("unclassified.xlsx");
    let mut workbook = Workbook::new();
    let sheet = workbook.add_worksheet();
    for (column, header) in [
        "ID",
        "Title",
        "Preconditions",
        "Steps",
        "Test Data",
        "Expected",
        "Observed",
        "Status",
        "Notes",
    ]
    .iter()
    .enumerate()
    {
        sheet.write_string(0, column as u16, *header).unwrap();
    }
    for (column, value) in [
        "",
        "Search loses filters",
        "Signed in",
        "Search for an item",
        "winter boots",
        "Filters remain",
        "Filters reset",
        "",
        "",
    ]
    .iter()
    .enumerate()
    {
        sheet.write_string(1, column as u16, *value).unwrap();
    }
    sheet.write_string(2, 1, "Comment row").unwrap();
    sheet.write_string(2, 8, "not an independent case").unwrap();
    sheet.write_string(3, 1, "Filtered draft").unwrap();
    sheet.write_string(3, 3, "Review hidden record").unwrap();
    sheet.autofilter(0, 0, 3, 8).unwrap();
    sheet.set_row_hidden(3).unwrap();
    workbook.save(&input).unwrap();

    let output = run(&input, &[]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["source"]["row_visibility"], "all_rows");
    assert_eq!(json["summary"]["total_records"], 2);
    assert_eq!(json["summary"]["hidden_records_excluded"], 0);
    assert!(json["records"][0]["id"].is_null());
    assert!(json["records"][0]["status"].is_null());
    assert_eq!(json["records"][0]["title"], "Search loses filters");
    assert_eq!(json["records"][0]["steps_raw"], "Search for an item");
    assert_eq!(json["records"][0]["expected"], "Filters remain");
    assert_eq!(json["records"][0]["observed"], "Filters reset");
    assert_eq!(json["records"][1]["title"], "Filtered draft");

    let output = run(&input, &["--visible-only"]);
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let json: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(json["source"]["row_visibility"], "visible_only");
    assert_eq!(json["summary"]["total_records"], 1);
    assert_eq!(json["summary"]["hidden_records_excluded"], 1);
    assert_eq!(json["records"][0]["title"], "Search loses filters");
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

#[cfg(unix)]
#[test]
fn refuses_to_overwrite_a_hard_link_to_the_input() {
    let dir = TestDir::new();
    let input = dir.path().join("bugs.csv");
    let alias = dir.path().join("alias.json");
    let original = "Title,Status\nBug,Fail\n";
    fs::write(&input, original).unwrap();
    fs::hard_link(&input, &alias).unwrap();

    let output = run(&input, &["--output", alias.to_str().unwrap()]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("same file"));
    assert_eq!(fs::read_to_string(&input).unwrap(), original);
    assert_eq!(fs::read_to_string(&alias).unwrap(), original);
}

#[cfg(unix)]
#[test]
fn refuses_an_output_symlink_without_modifying_its_target() {
    use std::os::unix::fs::symlink;

    let dir = TestDir::new();
    let input = dir.path().join("bugs.csv");
    let target = dir.path().join("existing.json");
    let output_path = dir.path().join("output.json");
    fs::write(&input, "Title,Status\nBug,Fail\n").unwrap();
    fs::write(&target, "do not replace").unwrap();
    symlink(&target, &output_path).unwrap();

    let output = run(&input, &["--output", output_path.to_str().unwrap()]);
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("symlink"));
    assert_eq!(fs::read_to_string(&target).unwrap(), "do not replace");
    assert!(fs::symlink_metadata(&output_path)
        .unwrap()
        .file_type()
        .is_symlink());
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
