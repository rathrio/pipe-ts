use crate::Args;
use chrono::prelude::*;
use colored::*;
use lazy_static::lazy_static;
use regex::Captures;
use regex::Regex;
use std::borrow::Cow;
use std::fmt::Display;
use std::fs;
use std::io::{self, BufRead};

const DEFAULT_DT_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn filter_file(path: &str, args: &Args) -> Result<(), std::io::Error> {
    let contents = fs::read_to_string(path)?;
    for line in contents.lines() {
        println!("{}", filter_line(line, args))
    }

    Ok(())
}

pub fn filter_stdin(args: &Args) {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut line = String::new();
    let mut eof = false;

    while !eof {
        match handle.read_line(&mut line) {
            Ok(0) => eof = true,
            Ok(_) => {
                print!("{}", filter_line(&line, args));
                line.clear();
            }
            Err(_) => panic!("Could not read line"),
        }
    }
}

fn filter_line<'a>(line: &'a str, args: &'a Args) -> Cow<'a, str> {
    replace(line, args.utc, args.no_highlight)
}

fn parse_and_format<T: TimeZone>(ts: &str, tz: T) -> String
where
    T::Offset: Display,
{
    tz.datetime_from_str(ts, "%s")
        .unwrap()
        .format(DEFAULT_DT_FORMAT)
        .to_string()
}

fn ts_to_date(ts: &str, utc: bool) -> String {
    if utc {
        parse_and_format(ts, Utc)
    } else {
        parse_and_format(ts, Local)
    }
}

fn replace(input: &str, utc: bool, no_highlight: bool) -> Cow<str> {
    lazy_static! {
        static ref TS_RE: Regex = Regex::new(r"\b(?P<ts>1\d{9})(\d\d\d)?\b").unwrap();
    }

    TS_RE.replace_all(input, |caps: &Captures| {
        let output = ts_to_date(caps.name("ts").unwrap().as_str(), utc);
        if no_highlight {
            return output;
        }

        output.red().to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace() {
        let actual = replace("foo 1646764906847 bar", false, false);
        let expected = format!("{} {} {}", "foo", "2022-03-08 19:41:46".red(), "bar");
        assert_eq!(actual, expected);

        let actual = replace("foo 1646764906 bar", false, false);
        assert_eq!(actual, expected);

        let actual = replace("foo 1646764906847 bar", true, false);
        let expected = format!("{} {} {}", "foo", "2022-03-08 18:41:46".red(), "bar");
        assert_eq!(actual, expected);
    }
}
