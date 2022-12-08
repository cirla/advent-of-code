use std::env;
use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

use itertools::Either;

pub struct Tree {
    pub height: u32,
    pub visible: bool,
}

pub struct TreeGrid {
    pub trees: Vec<Tree>,
    pub side_len: usize,
    pub num_visible: usize,
}

impl TreeGrid {
    pub fn new(tree_heights: impl Iterator<Item = u32>) -> Result<TreeGrid, String> {
        let trees: Vec<Tree> = tree_heights
            .map(|height| Tree {
                height,
                visible: false,
            })
            .collect();
        let side_len = (trees.len() as f32).sqrt() as usize;

        if trees.len() != side_len * side_len {
            Err(format!("Number of trees ({}) is not square", trees.len()))
        } else {
            let mut grid = TreeGrid {
                trees,
                side_len,
                num_visible: 0,
            };

            grid.mark_visibility();

            Ok(grid)
        }
    }

    fn interior(&self) -> impl DoubleEndedIterator<Item = usize> {
        // iterator over interior rows/cols
        1..self.side_len - 1
    }

    pub fn tree(&self, row: usize, col: usize) -> &Tree {
        &self.trees[row * self.side_len + col]
    }

    fn tree_mut(&mut self, row: usize, col: usize) -> &mut Tree {
        &mut self.trees[row * self.side_len + col]
    }

    fn mark_visibility(&mut self) {
        // edge trees are always visible
        self.num_visible = (self.side_len - 1) * 4;
        for row in 0..self.side_len {
            let cols = if row == 0 || row == self.side_len - 1 {
                Either::Left(0..self.side_len)
            } else {
                Either::Right([0, self.side_len - 1].into_iter())
            };
            for col in cols {
                self.tree_mut(row, col).visible = true;
            }
        }

        // set north/west invisibility
        let mut max_vs: Vec<u32> = self
            .interior()
            .map(|col| self.tree(0, col).height)
            .collect();

        for row in self.interior() {
            let mut max_h: u32 = self.tree(row, 0).height;
            for col in self.interior() {
                let mut tree = self.tree_mut(row, col);

                if tree.height > max_h {
                    max_h = tree.height;
                    tree.visible = true;
                }

                let max_v = &mut max_vs[col - 1];
                if tree.height > *max_v {
                    *max_v = tree.height;
                    tree.visible = true;
                }
            }
        }

        // set south/east invisibility and count
        max_vs = self
            .interior()
            .map(|col| self.tree(self.side_len - 1, col).height)
            .collect();

        for row in self.interior().rev() {
            let mut max_h: u32 = self.tree(row, self.side_len - 1).height;
            for col in self.interior().rev() {
                let mut tree = self.tree_mut(row, col);

                if tree.height > max_h {
                    max_h = tree.height;
                    tree.visible = true;
                }

                let max_v = &mut max_vs[col - 1];
                if tree.height > *max_v {
                    *max_v = tree.height;
                    tree.visible = true;
                }

                if tree.visible {
                    self.num_visible += 1;
                }
            }
        }
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

    println!("{}", grid.num_visible);

    Ok(())
}
