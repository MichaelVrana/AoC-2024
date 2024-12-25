use aoc_2024::{InputParser, ProblemSolver, Runner};
use regex::Regex;
use std::{collections::HashMap, fmt::Display, fs::read_to_string, iter::zip};

type Input = (Vec<u32>, Vec<u32>);

struct Parser;

fn read_lines(filename: &str) -> Vec<String> {
    match read_to_string(filename) {
        Err(_) => panic!("Could not read file {}", filename),
        Ok(file_content) => file_content.lines().map(String::from).collect(),
    }
}

impl InputParser<Input> for Parser {
    fn parse(&mut self, filename: &str) -> Input {
        let regex = Regex::new(r"^([0-9]+)\s+([0-9]+)$").unwrap();

        let mut left_vec = Vec::<u32>::new();
        let mut right_vec = Vec::<u32>::new();

        for line in read_lines(filename) {
            let captures = regex.captures(line.as_str()).unwrap();

            left_vec.push(captures.get(1).unwrap().as_str().parse::<u32>().unwrap());
            right_vec.push(captures.get(2).unwrap().as_str().parse::<u32>().unwrap());
        }

        (left_vec, right_vec)
    }
}

struct Result {
    difference: u32,
    similarity: u32,
}

impl Display for Result {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "difference is {}, similarity is {}",
            self.difference, self.similarity
        )
    }
}

struct Solver;

impl ProblemSolver<Input, Result> for Solver {
    fn solve(&mut self, mut input: Input) -> Result {
        input.0.sort();
        input.1.sort();

        let difference: u32 = zip(input.0.iter(), input.1.iter())
            .map(|(left, right)| left.abs_diff(*right))
            .sum();

        let counts = input
            .1
            .iter()
            .fold(HashMap::<u32, u32>::new(), |mut counts, value| {
                match counts.get(value) {
                    Some(count) => counts.insert(*value, count + 1),
                    None => counts.insert(*value, 1),
                };

                counts
            });

        let similarity = input
            .0
            .iter()
            .map(|value| value * counts.get(value).unwrap_or(&0))
            .sum();

        Result {
            difference,
            similarity,
        }
    }
}

fn main() {
    Runner::new(
        &mut Parser,
        &mut Solver,
        vec!["src/01/input_1.txt", "src/01/input_2.txt"],
    )
    .run();
}
