use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let calibration_value: u32 = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter_map(|line| {
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
