use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

use pathfinding::prelude::astar;

#[derive(Copy, Clone, Default, Eq, Hash, PartialEq)]
pub struct Point {
    x: usize,
    y: usize,
}

impl Point {
    pub fn origin() -> Point {
        Default::default()
    }

    pub fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }

    pub fn distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

pub struct HeightMap {
    pub heights: Vec<u8>,
    pub width: usize,
    pub height: usize,
    pub dest: Point,
}

impl HeightMap {
    pub fn at(&self, p: &Point) -> u8 {
        self.heights[p.x + p.y * self.width]
    }

    pub fn shortest_path_len(&self, from: &Point, to: &Point) -> Option<usize> {
        astar(
            from,
            |p| self.neighbors(p).into_iter().map(|loc| (loc, 1)),
            |p| p.distance(to),
            |p| p == to,
        )
        .map(|(path, _)| path.len() - 1)
    }

    pub fn neighbors(&self, p: &Point) -> Vec<Point> {
        let mut neighbors = Vec::new();

        if p.x > 0 {
            neighbors.push(Point::new(p.x - 1, p.y));
        }

        if p.x < self.width - 1 {
            neighbors.push(Point::new(p.x + 1, p.y));
        }

        if p.y > 0 {
            neighbors.push(Point::new(p.x, p.y - 1));
        }

        if p.y < self.height - 1 {
            neighbors.push(Point::new(p.x, p.y + 1));
        }

        neighbors
            .into_iter()
            .filter_map(|loc| {
                if self.at(p) >= self.at(&loc) || self.at(p).abs_diff(self.at(&loc)) <= 1 {
                    Some(loc)
                } else {
                    None
                }
            })
            .collect()
    }
}

impl<R: io::Read> TryFrom<io::BufReader<R>> for HeightMap {
    type Error = String;

    fn try_from(value: io::BufReader<R>) -> Result<Self, Self::Error> {
        let mut dest = Point::origin();
        let mut heights: Vec<u8> = Vec::new();

        let mut lines = value.lines().filter_map(|line| line.ok()).peekable();
        let width = lines.peek().ok_or("Nothing to read")?.len();

        for (y, line) in lines.enumerate() {
            heights.extend(
                line.bytes()
                    .enumerate()
                    .map(|(x, h)| match h {
                        b'S' => Ok(0),
                        b'E' => {
                            dest = Point::new(x, y);
                            Ok(b'z' - b'a')
                        }
                        b'a'..=b'z' => Ok(h - b'a'),
                        _ => Err(format!("Invalid height: {}", h)),
                    })
                    .collect::<Result<Vec<u8>, _>>()?,
            );
        }

        let height = heights.len() / width;

        Ok(HeightMap {
            heights,
            width,
            height,
            dest,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;

    let reader = io::BufReader::new(input);
    let map: HeightMap = reader.try_into()?;

    let shortest_path_len = (0..map.width)
        .map(|x| (0..map.height).map(move |y| Point::new(x, y)))
        .flatten()
        .filter_map(|p| {
            (map.at(&p) == 0)
                .then_some(p)
                .and_then(|p| map.shortest_path_len(&p, &map.dest))
        })
        .min()
        .unwrap();

    println!("{}", shortest_path_len);

    Ok(())
}
