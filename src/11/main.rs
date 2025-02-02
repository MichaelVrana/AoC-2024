use std::{collections::HashMap, fs::read_to_string};

use aoc_2024::{InputParser, ProblemSolver, Runner};

type StoneNumber = u64;

#[derive(PartialEq, Eq, Clone, Copy, Hash)]
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

type BlinkCount = u8;

const BLINK_COUNT: BlinkCount = 75;

#[derive(Default)]
struct StoneBlinker {
    results: HashMap<(StoneNumber, BlinkCount), Output>,
}

impl StoneBlinker {
    fn blink_at_stone(&mut self, stone: Stone, blink_count: BlinkCount) -> Output {
        if let Some(result) = self.results.get(&(stone.number, blink_count)) {
            return *result;
        }

        if blink_count == 0 {
            return 1;
        }

        let (new_stone, maybe_new_stone) = stone.blink();

        let new_stone_count = self.blink_at_stone(new_stone, blink_count - 1);

        let second_stone_count = maybe_new_stone
            .map(|second_stone| self.blink_at_stone(second_stone, blink_count - 1))
            .unwrap_or(0);

        let result = new_stone_count + second_stone_count;

        self.results.insert((stone.number, blink_count), result);

        result
    }
}

impl ProblemSolver<Input, Output> for Solver {
    fn solve(&self, stones: Input) -> Output {
        let mut stone_blinker = StoneBlinker::default();

        stones
            .into_iter()
            .map(|stone| stone_blinker.blink_at_stone(stone, BLINK_COUNT))
            .sum()
    }
}

fn main() {
    Runner::new(Parser, Solver).run(&vec!["src/11/input_1.txt", "src/11/input_2.txt"]);
}
