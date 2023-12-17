use std::collections::{hash_map::Entry, HashMap};
use std::env;
use std::fs::File;
use std::hash::Hash;
use std::io::{self, BufRead};

use anyhow::{anyhow, Context, Error, Result};
use ndarray::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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
    cells: Array2<Cell>,
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
            .collect::<Result<Vec<Cell>>>()?;

        let rows = cells.len() / cols;
        let cells =
            Array2::from_shape_vec((rows, cols), cells).context("Not all rows same length")?;

        Ok(Dish { cells })
    }

    pub fn tilt_cycles(&mut self, n: usize) {
        let mut memo = HashMap::from([(self.cells.clone(), 0)]);
        for i in 1..n {
            self.tilt_cycle();

            // use cached results to detect cycle
            match memo.entry(self.cells.clone()) {
                Entry::Occupied(o) => {
                    // cycle detected; shortcut
                    let cycle_start = o.get();
                    let cycle_period = i - cycle_start;
                    let cycle_end = cycle_start + (n - cycle_start) % cycle_period;
                    self.cells = memo.into_iter().find(|&(_, x)| x == cycle_end).unwrap().0;
                    return;
                }
                v @ Entry::Vacant(_) => {
                    // no cycle; cache and keep going
                    v.or_insert(i);
                }
            }
        }
    }

    pub fn tilt_cycle(&mut self) {
        self.tilt();
        self.rotate_right();
        self.tilt();
        self.rotate_right();
        self.tilt();
        self.rotate_right();
        self.tilt();
        self.rotate_right();
    }

    pub fn rows(&self) -> usize {
        self.cells.shape()[0]
    }

    pub fn cols(&self) -> usize {
        self.cells.shape()[1]
    }

    pub fn get<'a>(&'a self, row: usize, col: usize) -> &'a Cell {
        &self.cells[(row, col)]
    }

    pub fn set(&mut self, row: usize, col: usize, cell: Cell) {
        self.cells[(row, col)] = cell;
    }

    pub fn rotate_right(&mut self) {
        let mut rot = self.cells.clone().reversed_axes();
        for mut row in rot.rows_mut() {
            let rev = row.slice(s![..;-1]).to_owned();
            row.assign(&rev);
        }

        self.cells = rot;
    }

    pub fn tilt(&mut self) {
        for col in 0..self.cols() {
            for row in 1..self.rows() {
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
                Cell::Round => self.rows() - i / self.cols(),
            })
            .sum()
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let mut dish = Dish::try_from_lines(reader.lines().filter_map(|line| line.ok()))?;
    dish.tilt_cycles(1_000_000_000);

    println!("Total Load: {}", dish.total_load());

    Ok(())
}
