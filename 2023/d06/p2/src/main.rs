use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{anyhow, Context, Result};

#[derive(Debug)]
struct Race {
    /// Race time
    time: usize,
    /// Record distance
    distance: usize,
}

impl Race {
    pub fn ways_to_win(&self) -> usize {
        let lower_bound = (1..self.time)
            .skip_while(|hold_time| hold_time * (self.time - hold_time) <= self.distance)
            .next()
            .unwrap_or(0);

        if lower_bound == 0 {
            0
        } else {
            let upper_bound = (1..self.time)
                .rev()
                .skip_while(|hold_time| hold_time * (self.time - hold_time) <= self.distance)
                .next()
                .unwrap_or(0);

            upper_bound - lower_bound + 1
        }
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let mut lines = reader.lines().filter_map(|line| line.ok());

    let time: usize = lines
        .next()
        .ok_or(anyhow!("Missing time"))?
        .strip_prefix("Time:")
        .ok_or(anyhow!("Missing Time: prefix"))?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .context("Invalid time")?;

    let distance: usize = lines
        .next()
        .ok_or(anyhow!("Missing distance"))?
        .strip_prefix("Distance:")
        .ok_or(anyhow!("Missing Distance: prefix"))?
        .chars()
        .filter(|c| !c.is_whitespace())
        .collect::<String>()
        .parse()
        .context("Invalid distance")?;

    let race = Race { time, distance };

    println!("Ways to win: {}", race.ways_to_win());

    Ok(())
}
