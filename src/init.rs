use std::fs;
use colored::*;

extern crate dirs;

pub fn setup() {
    let home: std::path::PathBuf = dirs::home_dir()
        .expect("Failed to get home directory");

    let home_dir = home.display();

    fs::create_dir(format!("{}/.gtd", home_dir))
        .ok()
        .expect("Failed to initialize directory");

    fs::write(
        format!("{}/.gtd/config", home_dir),
        format!("dir = {}/.gtd", home_dir)
    )
    .expect("Failed to create config file");

    fs::write(
        format!("{}/.gtd/inbox", home_dir),
        "captureId,timestamp,text\n"
        )
        .expect("Failed to create inbox");

    println!("{}", "GTD Initialized".green())
}
