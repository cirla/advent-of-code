use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Copy, Clone)]
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
    pub program: Vec<Instruction>,
    pub inst_counter: usize,
    pipeline: usize,
}

impl Cpu {
    const PIXEL_WIDTH: usize = 40;
    const PIXEL_HEIGHT: usize = 6;

    pub fn with_program(program: Vec<Instruction>) -> Cpu {
        let pipeline = program[0].cycles();

        Cpu {
            x: 1,
            cycles: 0,
            program: program,
            inst_counter: 0,
            pipeline,
        }
    }

    pub fn render(&mut self) {
        for _ in 0..Self::PIXEL_HEIGHT {
            println!(
                "{}",
                (0..Self::PIXEL_WIDTH)
                    .map(|_| { self.tick() })
                    .collect::<String>()
            );
        }
    }

    fn tick(&mut self) -> char {
        let col = (self.cycles % Self::PIXEL_WIDTH) as isize;
        let pixel = if (col as i32 - self.x).abs() > 1 {
            '.'
        } else {
            '#'
        };

        self.cycles += 1;
        self.pipeline -= 1;

        if self.pipeline == 0 {
            self.execute();
            self.inst_counter += 1;
            self.pipeline = self.curr_inst().map(|i| i.cycles()).unwrap_or(0);
        }

        pixel
    }

    fn curr_inst(&self) -> Option<&Instruction> {
        self.program.get(self.inst_counter)
    }

    fn execute(&mut self) {
        match self.curr_inst().cloned() {
            Some(Instruction::AddX(x)) => {
                self.x += x;
            }
            _ => {}
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;

    let reader = io::BufReader::new(input);

    let program: Vec<Instruction> = reader
        .lines()
        .filter_map(|line| line.ok().and_then(|line| line.try_into().ok()))
        .collect();

    Cpu::with_program(program).render();

    Ok(())
}
