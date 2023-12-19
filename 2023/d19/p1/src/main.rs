use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};

#[derive(Debug)]
enum Op {
    GreaterThan(usize),
    LessThan(usize),
}

#[derive(Debug)]
struct Condition {
    category: char,
    op: Op,
}

impl Condition {
    pub fn eval(&self, part: &Part) -> bool {
        match self.op {
            Op::GreaterThan(x) => part.ratings[&self.category] > x,
            Op::LessThan(x) => part.ratings[&self.category] < x,
        }
    }
}

impl FromStr for Condition {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (mut parts, op) = if s.contains('>') {
            (s.split('>'), Op::GreaterThan(0))
        } else {
            (s.split('<'), Op::LessThan(0))
        };

        let category = parts
            .next()
            .ok_or(anyhow!("Missing category"))?
            .chars()
            .next()
            .ok_or(anyhow!("Empty category"))?;

        let value = parts
            .next()
            .ok_or(anyhow!("Missing value"))?
            .parse()
            .context("Invalid value")?;

        let op = match op {
            Op::GreaterThan(_) => Op::GreaterThan(value),
            Op::LessThan(_) => Op::LessThan(value),
        };

        Ok(Self { category, op })
    }
}

#[derive(Debug)]
enum Next {
    Accepted,
    Rejected,
    Workflow(String),
}

impl FromStr for Next {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "A" => Next::Accepted,
            "R" => Next::Rejected,
            w => Next::Workflow(w.into()),
        })
    }
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    next: Next,
}

impl Rule {
    pub fn process(&self, part: &Part) -> Option<&Next> {
        match &self.condition {
            Some(c) => {
                if c.eval(part) {
                    Some(&self.next)
                } else {
                    None
                }
            }
            None => Some(&self.next),
        }
    }
}

impl FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.contains(':') {
            let mut parts = s.split(':');
            let condition = Some(
                parts
                    .next()
                    .ok_or(anyhow!("Missing condition"))?
                    .parse()
                    .context("Invalid condition")?,
            );

            let next = parts
                .next()
                .ok_or(anyhow!("Missing result"))?
                .parse()
                .context("Invalid rule result")?;

            Ok(Self { condition, next })
        } else {
            let next = s.parse().context("Invalid rule result")?;
            Ok(Self {
                condition: None,
                next,
            })
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    pub fn process(&self, part: &Part) -> &Next {
        self.rules
            .iter()
            .filter_map(|r| r.process(part))
            .next()
            .expect("There should always be a matching rule")
    }
}

impl FromStr for Workflow {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.split(&[',', '{', '}']);

        let name = parts.next().ok_or(anyhow!("Missing workflow name"))?.into();
        let rules = parts
            .filter_map(|p| (!p.is_empty()).then(|| p.parse().context("Invalid rule")))
            .collect::<Result<_>>()?;

        Ok(Self { name, rules })
    }
}

#[derive(Debug)]
struct Workflows {
    workflows: HashMap<String, Workflow>,
}

impl Workflows {
    pub fn try_from_lines<I: IntoIterator<Item = String>>(lines: I) -> Result<Self> {
        let workflows = lines
            .into_iter()
            .map(|line| {
                line.parse()
                    .context("Invalid workflow")
                    .and_then(|w: Workflow| Ok((w.name.clone(), w)))
            })
            .collect::<Result<_>>()?;

        Ok(Self { workflows })
    }

    pub fn process(&self, part: &Part) -> bool {
        let mut workflow = &self.workflows["in"];
        loop {
            match workflow.process(part) {
                Next::Accepted => {
                    return true;
                }
                Next::Rejected => {
                    return false;
                }
                Next::Workflow(w) => {
                    workflow = &self.workflows[w];
                }
            }
        }
    }
}

#[derive(Debug)]
struct Part {
    ratings: HashMap<char, usize>,
}

impl Part {
    pub fn total_rating(&self) -> usize {
        self.ratings.values().sum()
    }
}

impl FromStr for Part {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let ratings = s
            .split(&[',', '{', '}'])
            .filter_map(|r| {
                (!r.is_empty()).then(|| {
                    let mut parts = r.split('=');
                    let category = parts
                        .next()
                        .ok_or(anyhow!("Missing category"))?
                        .chars()
                        .next()
                        .ok_or(anyhow!("Empty category"))?;
                    let rating = parts
                        .next()
                        .ok_or(anyhow!("Missing rating"))?
                        .parse()
                        .context("Invalid rating")?;
                    Ok((category, rating))
                })
            })
            .collect::<Result<_>>()?;

        Ok(Self { ratings })
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let mut lines = reader.lines().filter_map(|line| line.ok());
    let workflows = Workflows::try_from_lines(lines.by_ref().take_while(|line| !line.is_empty()))?;

    let parts: Vec<Part> = lines
        .map(|line| line.parse().context("Invalid Part"))
        .collect::<Result<_>>()?;

    let sum: usize = parts
        .iter()
        .filter_map(|p| workflows.process(p).then(|| p.total_rating()))
        .sum();

    println!("Sum of Ratings: {}", sum);

    Ok(())
}
