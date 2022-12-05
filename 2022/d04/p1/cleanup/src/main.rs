use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone)]
struct Assignment(u32, u32);

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        self.0 <= other.0 && self.1 >= other.1
    }
}

impl TryFrom<&str> for Assignment {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        value
            .split("-")
            .map(|x| x.parse::<u32>())
            .collect::<Result<Vec<u32>, _>>()
            .map(|x| Assignment(x[0].clone(), x[1].clone()))
            .map_err(|_| format!("Invalid assignment input: {}", value))
    }
}

struct Pair(Assignment, Assignment);

impl TryFrom<String> for Pair {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value
            .split(",")
            .map(|x| x.try_into())
            .collect::<Result<Vec<Assignment>, _>>()
            .map(|x| Pair(x[0].clone(), x[1].clone()))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let total = &reader
        .lines()
        .filter_map(|line| line.ok().and_then(|line| line.try_into().ok()))
        .filter(|p: &Pair| p.0.contains(&p.1) || p.1.contains(&p.0))
        .count();

    println!("{}", total);

    Ok(())
}
