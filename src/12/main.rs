use std::{collections::HashSet, fs::read_to_string, hash::Hash};

use aoc_2024::{InputParser, ProblemSolver, Runner};

type PlantType = u8;

#[derive(Clone, Copy)]
struct Vector {
    x: i8,
    y: i8,
}

impl Vector {
    fn reverse(&self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
        }
    }
}

const UP: Vector = Vector { x: 0, y: -1 };
const RIGHT: Vector = Vector { x: 1, y: 0 };
const DOWN: Vector = Vector { x: 0, y: 1 };
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
    plants: Vec<PlantType>,
    x_len: usize,
    y_len: usize,
}

impl Map {
    fn get_plant_type(&self, position: &Position) -> PlantType {
        self.plants[position.x + position.y * self.x_len]
    }

    fn is_within_bounds(&self, position: &Position) -> bool {
        position.x < self.x_len && position.y < self.y_len
    }
}

type Input = Map;

struct Parser;

impl InputParser<Input> for Parser {
    fn parse(&self, filename: &str) -> Input {
        let file = read_to_string(filename).unwrap();

        let x_len = file.lines().next().unwrap().len();

        let mut y_len = 0;

        let plants: Vec<PlantType> = file
            .lines()
            .flat_map(|line| {
                y_len += 1;

                line.chars().map(|plant_type| plant_type as PlantType)
            })
            .collect();

        Input {
            plants,
            x_len,
            y_len,
        }
    }
}

type Price = u64;

type Area = u32;

#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Border {
    in_region: Position,
    out_of_region: Position,
}

impl Border {
    fn border_dir(&self) -> Vector {
        if self.in_region.x == self.out_of_region.x {
            RIGHT
        } else {
            DOWN
        }
    }

    fn move_by(&self, dir: Vector) -> Border {
        Border {
            in_region: self.in_region.move_by(dir),
            out_of_region: self.out_of_region.move_by(dir),
        }
    }
}

struct RegionFinder<'a> {
    visited: HashSet<Position>,
    borders: HashSet<Border>,
    map: &'a Map,
}

impl<'a> RegionFinder<'a> {
    fn new(map: &'a Map) -> Self {
        RegionFinder {
            map,
            borders: HashSet::new(),
            visited: HashSet::new(),
        }
    }

    fn find_new_region_price(&mut self, position: Position) -> Price {
        if self.visited.contains(&position) {
            return 0;
        }

        let plant_type = self.map.get_plant_type(&position);

        let area = self.find_region(plant_type, position);
        let side_count = self.count_sides(plant_type);

        area as Price * side_count as Price
    }

    fn find_region(&mut self, plant_type: PlantType, position: Position) -> Area {
        if self.visited.contains(&position) {
            return 0;
        }

        self.visited.insert(position.clone());

        let neighboring_positions = POSSIBLE_DIRECTIONS.iter().map(|dir| position.move_by(*dir));

        neighboring_positions
            .clone()
            .filter(|neighboring_position| {
                !self.map.is_within_bounds(neighboring_position)
                    || self.map.get_plant_type(&neighboring_position) != plant_type
            })
            .for_each(|position_over_border| {
                self.borders.insert(Border {
                    in_region: position.clone(),
                    out_of_region: position_over_border,
                });
            });

        1 + neighboring_positions
            .filter(|neighboring_position| {
                self.map.is_within_bounds(&neighboring_position)
                    && self.map.get_plant_type(&neighboring_position) == plant_type
            })
            .map(|neighboring_position| self.find_region(plant_type, neighboring_position))
            .sum::<Area>()
    }

    fn remove_borders_in_dir(&mut self, plant_type: PlantType, mut border: Border, dir: Vector) {
        loop {
            self.borders.remove(&border);

            border = border.move_by(dir);

            if !self.map.is_within_bounds(&border.in_region)
                || self.map.get_plant_type(&border.in_region) != plant_type
                || (self.map.is_within_bounds(&border.out_of_region)
                    && self.map.get_plant_type(&border.out_of_region) == plant_type)
            {
                break;
            }
        }
    }

    fn count_sides(&mut self, plant_type: PlantType) -> u32 {
        let mut sides = 0;

        while let Some(border) = self.borders.iter().cloned().next() {
            sides += 1;

            let dir = border.border_dir();

            self.remove_borders_in_dir(plant_type, border.clone(), dir);
            self.remove_borders_in_dir(plant_type, border, dir.reverse());
        }

        sides
    }
}

type Output = Price;

struct Solver;

impl ProblemSolver<Input, Output> for Solver {
    fn solve(&self, input: Input) -> Output {
        let mut region_finder = RegionFinder::new(&input);

        (0..input.y_len)
            .flat_map(|y| (0..input.x_len).map(move |x| Position { x, y }))
            .map(|position| region_finder.find_new_region_price(position))
            .sum()
    }
}

fn main() {
    Runner::new(Parser, Solver).run(&vec![
        "src/12/input_1.txt",
        "src/12/input_2.txt",
        "src/12/input_3.txt",
        "src/12/input_4.txt",
        "src/12/input_5.txt",
    ]);
}
