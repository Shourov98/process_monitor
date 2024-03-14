use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::fs::write;
use serde_json;

// Struct representing a single monitor
#[derive(Debug, Deserialize, Serialize)]
struct Monitor {
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "monitor_id")]
    monitor_id: Option<i32>,
    name: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    script: Option<String>,
    #[serde(default)]
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    monitor_type: Option<String>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<Result>,
    code: String,
}

// Struct representing the 'Result' object
#[derive(Debug, Deserialize, Serialize)]
struct Result {
    value: i32,
    processed_at: i64,
}

// Struct representing the JSON data structure
#[derive(Debug, Deserialize, Serialize)]
struct MonitorData {
    monitors: Vec<Monitor>,
}

fn main() {
    // Define command-line arguments
    let matches = App::new("process_monitor")
        .arg(
            Arg::with_name("monitorFile")
                .long("monitorFile")
                .takes_value(true)
                .required(true)
                .help("Path to the monitors.json file"),
        )
        .arg(
            Arg::with_name("outputFile")
                .long("outputFile")
                .takes_value(true)
                .required(true)
                .help("Path to the output JSON file"),
        )
        .get_matches();

    // Extract the value of the "monitorFile" argument
    let monitor_file = matches.value_of("monitorFile").unwrap();

    // Open and read the JSON file
    let file = File::open(monitor_file).expect("Failed to open file");
    let reader = BufReader::new(file);

    // Parse JSON data
    let mut monitor_data: MonitorData = serde_json::from_reader(reader).expect("Error parsing JSON data");

    // Process and print monitor data
    for monitor in &mut monitor_data.monitors {
        // Generate random value
        let random_value = rand::random::<i32>();

        // Get current timestamp in seconds since Unix epoch
        let current_timestamp = chrono::Utc::now().timestamp();

        // Create Result instance with random value and current timestamp
        let result = Result {
            value: random_value,
            processed_at: current_timestamp,
        };

        // Update the result field of the monitor
        monitor.result = Some(result);
    }

    // Extract the value of the "outputFile" argument
    let output_file = matches.value_of("outputFile").unwrap();

    // Convert MonitorData back to JSON
    let json_data = serde_json::to_string_pretty(&monitor_data).expect("Failed to convert to JSON");

    // Write the JSON data to the output file
    write(output_file, json_data).expect("Failed to write to output file");
}
