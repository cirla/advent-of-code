use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let mut increase_count = 0;
    let mut prev_reading = u32::MAX;

    for reading in reader
        .lines()
        .filter_map(|line| line.ok().and_then(|x| x.parse::<u32>().ok()))
    {
        if reading > prev_reading {
            increase_count += 1;
        }
        prev_reading = reading;
    }

    println!("{}", increase_count);

    Ok(())
}
