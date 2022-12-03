use std::collections::BTreeSet;
use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

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
    fn dup(&self) -> Item {
        let mid = self.contents.len() / 2;
        let first_half: BTreeSet<&Item> = (&self.contents[..mid]).iter().collect();
        let second_half: BTreeSet<&Item> = (&self.contents[mid..]).iter().collect();
        **first_half.intersection(&second_half).next().unwrap()
    }
}

impl TryFrom<String> for Rucksack {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let items: Result<Vec<Item>, String> = value.chars().map(|x| x.try_into()).collect();

        items.map(|contents| Rucksack { contents })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let total = &reader
        .lines()
        .filter_map(|line| {
            line.ok()
                .and_then(|line| line.try_into().ok())
                .and_then(|r: Rucksack| Some(r.dup().priority))
        })
        .sum::<u32>();

    println!("{}", total);

    Ok(())
}
