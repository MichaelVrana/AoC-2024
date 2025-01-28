use std::fs::read_to_string;

use aoc_2024::{InputParser, ProblemSolver, Runner};

enum Tile {
    Space,
    Obstacle,
}

type Map = Vec<Vec<Tile>>;

type Position = (usize, usize);

struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    fn rotate_right(&self) -> Vector {
        Vector {
            x: -self.y,
            y: self.x,
        }
    }
}

struct Input {
    map: Map,
    starting_pos: Position,
    starting_dir: Vector
}

struct Parser;

impl InputParser<Input> for Parser {
    fn parse(&self, filename: &str) -> Input {
        let file = read_to_string(filename).unwrap();

        let mut starting_pos: Position = (0, 0);
        let mut starting_dir: Vector = { };

        let map: Map = file
            .lines()
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| match char {
                        '.' => Tile::Space,
                        '#' => Tile::Obstacle,
                        '^' => {
                            starting_pos = (x, y);

                            Tile::Space
                        }
                        _ => panic!("Unknown tile {}", char),
                    })
                    .collect()
            })
            .collect();

        Input { map, starting_pos }
    }
}

struct Solver;

type Output = u32;

impl ProblemSolver<Input, Output> for Solver {
    fn solve(&self, input: Input) -> Output {
        let mut pos = input.starting_pos;
        let direction = 


    }
}

fn main() {
    Runner::new(Parser, Solver).run(&vec!["src/06/input_1.txt"]);
}
