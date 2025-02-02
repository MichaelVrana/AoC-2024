use std::fs::read_to_string;

use aoc_2024::{InputParser, ProblemSolver, Runner};

type StoneNumber = u64;

struct Stone {
    number: StoneNumber,
}

impl Stone {
    fn blink(&self) -> (Stone, Option<Stone>) {
        if self.number == 0 {
            return (Stone { number: 1 }, None);
        }

        let digits = self.number.ilog10() + 1;

        if digits % 2 == 0 {
            let half_digits = digits / 2;
            let first_denominator = (10 as StoneNumber).pow(half_digits);

            return (
                Stone {
                    number: self.number / first_denominator,
                },
                Some(Stone {
                    number: self.number % first_denominator,
                }),
            );
        }

        (
            Stone {
                number: self.number * 2024,
            },
            None,
        )
    }
}

#[test]
fn test_blink_split() {
    let stone = Stone { number: 2024 };

    let result = stone.blink();

    assert!(result.0.number == 20);
    assert!(result.1.unwrap().number == 24);
}

type Input = Vec<Stone>;

struct Parser;

impl InputParser<Input> for Parser {
    fn parse(&self, filename: &str) -> Input {
        let file = read_to_string(filename).unwrap();

        file.split_ascii_whitespace()
            .map(|number| Stone {
                number: number.parse().unwrap(),
            })
            .collect()
    }
}

type Output = usize;

struct Solver;

const BLINK_COUNT: u8 = 25;

impl ProblemSolver<Input, Output> for Solver {
    fn solve(&self, mut stones: Input) -> Output {
        for _ in 0..BLINK_COUNT {
            let mut new_stones: Input = Vec::new();

            new_stones.reserve(stones.len() * 2);

            stones = stones.iter().fold(new_stones, |mut acc, stone| {
                let (new_stone, maybe_new_stone) = stone.blink();

                acc.push(new_stone);

                if let Some(second_new_stone) = maybe_new_stone {
                    acc.push(second_new_stone);
                }

                acc
            })
        }

        stones.len()
    }
}

fn main() {
    Runner::new(Parser, Solver).run(&vec!["src/11/input_1.txt", "src/11/input_2.txt"]);
}
