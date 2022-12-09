use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Motion {
    pub direction: Direction,
    pub steps: usize,
}

impl TryFrom<String> for Motion {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();

        let direction = parts
            .next()
            .map(|d| match d {
                "U" => Some(Direction::Up),
                "D" => Some(Direction::Down),
                "L" => Some(Direction::Left),
                "R" => Some(Direction::Right),
                _ => None,
            })
            .flatten()
            .ok_or(format!("Invalid direction input: {}", value))?;

        let steps: usize = parts
            .next()
            .map(|s| s.parse().ok())
            .flatten()
            .ok_or(format!("Invalid direction input: {}", value))?;

        Ok(Motion { direction, steps })
    }
}

#[derive(Clone, Copy, Default, Eq, Hash, PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn origin() -> Point {
        Default::default()
    }

    pub fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }

    pub fn r#move(&mut self, dir: Direction) {
        match dir {
            Direction::Up => {
                self.y += 1;
            }
            Direction::Down => {
                self.y -= 1;
            }
            Direction::Left => {
                self.x -= 1;
            }
            Direction::Right => {
                self.x += 1;
            }
        }
    }

    pub fn is_touching(&self, other: &Point) -> bool {
        (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1
    }
}

pub struct Rope {
    knots: [Point; 10],
    tail_visited: HashSet<Point>,
}

impl Rope {
    pub fn new() -> Rope {
        Default::default()
    }

    pub fn move_head(&mut self, motion: Motion) {
        for _ in 0..motion.steps {
            self.knots[0].r#move(motion.direction);
            self.catch_up();
        }
    }

    fn catch_up(&mut self) {
        // go through each pair of adjacent knots
        for i in 0..self.knots.len() - 1 {
            // if the next knot is still touching, nothing more to do
            if &self.knots[i] == &self.knots[i + 1] || self.knots[i].is_touching(&self.knots[i + 1])
            {
                break;
            }

            // catch up
            if self.knots[i].x == self.knots[i + 1].x {
                // same col
                self.knots[i + 1].y += (self.knots[i].y - self.knots[i + 1].y).signum();
            } else if self.knots[i].y == self.knots[i + 1].y {
                // same row
                self.knots[i + 1].x += (self.knots[i].x - self.knots[i + 1].x).signum();
            } else {
                // diagonal
                self.knots[i + 1].x += (self.knots[i].x - self.knots[i + 1].x).signum();
                self.knots[i + 1].y += (self.knots[i].y - self.knots[i + 1].y).signum();
            }

            // if next knot is the tail, update visited
            if i + 1 == self.knots.len() - 1 {
                self.tail_visited.insert(self.knots[i + 1].clone());
            }
        }
    }

    pub fn tail_visited(&self) -> usize {
        self.tail_visited.len()
    }
}

impl Default for Rope {
    fn default() -> Rope {
        Rope {
            knots: [Point::origin(); 10],
            tail_visited: HashSet::from([Point::origin()]),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;

    let reader = io::BufReader::new(input);

    let mut rope = Rope::new();
    for line in reader.lines().filter_map(|line| line.ok()) {
        rope.move_head(line.try_into()?);
    }

    println!("{}", rope.tail_visited());

    Ok(())
}
