use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

use regex::{Captures, Regex};

// trie regex - see create_regex.py
const RE_DIGITS: &str = r"(?:f(?:ive|our)|s(?:even|ix)|t(?:hree|wo)|eight|nine|one)";

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let digit_map: HashMap<&'static str, &'static str> = HashMap::from_iter(
        [
            ("one", "1"),
            ("two", "2"),
            ("three", "3"),
            ("four", "4"),
            ("five", "5"),
            ("six", "6"),
            ("seven", "7"),
            ("eight", "8"),
            ("nine", "9"),
        ]
        .into_iter(),
    );

    let re = Regex::new(RE_DIGITS).unwrap();

    let calibration_value: u32 = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
            // TODO: just replace the first and last
            // this doesn't replace overlaps so there may be overlapping
            // digit words masking the last digit
            let line = re.replace_all(line.as_ref(), |caps: &Captures| digit_map[&caps[0]]);

            let mut digits = line
                .chars()
                .filter_map(|c| c.is_digit(10).then_some(c as u32 - '0' as u32));

            digits
                .next()
                .and_then(|first| Some((first * 10) + digits.last().unwrap_or(first)))
        })
        .sum();

    println!("Calibration Value: {}", calibration_value);

    Ok(())
}
