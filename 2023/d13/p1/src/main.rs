use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{Context, Result};
use itertools::Itertools;

#[derive(Debug, Default)]
struct Pattern {
    rows: Vec<u32>,
    cols: Vec<u32>,
}

impl Pattern {
    pub fn try_from_lines<I: IntoIterator<Item = String>>(lines: I) -> Result<Self> {
        // there are only two possible values so parse each row as a binary number
        let rows = lines
            .into_iter()
            .map(|line| line.replace('.', "0").replace('#', "1"))
            .collect::<Vec<_>>();

        // there are only two possible values so parse each col as a binary number
        let cols = (0..rows[0].len())
            .map(|i| {
                u32::from_str_radix(
                    &rows
                        .iter()
                        .map(|r| r.as_bytes()[i] as char)
                        .collect::<String>(),
                    2,
                )
                .context("Invalid col")
            })
            .collect::<Result<_>>()?;

        let rows = rows
            .into_iter()
            .map(|r| u32::from_str_radix(&r, 2).context("Invalid row"))
            .collect::<Result<_>>()?;

        Ok(Self { rows, cols })
    }

    pub fn summarize(&self) -> usize {
        let find_reflect = |xs: &[u32]| {
            for i in 1..xs.len() {
                if (&xs[..i])
                    .iter()
                    .rev()
                    .zip((&xs[i..]).iter())
                    .all(|(a, b)| a == b)
                {
                    return i;
                }
            }

            0
        };

        find_reflect(&self.rows) * 100 + find_reflect(&self.cols)
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    // group lines separated by empty lines
    let groups = reader
        .lines()
        .filter_map(|line| line.ok())
        .group_by(|line| !line.is_empty());

    let groups = groups
        .into_iter()
        .filter_map(|(not_empty, group)| not_empty.then(|| group));

    let summary = groups
        .map(|g| Pattern::try_from_lines(g).and_then(|d| Ok(d.summarize())))
        .sum::<Result<usize>>()?;

    println!("Summary: {summary}");

    Ok(())
}
