use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{anyhow, Result};

fn hash(s: &str) -> u32 {
    s.chars().fold(0, |acc, c| ((acc + (c as u32)) * 17) % 256)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let line = reader
        .lines()
        .filter_map(|line| line.ok())
        .next()
        .ok_or(anyhow!("Missing input"))?;

    let sum: u32 = line.split(',').map(hash).sum();

    println!("Sum of Results: {}", sum);

    Ok(())
}
