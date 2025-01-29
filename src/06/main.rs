use aoc_2024::{InputParser, ProblemSolver, Runner};
use std::{collections::HashMap, fs::read_to_string, hash::Hash};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Space,
    Obstacle,
}

#[derive(Clone)]
struct Map {
    // one could just use a Vec<Vec<Tile>> but this saves space!
    // and also is contiguos
    tiles: Vec<Tile>,
    x_len: usize,
    y_len: usize,
}

impl Map {
    fn tile_idx(&self, position: &Position) -> usize {
        position.x + self.y_len * position.y
    }

    fn get_tile(&self, position: &Position) -> Tile {
        *self.tiles.get(self.tile_idx(position)).unwrap()
    }

    fn is_within_bounds(&self, position: &Position) -> bool {
        position.x < self.x_len && position.y < self.y_len
    }

    fn intersects_obstacle(&self, from: Position, dir: Vector) -> bool {
        let mut curr_pos = from;

        while self.is_within_bounds(&curr_pos) {
            if self.get_tile(&curr_pos) == Tile::Obstacle {
                return true;
            }

            curr_pos = curr_pos.move_in_dir(dir);
        }

        false
    }

    fn replace_tile(&self, at: &Position, tile: Tile) -> Map {
        let mut new_map = self.clone();

        let mut tile_ref = new_map.tiles.get_mut(self.tile_idx(at)).unwrap();

        *tile_ref = tile;

        new_map
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn move_in_dir(&self, dir: Vector) -> Position {
        Position {
            x: self.x.wrapping_add_signed(dir.x as isize),
            y: self.y.wrapping_add_signed(dir.y as isize),
        }
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Vector {
    x: i8,
    y: i8,
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
        let mut pos = input.starting_pos.clone();
        let mut dir = UP;

        let mut visited_pos_with_dirs = HashMap::<Position, Vec<Vector>>::new();

        loop {
            match visited_pos_with_dirs.get_mut(&pos) {
                Some(dirs) => {
                    if !dirs.contains(&dir) {
                        dirs.push(dir.clone());
                    }
                }
                None => {
                    let dirs = vec![dir.clone()];
                    visited_pos_with_dirs.insert(pos.clone(), dirs);
                }
            }

            let next_pos = pos.move_in_dir(dir);

            if !input.map.is_within_bounds(&next_pos) {
                break;
            }

            match input.map.get_tile(&next_pos) {
                Tile::Space => pos = next_pos,
                Tile::Obstacle => dir = dir.rotate_right(),
            }
        }

        let mut possible_positions = visited_pos_with_dirs
            .iter()
            .flat_map(|(pos, dirs)| {
                dirs.iter().filter_map(|dir| {
                    let next_pos = pos.move_in_dir(*dir);

                    if input.map.is_within_bounds(&next_pos)
                        && input.map.get_tile(&next_pos) == Tile::Space
                        && input
                            .map
                            .intersects_obstacle(pos.clone(), dir.rotate_right())
                    {
                        return Some(next_pos);
                    }

                    None
                })
            })
            .collect::<Vec<Position>>();

        possible_positions.sort();
        possible_positions.dedup();

        possible_positions.iter().filter(|pos| {
            let map = input.map.replace_tile(pos, Tile::Obstacle);

            let mut pos = input.starting_pos.clone();
            let mut dir = UP;

            let mut visited_pos_with_dirs = HashMap::<Position, Vec<Vector>>::new();

            loop {
                match visited_pos_with_dirs.get_mut(&pos) {
                    Some(dirs) => {
                        if dirs.contains(&dir) {
                            return true;
                        }

                        dirs.push(dir);
                    }
                    None => {
                        let dirs = vec![dir.clone()];
                        visited_pos_with_dirs.insert(pos.clone(), dirs);
                    }
                }

                let next_pos = pos.move_in_dir(dir);

                if !map.is_within_bounds(&next_pos) {
                    return false;
                }

                match map.get_tile(&next_pos) {
                    Tile::Space => pos = next_pos,
                    Tile::Obstacle => dir = dir.rotate_right(),
                }
            }
        }).count()
    }
}

fn main() {
    Runner::new(Parser, Solver).run(&vec!["src/06/input_1.txt", "src/06/input_2.txt"]);
}
