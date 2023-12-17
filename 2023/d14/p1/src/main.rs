use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{anyhow, Error, Result};

#[derive(Debug)]
enum Cell {
    Empty,
    Round,
    Cube,
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Self::Empty),
            'O' => Ok(Self::Round),
            '#' => Ok(Self::Cube),
            c => Err(anyhow!("Invalid cell: {}", c)),
        }
    }
}

#[derive(Debug)]
struct Dish {
    cells: Vec<Cell>,
    rows: usize,
    cols: usize,
}

impl Dish {
    pub fn try_from_lines<I: IntoIterator<Item = String>>(lines: I) -> Result<Self> {
        let mut lines = lines.into_iter().peekable();
        let cols = lines
            .peek()
            .ok_or(anyhow!("Missing input"))?
            .chars()
            .count();

        let cells = lines
            .flat_map(|line| line.chars().map(|c| c.try_into()).collect::<Vec<_>>())
            .collect::<Result<Vec<_>>>()?;

        let rows = cells.len() / cols;

        Ok(Dish { cells, rows, cols })
    }

    pub fn get<'a>(&'a self, row: usize, col: usize) -> &'a Cell {
        &self.cells[self.cols * row + col]
    }

    pub fn set(&mut self, row: usize, col: usize, cell: Cell) {
        self.cells[self.cols * row + col] = cell;
    }

    pub fn tilt(&mut self) {
        for col in 0..self.cols {
            for row in 1..self.rows {
                match self.get(row, col) {
                    Cell::Round => {
                        let new_row = (1..=row)
                            .filter_map(|i| match self.get(row - i, col) {
                                Cell::Empty => None,
                                _ => Some(row - i + 1),
                            })
                            .next()
                            .unwrap_or_default();

                        if new_row != row {
                            self.set(row, col, Cell::Empty);
                            self.set(new_row, col, Cell::Round);
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    pub fn total_load(&self) -> usize {
        self.cells
            .iter()
            .enumerate()
            .map(|(i, c)| match c {
                Cell::Empty | Cell::Cube => 0,
                Cell::Round => self.rows - i / self.cols,
            })
            .sum()
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let mut dish = Dish::try_from_lines(reader.lines().filter_map(|line| line.ok()))?;
    dish.tilt();

    println!("Total Load: {}", dish.total_load());

    Ok(())
}
