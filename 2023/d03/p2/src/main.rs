use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::{Context, Result};

/// Number with location in row
#[derive(Debug)]
struct Number {
    pub value: u32,
    pub start: usize,
    pub len: usize,
}

impl Number {
    /// Test if gear has adjacent column index with any of this numbers digits
    pub fn is_adjacent(&self, gear: usize) -> bool {
        self.start <= gear + 1 && gear - 1 <= (self.start + self.len - 1)
    }
}

/// Schematic row
#[derive(Debug)]
struct Row {
    /// numbers occurring in the row
    numbers: Vec<Number>,
    /// gears occurring in the row
    gear_indices: Vec<usize>,
}

impl FromStr for Row {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // partition into digits and gears
        let (digits, gears): (Vec<_>, Vec<_>) = s
            .chars()
            .enumerate()
            .filter_map(|(i, c)| match c {
                '*' | '0'..='9' => Some((i, c)),
                _ => None,
            })
            .partition(|(_, c)| c.is_digit(10));

        // accumulate adjacent digits into numbers
        let numbers = digits.iter().fold(Vec::new(), |mut acc, (i, c)| {
            let value = c.to_digit(10).expect("already checked is_digit");
            acc.last_mut()
                .and_then(|last: &mut Number| {
                    // update last if adjacent
                    (*i == last.start + last.len).then(|| {
                        last.value = last.value * 10 + value;
                        last.len += 1;
                    })
                })
                .unwrap_or_else(|| {
                    // add as new number
                    acc.push(Number {
                        value,
                        start: *i,
                        len: 1,
                    });
                });

            acc
        });

        // discard symbols, we just need indices
        let gear_indices = gears.iter().map(|(i, _)| *i).collect();

        Ok(Self {
            numbers,
            gear_indices,
        })
    }
}

/// Engine schematic
#[derive(Debug)]
struct Schematic {
    rows: Vec<Row>,
}

impl Schematic {
    pub fn new(rows: Vec<Row>) -> Self {
        Self { rows }
    }

    pub fn gear_ratios<'a>(&'a self) -> impl Iterator<Item = u32> + 'a {
        // self.rows.windows(3) would also work here, but got more verbose
        // with the special casing for first and last window
        (0..self.rows.len()).flat_map(|i| {
            let curr = &self.rows[i];

            // get iterator of all numbers in current and adjacent rows
            let curr_nums = curr.numbers.iter();
            let prev_nums = (i > 0)
                .then(|| &self.rows[i - 1].numbers)
                .into_iter()
                .flatten();
            let next_nums = (i < self.rows.len() - 1)
                .then(|| &self.rows[i + 1].numbers)
                .into_iter()
                .flatten();

            // collect into vec so we can re-use for each gear
            let numbers = prev_nums
                .chain(curr_nums)
                .chain(next_nums)
                .collect::<Vec<_>>();

            // get gear ratios for gears with exactly 2 adjacent numbers
            curr.gear_indices.iter().filter_map(move |g| {
                // get adjacent numbers
                let adj = numbers
                    .iter()
                    .filter_map(|n| n.is_adjacent(*g).then_some(n.value))
                    .collect::<Vec<_>>();

                // if there are exactly 2 adjacent gears, return ratio
                (adj.len() == 2).then(|| adj.iter().product())
            })
        })
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let schematic = Schematic::new(
        reader
            .lines()
            .filter_map(|line| line.ok())
            .enumerate()
            .map(|(i, line)| {
                line.parse()
                    .with_context(|| format!("Couldn't parse Row on line {}", i + 1))
            })
            .collect::<Result<Vec<Row>>>()?,
    );

    let sum: u32 = schematic.gear_ratios().sum();

    println!("Sum of Gear Ratios: {}", sum);

    Ok(())
}
