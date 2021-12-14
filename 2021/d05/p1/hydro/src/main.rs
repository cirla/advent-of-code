use std::cmp::{max, min};
use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: u32,
    y: u32,
}

struct Vent(Point, Point);

fn parse_vent(line: String) -> Result<Vent, Box<dyn Error>> {
    let points: Vec<_> = line
        .split(" -> ")
        .map(|p| p.split(",").filter_map(|x| x.parse::<u32>().ok()).collect())
        .map(|p: Vec<u32>| Point { x: p[0], y: p[1] })
        .collect();

    Ok(Vent(points[0], points[1]))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let vents: Vec<_> = reader
        .lines()
        .filter_map(|line| line.ok().and_then(|line| parse_vent(line).ok()))
        .collect();

    let mut counts = HashMap::<Point, u32>::new();
    for vent in vents.iter() {
        if vent.0.x == vent.1.x {
            // horizontal
            for y in min(vent.0.y, vent.1.y)..=max(vent.0.y, vent.1.y) {
                let p = Point { x: vent.0.x, y: y };
                let counter = counts.entry(p).or_insert(0);
                *counter += 1;
            }
        } else if vent.0.y == vent.1.y {
            // vertical
            for x in min(vent.0.x, vent.1.x)..=max(vent.0.x, vent.1.x) {
                let p = Point { x: x, y: vent.0.y };
                let counter = counts.entry(p).or_insert(0);
                *counter += 1;
            }
        }
    }

    println!("{}", counts.values().filter(|&&x| x > 1).count());

    Ok(())
}
