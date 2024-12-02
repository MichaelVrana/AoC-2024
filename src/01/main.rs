use regex::Regex;
use std::{fs::read_to_string, iter::zip};

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
    similarity: usize,
}

fn solve(mut input: Input) -> Result {
    input.0.sort();
    input.1.sort();

    let difference: u32 = zip(input.0.iter(), input.1.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum();

    let mut right_iter = input.1.into_iter();
    let mut right_num = right_iter.next().unwrap();
    let similarity: usize = input
        .0
        .into_iter()
        .map(|left_num| {
            if left_num < right_num {
                return 0;
            }

            if left_num > right_num {
                right_num = match right_iter.find(|right| left_num <= *right) {
                    None => return 0,
                    Some(right) => {
                        if right > left_num {
                            return 0;
                        }

                        right
                    }
                };
            }


            let mut tmp_right_iter = right_iter.clone();

            let len_before = tmp_right_iter.len();

            match tmp_right_iter.find(|right| *right != left_num) {
                None => 0,
                Some(_) => {
                    let len_after = tmp_right_iter.len();
                    (len_before - len_after) * (left_num as usize)
                }
            }
        })
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

#[test]
fn test() {
    let a = [1, 2, 3, 4];

    let mut iter = a.iter();

    assert_eq!(
        iter.position(|&x| {
            println!("{}", x);
            x >= 2
        }),
        Some(1)
    );
}
