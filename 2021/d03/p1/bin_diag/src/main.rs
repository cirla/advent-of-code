use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let input_len: usize = args[2].parse()?;
    let mut bit_counts: Vec<usize> = vec![0; input_len];

    let mut total: usize = 0;
    for line in reader.lines().filter_map(|line| line.ok()) {
        total += 1;
        for (i, c) in line.chars().enumerate() {
            match c {
                '1' => bit_counts[i] += 1,
                _ => {}
            }
        }
    }

    let gamma_str: String = bit_counts
        .iter()
        .map(|c| if 2 * c >= total { '1' } else { '0' })
        .collect();

    let gamma = u32::from_str_radix(&gamma_str, 2)?;

    let invert_mask: u32 = (1u32 << input_len) - 1;
    let epsilon = gamma ^ invert_mask;

    println!("{}", gamma * epsilon);

    Ok(())
}
