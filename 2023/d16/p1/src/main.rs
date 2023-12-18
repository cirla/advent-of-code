use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{anyhow, Error, Result};

#[derive(Debug)]
enum Cell {
    Empty,
    MirrorFwd,
    MirrorBack,
    SplitterHoriz,
    SplitterVert,
}

impl TryFrom<char> for Cell {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            '.' => Ok(Self::Empty),
            '/' => Ok(Self::MirrorFwd),
            '\\' => Ok(Self::MirrorBack),
            '-' => Ok(Self::SplitterHoriz),
            '|' => Ok(Self::SplitterVert),
            c => Err(anyhow!("Invalid cell: {}", c)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Photon {
    row: usize,
    col: usize,
    dir: Direction,
}

impl Photon {
    /// (Maybe) advance photon in its current direction
    pub fn advance(self, max_row: usize, max_col: usize) -> Option<Self> {
        match self.dir {
            Direction::Up => (self.row > 0).then(|| Self {
                row: self.row - 1,
                ..self
            }),
            Direction::Down => (self.row < max_row).then(|| Self {
                row: self.row + 1,
                ..self
            }),
            Direction::Left => (self.col > 0).then(|| Self {
                col: self.col - 1,
                ..self
            }),
            Direction::Right => (self.col < max_col).then(|| Self {
                col: self.col + 1,
                ..self
            }),
        }
    }

    /// Change photon direction
    pub fn with_dir(self, dir: Direction) -> Self {
        Self { dir, ..self }
    }
}

#[derive(Debug)]
struct Grid {
    cells: Vec<Cell>,
    rows: usize,
    cols: usize,
}

impl Grid {
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

        Ok(Grid { cells, rows, cols })
    }

    pub fn get<'a>(&'a self, row: usize, col: usize) -> &'a Cell {
        &self.cells[self.cols * row + col]
    }

    pub fn num_energized(&self) -> usize {
        let max_row = self.rows - 1;
        let max_col = self.cols - 1;

        // keep track of which cells have been energized
        let mut energized = vec![false; self.cells.len()];

        // start with one photon in top-left moving right
        let mut photons = vec![Photon {
            row: 0,
            col: 0,
            dir: Direction::Right,
        }];

        // fire all photons
        while !photons.is_empty() {
            let mut photon = photons.pop();

            // while the photon is still valid
            while let Some(mut p) = photon {
                let seen = energized[self.cols * p.row + p.col];
                match self.get(p.row, p.col) {
                    // always pass through empty cells
                    Cell::Empty => {
                        photon = p.advance(max_row, max_col);
                    }
                    // always reflect on mirrors
                    Cell::MirrorFwd => {
                        p = p.with_dir(match p.dir {
                            Direction::Up => Direction::Right,
                            Direction::Down => Direction::Left,
                            Direction::Left => Direction::Down,
                            Direction::Right => Direction::Up,
                        });
                        photon = p.advance(max_row, max_col);
                    }
                    // always reflect on mirrors
                    Cell::MirrorBack => {
                        p = p.with_dir(match p.dir {
                            Direction::Up => Direction::Left,
                            Direction::Down => Direction::Right,
                            Direction::Left => Direction::Up,
                            Direction::Right => Direction::Down,
                        });
                        photon = p.advance(max_row, max_col);
                    }
                    // only split if we haven't already energized this splitter
                    // to avoid infinite loop
                    Cell::SplitterHoriz if !seen => {
                        match p.dir {
                            // split into two photons going opposite horizontal directions
                            Direction::Up | Direction::Down => {
                                if let Some(split) =
                                    p.with_dir(Direction::Right).advance(max_row, max_col)
                                {
                                    photons.push(split);
                                }

                                p = p.with_dir(Direction::Left);
                            }
                            // pass through
                            _ => {}
                        };

                        photon = p.advance(max_row, max_col);
                    }
                    // only split if we haven't already energized this splitter
                    // to avoid infinite loop
                    Cell::SplitterVert if !seen => {
                        match p.dir {
                            // split into two photons going opposite vertical directions
                            Direction::Left | Direction::Right => {
                                if let Some(split) =
                                    p.with_dir(Direction::Down).advance(max_row, max_col)
                                {
                                    photons.push(split);
                                }

                                p = p.with_dir(Direction::Up);
                            }
                            // pass through
                            _ => {}
                        };

                        photon = p.advance(max_row, max_col);
                    }
                    _ => {
                        // stop photon to avoid infinite loop
                        photon = None;
                    }
                }

                // always energize after visit
                energized[self.cols * p.row + p.col] = true;
            }
        }

        // count energized cells
        energized.into_iter().filter(|x| *x).count()
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let grid = Grid::try_from_lines(reader.lines().filter_map(|line| line.ok()))?;

    println!("Energized Tiles: {}", grid.num_energized());

    Ok(())
}
