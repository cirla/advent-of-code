use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let mut crabs = HashMap::<i32, i32>::new();

    for c in read_to_string(&args[1])?
        .trim()
        .split(",")
        .filter_map(|x| x.parse::<i32>().ok())
    {
        *crabs.entry(c).or_insert(0) += 1;
    }

    let mut costs = HashMap::<i32, i32>::new();

    for &pos in crabs.keys() {
        *costs.entry(pos).or_insert(0) = crabs
            .iter()
            .map(|(p, n)| i32::abs(pos - p) * n)
            .sum::<i32>();
    }

    println!("{}", costs.values().min().unwrap());

    Ok(())
}
