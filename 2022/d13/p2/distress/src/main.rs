use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use serde::Deserialize;

#[derive(Clone, Deserialize, Eq, Hash, PartialEq)]
#[serde(untagged)]
pub enum Data {
    Integer(u32),
    List(Vec<Data>),
}

impl FromStr for Data {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s).map_err(|e| format!("{}", e))
    }
}

impl Ord for Data {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Integer(x), Self::Integer(y)) => x.cmp(y),
            (Self::List(x), Self::List(y)) => x
                .iter()
                .zip(y.iter())
                .map(|(a, b)| a.cmp(b))
                .filter(|o| !matches!(o, Ordering::Equal))
                .next()
                .unwrap_or(x.len().cmp(&y.len())),
            (x @ Self::Integer(_), y @ Self::List(_)) => Self::List(vec![x.clone()]).cmp(y),
            (x @ Self::List(_), y @ Self::Integer(_)) => x.cmp(&Self::List(vec![y.clone()])),
        }
    }
}

impl PartialOrd for Data {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;

    let reader = io::BufReader::new(input);

    let dividers: HashSet<Data> = ["[[2]]", "[[6]]"]
        .into_iter()
        .filter_map(|x| x.parse().ok())
        .collect();

    let decoder_key: usize = reader
        .lines()
        .filter_map(|line| {
            line.ok()
                .and_then(|line| if line.is_empty() { None } else { Some(line) })
                .and_then(|line| line.parse().ok())
        })
        .chain(dividers.iter().cloned())
        .collect::<BinaryHeap<Data>>()
        .into_sorted_vec()
        .into_iter()
        .enumerate()
        .filter_map(|(i, packet)| dividers.contains(&packet).then_some(i + 1))
        .product();

    println!("{}", decoder_key);

    Ok(())
}
