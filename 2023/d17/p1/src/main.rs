use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{anyhow, Result};
use pathfinding::prelude::astar;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    pub fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Crucible {
    row: usize,
    col: usize,
    dir: Direction,
    speed: u8,
}

impl Crucible {
    pub fn distance(&self, row: usize, col: usize) -> usize {
        self.row.abs_diff(row) + self.col.abs_diff(col)
    }

    pub fn advance(self, max_row: usize, max_col: usize) -> Option<Self> {
        if self.speed == 3 {
            None
        } else {
            let speed = self.speed + 1;
            match self.dir {
                Direction::Up => (self.row > 0).then(|| Self {
                    row: self.row - 1,
                    speed,
                    ..self
                }),
                Direction::Down => (self.row < max_row).then(|| Self {
                    row: self.row + 1,
                    speed,
                    ..self
                }),
                Direction::Left => (self.col > 0).then(|| Self {
                    col: self.col - 1,
                    speed,
                    ..self
                }),
                Direction::Right => (self.col < max_col).then(|| Self {
                    col: self.col + 1,
                    speed,
                    ..self
                }),
            }
        }
    }

    pub fn successors(&self, map: &Map) -> impl IntoIterator<Item = (Crucible, u32)> {
        let max_col = map.cols - 1;
        let max_row = map.rows - 1;

        [
            *self,
            Self {
                dir: self.dir.turn_left(),
                speed: 0,
                ..*self
            },
            Self {
                dir: self.dir.turn_right(),
                speed: 0,
                ..*self
            },
        ]
        .into_iter()
        .filter_map(move |c| {
            c.advance(max_row, max_col)
                .and_then(|c| Some((c, map.get(c.row, c.col))))
        })
        .collect::<Vec<_>>()
    }
}

#[derive(Debug)]
struct Map {
    blocks: Vec<u32>,
    rows: usize,
    cols: usize,
}

impl Map {
    pub fn try_from_lines<I: IntoIterator<Item = String>>(lines: I) -> Result<Self> {
        let mut lines = lines.into_iter().peekable();
        let cols = lines
            .peek()
            .ok_or(anyhow!("Missing input"))?
            .chars()
            .count();

        let blocks = lines
            .flat_map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).ok_or(anyhow!("Invalid digit")))
                    .collect::<Vec<_>>()
            })
            .collect::<Result<Vec<_>>>()?;

        let rows = blocks.len() / cols;

        Ok(Self { blocks, rows, cols })
    }

    pub fn get(&self, row: usize, col: usize) -> u32 {
        self.blocks[self.cols * row + col]
    }

    pub fn min_heat_loss(&self) -> Result<usize> {
        let goal = (self.rows - 1, self.cols - 1);

        let starts = &[
            Crucible {
                row: 0,
                col: 0,
                dir: Direction::Right,
                speed: 0,
            },
            Crucible {
                row: 0,
                col: 0,
                dir: Direction::Down,
                speed: 0,
            },
        ];

        let cost = starts
            .into_iter()
            .map(|start| {
                astar(
                    start,
                    |c| c.successors(self),
                    |c| c.distance(goal.0, goal.1) as u32 / 3,
                    |c| (c.row, c.col) == goal,
                )
                .ok_or(anyhow!("Unable to find path"))
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .map(|(_, c)| c)
            .min()
            .ok_or(anyhow!("Unable to find minimum heat loss"))?;

        Ok(cost as usize)
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let map = Map::try_from_lines(reader.lines().filter_map(|line| line.ok()))?;

    println!("Minimum heat loss: {}", map.min_heat_loss()?);

    Ok(())
}
