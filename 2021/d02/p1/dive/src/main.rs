use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

struct Position {
    horizontal: i32,
    depth: i32,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let mut position = Position {
        horizontal: 0,
        depth: 0,
    };

    for line in reader.lines().filter_map(|line| line.ok()) {
        let cmd: Vec<&str> = line.split(' ').collect();
        let amount: i32 = cmd[1].parse()?;
        match cmd[0] {
            "forward" => position.horizontal += amount,
            "down" => position.depth += amount,
            "up" => position.depth -= amount,
            _ => {}
        }
    }

    println!("{}", position.horizontal * position.depth);

    Ok(())
}
