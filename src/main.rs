use chrono::Datelike;
use chrono::{DateTime, FixedOffset, Utc};
use colored::Colorize;
use rayon::prelude::*;
use regex::Regex;
use std::env;
use std::fs;
use std::process;
// Todo:
//      - Remove hardcoded year...
//      - Some files have different time formats
//      - When converting to UTC (function get_time_utc), it could error out due to wrong date format, need to add error handling

fn get_time_utc(date_str: String) -> DateTime<Utc> {
    let date: Result<DateTime<FixedOffset>, chrono::ParseError> =
        DateTime::parse_from_str(&date_str, "%Y-%b-%d %H:%M:%S %z"); // Date format: 2023-Jan-01 01:00:00 +0000
    match date {
        Ok(_) => {
            let utc_date: DateTime<Utc> = date.unwrap().with_timezone(&Utc); // this might error if file contains incorrect dates, need to fix
            return utc_date;
        }
        Err(err) => {
            eprintln!("Error parsing date: {}", err.to_string().red());
            eprintln!("Date string should be in 'Month Day HH:MM:SS' format'");
            process::exit(1);
        }
    }
}

fn print_usage() {
    println!("Usage: <filename> <timezone offset in +-06:00 format>");
    println!("Converts log time entries to UTC time");
    println!("Example: ./convert_time messages -06:00");
}

fn read_file_details(contents: String, tz_offset: &String) -> Vec<String> {
    let lines: Vec<&str> = contents.lines().collect();
    let now: DateTime<Utc> = Utc::now();
    let current_year = now.year();

    let data: Vec<String> = lines
        .par_iter()
        .map(|line| {
            let columns: Vec<&str> = line.split_whitespace().take(3).collect();
            let date_str: String = format!(
                "{}-{}-{} {} {}", // todo, don't hardcode year...
                current_year, columns[0], columns[1], columns[2], tz_offset
            );
            let utc_date: DateTime<Utc> = get_time_utc(date_str);
            let utc_date_str: String = utc_date.to_string();
            let data_string: String = line
                .split_whitespace()
                .skip(3)
                .collect::<Vec<&str>>()
                .join(" ");
            format!("{} {}", utc_date_str, data_string)
        })
        .collect();
    return data;
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match (args.get(1).map(|s: &String| s.as_str()), args.len()) {
        (None, 1) => {
            println!("No arguments provided");
            print_usage();
            return;
        }
        (None, 2) => {
            println!("No arguments provided");
            print_usage();
            return;
        }
        (Some("-h") | Some("--help"), 2) => {
            print_usage();
            return;
        }
        (_, len) if len < 3 => {
            println!("Too few arguments provided, utility requires at least 2 arguments");
            print_usage();
            return;
        }
        (_, len) if len > 3 => {
            println!("Too many arguments provided, utility requires maximum 2 arguments");
            print_usage();
            return;
        }
        _ => {}
    }

    let filename: &String = &args[1];
    let tz_offset: &String = &args[2].to_string();
    let tz_pattern: Regex = Regex::new(r"^[+-](0[0-9]|1[0-4]):(00|30|45)$").unwrap(); // Pattern of timezone offset, should be +-HH:MM format
    if tz_pattern.is_match(tz_offset) == false {
        println!("Invalid timezone offset format: {}", tz_offset.red());
        println!("Correct format should be {}", "+-HH:MM".green());
        return;
    }
    match fs::read_to_string(filename) {
        // main logic
        Ok(contents) => {
            let converted_data: Vec<String> = read_file_details(contents, tz_offset);
            let file_name: String = format!("{}-chronox", filename);
            std::fs::write(file_name.clone(), converted_data.join("\n"))
                .expect("Failed to write the resulting file.");
            println!("File saved as {}", file_name.blue());
            println!("Original file has been kept");
        }
        Err(err) => {
            eprintln!("Error reading {}: {}", filename, err.to_string().red());
        }
    }
}
