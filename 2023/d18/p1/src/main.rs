use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'R' => Ok(Direction::Right),
            'L' => Ok(Direction::Left),
            _ => Err(anyhow!("Invalid direction: {}", c)),
        }
    }
}

#[derive(Debug)]
struct Step {
    dir: Direction,
    distance: usize,
}

impl FromStr for Step {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(' ');

        let dir = parts
            .next()
            .ok_or(anyhow!("Missing direction"))?
            .chars()
            .next()
            .ok_or(anyhow!("Empty direction"))?
            .try_into()
            .context("Invalid direction")?;

        let distance = parts
            .next()
            .ok_or(anyhow!("Missing distance"))?
            .parse()
            .context("Invalid distance")?;

        Ok(Step { dir, distance })
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Point {
    x: isize,
    y: isize,
}

impl Default for Point {
    fn default() -> Self {
        Self { x: 0, y: 0 }
    }
}

#[derive(Debug)]
struct Trench {
    vertices: Vec<Point>,
}

impl Trench {
    pub fn new() -> Self {
        Self {
            vertices: vec![Point::default()],
        }
    }

    pub fn dig(&mut self, step: Step) {
        let curr: Point = *self.vertices.last().expect("vertices is never empty");

        let (dx, dy): (isize, isize) = match step.dir {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        };

        // expand into individual boundary points so we can use
        // https://en.wikipedia.org/wiki/Pick%27s_theorem later
        for i in 1..=step.distance {
            self.vertices.push(Point {
                x: curr.x + dx * i as isize,
                y: curr.y + dy * i as isize,
            });
        }
    }

    pub fn is_closed(&self) -> bool {
        self.vertices[0] == *self.vertices.last().expect("vertices is never empty")
    }

    pub fn volume(&self) -> usize {
        // get total area of polygon using https://en.wikipedia.org/wiki/Shoelace_formula
        let area = self
            .vertices
            .windows(2)
            .map(|w| (w[0].x * w[1].y) - (w[1].x * w[0].y))
            .sum::<isize>()
            .abs() as usize
            / 2;

        // get number of interior points using https://en.wikipedia.org/wiki/Pick%27s_theorem
        let boundary = self.vertices.len() - 1;
        let interior = area - boundary / 2 + 1;
        boundary + interior
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let mut trench = Trench::new();
    for step in reader
        .lines()
        .filter_map(|line| line.ok())
        .map(|line| line.parse().context("Invalid step"))
    {
        trench.dig(step?);
    }

    assert!(trench.is_closed());

    println!("Lava Volume (m^3): {}", trench.volume());

    Ok(())
}
