use clap::Parser;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use tabular_to_json::{convert, Options};
use tempfile::Builder;

#[derive(Debug, Parser)]
#[command(
    version,
    about = "Convert QA CSV/XLSX files to deterministic qa-import JSON"
)]
struct Cli {
    /// Input .csv or .xlsx file
    input: PathBuf,

    /// Import only this XLSX sheet (all detectable sheets by default)
    #[arg(long)]
    sheet: Option<String>,

    /// 1-based header row (auto-detected by default)
    #[arg(long)]
    header: Option<usize>,

    /// Write JSON to this file instead of stdout
    #[arg(short, long)]
    output: Option<PathBuf>,

    /// Emit compact JSON instead of pretty JSON
    #[arg(long)]
    compact: bool,
}

fn main() {
    if let Err(error) = run(Cli::parse()) {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn run(cli: Cli) -> Result<(), String> {
    let document = convert(
        &cli.input,
        Options {
            sheet: cli.sheet.as_deref(),
            header_row: cli.header,
        },
    )?;
    let mut json = if cli.compact {
        serde_json::to_string(&document)
    } else {
        serde_json::to_string_pretty(&document)
    }
    .map_err(|error| format!("could not serialize JSON: {error}"))?;
    json.push('\n');

    if let Some(path) = cli.output {
        write_output_atomically(&cli.input, &path, json.as_bytes())?;
    } else {
        io::stdout()
            .write_all(json.as_bytes())
            .map_err(|error| format!("could not write stdout: {error}"))?;
    }
    Ok(())
}

fn write_output_atomically(input: &Path, output: &Path, contents: &[u8]) -> Result<(), String> {
    let input_metadata = fs::metadata(input)
        .map_err(|error| format!("could not inspect input '{}': {error}", input.display()))?;
    check_output_path(input, &input_metadata, output)?;

    let parent = output
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));
    let mut temporary = Builder::new()
        .prefix(".tabular-to-json-")
        .tempfile_in(parent)
        .map_err(|error| {
            format!(
                "could not create temporary output in '{}': {error}",
                parent.display()
            )
        })?;
    temporary
        .write_all(contents)
        .and_then(|()| temporary.as_file().sync_all())
        .map_err(|error| format!("could not write output '{}': {error}", output.display()))?;

    // Recheck immediately before the atomic rename. If the path changes after
    // this point, rename replaces the directory entry rather than following it.
    check_output_path(input, &input_metadata, output)?;
    temporary.persist(output).map_err(|error| {
        format!(
            "could not persist output '{}': {}",
            output.display(),
            error.error
        )
    })?;
    Ok(())
}

fn check_output_path(
    input: &Path,
    input_metadata: &fs::Metadata,
    output: &Path,
) -> Result<(), String> {
    let output_metadata = match fs::symlink_metadata(output) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(()),
        Err(error) => {
            return Err(format!(
                "could not inspect output '{}': {error}",
                output.display()
            ));
        }
    };
    if output_metadata.file_type().is_symlink() {
        return Err(format!(
            "output '{}' must not be a symlink",
            output.display()
        ));
    }
    if same_file(input_metadata, &output_metadata)
        || canonical_paths_match(input, output).unwrap_or(false)
    {
        return Err("output must not be the same file as input".to_string());
    }
    Ok(())
}

#[cfg(unix)]
fn same_file(left: &fs::Metadata, right: &fs::Metadata) -> bool {
    use std::os::unix::fs::MetadataExt;

    left.dev() == right.dev() && left.ino() == right.ino()
}

#[cfg(not(unix))]
fn same_file(_left: &fs::Metadata, _right: &fs::Metadata) -> bool {
    false
}

fn canonical_paths_match(left: &Path, right: &Path) -> io::Result<bool> {
    Ok(fs::canonicalize(left)? == fs::canonicalize(right)?)
}
