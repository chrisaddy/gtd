use std::io;
use uuid::Uuid;
use colored::*;
use std::io::prelude::*;
use chrono::{Datelike, Utc};
use std::fs::OpenOptions;

extern crate dirs;


pub fn input() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read capture");

    let now = Utc::now();
    let captured = input.trim().to_string();

    let capture_id = Uuid::new_v4();

    let home: std::path::PathBuf = dirs::home_dir()
        .expect("Failed to locate home directory");

    let home_dir = home.display();

    let mut file = OpenOptions::new()
        .append(true)
        .open(format!("{}/.gtd/inbox", home_dir))
        .unwrap();

    writeln!(&mut file, "{}\t{}\t{}", capture_id, now, captured)
        .expect("Failed to capture");

    println!("{}", "Input captured successfully".yellow());
    println!("{}: {}", "CAPTURE ID".green(), capture_id);
    println!("{}: {}/{}/{}", "      DATE".green(), now.month(), now.day(), now.year());
    println!("{}: {}", "     INPUT".green(), captured);
}
