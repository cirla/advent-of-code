use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let total = &reader
        .lines()
        .filter_map(|line| line.ok())
        .group_by(|x| !x.is_empty())
        .into_iter()
        .filter_map(|(key, group)| {
            if key {
                Some(group.filter_map(|x| x.parse::<u32>().ok()).sum::<u32>())
            } else {
                None
            }
        })
        .sorted()
        .rev()
        .take(3)
        .sum::<u32>();

    println!("{}", total);

    Ok(())
}
