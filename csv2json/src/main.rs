extern crate csv;
extern crate serde;
extern crate serde_json;

use serde_derive::Serialize;
use std::error::Error;
use std::fs::File;
use serde_json::to_writer_pretty;

#[derive(Debug, Serialize)]
struct Record {
    // Define the structure of your CSV records here
    // For example: 
    name: String,
    age: i32,
    email: String,
    // ...
}

fn read_csv(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
    let mut reader = csv::Reader::from_path(file_path)?;
    let mut records = Vec::new();

    for result in reader.records() {
        let record = result?;
        // Parse the CSV record into your defined struct
        let parsed_record = Record {
            // Initialize the fields here
            // For example: 
            name: record.get(0).unwrap().to_string(),
            age: record.get(1).unwrap().parse().unwrap(),
            email: record.get(2).unwrap().to_string(),
            // ...
        };
        records.push(parsed_record);
    }

    Ok(records)
}

fn write_json(records: &[Record], output_file: &str) -> Result<(), Box<dyn Error>> {
    let json_file = File::create(output_file)?;
    to_writer_pretty(json_file, records)?;

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let csv_file = "../resources/source.csv";  // Replace with your CSV file path
    let json_file = "../resources/target.json"; // Replace with the desired JSON output file path

    let records = read_csv(csv_file)?;
    write_json(&records, json_file)?;

    println!("Conversion successful!");

    Ok(())
}

