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
    let project = find_project(&captured);
    let context = find_context(&captured);

    let capture_id = Uuid::new_v4();

    let home: std::path::PathBuf = dirs::home_dir()
        .expect("Failed to locate home directory");

    let home_dir = home.display();

    let mut file = OpenOptions::new()
        .append(true)
        .open(format!("{}/.gtd/inbox", home_dir))
        .unwrap();

    writeln!(&mut file, "{},{},{},{}", capture_id, now, captured, project)
        .expect("Failed to capture");

    println!("{}", "Input captured successfully".yellow());
    println!("{}: {}", "CAPTURE ID".green(), capture_id);
    println!("{}: {}/{}/{}", "      DATE".green(), now.month(), now.day(), now.year());
    println!("{}: {}", "     INPUT".green(), colorize_output(&captured));
    println!("{}: {}", "   PROJECT".green(), project);
    println!("{}: {}", "   CONTEXT".green(), context);
}


fn colorize_token(token: &str) -> std::string::String {
    if token.starts_with("#") {
        return format!("{}", token.green())
    }

    if token.starts_with("@") {
        return format!("{}", token.yellow())
    }

    token.to_string()
}

fn colorize_output(output: &str) -> std::string::String {
    let tokens: Vec<String> = output
        .split_whitespace()
        .map(|token| colorize_token(token))
        .collect();

    tokens.join(" ")
}


fn remove_first(s: &str) -> &str {
    let mut chars = s.chars();
    chars.next();
    chars.as_str()
}

fn find_context(text: &str) -> &str {
    let contexts: Vec<&str> = text
        .split_whitespace()
        .filter(|token| token.starts_with("#"))
        .map(|token| remove_first(token))
        .collect();

    if contexts.len() == 0 {
        return ""
    }

    contexts[0]
}

fn find_project(text: &str) -> &str {
    let projects: Vec<&str> = text
        .split_whitespace()
        .filter(|token| token.starts_with("@"))
        .map(|token| remove_first(token))
        .collect();

    if projects.len() == 0 {
        return ""
    }

    projects[0]
}


#[cfg(test)]
mod tests {
    use crate::capture::*;

    #[test]
    fn test_string_crop() {
        assert_eq!(remove_first("@project"), "project");
        assert_eq!(remove_first("#context"), "context");
        assert_eq!(remove_first("@#project"), "#project");
        assert_eq!(remove_first("@@project"), "@project");
    }

    #[test]
    fn test_project_parse() {
        assert_eq!(find_project("this is a @project with no context"), "project");
        let empty_string = String::new();
        assert_eq!(find_project("this has no project"), empty_string);
        assert_eq!(find_project("this @project has a @second_project"), "project");
    }

    #[test]
    fn test_context_parse() {
        assert_eq!(find_context("this is a @project with a #context"), "context");
        let empty_string = String::new();
        assert_eq!(find_context("this is a @project with no context"), empty_string);
        assert_eq!(find_context("this is a #project with #context"), "project");
    }
}
