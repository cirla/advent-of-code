use std::env;
use std::error::Error;
use std::fs::read_to_string;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let mut fish: [u64; 9] = [0; 9];

    for f in read_to_string(&args[1])?
        .trim()
        .split(",")
        .filter_map(|x| x.parse::<usize>().ok())
    {
        fish[f] += 1;
    }

    let days: usize = args[2].parse()?;

    for _ in 0..days {
        fish.rotate_left(1);
        fish[6] += fish[8];
    }

    println!("{}", fish.iter().sum::<u64>());

    Ok(())
}
