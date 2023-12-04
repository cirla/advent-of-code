use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::ops::Range;
use std::str::FromStr;

use anyhow::{Context, Result};
use bit_set::BitSet;

/// Number with location in row
#[derive(Debug)]
struct Number {
    pub value: u32,
    pub start: usize,
    pub len: usize,
}

impl Number {
    /// Range of column indices this number occupies
    pub fn range(&self) -> Range<usize> {
        self.start..(self.start + self.len)
    }
}

/// Schematic row
#[derive(Debug)]
struct Row {
    /// numbers occurring in the row
    numbers: Vec<Number>,
    /// bits are set for indices containing numeric digits
    number_set: BitSet,
    /// bits are set for indices containing or adjacent to a symbol
    symbol_set: BitSet,
}

impl Row {
    /// Create a new row from the given numbers and symbol indices
    pub fn new<S: IntoIterator<Item = usize>>(numbers: Vec<Number>, symbol_indices: S) -> Self {
        // bits are set for indices containing numeric digits
        let number_set = BitSet::from_iter(numbers.iter().flat_map(Number::range));

        // bits are set for indices containing or adjacent to a symbol
        let symbol_set =
            BitSet::from_iter(symbol_indices.into_iter().flat_map(|i| (i - 1)..=(i + 1)));

        Self {
            numbers,
            number_set,
            symbol_set,
        }
    }
}

impl FromStr for Row {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // partition into digits and symbols
        let (digits, symbols): (Vec<_>, Vec<_>) = s
            .chars()
            .enumerate()
            .filter_map(|(i, c)| (c != '.').then_some((i, c)))
            .partition(|(_, c)| c.is_digit(10));

        // accumulate adjacent digits into numbers
        let numbers = digits.iter().fold(Vec::new(), |mut acc, (i, c)| {
            let value = c.to_digit(10).expect("already checked is_digit");
            acc.last_mut()
                .and_then(|last: &mut Number| {
                    // update last if adjacent
                    (*i == last.start + last.len).then(|| {
                        last.value = last.value * 10 + value;
                        last.len += 1;
                    })
                })
                .unwrap_or_else(|| {
                    // add as new number
                    acc.push(Number {
                        value,
                        start: *i,
                        len: 1,
                    });
                });

            acc
        });

        // discard symbols, we just need indices
        let symbol_indices = symbols.iter().map(|(i, _)| *i);

        Ok(Self::new(numbers, symbol_indices))
    }
}

/// Engine schematic
#[derive(Debug)]
struct Schematic {
    rows: Vec<Row>,
}

impl Schematic {
    pub fn new(rows: Vec<Row>) -> Self {
        Self { rows }
    }

    pub fn part_numbers<'a>(&'a self) -> impl Iterator<Item = u32> + 'a {
        // self.rows.windows(3) would also work here, but got more verbose
        // with the special casing for first and last window
        (0..self.rows.len()).flat_map(|i| {
            let curr = &self.rows[i];

            // create a mask of the adjacent symbol sets
            let mut mask = curr.symbol_set.clone();
            if i > 0 {
                mask.union_with(&self.rows[i - 1].symbol_set);
            }
            if i < self.rows.len() - 1 {
                mask.union_with(&self.rows[i + 1].symbol_set);
            }

            // find where it overlaps with digits
            mask.intersect_with(&curr.number_set);

            // get the numbers at the intersections
            curr.numbers
                .iter()
                .filter_map(move |n| n.range().any(|x| mask.contains(x)).then_some(n.value))
        })
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let schematic = Schematic::new(
        reader
            .lines()
            .filter_map(|line| line.ok())
            .enumerate()
            .map(|(i, line)| {
                line.parse()
                    .with_context(|| format!("Couldn't parse Row on line {}", i + 1))
            })
            .collect::<Result<Vec<Row>>>()?,
    );

    let sum: u32 = schematic.part_numbers().sum();

    println!("Sum of Part Numbers: {}", sum);

    Ok(())
}
