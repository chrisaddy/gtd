use std::error::Error;
use csv::Reader;
use std::io::{BufReader, prelude::*};
use std::fs::File;
use serde::Deserialize;


#[derive(Debug, Deserialize, Eq, PartialEq)]
struct Row {
    city: String,
    country: String,
    #[serde(rename = "popcount")]
    population: u64,
}

fn read_inbox() -> Result<(), Box<dyn Error>> {
    let home: std::path::PathBuf = dirs::home_dir()
        .expect("Failed to get home directory");
    let home_dir = home.display();

    let mut rdr = Reader::from_path(format!("{}/.gtd/inbox", home_dir))?;
    for result in rdr.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}


