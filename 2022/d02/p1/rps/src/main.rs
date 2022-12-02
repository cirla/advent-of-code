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

    fn play(&self, other: &Shape) -> Outcome {
        match (self, other) {
            (Shape::Rock, Shape::Scissors)
            | (Shape::Paper, Shape::Rock)
            | (Shape::Scissors, Shape::Paper) => Outcome::Win,

            (Shape::Rock, Shape::Paper)
            | (Shape::Paper, Shape::Scissors)
            | (Shape::Scissors, Shape::Rock) => Outcome::Loss,

            _ => Outcome::Draw,
        }
    }
}

impl TryFrom<char> for Shape {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissors),
            _ => Err(format!("Invalid shape input: {}", value)),
        }
    }
}

struct Round {
    opponent: Shape,
    response: Shape,
}

impl Round {
    fn score(&self) -> u32 {
        self.response.score() + self.response.play(&self.opponent).score()
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
            response: response.try_into()?,
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
