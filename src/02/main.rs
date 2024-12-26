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

                    match validate_sequence(iter) {
                        LineResult::Invalid { index: _ } => continue,
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
        vec!["src/02/input_1.txt", "src/02/input_2.txt", "src/02/input_3.txt"],
    )
    .run();
}
