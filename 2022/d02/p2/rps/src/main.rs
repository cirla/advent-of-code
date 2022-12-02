use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Outcome {
    fn score(&self) -> u32 {
        match *self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(format!("Invalid outcome input: {}", value)),
        }
    }
}

enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn score(&self) -> u32 {
        match *self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    fn response(&self, outcome: &Outcome) -> Shape {
        match (self, outcome) {
            (Shape::Rock, Outcome::Win)
            | (Shape::Paper, Outcome::Draw)
            | (Shape::Scissors, Outcome::Loss) => Shape::Paper,

            (Shape::Rock, Outcome::Loss)
            | (Shape::Paper, Outcome::Win)
            | (Shape::Scissors, Outcome::Draw) => Shape::Scissors,

            _ => Shape::Rock,
        }
    }
}

impl TryFrom<char> for Shape {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Shape::Rock),
            'B' => Ok(Shape::Paper),
            'C' => Ok(Shape::Scissors),
            _ => Err(format!("Invalid shape input: {}", value)),
        }
    }
}

struct Round {
    opponent: Shape,
    outcome: Outcome,
}

impl Round {
    fn score(&self) -> u32 {
        self.opponent.response(&self.outcome).score() + self.outcome.score()
    }
}

impl TryFrom<String> for Round {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut chars = value.chars();

        let (opponent, response) = (
            chars
                .next()
                .ok_or(format!("Invalid round input: {}", value))?,
            chars
                .skip(1) // skip space
                .next()
                .ok_or(format!("Invalid round input: {}", value))?,
        );

        Ok(Round {
            opponent: opponent.try_into()?,
            outcome: response.try_into()?,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let score = &reader
        .lines()
        .filter_map(|line| {
            line.ok()
                .and_then(|line| line.try_into().ok())
                .and_then(|r: Round| Some(r.score()))
        })
        .sum::<u32>();

    println!("{}", score);

    Ok(())
}
