use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};

use anyhow::{anyhow, Error, Result};
use itertools::{
    FoldWhile::{Continue, Done},
    Itertools,
};
use num::integer::lcm;

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

    fn start<'a>(&'a self) -> Vec<(&'a String, usize)> {
        // get all starting nodes and keep a count of how many steps
        // it takes to get to each one from the beginning
        self.nodes
            .keys()
            .filter_map(|n| n.ends_with('A').then_some((n, 0usize)))
            .collect::<Vec<_>>()
    }

    fn left<'a>(&'a self, node: &'a String) -> &'a String {
        &self.nodes[node].0
    }

    fn right<'a>(&'a self, node: &'a String) -> &'a String {
        &self.nodes[node].1
    }

    pub fn traverse(&self) -> usize {
        self.directions
            .iter()
            .cycle()
            .fold_while(self.start(), |mut nodes, dir| {
                let next = match dir {
                    Direction::Left => Self::left,
                    Direction::Right => Self::right,
                };

                // ignore nodes that are already on an end node as we would
                // eventually get back to them. after finding the counts for
                // every starting node to an ending node, we can calculate
                // the least common multiple of the counts for all nodes
                // where they would re-align
                nodes
                    .iter_mut()
                    .filter(|(node, _)| !node.ends_with('Z'))
                    .for_each(|(node, count)| {
                        *node = next(&self, node);
                        *count += 1;
                    });

                if nodes.iter().all(|(node, _)| node.ends_with('Z')) {
                    Done(nodes)
                } else {
                    Continue(nodes)
                }
            })
            .into_inner()
            .into_iter()
            .map(|(_, count)| count)
            .reduce(lcm)
            .unwrap()
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
