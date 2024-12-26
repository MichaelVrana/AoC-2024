use aoc_2024::{InputParser, ProblemSolver, Runner};
use std::fs::read_to_string;

type Input = Vec<Vec<i32>>;

struct Parser;

impl InputParser<Input> for Parser {
    fn parse(&mut self, filename: &str) -> Input {
        let file = read_to_string(filename).unwrap();

        file.lines()
            .map(|line| {
                line.split_ascii_whitespace()
                    .map(|num_str| num_str.parse::<i32>().unwrap())
                    .collect()
            })
            .collect()
    }
}

type Result = u32;

struct Solver;

enum LineResult {
    Valid,
    Invalid { index: usize },
}

fn validate_sequence<T: Iterator<Item = i32>>(mut iter: T) -> LineResult {
    let mut prev = iter.next().unwrap();

    let mut index = 0;

    for value in iter {
        let difference = value - prev;

        if difference < 1 || difference > 3 {
            return LineResult::Invalid { index };
        }

        prev = value;
        index = index + 1;
    }

    LineResult::Valid
}

impl ProblemSolver<Input, Result> for Solver {
    fn solve(&mut self, input: Input) -> Result {
        input
            .iter()
            .map(|sequence| {
                for invert_coef in [-1, 1] {
                    let iter = sequence.iter().map(|value| value * invert_coef);

                    match validate_sequence(iter.clone()) {
                        LineResult::Invalid { index } => {
                            for index_to_remove in [index, index + 1] {
                                let head = iter.clone().take(index_to_remove);
                                let tail = iter.clone().skip(index_to_remove + 1);

                                let repaired_sequence = head.chain(tail);

                                match validate_sequence(repaired_sequence) {
                                    LineResult::Invalid { index: _ } => continue,
                                    LineResult::Valid => return 1,
                                }
                            }
                        }
                        LineResult::Valid => return 1,
                    }
                }

                0
            })
            .sum()
    }
}

fn main() {
    Runner::new(
        &mut Parser,
        &mut Solver,
        vec![
            "src/02/input_1.txt",
            "src/02/input_2.txt",
            "src/02/input_3.txt",
            "src/02/input_4.txt",
        ],
    )
    .run();
}
