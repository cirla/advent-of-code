use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

struct Vent(Point, Point);

fn parse_vent(line: String) -> Result<Vent, Box<dyn Error>> {
    let points: Vec<_> = line
        .split(" -> ")
        .map(|p| p.split(",").filter_map(|x| x.parse::<i32>().ok()).collect())
        .map(|p: Vec<i32>| Point { x: p[0], y: p[1] })
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
        let horizontal_step = if vent.0.x == vent.1.x {
            0
        } else {
            if vent.0.x < vent.1.x {
                1
            } else {
                -1
            }
        };

        let vertical_step = if vent.0.y == vent.1.y {
            0
        } else {
            if vent.0.y < vent.1.y {
                1
            } else {
                -1
            }
        };

        let mut p = vent.0;
        while {
            let counter = counts.entry(p).or_insert(0);
            *counter += 1;
            p.x += horizontal_step;
            p.y += vertical_step;

            p != vent.1
        } {}

        let counter = counts.entry(p).or_insert(0);
        *counter += 1;
    }

    println!("{}", counts.values().filter(|&&x| x > 1).count());

    Ok(())
}
