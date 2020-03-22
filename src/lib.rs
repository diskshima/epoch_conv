use std::io;
use chrono::{DateTime, FixedOffset};

pub fn get_input() -> String {
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    input.trim().to_owned()
}

pub fn parse_datetime(input: &str) -> DateTime<FixedOffset> {
    DateTime::parse_from_str(input, "%Y-%m-%d %H:%M:%S %z")
        .expect("Failed to parse date time")
}

pub fn to_epoch(dt: DateTime<FixedOffset>) -> i64 {
    dt.timestamp()
}

pub fn run() {
    let input = get_input();

    let dt = parse_datetime(input.as_str());

    let epoch = to_epoch(dt);

    let epoch_ms = epoch * 1_000;

    println!("{}", epoch_ms);
}
