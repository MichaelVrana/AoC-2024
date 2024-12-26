use aoc_2024::{InputParser, ProblemSolver, Runner};
use regex::Regex;
use std::fs::read_to_string;

enum Instruction {
    Mul(u32, u32),
    Dont,
    Do,
}

type Input = Vec<Instruction>;

struct Parser;

impl InputParser<Input> for Parser {
    fn parse(&self, filename: &str) -> Input {
        let file = read_to_string(filename).unwrap();

        let regex = Regex::new(
            r"(?<mul>mul\((?<left>[0-9]{1,3}),(?<right>[0-9]{1,3})\))|(?<dont>don't\(\))|(?<do>do\(\))",
        )
        .unwrap();

        regex
            .captures_iter(file.as_str())
            .map(|captures| {
                if let Some(_) = captures.name("mul") {
                    let left = captures.name("left").unwrap().as_str();
                    let right = captures.name("right").unwrap().as_str();

                    return Instruction::Mul(left.parse().unwrap(), right.parse().unwrap());
                }

                if let Some(_) = captures.name("dont") {
                    return Instruction::Dont;
                }

                if let Some(_) = captures.name("do") {
                    return Instruction::Do;
                }

                panic!("Matched unknown instruction!")
            })
            .collect()
    }
}

type Result = u32;

struct Interpreter {
    enabled: bool,
    result: u32,
}

impl Interpreter {
    fn new() -> Self {
        Interpreter {
            enabled: true,
            result: 0,
        }
    }

    fn disable(self) -> Self {
        Interpreter {
            enabled: false,
            result: self.result,
        }
    }

    fn enable(self) -> Self {
        Interpreter {
            enabled: true,
            result: self.result,
        }
    }

    fn add_to_result(self, value: u32) -> Self {
        if !self.enabled {
            return self;
        }

        Interpreter {
            enabled: true,
            result: self.result + value,
        }
    }
}

struct Solver;

impl ProblemSolver<Input, Result> for Solver {
    fn solve(&self, input: Input) -> Result {
        input
            .iter()
            .fold(Interpreter::new(), |interpreter, instr| match instr {
                Instruction::Mul(left, right) => interpreter.add_to_result(left * right),
                Instruction::Dont => interpreter.disable(),
                Instruction::Do => interpreter.enable(),
            })
            .result
    }
}

fn main() {
    Runner::new(Parser, Solver).run(&vec![
        "src/03/input_1.txt",
        "src/03/input_2.txt",
        "src/03/input_3.txt",
    ]);
}
