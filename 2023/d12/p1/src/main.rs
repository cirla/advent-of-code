use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};

#[derive(Debug)]
enum Spring {
    Operational,
    Broken,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Spring::Operational),
            '#' => Ok(Spring::Broken),
            '?' => Ok(Spring::Unknown),
            _ => Err(anyhow!("Invalid character: {}", c)),
        }
    }
}

#[derive(Debug)]
struct Row {
    springs: Vec<Spring>,
    groups: Vec<usize>,
}

impl Row {
    /// Get number of possible arrangements for a row
    pub fn possible_arrangements(&self) -> usize {
        // recursively get possible arrangements for a given vector of springs,
        // vector of group sizes to match, and length of current partially matched group
        fn inner(springs: &[Spring], groups: &[usize], group_len: usize) -> usize {
            // base case; end of springs input
            if springs.is_empty() {
                return match (groups.len(), group_len) {
                    // end of groups input, no partially matched group; exactly 1 possibility
                    (0, 0) => 1,
                    // one group left and it matches partially matched group; exactly 1 possibility
                    (1, x) if x == groups[0] => 1,
                    // can't match anymore; impossible
                    _ => 0,
                };
            }

            // no groups left to match; impossible if we are in a partially matched group
            if groups.is_empty() && group_len > 0 {
                return 0;
            }

            match springs[0] {
                Spring::Operational => match group_len {
                    // no group yet, keep looking for Broken or Unknown
                    0 => inner(&springs[1..], groups, 0),
                    // group ended and size mismatch; no possible arrangements
                    len if len != groups[0] => 0,
                    // end of matching group; advance to next group and reset length
                    _ => inner(&springs[1..], &groups[1..], 0),
                },
                Spring::Broken => match group_len {
                    // new group started
                    0 => inner(&springs[1..], groups, 1),
                    // continue matching group
                    x => inner(&springs[1..], groups, x + 1),
                },
                Spring::Unknown => match group_len {
                    // new group started
                    // sum possibilities for Operational and Broken
                    0 => inner(&springs[1..], groups, 0) + inner(&springs[1..], groups, 1),
                    // continue matching group
                    // sum possibilities for Broken and, if applicable, Operational
                    x => {
                        let broken_possibilities = inner(&springs[1..], groups, x + 1);

                        // if we can match a group, add possibilities for operational
                        // (e.g. start new group)
                        broken_possibilities
                            + (groups[0] == x)
                                .then(|| inner(&springs[1..], &groups[1..], 0))
                                .unwrap_or_default()
                    }
                },
            }
        }

        inner(&self.springs, &self.groups, 0)
    }
}

impl FromStr for Row {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(' ');

        let springs = parts
            .next()
            .ok_or(anyhow!("Missing springs"))?
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<_>>()?;

        let groups = parts
            .next()
            .ok_or(anyhow!("Missing group lengths"))?
            .split(',')
            .map(|c| c.parse().context("Invalid group length"))
            .collect::<Result<_>>()?;

        Ok(Row { springs, groups })
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let sum = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.parse()
                .context("Invalid Row")
                .and_then(|r: Row| Ok(r.possible_arrangements()))
        })
        .sum::<Result<usize>>()?;

    println!("Sum of possible arrangements: {}", sum);

    Ok(())
}
