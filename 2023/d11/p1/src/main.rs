use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    pub fn shortest_path(&self, other: &Galaxy) -> usize {
        // (self.x - other.x).abs() + (self.y - other.y).abs()
        self.x
            .checked_sub(other.x)
            .unwrap_or_else(|| other.x - self.x)
            + self
                .y
                .checked_sub(other.y)
                .unwrap_or_else(|| other.y - self.y)
    }
}

#[derive(Debug)]
struct Image {
    pub galaxies: Vec<Galaxy>,
    rows: usize,
    cols: usize,
}

impl Image {
    pub fn try_from_lines<I: IntoIterator<Item = String>>(lines: I) -> Result<Self> {
        let mut galaxies = Vec::new();

        let mut lines = lines.into_iter().peekable();
        let cols = lines
            .peek()
            .ok_or(anyhow!("Missing input"))?
            .chars()
            .count();

        let mut rows = 0;
        for line in lines {
            line.chars()
                .enumerate()
                .filter_map(|(col, c)| (c == '#').then(|| Galaxy { x: col, y: rows }))
                .for_each(|g| galaxies.push(g));
            rows += 1;
        }

        let mut image = Image {
            galaxies,
            rows,
            cols,
        };
        image.expand();

        Ok(image)
    }

    fn expand(&mut self) {
        // get columns and rows that are not empty
        let (occupied_cols, occupied_rows): (BTreeSet<usize>, BTreeSet<usize>) =
            self.galaxies.iter().map(|g| (g.x, g.y)).unzip();

        // invert to get empty rows and columns
        let rows = BTreeSet::from_iter(0..self.rows);
        let empty_rows = rows.difference(&occupied_rows).collect::<BTreeSet<_>>();

        let cols = BTreeSet::from_iter(0..self.cols);
        let empty_cols = cols.difference(&occupied_cols).collect::<BTreeSet<_>>();

        // expand empty rows and columns
        for g in self.galaxies.iter_mut() {
            g.x += empty_cols.range(..g.x).count();
            g.y += empty_rows.range(..g.y).count();
        }
    }

    pub fn galaxy_pairs<'a>(&'a self) -> impl Iterator<Item = (&Galaxy, &Galaxy)> + 'a {
        self.galaxies.iter().combinations(2).map(|c| (c[0], c[1]))
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let image = Image::try_from_lines(reader.lines().filter_map(|line| line.ok()))?;
    let sum: usize = image.galaxy_pairs().map(|(a, b)| a.shortest_path(&b)).sum();

    println!("Sum of Shortest paths: {}", sum);

    Ok(())
}
