use charger_uptime::{parser::parse_input, uptime::compute_station_uptime};
use clap::Parser;
use std::{fs, process::ExitCode};

/// CLI for computing station uptimes from an input file
#[derive(Parser, Debug)]
#[command(
    name = "charger-uptime",
    version,
    about = "Compute station uptime from availability reports"
)]
struct Cli {
    /// Path to the input file
    input_path: String,
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    let input = match fs::read_to_string(&cli.input_path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("failed to read input: {e}");
            println!("ERROR");
            return ExitCode::SUCCESS;
        }
    };

    let (stations, reports) = match parse_input(&input) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("parse error: {e}");
            println!("ERROR");
            return ExitCode::SUCCESS;
        }
    };

    match compute_station_uptime(&stations, &reports) {
        Ok(mut results) => {
            results.sort_by_key(|(sid, _)| *sid);
            for (sid, pct) in results {
                println!("{} {}", sid.0, pct);
            }
            ExitCode::SUCCESS
        }
        Err(e) => {
            eprintln!("uptime error: {e}");
            println!("ERROR");
            ExitCode::SUCCESS
        }
    }
}
