use std::fs::read_to_string;

use aoc_2024::{InputParser, ProblemSolver, Runner};

type Int = u64;

struct Equation {
    result: Int,
    operands: Vec<Int>,
}

type Input = Vec<Equation>;

struct Parser;

impl InputParser<Input> for Parser {
    fn parse(&self, filename: &str) -> Input {
        let file = read_to_string(filename).unwrap();

        file.lines()
            .map(|line| {
                let (result_str, operands_str) = line.split_once(":").unwrap();

                let operands = operands_str
                    .split_whitespace()
                    .map(|operand_str| operand_str.parse().unwrap())
                    .collect();

                Equation {
                    result: result_str.parse().unwrap(),
                    operands,
                }
            })
            .collect()
    }
}

type Output = Int;

struct Solver;

fn glue(left: Int, right: Int) -> Int {
    let right_digits = right.ilog10() + 1;

    left * 10_u64.pow(right_digits) + right
}

#[test]
fn test_glue() {
    assert!(glue(12, 345) == 12345)
}

fn is_calculable<'a, TIter: Iterator<Item = &'a Int> + Clone>(
    acc: Int,
    result: Int,
    mut operands: TIter,
) -> bool {
    match operands.next() {
        None => acc == result,
        Some(operand) => [glue(acc, *operand), acc * operand, acc + operand]
            .into_iter()
            .any(|next_acc| {
                next_acc <= result && is_calculable(next_acc, result, operands.clone())
            }),
    }
}

impl ProblemSolver<Input, Output> for Solver {
    fn solve(&self, input: Input) -> Output {
        input
            .iter()
            .map(|equation| {
                let mut operands_iter = equation.operands.iter();

                if is_calculable(
                    *operands_iter.next().unwrap(),
                    equation.result,
                    operands_iter,
                ) {
                    return equation.result;
                };

                0
            })
            .sum()
    }
}

fn main() {
    Runner::new(Parser, Solver).run(&vec!["src/07/input_1.txt", "src/07/input_2.txt"]);
}
