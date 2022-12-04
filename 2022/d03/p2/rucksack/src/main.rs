use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Item {
    priority: u32,
}

impl Item {
    const LOWERCASE_OFFSET: u32 = ('a' as u32) - 1;
    const UPPERCASE_OFFSET: u32 = ('A' as u32) - 1 - 26;
}

impl TryFrom<char> for Item {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'a'..='z' => Ok(Self {
                priority: (value as u32) - Self::LOWERCASE_OFFSET,
            }),
            'A'..='Z' => Ok(Self {
                priority: (value as u32) - Self::UPPERCASE_OFFSET,
            }),
            _ => Err(format!("Invalid item input: {}", value)),
        }
    }
}

struct Rucksack {
    contents: Vec<Item>,
}

impl Rucksack {
    fn common(&self, other: Rucksack) -> Rucksack {
        let mine: BTreeSet<&Item> = self.contents.iter().collect();
        let theirs: BTreeSet<&Item> = other.contents.iter().collect();
        Rucksack {
            contents: mine.intersection(&theirs).cloned().cloned().collect(),
        }
    }
}

impl TryFrom<String> for Rucksack {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value
            .chars()
            .map(|x| x.try_into())
            .collect::<Result<Vec<Item>, _>>()
            .map(|contents| Rucksack { contents })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let total = &reader
        .lines()
        .filter_map(|line| line.ok().and_then(|line| line.try_into().ok()))
        .chunks(3)
        .into_iter()
        .map(|group| {
            group
                .reduce(|acc, r: Rucksack| r.common(acc))
                .unwrap()
                .contents
                .pop()
                .unwrap()
                .priority
        })
        .sum::<u32>();

    println!("{}", total);

    Ok(())
}
