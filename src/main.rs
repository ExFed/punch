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

type DateTime = chrono::DateTime<Utc>;
fn current_time() -> Option<DateTime> {
    Option::Some(chrono::Utc::now())
}

fn parse_time(s: &str) -> Option<DateTime> {
    match s {
        "now" => chrono::Utc::now(),
        _ => {
            let parse = chrono::NaiveDateTime::parse_from_str;
            let ndt = parse(s, "%Y-%m-%d %H:%M:%S").expect("could not parse date");
            chrono::DateTime::from_utc(ndt, Utc)
        }
    }
}

fn main() -> Result<(), MainError> {
    let command = env::args().nth(1).expect("command missing");
    let at_time: DateTime = env::args().nth(2)
        .and_then(parse_time)
        .or_else(|| Option::Some(chrono::Utc::now())).unwrap();

    Ok(())
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
