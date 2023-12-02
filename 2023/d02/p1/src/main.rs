use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};

const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<Draw>,
}

impl Game {
    pub fn is_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        for draw in &self.draws {
            if draw.red > red || draw.green > green || draw.blue > blue {
                return false;
            }
        }
        true
    }
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s
            .strip_prefix("Game ")
            .ok_or(anyhow!("Missing 'Game ' prefix"))?
            .split(": ");

        let id = parts
            .next()
            .ok_or(anyhow!("Missing id"))?
            .parse()
            .context("Invalid id")?;

        let draws = parts
            .next()
            .ok_or(anyhow!("Missing draws"))?
            .split("; ")
            .map(|d| d.parse().context("Invalid draw"))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Game { id, draws })
    }
}

#[derive(Debug, Default)]
struct Draw {
    red: usize,
    green: usize,
    blue: usize,
}

impl FromStr for Draw {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split(", ").try_fold(Draw::default(), |draw, color_str| {
            let mut parts = color_str.split(" ");

            let val = parts
                .next()
                .ok_or(anyhow!("Missing color value"))?
                .parse::<usize>()
                .context("Invalid color value")?;

            let color = parts.next().ok_or(anyhow!("Missing color"))?;

            match color {
                "red" => Ok(Draw { red: val, ..draw }),
                "green" => Ok(Draw { green: val, ..draw }),
                "blue" => Ok(Draw { blue: val, ..draw }),
                _ => Err(anyhow!("Invalid color: {}", color)),
            }
        })
    }
}

fn main() -> Result<(), Error> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let games = reader
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .map(|(i, line)| {
            line.parse()
                .with_context(|| format!("Couldn't parse Game on line {}", i + 1))
        })
        .collect::<Result<Vec<Game>, _>>()?;

    let sum: usize = games
        .iter()
        .filter_map(|g| g.is_possible(MAX_RED, MAX_GREEN, MAX_BLUE).then_some(g.id))
        .sum();

    println!("Sum: {}", sum);

    Ok(())
}
