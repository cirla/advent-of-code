use std::collections::HashSet;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

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

#[derive(Clone, Default, Eq, Hash, PartialEq)]
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

    pub fn r#move(&mut self, dir: &Direction) {
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
    pub head: Point,
    pub tail: Point,
    tail_visited: HashSet<Point>,
}

impl Rope {
    pub fn new() -> Rope {
        Default::default()
    }

    pub fn move_head(&mut self, motion: Motion) {
        for _ in 0..motion.steps {
            if self.head == self.tail {
                self.head.r#move(&motion.direction);
            } else {
                let prev_head = self.head.clone();
                self.head.r#move(&motion.direction);

                if !self.head.is_touching(&self.tail) {
                    self.tail = prev_head;
                    self.tail_visited.insert(self.tail.clone());
                }
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
            head: Point::origin(),
            tail: Point::origin(),
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
