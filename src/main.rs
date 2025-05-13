use clap::{Parser};
use rand::Rng;
use serde_json::{json, Value};
use std::{fs::OpenOptions, io::Write};
use chrono::Local;

#[derive(Parser, Debug)]
#[command(name = "log_generator")]
#[command(version = "1.0")]
#[command(author = "Pavlos Rontidis")]
#[command(about = "Generates random logs in JSON format.")]
struct Args {
    /// The number of logs to generate
    #[arg(short, long, default_value_t = 1000)]
    num_logs: usize,

    /// The file path to output the logs
    #[arg(short, long, default_value = "/var/log/a_custom.log")]
    output: String,
}

fn generate_log() -> Value {
    // Randomly generate a log entry
    let log_level = match rand::rng().random_range(0..3) {
        0 => "INFO",
        1 => "ERROR",
        _ => "WARN",
    };

    json!({
        "timestamp": Local::now().to_rfc3339(),
        "level": log_level,
        "message": "This is a random log message",
        "source": "log_generator"
    })
}

fn write_logs(file_path: &str, num_logs: usize) -> Result<(), std::io::Error> {
    // Open or create the log file
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(file_path)?;

    // Generate and write logs
    for _ in 0..num_logs {
        let log = generate_log();
        writeln!(file, "{}", log.to_string())?;
    }

    Ok(())
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Generate and write the logs
    if let Err(e) = write_logs(&args.output, args.num_logs) {
        eprintln!("Error writing logs: {}", e);
        std::process::exit(1);
    }

    println!("Generated {} logs to {}", args.num_logs, args.output);
}
