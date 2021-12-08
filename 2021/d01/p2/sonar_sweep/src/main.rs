use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);
    let window_size: usize = args[2].parse()?;

    let readings: Vec<u32> = reader
        .lines()
        .filter_map(|line| line.ok().and_then(|x| x.parse::<u32>().ok()))
        .collect();

    let mut increase_count = 0;
    let mut prev_sum = u32::MAX;

    for window in (&readings[..]).windows(window_size) {
        let window_sum: u32 = window.iter().sum();
        if window_sum > prev_sum {
            increase_count += 1;
        }
        prev_sum = window_sum;
    }

    println!("{}", increase_count);

    Ok(())
}
