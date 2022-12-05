use std::cell::RefCell;
use std::collections::HashMap;
use std::convert::TryFrom;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

struct Instruction {
    source: char,
    dest: char,
    amount: usize,
}

impl TryFrom<String> for Instruction {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split_whitespace().collect();

        Ok(Instruction {
            source: parts[3].chars().next().unwrap(),
            dest: parts[5].chars().next().unwrap(),
            amount: parts[1]
                .parse()
                .map_err(|_| format!("Invalid instruction input: {}", value))?,
        })
    }
}

struct Stacks {
    stacks: HashMap<char, RefCell<Vec<char>>>,
    labels: Vec<char>,
}

impl Stacks {
    fn update(&self, instruction: Instruction) {
        let mut source = self.stacks[&instruction.source].borrow_mut();
        let mut dest = self.stacks[&instruction.dest].borrow_mut();
        let offset = source.len() - instruction.amount;
        dest.extend(source.drain(offset..).rev())
    }

    fn tops(&self) -> String {
        self.labels
            .iter()
            .map(|label| self.stacks[label].borrow().last().unwrap().clone())
            .collect()
    }
}

impl TryFrom<Vec<String>> for Stacks {
    type Error = String;

    fn try_from(mut value: Vec<String>) -> Result<Self, Self::Error> {
        let raw_labels = value.pop().unwrap();
        let labels: Vec<char> = raw_labels
            .split_whitespace()
            .map(|x| x.chars().next().unwrap())
            .collect();

        let mut stacks: HashMap<char, RefCell<Vec<char>>> = HashMap::new();
        for level in value.iter().rev() {
            for (i, label) in level
                .chars()
                .skip(1)
                .step_by(4)
                .enumerate()
                .filter(|&(_, c)| c != ' ')
            {
                stacks.entry(labels[i]).or_default().get_mut().push(label)
            }
        }

        Ok(Stacks { stacks, labels })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;

    let reader = io::BufReader::new(input);

    let lines = &mut reader.lines().filter_map(|line| line.ok());
    let stacks: Stacks = lines
        .by_ref()
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>()
        .try_into()?;

    for i in lines.filter_map(|line| line.try_into().ok()) {
        stacks.update(i);
    }

    println!("{}", stacks.tops());

    Ok(())
}
