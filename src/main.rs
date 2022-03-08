use chrono::prelude::*;
use colored::*;
use lazy_static::lazy_static;
use regex::Captures;
use regex::Regex;
use std::borrow::Cow;
use std::io::Read;
use std::io::{self, BufRead};

const DEFAULT_DT_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

fn naive() {
    for line in io::stdin().lock().lines() {
        let line = line.expect("Could not read line");
        println!("{}", replace(&line));
    }
}

fn fewer_allocations() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();
    let mut eof = false;

    while !eof {
        match handle.read_line(&mut line) {
            Ok(0) => eof = true,
            Ok(_) => {
                print!("{}", replace(&line));
                line.clear();
            }
            Err(_) => panic!("Could not read line"),
        }
    }
}

fn read_all() {
    let mut contents = String::new();
    io::stdin().lock().read_to_string(&mut contents).unwrap();
    print!("{}", replace(&contents));
}

fn main() {
    fewer_allocations();
}

fn ts_to_date(ts: &str) -> String {
    let dt = Local.datetime_from_str(&ts, "%s").unwrap();
    dt.format(DEFAULT_DT_FORMAT).to_string()
}

fn replace(input: &str) -> Cow<str> {
    lazy_static! {
        static ref TS_RE: Regex = Regex::new(r"\b(?P<ts>1\d{9})(\d\d\d)?\b").unwrap();
    }

    let output = TS_RE.replace_all(input, |caps: &Captures| {
        ts_to_date(caps.name("ts").unwrap().as_str())
            .red()
            .to_string()
    });

    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foobar() {
        let actual = replace("foo 1646764906847 bar");
        let expected = format!("{} {} {}", "foo", "2022-03-08 19:41:46".red(), "bar");
        assert_eq!(actual, expected);

        let actual = replace("foo 1646764906 bar");
        assert_eq!(actual, expected);
    }
}
