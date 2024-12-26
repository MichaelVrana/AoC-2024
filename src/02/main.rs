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

const INVALID: u32 = 0;
const VALID: u32 = 1;

impl ProblemSolver<Input, Result> for Solver {
    fn solve(&mut self, input: Input) -> Result {
        input
            .iter()
            .map(|sequence| match (sequence.get(0), sequence.get(1)) {
                (Some(first), Some(second)) => {
                    let descending = *first > *second;

                    let invert_coef = if descending { -1 } else { 1 };

                    let iter = sequence.iter().skip(1).map(|value| value * invert_coef);

                    let mut prev = first * invert_coef;

                    for value in iter {
                        let difference = value - prev;

                        if difference < 1 || difference > 3 {
                            return INVALID;
                        }

                        prev = value;
                    }

                    VALID
                }
                _ => INVALID,
            })
            .sum()
    }
}

fn main() {
    Runner::new(
        &mut Parser,
        &mut Solver,
        vec!["src/02/input_1.txt", "src/02/input_2.txt"],
    )
    .run();
}
