use aoc_2024::{InputParser, ProblemSolver, Runner};
use std::{fs::read_to_string, str::Chars};

type Input = Vec<Vec<char>>;

struct Parser;

impl InputParser<Input> for Parser {
    fn parse(&mut self, filename: &str) -> Input {
        let file = read_to_string(filename).unwrap();

        file.lines().map(|line| line.chars().collect()).collect()
    }
}

type Vector = (i32, i32);

const UP: Vector = (0, -1);
const UP_RIGHT: Vector = (1, -1);
const RIGHT: Vector = (1, 0);
const DOWN_RIGHT: Vector = (1, 1);
const DOWN: Vector = (0, 1);
const DOWN_LEFT: Vector = (-1, 1);
const LEFT: Vector = (-1, 0);
const UP_LEFT: Vector = (-1, -1);

const DIRECTIONS: [Vector; 8] = [
    UP, UP_RIGHT, RIGHT, DOWN_RIGHT, DOWN, DOWN_LEFT, LEFT, UP_LEFT,
];

const SEARCH_STR: &str = "XMAS";

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
    fn solve(&mut self, input: Input) -> Result {
        let coords = input
            .iter()
            .enumerate()
            .flat_map(|(y, row)| (0..row.len()).map(move |x| (x, y)));

        coords
            .map(|coord| -> u32 {
                DIRECTIONS
                    .iter()
                    .map(|direction| {
                        input.search_in_direction(*direction, SEARCH_STR.chars(), coord) as u32
                    })
                    .sum()
            })
            .sum()
    }
}

fn main() {
    Runner::new(
        &mut Parser,
        &mut Solver,
        vec!["src/04/input_1.txt", "src/04/input_2.txt"],
    )
    .run();
}
