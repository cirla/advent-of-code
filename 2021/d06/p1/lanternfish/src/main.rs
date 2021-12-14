use std::env;
use std::error::Error;
use std::fs::read_to_string;
use std::iter::repeat;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    let mut fish: Vec<_> = read_to_string(&args[1])?
        .trim()
        .split(",")
        .filter_map(|x| x.parse::<u32>().ok())
        .collect();

    let days: usize = args[2].parse()?;

    for _ in 0..days {
        let mut new_fish = 0;

        for i in 0..fish.len() {
            if fish[i] == 0 {
                fish[i] = 6;
                new_fish += 1;
            } else {
                fish[i] -= 1;
            }
        }

        fish.extend(repeat(8).take(new_fish));
    }

    println!("{}", fish.len());

    Ok(())
}
