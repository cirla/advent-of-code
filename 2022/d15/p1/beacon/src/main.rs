use std::collections::{BTreeSet, HashMap};
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

#[derive(PartialEq)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(", ");

        let x = parts
            .next()
            .and_then(|s| s.trim_start_matches("x=").parse().ok())
            .ok_or(format!("Invalid point input: {}", s))?;

        let y = parts
            .next()
            .and_then(|s| s.trim_start_matches("y=").parse().ok())
            .ok_or(format!("Invalid point input: {}", s))?;

        Ok(Point::new(x, y))
    }
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct RangeInclusive {
    pub start: isize,
    pub end: isize,
}

impl RangeInclusive {
    pub fn new(start: isize, end: isize) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        (self.end - self.start + 1) as usize
    }
}

pub struct Sensor {
    pub loc: Point,
    pub closest_beacon: Point,
    pub beacon_distance: usize,
}

impl Sensor {
    fn new(loc: Point, closest_beacon: Point) -> Self {
        let beacon_distance = loc.distance(&closest_beacon);

        Self {
            loc,
            closest_beacon,
            beacon_distance,
        }
    }
}

impl FromStr for Sensor {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");

        let loc = parts
            .next()
            .and_then(|s| s.trim_start_matches("Sensor at ").parse().ok())
            .ok_or(format!("Invalid sensor input: {}", s))?;

        let closest_beacon = parts
            .next()
            .and_then(|s| s.trim_start_matches("closest beacon is at ").parse().ok())
            .ok_or(format!("Invalid sensor input: {}", s))?;

        Ok(Sensor::new(loc, closest_beacon))
    }
}

pub struct Tunnels {
    sensors: Vec<Sensor>,
    known_not_empty: HashMap<isize, BTreeSet<isize>>,
}

impl Tunnels {
    pub fn new(sensors: Vec<Sensor>) -> Self {
        let mut known_not_empty: HashMap<isize, BTreeSet<isize>> = HashMap::new();

        // keep track of non-empty columns indexed by row
        for s in sensors.iter() {
            known_not_empty.entry(s.loc.y).or_default().insert(s.loc.x);
            known_not_empty
                .entry(s.closest_beacon.y)
                .or_default()
                .insert(s.closest_beacon.x);
        }

        Self {
            sensors,
            known_not_empty,
        }
    }

    pub fn count_empty(&self, y: isize) -> usize {
        let mut ranges = BTreeSet::new();

        // get ranges of x values inside each sensor's exclusive zone
        for s in self.sensors.iter() {
            let closest_point = Point::new(s.loc.x, y);
            let closest_distance = closest_point.distance(&s.loc);

            if closest_distance <= s.beacon_distance {
                let offset = (s.beacon_distance - closest_distance) as isize;
                ranges.insert(RangeInclusive::new(s.loc.x - offset, s.loc.x + offset));
            }
        }

        if ranges.is_empty() {
            0
        } else {
            // merge overlapping ranges
            let mut merged_ranges = Vec::new();
            merged_ranges.push(ranges.pop_first().unwrap());

            for r in ranges.into_iter() {
                let top = merged_ranges.last_mut().unwrap();
                if top.end < r.start {
                    merged_ranges.push(r);
                } else if top.end < r.end {
                    top.end = r.end;
                }
            }

            // sum lengths of ranges, accounting for sensors within the ranges
            let not_empty = self.known_not_empty.get(&y);
            merged_ranges
                .iter()
                .map(|r| {
                    r.len()
                        - not_empty
                            .map(|x| x.range(r.start..=r.end).count())
                            .unwrap_or(0)
                })
                .sum()
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let row: isize = args[2].parse()?;

    let reader = io::BufReader::new(input);

    let sensors = reader
        .lines()
        .filter_map(|line| line.ok().and_then(|line| line.parse().ok()))
        .collect();

    let tunnels = Tunnels::new(sensors);
    println!("{}", tunnels.count_empty(row));

    Ok(())
}
