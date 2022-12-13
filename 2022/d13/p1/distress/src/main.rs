use std::cmp::Ordering;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;
use serde::Deserialize;

#[derive(Clone, Deserialize, Eq, PartialEq)]
#[serde(untagged)]
pub enum Data {
    Integer(u32),
    List(Vec<Data>),
}

impl Data {}

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

    let idx_sum: usize = reader
        .lines()
        .filter_map(|line| line.ok())
        .group_by(|line| !line.is_empty())
        .into_iter()
        .filter_map(|(key, group)| {
            key.then(|| {
                group
                    .map(|line| serde_json::from_str(&line))
                    .collect::<Result<Vec<Data>, _>>()
                    .ok()
            })
            .flatten()
        })
        .enumerate()
        .filter_map(|(i, packets)| (packets[0] < packets[1]).then_some(i + 1))
        .sum();

    println!("{}", idx_sum);

    Ok(())
}
