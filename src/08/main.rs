use std::fs::read_to_string;

use aoc_2024::{InputParser, ProblemSolver, Runner};

struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    fn reverse(&self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }

    fn div(&self, scalar: isize) -> Option<Vector> {
        if self.x % scalar != 0 || self.y % scalar != 0 {
            return None;
        }

        Some(Vector {
            x: self.x / scalar,
            y: self.y / scalar,
        })
    }
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn vector_to(&self, other: &Position) -> Vector {
        Vector {
            x: other.x as isize - self.x as isize,
            y: other.y as isize - self.y as isize,
        }
    }

    fn move_by(&self, dir: &Vector) -> Position {
        Position {
            x: self.x.wrapping_add_signed(dir.x),
            y: self.y.wrapping_add_signed(dir.y),
        }
    }
}

type Frequency = u8;

type Antenas = Vec<Vec<Position>>;

struct Input {
    // since there are only less than 255 possible frequencies, we can just use a vec to store the more effectively than by using a hashmap
    // I wanted to use a slice to avoid heap allocations but rust wont let me do that easily
    antenas: Antenas,
    x_len: usize,
    y_len: usize,
}

impl Input {
    fn is_within_bounds(&self, position: &Position) -> bool {
        position.x < self.x_len && position.y < self.y_len
    }
}

struct Parser;

impl InputParser<Input> for Parser {
    fn parse(&self, filename: &str) -> Input {
        let file = read_to_string(filename).unwrap();

        let mut antenas: Antenas = vec![Vec::new(); Frequency::MAX as usize];

        let x_len = file.lines().next().unwrap().len();
        let mut y_len: usize = 0;

        for (y, line) in file.lines().enumerate() {
            y_len += 1;

            for (x, char) in line.chars().enumerate() {
                if char == '.' {
                    continue;
                }

                antenas
                    .get_mut(char as usize)
                    .unwrap()
                    .push(Position { x, y });
            }
        }

        Input {
            antenas,
            x_len,
            y_len,
        }
    }
}

type Output = usize;

struct Solver;

impl ProblemSolver<Input, Output> for Solver {
    fn solve(&self, input: Input) -> Output {
        let mut antinode_positions = (0..Frequency::MAX)
            .flat_map(|frequency| {
                let antena_positions = input.antenas.get(frequency as usize).unwrap();

                antena_positions
                    .iter()
                    .enumerate()
                    .flat_map(|(idx, position_a)| {
                        antena_positions
                            .iter()
                            .skip(idx + 1)
                            .flat_map(|position_b| {
                                let vector_to_b = position_a.vector_to(position_b);

                                let outward_antinodes = [
                                    position_a.move_by(&vector_to_b.reverse()),
                                    position_b.move_by(&vector_to_b),
                                ]
                                .into_iter()
                                .filter(|antinode_pos| input.is_within_bounds(&antinode_pos));

                                let inward_antinodes = [
                                    vector_to_b
                                        .div(3)
                                        .map(|one_third_to_b| position_a.move_by(&one_third_to_b)),
                                    vector_to_b.div(3).map(|one_third_to_b| {
                                        position_b.move_by(&one_third_to_b.reverse())
                                    }),
                                ]
                                .into_iter()
                                .filter_map(|val| val);

                                outward_antinodes.chain(inward_antinodes)
                            })
                    })
            })
            .collect::<Vec<Position>>();

        antinode_positions.sort();
        antinode_positions.dedup();
        antinode_positions.len()
    }
}

fn main() {
    Runner::new(Parser, Solver).run(&vec!["src/08/input_1.txt", "src/08/input_2.txt"]);
}
