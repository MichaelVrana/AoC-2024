use std::{collections::HashSet, fs::read_to_string, hash::Hash};

use aoc_2024::{InputParser, ProblemSolver, Runner};

type PlantType = u8;

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
type Perimiter = u32;

#[derive(Clone, Copy)]
struct Region {
    area: Area,
    perimeter: Perimiter,
}

impl Region {
    fn price(&self) -> Price {
        self.perimeter as Price * self.area as Price
    }

    fn merge_part(&self, other: &Region) -> Region {
        Region {
            area: self.area + other.area,
            perimeter: self.perimeter + other.perimeter,
        }
    }
}

struct RegionFinder<'a> {
    visited: HashSet<Position>,
    map: &'a Map,
}

impl<'a> RegionFinder<'a> {
    fn new(map: &'a Map) -> Self {
        RegionFinder {
            map,
            visited: HashSet::<Position>::new(),
        }
    }

    fn find_new_region_price(&mut self, position: Position) -> Price {
        if self.visited.contains(&position) {
            return 0;
        }

        let plant_type = self.map.get_plant_type(&position);

        self.find_region(plant_type, position).price()
    }

    fn find_region(&mut self, plant_type: PlantType, position: Position) -> Region {
        if self.visited.contains(&position) {
            return Region {
                area: 0,
                perimeter: 0,
            };
        }

        self.visited.insert(position.clone());

        let neighboring_positions = POSSIBLE_DIRECTIONS.iter().map(|dir| position.move_by(*dir));

        let perimeter = neighboring_positions
            .clone()
            .filter(|neighboring_position| {
                !self.map.is_within_bounds(neighboring_position)
                    || self.map.get_plant_type(&neighboring_position) != plant_type
            })
            .count() as Perimiter;

        let region_part = Region { area: 1, perimeter };

        neighboring_positions
            .filter(|neighboring_position| {
                self.map.is_within_bounds(&neighboring_position)
                    && self.map.get_plant_type(&neighboring_position) == plant_type
            })
            .map(|neighboring_position| self.find_region(plant_type, neighboring_position))
            .fold(region_part, |region, region_part| {
                region.merge_part(&region_part)
            })
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
    ]);
}
