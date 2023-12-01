use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

use regex::{Match, Regex};

const DIGITS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const DIGITS_REV: [&str; 9] = [
    "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

// trie regex - see create_regex.py
// matches any numeric or word digit efficiently
const RE_DIGITS: &str = r"(?:f(?:ive|our)|s(?:even|ix)|t(?:hree|wo)|eight|nine|one|\d)";
const RE_DIGITS_REV: &str = r"(?:e(?:n(?:in|o)|erht|vif)|neves|thgie|ruof|owt|xis|\d)";

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let re_digits = Regex::new(RE_DIGITS).unwrap();
    let re_digits_rev = Regex::new(RE_DIGITS_REV).unwrap();

    // map digit word to digit value, e.g. "two" -> 2
    let mut digit_map: HashMap<&'static str, usize> =
        HashMap::from_iter(DIGITS.iter().enumerate().map(|(i, d)| (*d, i + 1)));

    // also map reversed word to digit value, e.g. "owt" -> 2
    digit_map.extend(DIGITS_REV.iter().enumerate().map(|(i, d)| (*d, i + 1)));

    // convert regex match to usize
    let to_digit = |m: Match| match m.len() {
        1 => m.as_str().parse::<usize>().unwrap(),
        _ => digit_map[m.as_str()],
    };

    let calibration_value: usize = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            re_digits.find(&line).map(to_digit).and_then(|first| {
                let rev = &line.chars().rev().collect::<String>();
                let last = re_digits_rev.find(&rev).map(to_digit).unwrap_or(first);
                Some(first * 10 + last)
            })
        })
        .sum();

    println!("Calibration Value: {}", calibration_value);

    Ok(())
}
