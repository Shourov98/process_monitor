use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::BufReader;
use std::fs::write;
use serde_json;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

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

fn process_monitors(monitor_data: Arc<Mutex<MonitorData>>, output_file: String) {
    let end_time = chrono::Utc::now() + chrono::Duration::try_minutes(5).unwrap();
    let output_file_cloned = output_file.clone();
    
    // Clone monitor_data before moving into the closure
    let monitor_data_clone = monitor_data.clone();
    
    // Run the update_monitors() and store_monitors() in parallel
    let update_handle = thread::spawn(move || update_monitors(monitor_data_clone, end_time));
    let store_handle = thread::spawn(move || store_monitors(monitor_data, end_time, output_file_cloned));

    // Wait for the threads to finish
    update_handle.join().unwrap();
    store_handle.join().unwrap();
}


fn update_monitors(monitor_data: Arc<Mutex<MonitorData>>, end_time: chrono::DateTime<chrono::Utc>) {
    while chrono::Utc::now() < end_time {
        // Lock the mutex to access monitor_data
        let mut data = monitor_data.lock().unwrap();
        
        // Process and update monitor data
        for monitor in &mut data.monitors {
            let random_value = rand::random::<i32>();
            let current_timestamp = chrono::Utc::now().timestamp();
            let result = Result {
                value: random_value,
                processed_at: current_timestamp,
            };
            monitor.result = Some(result);
        }
    }
}

fn store_monitors(monitor_data: Arc<Mutex<MonitorData>>, end_time: chrono::DateTime<chrono::Utc>, _output_file: String) {
    let mut current_time = chrono::Utc::now();
    while current_time < end_time {
        // Lock the mutex to access monitor_data
        let mut data = monitor_data.lock().unwrap();

        // Process and update monitor data
        for monitor in &mut data.monitors {
            let random_value = rand::random::<i32>();
            let current_timestamp = chrono::Utc::now().timestamp();
            let result = Result {
                value: random_value,
                processed_at: current_timestamp,
            };
            monitor.result = Some(result);
        }
        
        // Convert MonitorData to JSON
        let json_data = serde_json::to_string_pretty(&*data).expect("Failed to convert to JSON");

        // Generate filename based on current time
        let filename = format!("{}_monitors.json", current_time.timestamp());

        // Write JSON data to file
        write(&filename, json_data).expect("Failed to write to output file");

        // Wait for 1 minute before storing again
        thread::sleep(Duration::from_secs(60));
        current_time = chrono::Utc::now();
    }
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
    let monitor_data: MonitorData = serde_json::from_reader(reader).expect("Error parsing JSON data");
    let arc_monitor_data = Arc::new(Mutex::new(monitor_data));

    // Extract the value of the "outputFile" argument
    let output_file = matches.value_of("outputFile").unwrap().to_string();

    // Process and update monitors
    process_monitors(arc_monitor_data, output_file);
}
