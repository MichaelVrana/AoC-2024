use aoc_2024::{InputParser, ProblemSolver, Runner};
use std::{fs::read_to_string, str::Chars};

type Input = Vec<Vec<char>>;

struct Parser;

impl InputParser<Input> for Parser {
    fn parse(&self, filename: &str) -> Input {
        let file = read_to_string(filename).unwrap();

        file.lines().map(|line| line.chars().collect()).collect()
    }
}

type Vector = (i32, i32);

const UP_RIGHT: Vector = (1, -1);
const DOWN_RIGHT: Vector = (1, 1);
const DOWN_LEFT: Vector = (-1, 1);
const UP_LEFT: Vector = (-1, -1);

const POSSIBLE_DIRECTIONS: [[Vector; 2]; 4] = [
    [DOWN_RIGHT, DOWN_LEFT],
    [DOWN_LEFT, UP_LEFT],
    [UP_LEFT, UP_RIGHT],
    [UP_RIGHT, DOWN_RIGHT],
];

const SEARCH_STR: &str = "MAS";

type Coord = (usize, usize);

trait SearchInDirection {
    fn search_in_direction(&self, direction: Vector, search_chars: Chars, from: Coord) -> bool;
}

impl SearchInDirection for Input {
    fn search_in_direction(
        &self,
        direction: Vector,
        mut search_chars: Chars,
        (x, y): Coord,
    ) -> bool {
        match search_chars.next() {
            None => true,
            Some(curr_search_char) => {
                if let Some(curr_char) = self.get(y).unwrap_or(&Vec::<char>::default()).get(x) {
                    if *curr_char != curr_search_char {
                        return false;
                    }

                    let (x_dir, y_dir) = direction;

                    // it is safe to do wrapping add since our code handles out of bounds coords correctly
                    let next_coord = (
                        x.wrapping_add_signed(x_dir as isize),
                        y.wrapping_add_signed(y_dir as isize),
                    );

                    return self.search_in_direction(direction, search_chars, next_coord);
                }

                false
            }
        }
    }
}

type Result = u32;

struct Solver;

impl ProblemSolver<Input, Result> for Solver {
    fn solve(&self, input: Input) -> Result {
        let coords = input
            .iter()
            .enumerate()
            .flat_map(|(y, row)| (0..row.len()).map(move |x| (x, y)));

        coords
            .map(|(x, y)| -> u32 {
                POSSIBLE_DIRECTIONS
                    .iter()
                    .map(|directions| {
                        directions.iter().all(|direction| {
                            let (dir_x, dir_y) = direction;

                            let start_coord: Coord = (
                                x.wrapping_add_signed(-dir_x as isize),
                                y.wrapping_add_signed(-dir_y as isize),
                            );

                            input.search_in_direction(*direction, SEARCH_STR.chars(), start_coord)
                        }) as u32
                    })
                    .sum()
            })
            .sum()
    }
}

fn main() {
    Runner::new(Parser, Solver).run(&vec!["src/04/input_1.txt", "src/04/input_2.txt"]);
}
