use std::{fs::read_to_string, iter};

use aoc_2024::{InputParser, ProblemSolver, Runner};

type FileId = u32;

#[derive(Clone, Copy)]
enum Block {
    Empty,
    File(FileId),
}

type Input = Vec<Block>;

struct Parser;

impl InputParser<Input> for Parser {
    fn parse(&self, filename: &str) -> Input {
        let file = read_to_string(filename).unwrap();

        let mut file_id = 0;

        file.trim_end()
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
            .collect()
    }
}

type Output = u64;

struct Solver;

impl ProblemSolver<Input, Output> for Solver {
    fn solve(&self, mut input: Input) -> Output {
        let mut curr_idx: usize = 0;
        let mut end_idx = input.len()
            - 1
            - input
                .iter()
                .rev()
                .enumerate()
                .find_map(|(i, elem)| match elem {
                    Block::Empty => None,
                    Block::File(_) => Some(i),
                })
                .unwrap_or(0);

        while curr_idx < end_idx {
            match input.get(curr_idx) {
                Some(Block::Empty) => {
                    input.swap(curr_idx, end_idx);

                    while let Some(Block::Empty) = input.get(end_idx) {
                        end_idx -= 1
                    }
                }
                Some(Block::File(_)) => {}
                None => break,
            }

            curr_idx += 1;
        }

        input
            .into_iter()
            .enumerate()
            .map_while(|(idx, block)| match block {
                Block::File(file_id) => Some(idx as u64 * file_id as u64),
                _ => None,
            })
            .sum()
    }
}

fn main() {
    Runner::new(Parser, Solver).run(&vec!["src/09/input_1.txt", "src/09/input_2.txt"]);
}
