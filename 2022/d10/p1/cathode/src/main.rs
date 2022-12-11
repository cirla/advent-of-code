use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

pub enum Instruction {
    NoOp,
    AddX(i32),
}

impl Instruction {
    pub fn cycles(&self) -> usize {
        match *self {
            Self::NoOp => 1,
            Self::AddX(_) => 2,
        }
    }
}

impl TryFrom<String> for Instruction {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();

        match parts.next() {
            Some("noop") => Ok(Instruction::NoOp),
            Some("addx") => {
                let op = parts
                    .next()
                    .map(|x| x.parse().ok())
                    .flatten()
                    .ok_or(format!("Invalid instruction input: {}", value))?;
                Ok(Instruction::AddX(op))
            }
            _ => Err(format!("Invalid instruction input: {}", value)),
        }
    }
}

pub struct Cpu {
    pub x: i32,
    pub cycles: usize,
    pub sampled_signals: Vec<isize>,
    next_sample: usize,
}

impl Cpu {
    const SAMPLE_START: usize = 20;
    const SAMPLE_FREQ: usize = 40;

    fn new() -> Cpu {
        Cpu {
            x: 1,
            cycles: 0,
            sampled_signals: Vec::new(),
            next_sample: Self::SAMPLE_START,
        }
    }

    fn signal_strength(&self) -> isize {
        (self.x as isize) * (self.cycles as isize)
    }

    fn execute(&mut self, inst: Instruction) {
        let inst_cycles = inst.cycles();
        let post_cycles = self.cycles + inst_cycles;

        if post_cycles >= self.next_sample {
            self.cycles = self.next_sample;
            self.next_sample += Self::SAMPLE_FREQ;
            self.sampled_signals.push(self.signal_strength());
        }

        match inst {
            Instruction::NoOp => {}
            Instruction::AddX(x) => {
                self.x += x;
            }
        }

        self.cycles = post_cycles;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;

    let reader = io::BufReader::new(input);

    let mut cpu = Cpu::new();
    for line in reader.lines().filter_map(|line| line.ok()) {
        cpu.execute(line.try_into()?);
    }

    println!("{}", cpu.sampled_signals.iter().sum::<isize>());

    Ok(())
}
