use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{anyhow, Error, Result};
use itertools::Itertools;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

#[derive(Clone, Copy, Debug, EnumIter, Eq, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    pub fn opposite(&self) -> Direction {
        match self {
            Self::North => Self::South,
            Self::South => Self::North,
            Self::East => Self::West,
            Self::West => Self::East,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Point {
    x: usize,
    y: usize,
}

#[derive(Debug)]
enum Cell {
    Ground,
    Pipe(Direction, Direction),
    /// Placeholder until replaced with Pipe
    Start,
}

impl Cell {
    /// Try to enter a pipe from a given direction
    /// returns exit direction
    fn try_enter(&self, dir: Direction) -> Option<Direction> {
        match self {
            Self::Ground => None,
            &Self::Pipe(end1, end2) => match dir {
                _ if dir == end1 => Some(end2),
                _ if dir == end2 => Some(end1),
                _ => None,
            },
            // Exit direction doesn't matter
            Self::Start => Some(dir),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Self::Ground),
            '|' => Ok(Self::Pipe(Direction::North, Direction::South)),
            '-' => Ok(Self::Pipe(Direction::East, Direction::West)),
            'L' => Ok(Self::Pipe(Direction::North, Direction::East)),
            'J' => Ok(Self::Pipe(Direction::North, Direction::West)),
            '7' => Ok(Self::Pipe(Direction::South, Direction::West)),
            'F' => Ok(Self::Pipe(Direction::South, Direction::East)),
            'S' => Ok(Self::Start),
            _ => Err(anyhow!("Invalid cell: {}", c)),
        }
    }
}

#[derive(Debug)]
struct Sketch {
    grid: Vec<Cell>,
    rows: usize,
    cols: usize,
    start: Point,
    loop_len: usize,
}

impl Sketch {
    pub fn try_from_lines<I: IntoIterator<Item = String>>(lines: I) -> Result<Self> {
        let mut grid = Vec::new();

        let mut rows = 0;
        for line in lines.into_iter() {
            let mut row = line.chars().map(|c| c.try_into()).try_collect()?;
            grid.append(&mut row);
            rows += 1;
        }

        let cols = grid.len() / rows;
        let start = grid
            .iter()
            .position(|c| match c {
                Cell::Start => true,
                _ => false,
            })
            .ok_or(anyhow!("Missing start cell"))?;

        let mut grid = Self {
            grid,
            rows,
            cols,
            start: Point {
                x: start % cols,
                y: start / cols,
            },
            loop_len: 0,
        };

        grid.resolve_start()?;

        Ok(grid)
    }

    pub fn max_distance_from_start(&self) -> usize {
        self.loop_len / 2 + self.loop_len % 2
    }

    fn get<'a>(&'a self, loc: &Point) -> &'a Cell {
        &self.grid[loc.y * self.cols + loc.x]
    }

    fn set_start(&mut self, pipe: Cell) {
        let start = &mut self.grid[self.start.y * self.cols + self.start.x];
        *start = pipe;
    }

    /// Try to move from a given point in a given direction.
    /// If the move is impossible, None is returned.
    /// Otherwise, the return value contains a tuple of the new location
    /// and exit direction.
    fn try_move(&self, loc: Point, dir: Direction) -> Option<(Point, Direction)> {
        let next = match (dir, loc.x, loc.y) {
            (Direction::North, x, y) if y > 0 => Some(Point { x, y: y - 1 }),
            (Direction::South, x, y) if y < self.rows - 1 => Some(Point { x, y: y + 1 }),
            (Direction::East, x, y) if x < self.cols - 1 => Some(Point { x: x + 1, y }),
            (Direction::West, x, y) if x > 0 => Some(Point { x: x - 1, y }),
            _ => None,
        };

        next.and_then(|next| {
            self.get(&next)
                .try_enter(dir.opposite())
                .map(|exit| (next, exit))
        })
    }

    /// Replace the Cell::Start placeholder with a resolved Pipe
    fn resolve_start(&mut self) -> Result<()> {
        for start_dir in Direction::iter() {
            let mut distance = 1;
            let mut curr = self.try_move(self.start, start_dir);
            while let Some((loc, dir)) = curr {
                if loc == self.start {
                    self.loop_len = distance;
                    self.set_start(Cell::Pipe(start_dir, dir.opposite()));
                    return Ok(());
                }

                curr = self.try_move(loc, dir);
                distance += 1;
            }
        }

        Err(anyhow!("Could not resolve start"))
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let sketch = Sketch::try_from_lines(reader.lines().filter_map(|line| line.ok()))?;

    println!("Steps: {}", sketch.max_distance_from_start());

    Ok(())
}
