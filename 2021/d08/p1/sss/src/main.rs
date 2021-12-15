use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let unique_seg_counts: HashSet<usize> = vec![2, 3, 4, 7].into_iter().collect();

    let counts = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.split(" | ")
                .skip(1)
                .next()
                .unwrap()
                .split(' ')
                .filter(|seg| unique_seg_counts.contains(&seg.len()))
                .count()
        })
        .sum::<usize>();

    println!("{}", counts);

    Ok(())
}
