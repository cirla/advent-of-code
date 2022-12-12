use std::collections::VecDeque;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

use itertools::Itertools;

pub enum Operator {
    Add,
    Multiply,
}

impl TryFrom<&str> for Operator {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "+" => Ok(Operator::Add),
            "*" => Ok(Operator::Multiply),
            _ => Err(format!("Invalid operator: {}", value)),
        }
    }
}

pub enum Operand {
    Old,
    Literal(u32),
}

impl TryFrom<&str> for Operand {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "old" => Ok(Operand::Old),
            _ => value
                .parse()
                .map(|x: u32| Operand::Literal(x))
                .map_err(|_| format!("Invalid operand: {}", value)),
        }
    }
}

pub struct Operation {
    pub left: Operand,
    pub right: Operand,
    pub op: Operator,
}

impl Operation {
    fn apply(&self, x: u32) -> u32 {
        let left = match self.left {
            Operand::Old => x,
            Operand::Literal(y) => y,
        };

        let right = match self.right {
            Operand::Old => x,
            Operand::Literal(y) => y,
        };

        match self.op {
            Operator::Add => left + right,
            Operator::Multiply => left * right,
        }
    }
}

impl TryFrom<String> for Operation {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parts = value
            .trim_start_matches("  Operation: new = ")
            .split_whitespace();

        let left: Operand = parts
            .next()
            .map(|x| x.try_into().ok())
            .flatten()
            .ok_or("Invalid left operand")?;

        let op: Operator = parts
            .next()
            .map(|x| x.try_into().ok())
            .flatten()
            .ok_or("Invalid operation")?;

        let right: Operand = parts
            .next()
            .map(|x| x.try_into().ok())
            .flatten()
            .ok_or("Invalid right operand")?;

        Ok(Operation { left, right, op })
    }
}

pub enum Condition {
    DivisibleBy(u32),
}

impl Condition {
    fn applies_to(&self, x: u32) -> bool {
        match *self {
            Self::DivisibleBy(y) => x % y == 0,
        }
    }
}

impl TryFrom<String> for Condition {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        value
            .trim_start_matches("  Test: divisible by ")
            .parse()
            .map(|x| Condition::DivisibleBy(x))
            .map_err(|_| format!("Invalid condition: {}", value))
    }
}

pub struct Test {
    condition: Condition,
    true_throw: usize,
    false_throw: usize,
}

pub struct Monkey {
    items: VecDeque<u32>,
    operation: Operation,
    test: Test,
    inspect_count: usize,
}

impl TryFrom<Vec<String>> for Monkey {
    type Error = String;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let mut iter = value.into_iter();

        let items = iter
            .by_ref()
            .skip(1)
            .next()
            .map(|line| {
                line.split(": ")
                    .skip(1)
                    .next()
                    .map(|items| {
                        items
                            .split(", ")
                            .map(|x| x.parse())
                            .collect::<Result<VecDeque<u32>, _>>()
                            .ok()
                    })
                    .flatten()
            })
            .flatten()
            .ok_or("Cannot parse items")?;

        let operation = iter
            .by_ref()
            .next()
            .map(|line| line.try_into().ok())
            .flatten()
            .ok_or("Cannot parse operation")?;

        let condition = iter
            .by_ref()
            .next()
            .map(|line| line.try_into().ok())
            .flatten()
            .ok_or("Cannot parse test condition")?;

        let true_throw = iter
            .by_ref()
            .next()
            .map(|line| {
                line.trim_start_matches("    If true: throw to monkey ")
                    .parse()
                    .ok()
            })
            .flatten()
            .ok_or("Cannot parse true case")?;

        let false_throw = iter
            .by_ref()
            .next()
            .map(|line| {
                line.trim_start_matches("    If false: throw to monkey ")
                    .parse()
                    .ok()
            })
            .flatten()
            .ok_or("Cannot parse false case")?;

        Ok(Monkey {
            items,
            operation,
            test: Test {
                condition,
                true_throw,
                false_throw,
            },
            inspect_count: 0,
        })
    }
}

pub struct Monkeys {
    pub monkeys: Vec<Monkey>,
}

impl Monkeys {
    fn throw(&mut self) {
        // just going wild with indexing here since doing a let binding
        // makes an immutable ref to self, preventing mutable borrows after.
        // don't feel like fighting with the borrow checker or using RefCell
        for i in 0..self.monkeys.len() {
            let n = self.monkeys[i].items.len();
            for _ in 0..n {
                self.monkeys[i].inspect_count += 1;

                let mut item = self.monkeys[i].items.pop_front().unwrap();
                item = self.monkeys[i].operation.apply(item) / 3;

                let throw_to = if self.monkeys[i].test.condition.applies_to(item) {
                    self.monkeys[i].test.true_throw
                } else {
                    self.monkeys[i].test.false_throw
                };

                self.monkeys[throw_to].items.push_back(item);
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;

    let reader = io::BufReader::new(input);

    let mut monkeys = Monkeys {
        monkeys: reader
            .lines()
            .filter_map(|line| line.ok())
            .group_by(|line| !line.is_empty())
            .into_iter()
            .filter_map(|(key, group)| {
                key.then(|| group.collect::<Vec<String>>().try_into().ok())
                    .flatten()
            })
            .collect(),
    };

    for _ in 0..20 {
        monkeys.throw();
    }

    let monkey_business: usize = monkeys
        .monkeys
        .iter()
        .map(|m| m.inspect_count)
        .sorted()
        .rev()
        .take(2)
        .product();

    println!("{:?}", monkey_business);

    Ok(())
}
