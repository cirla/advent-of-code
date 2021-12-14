use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

struct Board {
    numbers: [u8; 25],
    marked: [bool; 25],
}

impl Board {
    pub fn new(numbers: [u8; 25]) -> Board {
        Board {
            numbers: numbers,
            marked: [false; 25],
        }
    }

    pub fn mark(&mut self, num: u8) {
        if let Some(i) = self.numbers.iter().position(|&x| x == num) {
            self.marked[i] = true;
        }
    }

    pub fn winner(&self) -> bool {
        self.marked[0..5].iter().all(|&x| x)
            || self.marked[5..10].iter().all(|&x| x)
            || self.marked[10..15].iter().all(|&x| x)
            || self.marked[15..20].iter().all(|&x| x)
            || self.marked[20..25].iter().all(|&x| x)
            || self.marked.iter().step_by(5).all(|&x| x)
            || self.marked.iter().skip(1).step_by(5).all(|&x| x)
            || self.marked.iter().skip(2).step_by(5).all(|&x| x)
            || self.marked.iter().skip(3).step_by(5).all(|&x| x)
            || self.marked.iter().skip(4).step_by(5).all(|&x| x)
    }

    pub fn score(&self) -> u32 {
        self.marked
            .iter()
            .enumerate()
            .filter(|(_, &x)| !x)
            .map(|(i, _)| self.numbers[i] as u32)
            .sum()
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;
    let reader = io::BufReader::new(input);

    let mut lines = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.is_empty());

    let draws: Vec<u8> = lines
        .next()
        .unwrap()
        .split(",")
        .filter_map(|x| x.parse::<u8>().ok())
        .collect();

    let board_nums: Vec<u8> = lines
        .flat_map(|line| {
            line.split(' ')
                .filter_map(|x| x.parse::<u8>().ok())
                .collect::<Vec<u8>>()
        })
        .collect();

    let mut boards: Vec<Board> = board_nums
        .chunks(25)
        .filter_map(|chunk| {
            chunk
                .try_into()
                .ok()
                .and_then(|arr: [u8; 25]| Some(Board::new(arr)))
        })
        .collect();

    let mut winners = vec![false; boards.len()];
    let mut remaining = boards.len();

    let mut last_idx = 0;
    let mut last_draw = 0;

    for draw in draws {
        for (i, board) in boards.iter_mut().enumerate() {
            if winners[i] {
                continue;
            }

            board.mark(draw);
            if board.winner() {
                winners[i] = true;
                remaining -= 1;
                last_idx = i;
            }
        }

        if remaining == 0 {
            last_draw = draw;
            break;
        }
    }

    println!("{}", boards[last_idx].score() * (last_draw as u32));

    Ok(())
}
