use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};

#[derive(Debug)]
struct Card {
    _id: usize,
    winning_numbers: BTreeSet<u8>,
    card_numbers: BTreeSet<u8>,
}

impl Card {
    pub fn value(&self) -> usize {
        match self
            .card_numbers
            .intersection(&self.winning_numbers)
            .count()
        {
            0 => 0,
            c => 2usize.pow(c as u32 - 1),
        }
    }
}

impl FromStr for Card {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s
            .strip_prefix("Card ")
            .ok_or(anyhow!("Missing 'Card ' prefix"))?
            .split(": ");

        let id = parts
            .next()
            .ok_or(anyhow!("Missing id"))?
            .trim()
            .parse()
            .context("Invalid id")?;

        let mut numbers = parts
            .next()
            .ok_or(anyhow!("Missing all card numbers"))?
            .split(" | ");

        let winning_numbers = numbers
            .next()
            .ok_or(anyhow!("Missing winning numbers"))?
            .split(" ")
            .filter_map(|c| {
                (!c.is_empty()).then(|| {
                    c.parse()
                        .with_context(|| format!("Invalid winning number: {}", c))
                })
            })
            .collect::<Result<_>>()?;

        let card_numbers = numbers
            .next()
            .ok_or(anyhow!("Missing card numbers"))?
            .split(" ")
            .filter_map(|c| {
                (!c.is_empty()).then(|| {
                    c.parse()
                        .with_context(|| format!("Invalid card number: {}", c))
                })
            })
            .collect::<Result<_>>()?;

        Ok(Card {
            _id: id,
            winning_numbers,
            card_numbers,
        })
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let cards = reader
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .map(|(i, line)| {
            line.parse()
                .with_context(|| format!("Couldn't parse Card on line {}", i + 1))
        })
        .collect::<Result<Vec<_>>>()?;

    let sum: usize = cards.iter().map(Card::value).sum();

    println!("Sum of Values: {}", sum);

    Ok(())
}
