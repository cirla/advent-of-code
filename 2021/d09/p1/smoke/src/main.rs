use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let map: Vec<_> = reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect();

    let height = map.len();
    let width = map[0].len();
    let mut sum = 0;

    for i in 0..height {
        for j in 0..width {
            let val = map[i][j];
            let up = if i > 0 { map[i - 1][j] } else { u32::MAX };
            let down = if i < (height - 1) {
                map[i + 1][j]
            } else {
                u32::MAX
            };
            let left = if j > 0 { map[i][j - 1] } else { u32::MAX };
            let right = if j < (width - 1) {
                map[i][j + 1]
            } else {
                u32::MAX
            };

            if val < up && val < down && val < left && val < right {
                sum += val + 1;
            }
        }
    }

    println!("{}", sum);

    Ok(())
}
