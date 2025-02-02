use std::{
    collections::BTreeSet,
    fs::read_to_string,
    hash::{DefaultHasher, Hash, Hasher},
};

use aoc_2024::{InputParser, ProblemSolver, Runner};

type Height = u8;

const MIN_HEIGHT: Height = 0;
const MAX_HEIGHT: Height = 9;

#[derive(Clone, Copy)]
struct Vector {
    x: i8,
    y: i8,
}

const UP: Vector = Vector { x: 0, y: 1 };
const RIGHT: Vector = Vector { x: 1, y: 0 };
const DOWN: Vector = Vector { x: 0, y: -1 };
const LEFT: Vector = Vector { x: -1, y: 0 };

const POSSIBLE_DIRECTIONS: [Vector; 4] = [UP, RIGHT, DOWN, LEFT];

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_by(&self, vector: Vector) -> Position {
        Position {
            x: self.x.wrapping_add_signed(vector.x as isize),
            y: self.y.wrapping_add_signed(vector.y as isize),
        }
    }
}

struct Map {
    heights: Vec<Height>,
    x_len: usize,
    y_len: usize,
}

impl Map {
    fn height(&self, position: &Position) -> Height {
        self.heights[position.x + position.y * self.x_len]
    }

    fn is_within_bounds(&self, position: &Position) -> bool {
        position.x < self.x_len && position.y < self.y_len
    }
}

type HikingScore = usize;

struct HikeTrailScorer<'a> {
    map: &'a Map,
    trail_hashes: BTreeSet<u64>,
}

impl<'a> HikeTrailScorer<'a> {
    fn new(map: &'a Map) -> Self {
        Self {
            map,
            trail_hashes: BTreeSet::new(),
        }
    }

    fn score(mut self, position: Position) -> HikingScore {
        self.traverse(DefaultHasher::new(), position);
        self.trail_hashes.len()
    }

    fn traverse(&mut self, mut hasher: DefaultHasher, position: Position) {
        position.hash(&mut hasher);

        let height = self.map.height(&position);

        if height == MAX_HEIGHT {
            self.trail_hashes.insert(hasher.finish());
            return;
        }

        POSSIBLE_DIRECTIONS
            .iter()
            .map(|dir| position.move_by(*dir))
            .filter(|position| {
                self.map.is_within_bounds(&position) && self.map.height(position) == height + 1
            })
            .for_each(|next_position| self.traverse(hasher.clone(), next_position));
    }
}

type Input = Map;

struct Parser;

impl InputParser<Input> for Parser {
    fn parse(&self, filename: &str) -> Input {
        let file = read_to_string(filename).unwrap();

        let x_len = file.lines().next().unwrap().len();

        let mut y_len = 0;

        let heights: Vec<Height> = file
            .lines()
            .flat_map(|line| {
                y_len += 1;

                line.chars().map(|height| {
                    if height == '.' {
                        MAX_HEIGHT + 2 // impassable height for debugging
                    } else {
                        height.to_digit(10).unwrap() as Height
                    }
                })
            })
            .collect();

        Input {
            heights,
            x_len,
            y_len,
        }
    }
}

struct Solver;

type Output = HikingScore;

impl ProblemSolver<Input, Output> for Solver {
    fn solve(&self, map: Input) -> Output {
        (0..map.y_len)
            .flat_map(|y| (0..map.x_len).map(move |x| Position { x, y }))
            .filter(|position| map.height(position) == MIN_HEIGHT)
            .map(|hike_start_pos| HikeTrailScorer::new(&map).score(hike_start_pos))
            .sum()
    }
}

fn main() {
    Runner::new(Parser, Solver).run(&vec![
        "src/10/input_1.txt",
        "src/10/input_2.txt",
        "src/10/input_3.txt",
    ]);
}
