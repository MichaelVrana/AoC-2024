use aoc_2024::{InputParser, ProblemSolver, Runner};
use std::{
    collections::BTreeSet,
    fs::read_to_string,
    hash::{DefaultHasher, Hash, Hasher},
};

#[derive(Clone, Copy)]
enum Tile {
    Space,
    Obstacle,
}

struct Map {
    // one could just use a Vec<Vec<Tile>> but this saves space!
    // and also is contiguos
    tiles: Vec<Tile>,
    x_len: usize,
    y_len: usize,
}

impl Map {
    fn get_tile(&self, position: &Position) -> Tile {
        *self
            .tiles
            .get(position.x + self.y_len * position.y)
            .unwrap()
    }

    fn is_within_bounds(&self, position: &Position) -> bool {
        position.x < self.x_len && position.y < self.y_len
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_in_dir(&self, dir: &Vector) -> Position {
        Position {
            x: self.x.wrapping_add_signed(dir.x),
            y: self.y.wrapping_add_signed(dir.y),
        }
    }
}

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

const UP: Vector = Vector { x: 0, y: -1 };

struct Input {
    map: Map,
    starting_pos: Position,
}

struct Parser;

impl InputParser<Input> for Parser {
    fn parse(&self, filename: &str) -> Input {
        let file = read_to_string(filename).unwrap();

        let mut starting_pos = Position { x: 0, y: 0 };

        let x_len = file.lines().next().unwrap().len();

        let tiles: Vec<Tile> = file
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, char)| match char {
                        '.' => Tile::Space,
                        '#' => Tile::Obstacle,
                        '^' => {
                            starting_pos = Position { x, y };

                            Tile::Space
                        }
                        _ => panic!("Unknown tile {}", char),
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect();

        Input {
            starting_pos: starting_pos,
            map: Map {
                x_len,
                y_len: tiles.len() / x_len,
                tiles,
            },
        }
    }
}

struct Solver;

type Output = usize;

impl ProblemSolver<Input, Output> for Solver {
    fn solve(&self, input: Input) -> Output {
        let mut pos = input.starting_pos;
        let mut dir = UP;

        // hashes are just u64, thus they are half the size of positions
        // you could just do this simply with a HashSet, but i thought this was more fun
        let mut visited_pos_hashes = BTreeSet::<u64>::new();

        loop {
            let mut hasher = DefaultHasher::new();

            pos.hash(&mut hasher);

            visited_pos_hashes.insert(hasher.finish());

            let next_pos = pos.move_in_dir(&dir);

            if !input.map.is_within_bounds(&next_pos) {
                break;
            }

            match input.map.get_tile(&next_pos) {
                Tile::Space => pos = next_pos,
                Tile::Obstacle => dir = dir.rotate_right(),
            }
        }

        visited_pos_hashes.len()
    }
}

fn main() {
    Runner::new(Parser, Solver).run(&vec!["src/06/input_1.txt", "src/06/input_2.txt"]);
}
