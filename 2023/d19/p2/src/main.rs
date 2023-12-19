use std::cmp;
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter;
use std::str::FromStr;

use anyhow::{anyhow, Context, Error, Result};

#[derive(Debug)]
enum Op {
    GreaterThan(usize),
    LessThan(usize),
}

#[derive(Debug)]
struct ConditionResult {
    category: char,
    matching_range: RangeInclusive,
}

#[derive(Debug)]
struct Condition {
    category: char,
    op: Op,
}

impl Condition {
    pub fn process(&self, part: &Part) -> Option<ConditionResult> {
        let rating = &part.ratings[&self.category];
        let matching_range = match self.op {
            Op::GreaterThan(x) => RangeInclusive::new(cmp::max(x + 1, rating.start), rating.end),
            Op::LessThan(x) => RangeInclusive::new(rating.start, cmp::min(x - 1, rating.end)),
        };

        (!matching_range.is_empty()).then(|| ConditionResult {
            category: self.category,
            matching_range,
        })
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
struct RuleResult {
    accepted: Option<Part>,
    unmatched: Option<Part>,
    workflow: Option<(String, Part)>,
}

#[derive(Debug)]
struct Rule {
    condition: Option<Condition>,
    next: Next,
}

impl Rule {
    pub fn process(&self, part: &Part) -> Option<RuleResult> {
        let (matching_part, unmatched_part) = match &self.condition {
            Some(c) => match c.process(part) {
                Some(cr) => {
                    let unmatched_range = part.ratings[&cr.category].diff(&cr.matching_range);

                    (
                        Some(part.with_rating(cr.category, cr.matching_range)),
                        Some(part.with_rating(cr.category, unmatched_range)),
                    )
                }
                None => (None, Some(part.clone())),
            },
            None => (Some(part.clone()), None),
        };

        match &self.next {
            Next::Accepted => matching_part.map(|matched| RuleResult {
                accepted: Some(matched),
                unmatched: unmatched_part,
                workflow: None,
            }),
            Next::Rejected => unmatched_part.map(|unmatched| RuleResult {
                accepted: None,
                unmatched: Some(unmatched),
                workflow: None,
            }),
            Next::Workflow(w) => matching_part.map(|matched| RuleResult {
                accepted: None,
                unmatched: unmatched_part,
                workflow: Some((w.to_string(), matched)),
            }),
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
struct WorkflowResult {
    accepted: Vec<Part>,
    unresolved: Vec<(String, Part)>,
}

impl WorkflowResult {
    pub fn new() -> Self {
        Self {
            accepted: Vec::new(),
            unresolved: Vec::new(),
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}

impl Workflow {
    pub fn process(&self, part: &Part) -> WorkflowResult {
        // results this workflow is done with
        let mut workflow_res = WorkflowResult::new();

        // results this workflow is not done with
        let mut unresolved = vec![RuleResult {
            accepted: None,
            unmatched: Some(part.clone()),
            workflow: None,
        }];

        // apply rules to remaining unresolved, potentially creating
        // more unresolved
        for rule in &self.rules {
            unresolved = unresolved
                .into_iter()
                .filter_map(|res| {
                    // split out accepted -- nothing more to do
                    if let Some(accepted) = res.accepted {
                        workflow_res.accepted.push(accepted);
                    };

                    // split out workflow -- nothing more to do
                    if let Some((w, p)) = res.workflow {
                        workflow_res.unresolved.push((w, p));
                    };

                    // process unmatched with rule
                    res.unmatched.and_then(|u| rule.process(&u))
                })
                .collect::<Vec<_>>();
        }

        // the final rule should have matched the remainder
        for res in unresolved.into_iter() {
            if let Some(accepted) = res.accepted {
                assert!(res.unmatched.is_none());
                assert!(res.workflow.is_none());
                workflow_res.accepted.push(accepted);
            } else if let Some((w, p)) = res.workflow {
                assert!(res.unmatched.is_none());
                assert!(res.accepted.is_none());
                workflow_res.unresolved.push((w, p));
            } else {
                unreachable!("Should not be any unmatched left");
            }
        }

        workflow_res
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

    pub fn accepted_combinations(&self) -> usize {
        // initially everything is unresolved and queued for the "in" workflow
        let mut res = WorkflowResult {
            accepted: Vec::new(),
            unresolved: vec![("in".into(), Part::new())],
        };

        // propagate unresolved to workflows until all are resolved
        while !res.unresolved.is_empty() {
            res.unresolved = res
                .unresolved
                .into_iter()
                .flat_map(|(w, p)| {
                    // propagate to workflow and consume result
                    let r = self.workflows[&w].process(&p);
                    res.accepted.extend(r.accepted.into_iter());
                    r.unresolved.into_iter()
                })
                .collect();
        }

        res.accepted.into_iter().map(|p| p.combinations()).sum()
    }
}

// std::ops::RangeInclusive doesn't implement Copy
#[derive(Clone, Copy, Debug)]
struct RangeInclusive {
    pub start: usize,
    pub end: usize,
}

impl RangeInclusive {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    pub fn len(&self) -> usize {
        self.end - self.start + 1
    }

    pub fn is_empty(&self) -> bool {
        self.start > self.end
    }

    fn diff(&self, other: &Self) -> Self {
        // assumes other is a subrange of self with either the same start
        // or the same end created by processing a Part through a Condition
        if self.start == other.start {
            Self::new(other.end + 1, self.end)
        } else {
            assert!(self.end == other.end);
            Self::new(self.start, other.start - 1)
        }
    }
}

#[derive(Clone, Debug)]
struct Part {
    ratings: HashMap<char, RangeInclusive>,
}

impl Part {
    const MIN_RATING: usize = 1;
    const MAX_RATING: usize = 4000;

    pub fn new() -> Self {
        Self {
            ratings: HashMap::from_iter("xmas".chars().zip(iter::repeat(RangeInclusive::new(
                Self::MIN_RATING,
                Self::MAX_RATING,
            )))),
        }
    }

    fn with_rating(&self, category: char, rating: RangeInclusive) -> Self {
        Self {
            ratings: self
                .ratings
                .iter()
                .map(|(c, r)| (*c, if *c == category { rating } else { *r }))
                .collect(),
        }
    }

    pub fn combinations(&self) -> usize {
        self.ratings.values().map(|r| r.len()).product()
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let workflows = Workflows::try_from_lines(
        reader
            .lines()
            .filter_map(|line| line.ok())
            .take_while(|line| !line.is_empty()),
    )?;

    println!(
        "Distinct accepted combinations: {}",
        workflows.accepted_combinations()
    );

    Ok(())
}
