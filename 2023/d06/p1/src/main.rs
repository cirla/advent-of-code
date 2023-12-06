use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{anyhow, Context, Result};

#[derive(Debug)]
struct Race {
    /// Race time
    time: u32,
    /// Record distance
    distance: u32,
}

impl Race {
    pub fn outcomes<'a>(&'a self) -> impl Iterator<Item = Outcome> + 'a {
        // ignore hold times that always result in 0 distance
        (1..self.time).map(|hold_time| Outcome {
            _hold_time: hold_time,
            distance: hold_time * (self.time - hold_time),
        })
    }
}

#[derive(Debug)]
struct Outcome {
    /// Button hold time
    _hold_time: u32,
    /// Boat distance traveled
    distance: u32,
}

impl Outcome {
    pub fn is_winner(&self, race: &Race) -> bool {
        self.distance > race.distance
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let mut lines = reader.lines().filter_map(|line| line.ok());

    let times: Vec<u32> = lines
        .next()
        .ok_or(anyhow!("Missing times"))?
        .strip_prefix("Time:")
        .ok_or(anyhow!("Missing Time: prefix"))?
        .split_whitespace()
        .filter_map(|t| (!t.is_empty()).then(|| t.parse().context("Invalid time")))
        .collect::<Result<_>>()?;

    let distances: Vec<u32> = lines
        .next()
        .ok_or(anyhow!("Missing distances"))?
        .strip_prefix("Distance:")
        .ok_or(anyhow!("Missing Distance: prefix"))?
        .split_whitespace()
        .filter_map(|t| (!t.is_empty()).then(|| t.parse().context("Invalid distance")))
        .collect::<Result<_>>()?;

    let ways_to_win: usize = times
        .into_iter()
        .zip(distances.into_iter())
        .map(|(time, distance)| Race { time, distance })
        .map(|r| r.outcomes().filter(|o| o.is_winner(&r)).count())
        .product();

    println!("Ways to win: {ways_to_win}");

    Ok(())
}
