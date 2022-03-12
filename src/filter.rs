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
        println!("{}", replace(line, args))
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
                print!("{}", replace(&line, args));
                line.clear();
            }
            Err(_) => panic!("Could not read line"),
        }
    }
}

fn parse_and_format<T>(ts: &str, tz: T, args: &Args) -> String
where
    T: TimeZone,
    T::Offset: Display,
{
    let dt = tz.datetime_from_str(ts, "%s").unwrap();
    if args.rfc3339 {
        dt.to_rfc3339_opts(SecondsFormat::Secs, true)
    } else {
        dt.format(DEFAULT_DT_FORMAT).to_string()
    }
}

fn ts_to_date(ts: &str, args: &Args) -> String {
    if args.utc {
        parse_and_format(ts, Utc, args)
    } else {
        parse_and_format(ts, Local, args)
    }
}

fn replace<'a>(input: &'a str, args: &'a Args) -> Cow<'a, str> {
    lazy_static! {
        static ref TS_RE: Regex = Regex::new(r"\b(?P<ts>1\d{9})(\d\d\d)?\b").unwrap();
    }

    TS_RE.replace_all(input, |caps: &Captures| {
        let output = ts_to_date(caps.name("ts").unwrap().as_str(), args);
        if args.no_highlight {
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
        let mut args = Args::default();
        args.no_highlight = true;

        let actual = replace("foo 1646764906847 bar", &args);
        let expected = format!("{} {} {}", "foo", "2022-03-08 19:41:46", "bar");
        assert_eq!(actual, expected);

        let actual = replace("foo 1646764906 bar", &args);
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_replace_utc() {
        let mut args = Args::default();
        args.no_highlight = true;
        args.utc = true;

        let actual = replace("foo 1646764906847 bar", &args);
        let expected = format!("{} {} {}", "foo", "2022-03-08 18:41:46", "bar");
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_replace_rfc3339() {
        let mut args = Args::default();
        args.no_highlight = true;
        args.rfc3339 = true;

        let actual = replace("foo 1646764906847 bar", &args);
        let expected = format!("{} {} {}", "foo", "2022-03-08T19:41:46+01:00", "bar");
        assert_eq!(actual, expected);

        let mut args = args.clone();
        args.no_highlight = true;
        args.utc = true;
        args.rfc3339 = true;

        let actual = replace("foo 1646764906847 bar", &args);
        let expected = format!("{} {} {}", "foo", "2022-03-08T18:41:46Z", "bar");
        assert_eq!(actual, expected);
    }
}
