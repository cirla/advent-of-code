use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

const TOTAL_SPACE: usize = 70_000_000;
const DESIRED_SPACE: usize = 30_000_000;

enum Output {
    Dir(String),
    File(String, usize),
}

impl TryFrom<String> for Output {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parts = value.split_whitespace();
        match parts.next().unwrap() {
            "dir" => Ok(Output::Dir(parts.next().unwrap().into())),
            size => Ok(Output::File(
                parts.next().unwrap().into(),
                size.parse()
                    .map_err(|_| format!("Invalid output: {}", value))?,
            )),
        }
    }
}

struct ChangeDir {
    dir: String,
}

impl ChangeDir {
    const ROOT: &str = "/";
    const UP: &str = "..";
}

#[derive(Default)]
struct ListDir {
    output: Vec<Output>,
}

enum Command {
    ChangeDir(ChangeDir),
    ListDir(ListDir),
}

impl TryFrom<String> for Command {
    type Error = String;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.starts_with("$") {
            let mut parts = value.split_whitespace().skip(1);
            match parts.next().unwrap() {
                "cd" => Ok(Command::ChangeDir(ChangeDir {
                    dir: parts.next().unwrap().into(),
                })),
                "ls" => Ok(Command::ListDir(Default::default())),
                _ => Err(format!("Invalid command: {}", value)),
            }
        } else {
            Err(format!("Invalid command: {}", value))
        }
    }
}

#[derive(Default)]
struct Terminal {
    pwd: Vec<String>,
    dir_sizes: HashMap<String, usize>,
}

impl Terminal {
    fn process(&mut self, cmd: &Command) {
        match cmd {
            Command::ChangeDir(cd) => match cd.dir.as_str() {
                ChangeDir::ROOT => self.pwd.clear(),
                ChangeDir::UP => {
                    self.pwd.pop();
                }
                dir => self.pwd.push(dir.into()),
            },
            Command::ListDir(ls) => {
                let size = ls
                    .output
                    .iter()
                    .map(|o| match o {
                        Output::Dir(_) => 0,
                        Output::File(_, size) => *size,
                    })
                    .sum::<usize>();

                for i in 0..self.pwd.len() + 1 {
                    let dir = Terminal::fmt_dirs(&self.pwd[0..i]);
                    self.dir_sizes
                        .entry(dir)
                        .and_modify(|e| *e += size)
                        .or_insert(size);
                }
            }
        }
    }

    fn fmt_dirs(dirs: &[String]) -> String {
        format!("/{}", dirs.join(ChangeDir::ROOT))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;

    let reader = io::BufReader::new(input);
    let mut commands: Vec<Command> = Vec::new();

    for line in reader.lines().filter_map(|line| line.ok()) {
        match line.clone().try_into() {
            Ok(cmd) => commands.push(cmd),
            Err(_) => match commands.last_mut().unwrap() {
                Command::ListDir(c) => {
                    c.output.push(line.try_into()?);
                }
                _ => panic!("Unreachable"),
            },
        }
    }

    let mut term: Terminal = Default::default();
    for cmd in commands.iter() {
        term.process(cmd);
    }

    let free_space = TOTAL_SPACE - term.dir_sizes[ChangeDir::ROOT];
    let threshold = DESIRED_SPACE - free_space;

    let min_size: usize = term
        .dir_sizes
        .values()
        .copied()
        .filter(|x| *x >= threshold)
        .min()
        .unwrap();

    println!("{}", min_size);

    Ok(())
}
