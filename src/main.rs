use chrono::prelude::*;
use std::env;
use std::fs::File;
use std::fs::OpenOptions;
use std::io;
use std::path::Path;

// extern crate directories;
// use directories::BaseDirs;

#[derive(Debug, PartialEq)]
enum PunchCmd {
    Mark,
    In,
    Out,
}

type MainError = String;

impl std::str::FromStr for PunchCmd {
    type Err = MainError;
    fn from_str(cmd: &str) -> Result<Self, Self::Err> {
        match cmd.to_lowercase().as_ref() {
            "mark" => Ok(PunchCmd::Mark),
            "in" => Ok(PunchCmd::In),
            "out" => Ok(PunchCmd::Out),
            _ => Err(format!("Could not parse: {}", cmd)),
        }
    }
}

mod chrono_ext {
    use chrono::format::ParseError;

    // round-the-barn hack to convert chrono::format::ParseError to std::string::String
    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub struct TimeParseError(ParseError);

    impl From<ParseError> for TimeParseError {
        fn from(pe: ParseError) -> Self {
            TimeParseError(pe)
        }
    }

    impl From<TimeParseError> for String {
        fn from(e: TimeParseError) -> Self {
            format!("could not parse time: {}", e.0)
        }
    }
}

use chrono_ext::TimeParseError;

type DateTime = chrono::DateTime<Local>;
fn current_time() -> DateTime {
    chrono::Local::now()
}

fn parse_time(time_string: String) -> Result<DateTime, TimeParseError> {
    let time_str = time_string.as_ref();
    match time_str {
        "now" => Ok(current_time()),
        _ => {
            let tz = Local::now().timezone();
            let dt = tz.datetime_from_str(time_str, "%Y-%m-%dT%H:%M:%S")?;
            Ok(dt)
        }
    }
}

#[allow(dead_code)]
fn open_punchfile<P: AsRef<Path>>(path: P) -> io::Result<File> {
    OpenOptions::new()
        .create(true)
        .append(true)
        .read(true)
        .write(true)
        .open(path.as_ref())
}

fn main() -> Result<(), MainError> {
    let command = env::args().nth(1).expect("command missing");

    let time_arg = match env::args().nth(2) {
        None => current_time(),
        Some(dt) => parse_time(dt)?
    };

    println!("command: {}", command);
    println!("time_arg: {}", time_arg.to_rfc3339());

    Ok(())
}
