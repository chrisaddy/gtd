use std::io::{BufReader, prelude::*};
use std::fs::File;

pub fn print_inbox() -> std::io::Result<()> {
    let home: std::path::PathBuf = dirs::home_dir()
        .expect("Failed to get home directory");

    let home_dir = home.display();

    let file = File::open(format!("{}/.gtd/inbox", home_dir))?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }
    Ok(())
}
