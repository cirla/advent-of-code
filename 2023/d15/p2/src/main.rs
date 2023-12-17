use std::array;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};

#[derive(Debug)]
enum Op {
    Assign(u32),
    Remove,
}

#[derive(Debug)]
struct Step {
    label: String,
    op: Op,
}

impl FromStr for Step {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (label, op) = if s.contains('=') {
            let mut parts = s.split('=');
            let label = parts.next().ok_or(anyhow!("Missing label"))?.into();
            let focal_length = parts
                .next()
                .ok_or(anyhow!("Missing focal length"))?
                .parse()
                .context("Invalid focal length")?;
            (label, Op::Assign(focal_length))
        } else {
            let label = s.strip_suffix('-').ok_or(anyhow!("Invalid step"))?.into();
            (label, Op::Remove)
        };

        Ok(Self { label, op })
    }
}

#[derive(Debug)]
struct Lens {
    label: String,
    focal_length: u32,
}

#[derive(Debug)]
struct Boxes {
    boxes: [Vec<Lens>; 256],
}

impl Boxes {
    pub fn new() -> Self {
        Self {
            boxes: array::from_fn(|_| Vec::new()),
        }
    }

    pub fn step(&mut self, step: Step) {
        let r#box = &mut self.boxes[hash(&step.label)];
        match step.op {
            Op::Assign(focal_length) => {
                if let Some(i) = r#box.iter().position(|lens| lens.label == step.label) {
                    r#box[i].focal_length = focal_length;
                } else {
                    r#box.push(Lens {
                        label: step.label,
                        focal_length,
                    });
                }
            }
            Op::Remove => {
                if let Some(i) = r#box.iter().position(|lens| lens.label == step.label) {
                    r#box.remove(i);
                }
            }
        }
    }

    pub fn focusing_power(&self) -> usize {
        self.boxes.iter().enumerate().fold(0, |total, (i, r#box)| {
            total
                + r#box.iter().enumerate().fold(0, |box_total, (j, lens)| {
                    box_total + ((i + 1) * (j + 1) * lens.focal_length as usize)
                })
        })
    }
}

fn hash(s: &str) -> usize {
    s.chars()
        .fold(0, |acc, c| ((acc + (c as usize)) * 17) % 256)
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let line = reader
        .lines()
        .filter_map(|line| line.ok())
        .next()
        .ok_or(anyhow!("Missing input"))?;

    let mut boxes = Boxes::new();

    for step in line.split(',').map(|s| s.parse().context("Invalid step")) {
        boxes.step(step?);
    }

    println!("Focusing Power: {}", boxes.focusing_power());

    Ok(())
}
