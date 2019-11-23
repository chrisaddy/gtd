use std::env;
use colored::*;

mod help;
mod init;
mod capture;
mod inbox;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("{}", help::MESSAGE);

        return
    }

    let command = &args[1];

    if command == "inbox" {
        inbox::print_inbox()
            .expect("Failed to open inbox");
        return
    }

    match command.as_ref() {
        "help" => println!("{}", help::MESSAGE),
        "init" => init::setup(),
        "capture" => capture::input(),
        _ => println!("{} {} {}\n\n{}", "command".red().bold(), &command.green(), "not recognized".red().bold(), help::MESSAGE)
    }
}
