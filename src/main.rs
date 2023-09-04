use chrono::{DateTime, FixedOffset, Utc};
use regex::Regex;
use std::env;
use std::fs;

fn get_time_utc(date_str: String) -> DateTime<Utc> {
    let date: Result<DateTime<FixedOffset>, chrono::ParseError> =
        DateTime::parse_from_str(&date_str, "%Y-%b-%d %H:%M:%S %z"); // Date format: 2023-Jan-01 01:00:00 +0000
    let utc_date: DateTime<Utc> = date.unwrap().with_timezone(&Utc);
    print!("Local: {} \t", date.unwrap());
    return utc_date;
}

fn read_file_details(contents: String, tz_offset: &String) {
    let lines: Vec<&str> = contents.lines().collect();
    for line in lines.iter() {
        let columns: Vec<&str> = line.split_whitespace().take(3).collect();
        let date_str: String = format!(
            "2023-{}-{} {} {}",
            columns[0], columns[1], columns[2], tz_offset
        );
        let utc_date: DateTime<Utc> = get_time_utc(date_str);
        print!("UTC: {}", utc_date);
        println!();
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Usage: <filename> <timezone offset in +-05:00 format>");
        return;
    }
    let filename: &String = &args[1];
    let tz_offset: &String = &args[2].to_string();
    let tz_pattern: Regex = Regex::new(r"^[+-](0[0-9]|1[0-4]):(00|30|45)$").unwrap(); // Pattern of timezone offset, should be +-HH:MM format
    if tz_pattern.is_match(tz_offset) == false {
        println!("Invalid timezone offset format: {}", tz_offset);
        return;
    }
    match fs::read_to_string(filename) {
        Ok(contents) => {
            read_file_details(contents, tz_offset);
        }
        Err(err) => {
            eprintln!("Error reading {}: {}", filename, err);
        }
    }
}
