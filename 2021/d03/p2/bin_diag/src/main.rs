use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let input_len: usize = args[2].parse()?;

    let readings: Vec<u32> = reader
        .lines()
        .filter_map(|line| line.ok().and_then(|r| u32::from_str_radix(&r, 2).ok()))
        .collect();

    let oxy_rating = get_rating(&readings, input_len, true)?;
    let co2_rating = get_rating(&readings, input_len, false)?;

    println!("{}", oxy_rating * co2_rating);

    Ok(())
}

fn get_rating(
    readings: &[u32],
    input_len: usize,
    most_common: bool,
) -> Result<u32, Box<dyn Error>> {
    let mut subset: Vec<u32>;
    let mut remaining = readings;
    let mut mask = 1u32 << (input_len - 1);

    for _ in 0..input_len {
        // count ones in column
        let bits_set = remaining.iter().filter(|&x| x & mask > 0).count();

        // ones considered most common in a tie, which works in favor of ones when
        // most_common is true and in favor of zeroes when most_common is false
        let ones_common = 2 * bits_set >= remaining.len();

        // desired bit should be set when ones are most common and most common wins or when
        // zeroes are most common and most common loses. otherwise, desired bit should not be set
        let desired_bit = if (ones_common && most_common) || (!ones_common && !most_common) {
            mask
        } else {
            0
        };

        // collect numbers with desired bit (un)set
        subset = remaining
            .iter()
            .filter(|&x| x & mask == desired_bit)
            .copied()
            .collect();
        remaining = &subset;

        if remaining.len() == 1 {
            return Ok(remaining[0]);
        }

        // move bit mask to next column
        mask >>= 1;
    }

    Err("Reached end of readings.")?
}
