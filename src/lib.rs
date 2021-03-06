use std::env;
use std::io;
use chrono::{DateTime, FixedOffset};

#[cfg(test)]
use chrono::offset::TimeZone;

pub struct Config {
    pub input: Option<String>,
}

impl Config {
    pub fn new(mut args: env::Args) -> Config {
        args.next();

        let input = args.next();

        Config { input }
    }
}

pub fn get_input(config: Config) -> String {
    match config.input {
        Some(input) => input,
        None => {
            let mut input = String::new();

            io::stdin().read_line(&mut input)
                .expect("Failed to read line");

            input.trim().to_owned()
        },
    }
}

pub fn parse_datetime(input: &str) -> DateTime<FixedOffset> {
    DateTime::parse_from_str(input, "%+").expect("Failed to parse date time")
}

pub fn to_epoch(dt: DateTime<FixedOffset>) -> i64 {
    dt.timestamp()
}

pub fn run() {
    let config = Config::new(env::args());

    let input = get_input(config);

    let dt = parse_datetime(input.as_str());

    let epoch = to_epoch(dt);

    let epoch_ms = epoch * 1_000;

    println!("{}", epoch_ms);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_datetime() {
        let date_str = "2020-03-22T23:40:00+09:00";
        let datetime = FixedOffset::east(9 * 3600).ymd(2020, 3, 22).and_hms(23, 40, 0);

        assert_eq!(datetime, parse_datetime(date_str));
    }
}
