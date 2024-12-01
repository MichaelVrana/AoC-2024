use regex::Regex;
use std::{fs::read_to_string, iter::Zip};

type Input = (Vec<i32>, Vec<i32>);

fn read_lines(filename: &str) -> Vec<String> {
    match read_to_string(filename) {
        Err(_) => panic!("Could not read file {}", filename),
        Ok(file_content) => file_content.lines().map(String::from).collect(),
    }
}

fn parse_input(file: &str) -> Input {
    let regex = Regex::new(r"^([0-9]+)\s+([0-9]+)$").unwrap();

    let mut left_vec = Vec::<i32>::new();
    let mut right_vec = Vec::<i32>::new();

    for line in read_lines(file) {
        let captures = regex.captures(line.as_str()).unwrap();

        left_vec.push(captures.get(1).unwrap().as_str().parse::<i32>().unwrap());
        right_vec.push(captures.get(2).unwrap().as_str().parse::<i32>().unwrap());
    }

    (left_vec, right_vec)
}

fn solve(mut input: Input) -> u32 {
    input.0.sort();
    input.1.sort();

    std::iter::zip(input.0.into_iter(), input.1.into_iter())
        .map(|(left, right)| left.abs_diff(right))
        .sum()
}

fn main() {
    let input_files = vec!["src/01/input_1.txt", "src/01/input_2.txt"];

    for file in input_files {
        let input = parse_input(file);
        let result = solve(input);

        println!("Result for file {} is {}", file, result)
    }
}
