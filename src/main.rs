use std::env;
use std::fs;
use chrono::{DateTime, FixedOffset, Utc};



fn read_file_details(contents: String, tz_offset: &String) {
    let lines: Vec<&str> = contents.lines().collect();
    for (line_number, line) in lines.iter().enumerate() {
        let columns: Vec<&str> = line.split_whitespace().take(3).collect();
        let date_str: String = format!("2023-{}-{} {} {}", columns[0], columns[1], columns[2], tz_offset);
        let date: Result<DateTime<FixedOffset>, chrono::ParseError> = DateTime::parse_from_str(&date_str, "%Y-%b-%d %H:%M:%S %z");
        let utc_date: DateTime<Utc> = date.unwrap().with_timezone(&Utc);
        println!("Local: {} \t UTC: {}", date.unwrap(), utc_date);
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <filename>", args[0]);
        return;
    }
    let filename: &String = &args[1];
    let tz_offset: &String = &args[2].to_string();

    match fs::read_to_string(filename) {
        Ok(contents) => {
            read_file_details(contents, tz_offset);
        }
        Err(err) => {
            eprintln!("Error reading {}: {}", filename, err);
        }
    }
}

