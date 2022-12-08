use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

pub struct ScenicScore {
    pub north: u32,
    pub south: u32,
    pub east: u32,
    pub west: u32,
}

impl ScenicScore {
    fn score(&self) -> u32 {
        self.north * self.south * self.east * self.west
    }
}

pub struct Tree {
    pub height: u32,
    pub scenic_score: u32,
}

pub struct TreeGrid {
    pub trees: Vec<Tree>,
    pub side_len: usize,
}

impl TreeGrid {
    pub fn new(tree_heights: impl Iterator<Item = u32>) -> Result<TreeGrid, String> {
        let trees: Vec<Tree> = tree_heights
            .map(|height| Tree {
                height,
                scenic_score: 0,
            })
            .collect();
        let side_len = (trees.len() as f32).sqrt() as usize;

        if trees.len() != side_len * side_len {
            Err(format!("Number of trees ({}) is not square", trees.len()))
        } else {
            let mut grid = TreeGrid { trees, side_len };

            grid.score_trees();

            Ok(grid)
        }
    }

    fn interior(&self) -> impl Iterator<Item = usize> {
        // iterator over interior rows/cols
        1..self.side_len - 1
    }

    pub fn tree(&self, row: usize, col: usize) -> &Tree {
        &self.trees[row * self.side_len + col]
    }

    fn tree_mut(&mut self, row: usize, col: usize) -> &mut Tree {
        &mut self.trees[row * self.side_len + col]
    }

    fn score_trees(&mut self) {
        // edge trees are always 0 so can be ignored
        for row in self.interior() {
            for col in self.interior() {
                self.score(row, col)
            }
        }
    }

    fn score(&mut self, row: usize, col: usize) {
        let height = self.tree(row, col).height;

        // initialize to highest possible scores
        let mut score = ScenicScore {
            north: row as u32,
            south: (self.side_len - 1 - row) as u32,
            east: (self.side_len - 1 - col) as u32,
            west: col as u32,
        };

        // north
        for r in (0..row).rev() {
            if self.tree(r, col).height >= height {
                score.north = (row - r) as u32;
                break;
            }
        }

        // south
        for r in (row..self.side_len).skip(1) {
            if self.tree(r, col).height >= height {
                score.south = (r - row) as u32;
                break;
            }
        }

        // east
        for c in (col..self.side_len).skip(1) {
            if self.tree(row, c).height >= height {
                score.east = (c - col) as u32;
                break;
            }
        }

        // west
        for c in (0..col).rev() {
            if self.tree(row, c).height >= height {
                score.west = (col - c) as u32;
                break;
            }
        }

        self.tree_mut(row, col).scenic_score = score.score();
    }
}

impl<R: io::Read> TryFrom<io::BufReader<R>> for TreeGrid {
    type Error = String;

    fn try_from(value: io::BufReader<R>) -> Result<Self, Self::Error> {
        let tree_heights = value
            .bytes()
            .filter_map(|x| x.ok().and_then(|c| (c as char).to_digit(10)));

        TreeGrid::new(tree_heights)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();
    let input = File::open(&args[1])?;

    let reader = io::BufReader::new(input);
    let grid: TreeGrid = reader.try_into()?;

    let max_score = grid.trees.iter().map(|t| t.scenic_score).max().unwrap();

    println!("{}", max_score);

    Ok(())
}
