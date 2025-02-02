use std::{fs::read_to_string, iter};

use aoc_2024::{InputParser, ProblemSolver, Runner};

type FileId = u32;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Block {
    Empty,
    File(FileId),
}

type Blocks = Vec<Block>;

struct Input {
    blocks: Vec<Block>,
    last_file_id: u32,
}

struct Parser;

impl InputParser<Input> for Parser {
    fn parse(&self, filename: &str) -> Input {
        let file = read_to_string(filename).unwrap();

        let mut file_id = 0;

        Input {
            blocks: file
                .trim_end()
                .chars()
                .enumerate()
                .flat_map(|(idx, char)| {
                    let count = char.to_digit(10).unwrap();

                    if idx % 2 == 0 {
                        let result = iter::repeat_n(Block::File(file_id), count as usize);

                        file_id += 1;

                        return result;
                    }

                    iter::repeat_n(Block::Empty, count as usize)
                })
                .collect(),
            last_file_id: file_id - 1,
        }
    }
}

type Output = u64;

type BlockSize = u8;

trait GetBlockSize {
    fn block_size_forwards(&self, block_idx: usize) -> BlockSize;
    fn block_size_backwards(&self, block_dx: usize) -> BlockSize;
}

impl GetBlockSize for Blocks {
    fn block_size_forwards(&self, block_idx: usize) -> BlockSize {
        let block = self[block_idx];

        let mut end_idx = block_idx;

        while let Some(peeked_block) = self.get(end_idx) {
            if *peeked_block == block && end_idx != self.len() {
                end_idx += 1;
            } else {
                break;
            }
        }

        (end_idx - block_idx) as BlockSize
    }

    fn block_size_backwards(&self, block_idx: usize) -> BlockSize {
        let block = self[block_idx];

        let mut start_idx = block_idx;

        while let Some(peeked_block) = self.get(start_idx) {
            if *peeked_block == block {
                if start_idx == 0 {
                    // this prevents underflow
                    return (block_idx - start_idx + 1) as BlockSize;
                }

                start_idx -= 1;
            } else {
                break;
            }
        }

        (block_idx - start_idx) as BlockSize
    }
}

#[test]
fn test_block_size_forwards() {
    let blocks = vec![
        Block::Empty,
        Block::Empty,
        Block::Empty,
        Block::File(0),
        Block::File(0),
        Block::Empty,
    ];

    assert!(blocks.block_size_forwards(0) == 3);
    assert!(blocks.block_size_forwards(1) == 2);
    assert!(blocks.block_size_forwards(2) == 1);
    assert!(blocks.block_size_forwards(3) == 2);
    assert!(blocks.block_size_forwards(4) == 1);
    assert!(blocks.block_size_forwards(5) == 1);
}

#[test]
fn test_block_size_backwards() {
    let blocks = vec![
        Block::Empty,
        Block::Empty,
        Block::Empty,
        Block::File(0),
        Block::File(0),
        Block::Empty,
    ];

    assert!(blocks.block_size_backwards(5) == 1);
    assert!(blocks.block_size_backwards(4) == 2);
    assert!(blocks.block_size_backwards(3) == 1);
    assert!(blocks.block_size_backwards(2) == 3);
    assert!(blocks.block_size_backwards(1) == 2);
    assert!(blocks.block_size_backwards(0) == 1);
}

struct Solver;

impl ProblemSolver<Input, Output> for Solver {
    fn solve(&self, input: Input) -> Output {
        let mut blocks = input.blocks;
        let mut curr_file_id = input.last_file_id;
        let mut curr_file_end_idx = blocks.len() - 1;

        loop {
            while blocks[curr_file_end_idx] != Block::File(curr_file_id) {
                curr_file_end_idx -= 1;
            }

            let curr_file_size = blocks.block_size_backwards(curr_file_end_idx);

            let mut forwards_idx = 0;

            while let Some(block) = blocks.get(forwards_idx) {
                if forwards_idx >= curr_file_end_idx {
                    break;
                }

                let block_size = blocks.block_size_forwards(forwards_idx);

                if *block == Block::Empty && block_size >= curr_file_size {
                    for _ in 0..curr_file_size {
                        blocks.swap(forwards_idx, curr_file_end_idx);

                        forwards_idx += 1;
                        curr_file_end_idx -= 1;
                    }

                    break;
                }

                forwards_idx += block_size as usize;
            }

            if curr_file_id == 0 {
                break;
            }

            curr_file_id -= 1;
        }

        blocks
            .into_iter()
            .enumerate()
            .map(|(idx, block)| match block {
                Block::File(file_id) => idx as u64 * file_id as u64,
                _ => 0,
            })
            .sum()
    }
}

fn main() {
    Runner::new(Parser, Solver).run(&vec!["src/09/input_1.txt", "src/09/input_2.txt"]);
}
