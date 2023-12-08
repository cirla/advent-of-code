use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{anyhow, Error, Result};
use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};
use rayon::prelude::*;

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = Error;

    fn try_from(c: char) -> Result<Self> {
        match c {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            _ => Err(anyhow!("Invalid direction: {}", c)),
        }
    }
}

#[derive(Debug)]
struct Map {
    directions: Vec<Direction>,
    nodes: HashMap<String, (String, String)>,
}

impl Map {
    pub fn try_from_lines<I: IntoIterator<Item = String>>(lines: I) -> Result<Self> {
        let mut lines = lines.into_iter();

        let directions = lines
            .next()
            .ok_or(anyhow!("Missing directions"))?
            .chars()
            .map(|c| c.try_into())
            .collect::<Result<_>>()?;

        let nodes = lines
            .skip(1)
            .map(|line| {
                let mut parts = line.split(" = ");
                let key = parts.next().ok_or(anyhow!("Missing node key"))?.to_string();

                parts = parts
                    .next()
                    .ok_or(anyhow!("Invalid node destinations"))?
                    .split(", ");

                let left = parts
                    .next()
                    .ok_or(anyhow!("Missing left destination"))?
                    .trim_start_matches('(')
                    .to_string();

                let right = parts
                    .next()
                    .ok_or(anyhow!("Missing right destination"))?
                    .trim_end_matches(')')
                    .to_string();

                Ok((key, (left, right)))
            })
            .collect::<Result<_>>()?;

        Ok(Self { directions, nodes })
    }

    fn start<'a>(&'a self) -> Vec<&'a String> {
        self.nodes.keys().filter(|n| n.ends_with('A')).collect()
    }

    fn is_end(nodes: &Vec<&String>) -> bool {
        nodes.par_iter().all(|n| n.ends_with('Z'))
    }

    pub fn traverse(&self) -> usize {
        self.directions
            .iter()
            .cycle()
            .fold_while((0, self.start()), |(i, mut nodes), dir| {
                nodes.par_iter_mut().for_each(|node| {
                    *node = match dir {
                        Direction::Left => &self.nodes[*node].0,
                        Direction::Right => &self.nodes[*node].1,
                    }
                });

                if Self::is_end(&nodes) {
                    Done((i + 1, nodes))
                } else {
                    Continue((i + 1, nodes))
                }
            })
            .into_inner()
            .0
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let map = Map::try_from_lines(reader.lines().filter_map(|line| line.ok()))?;

    println!("Steps: {}", map.traverse());

    Ok(())
}
