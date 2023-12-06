use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};
use itertools::Itertools;
use rayon::prelude::*;

/// std::ops::Range<usize> doesn't implement Ord
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Range {
    /// start of range, inclusive
    start: usize,
    /// end of range, exclusive
    end: usize,
}

impl Range {
    pub fn new(start: usize, len: usize) -> Self {
        Self {
            start,
            end: start + len,
        }
    }

    pub fn contains(&self, idx: usize) -> bool {
        self.start <= idx && idx < self.end
    }
}

/// Maps a source range to destination range of equal length
#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct RangeMapping {
    /// source range
    src: Range,
    /// destination range
    dst: Range,
}

impl RangeMapping {
    pub fn new(src_start: usize, dst_start: usize, len: usize) -> Self {
        Self {
            src: Range::new(src_start, len),
            dst: Range::new(dst_start, len),
        }
    }

    pub fn convert(&self, idx: usize) -> Option<usize> {
        self.src
            .contains(idx)
            .then(|| match self.src.start.cmp(&self.dst.start) {
                Ordering::Less => idx + (self.dst.start - self.src.start),
                Ordering::Greater => idx - (self.src.start - self.dst.start),
                Ordering::Equal => idx,
            })
    }
}

impl FromStr for RangeMapping {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(" ");
        let dst_start = parts
            .next()
            .ok_or(anyhow!("Missing destination range start"))?
            .parse()
            .context("Invalid destination range start")?;

        let src_start = parts
            .next()
            .ok_or(anyhow!("Missing source range start"))?
            .parse()
            .context("Invalid source range start")?;

        let len = parts
            .next()
            .ok_or(anyhow!("Missing range length"))?
            .parse()
            .context("Invalid range length")?;

        Ok(RangeMapping::new(src_start, dst_start, len))
    }
}

#[derive(Debug, Default)]
struct Map {
    ranges: BTreeSet<RangeMapping>,
}

impl Map {
    pub fn try_from_lines<I: IntoIterator<Item = String>>(lines: I) -> Result<Self> {
        let ranges = lines
            .into_iter()
            // skip header; maps are always in order in the input so we don't need to care about the name
            .skip(1)
            // input shouldn't have overlapping ranges as there's no way to determine precedence
            .map(|line| line.parse())
            .collect::<Result<_>>()?;

        Ok(Self { ranges })
    }

    pub fn get(&self, idx: usize) -> usize {
        self.ranges
            .iter()
            .filter_map(|r| r.convert(idx))
            .next()
            .unwrap_or(idx)
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<Range>,
    maps: Vec<Map>,
}

impl Almanac {
    pub fn try_from_lines<I: IntoIterator<Item = String>>(lines: I) -> Result<Self> {
        // group lines separated by empty lines
        let groups = lines.into_iter().group_by(|x| !x.is_empty());
        let mut groups = groups
            .into_iter()
            .filter_map(|(not_empty, group)| not_empty.then(|| group));

        // first group is seeds line
        let seeds: Vec<usize> = groups
            .next()
            .ok_or(anyhow!("Missing seeds group"))?
            .next()
            .ok_or(anyhow!("Missing seeds line"))?
            .strip_prefix("seeds: ")
            .ok_or(anyhow!("missing 'seeds:' prefix"))?
            .split(" ")
            .map(|s| s.parse().context("Invalid seed value"))
            .collect::<Result<_>>()?;

        let seeds = seeds
            .chunks_exact(2)
            .map(|c| Range::new(c[0], c[1]))
            .collect();

        // remaining groups are maps
        let maps = groups
            .map(|g| Map::try_from_lines(g))
            .collect::<Result<_>>()?;

        Ok(Self { seeds, maps })
    }

    pub fn locations(&self) -> impl Iterator<Item = usize> {
        let seeds = self
            .seeds
            .iter()
            .flat_map(|s| s.start..s.end)
            .collect::<Vec<_>>();

        self.maps
            .iter()
            .fold(seeds, |mut acc, map| {
                acc.par_iter_mut().for_each(|i| *i = map.get(*i));
                acc
            })
            .into_iter()
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let almanac = Almanac::try_from_lines(reader.lines().filter_map(|line| line.ok()))?;

    println!("Minimum Location: {}", almanac.locations().min().unwrap());

    Ok(())
}
