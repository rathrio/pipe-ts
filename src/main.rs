use chrono::prelude::*;
use colored::*;
use lazy_static::lazy_static;
use regex::Captures;
use regex::Regex;
use std::io::{self, BufRead};

const DEFAULT_DT_FORMAT: &str = "%Y-%m-%d %H:%M:%S";

fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.expect("Could not read line");
        println!("{}", replace(&line));
    }
}

fn ts_to_date(ts: &str) -> String {
    let dt = Local.datetime_from_str(&ts[..10], "%s").unwrap();
    dt.format(DEFAULT_DT_FORMAT).to_string()
}

fn replace(input: &str) -> String {
    lazy_static! {
        static ref MS_RE: Regex = Regex::new(r"\b(?P<ts>\d{10, 13})\b").unwrap();
    }

    let output = MS_RE
        .replace_all(input, |caps: &Captures| {
            ts_to_date(caps.name("ts").unwrap().as_str())
                .red()
                .to_string()
        })
        .to_string();
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
