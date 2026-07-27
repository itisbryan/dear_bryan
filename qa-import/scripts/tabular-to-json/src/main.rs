use clap::Parser;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use tabular_to_json::{convert, Options};

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
    if let Some(output) = &cli.output {
        let input_path = fs::canonicalize(&cli.input).map_err(|error| {
            format!("could not resolve input '{}': {error}", cli.input.display())
        })?;
        if output.exists()
            && fs::canonicalize(output).is_ok_and(|output_path| output_path == input_path)
        {
            return Err("output must not be the same file as input".to_string());
        }
    }
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
        fs::write(&path, json)
            .map_err(|error| format!("could not write output '{}': {error}", path.display()))?;
    } else {
        io::stdout()
            .write_all(json.as_bytes())
            .map_err(|error| format!("could not write stdout: {error}"))?;
    }
    Ok(())
}
