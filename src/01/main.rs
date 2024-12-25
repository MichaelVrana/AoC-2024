use regex::Regex;
use std::{collections::HashMap, fs::read_to_string, iter::zip};

type Input = (Vec<u32>, Vec<u32>);

fn read_lines(filename: &str) -> Vec<String> {
    match read_to_string(filename) {
        Err(_) => panic!("Could not read file {}", filename),
        Ok(file_content) => file_content.lines().map(String::from).collect(),
    }
}

fn parse_input(file: &str) -> Input {
    let regex = Regex::new(r"^([0-9]+)\s+([0-9]+)$").unwrap();

    let mut left_vec = Vec::<u32>::new();
    let mut right_vec = Vec::<u32>::new();

    for line in read_lines(file) {
        let captures = regex.captures(line.as_str()).unwrap();

        left_vec.push(captures.get(1).unwrap().as_str().parse::<u32>().unwrap());
        right_vec.push(captures.get(2).unwrap().as_str().parse::<u32>().unwrap());
    }

    (left_vec, right_vec)
}

struct Result {
    difference: u32,
    similarity: u32,
}

fn solve(mut input: Input) -> Result {
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

fn main() {
    let input_files = vec!["src/01/input_1.txt", "src/01/input_2.txt"];

    for file in input_files {
        let input = parse_input(file);
        let result = solve(input);

        println!(
            "File {}, difference is {}, similarity is {}",
            file, result.difference, result.similarity
        )
    }
}
