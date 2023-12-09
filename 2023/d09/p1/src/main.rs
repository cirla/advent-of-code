use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::{Context, Error, Result};
use tailcall::tailcall;

#[derive(Debug)]
struct History {
    history: Vec<isize>,
}

impl History {
    pub fn extrapolate(&self) -> isize {
        #[tailcall]
        fn inner(acc: isize, input: Vec<isize>) -> isize {
            let next = input.windows(2).map(|w| w[1] - w[0]).collect::<Vec<_>>();
            let last = next.last().unwrap();
            if next.len() == 1 || next.windows(2).all(|w| w[0] == w[1]) {
                acc + last
            } else {
                inner(acc + last, next)
            }
        }

        let last = self.history.last().expect("History should not be empty");
        inner(*last, self.history.clone())
    }
}

impl FromStr for History {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let history = s
            .split(' ')
            .map(|x| x.parse().context("Invalid history value"))
            .collect::<Result<_>>()?;

        Ok(Self { history })
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let sum = reader
        .lines()
        .filter_map(|line| line.ok())
        .enumerate()
        .map(|(i, line)| {
            line.parse()
                .with_context(|| format!("Invalid history on line {i}"))
                .map(|h: History| h.extrapolate())
        })
        .sum::<Result<isize>>()?;

    println!("Sum: {}", sum);

    Ok(())
}
