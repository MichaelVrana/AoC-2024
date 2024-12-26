use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use aoc_2024::{InputParser, ProblemSolver, Runner};

type Deps = Vec<(u32, u32)>;

type Updates = Vec<Vec<u32>>;

struct Input {
    deps: Deps,
    updates: Updates,
}

type Dependencies = HashMap<u32, HashSet<u32>>;

fn add_transitive_deps(deps: Dependencies) -> Dependencies {
    deps.
}

impl Input {
    fn get_deps(&self) {
        let deps =
            self.deps
                .iter()
                .fold(Dependencies::new(), |mut deps, (depends_on, dependant)| {
                    match deps.get_mut(dependant) {
                        None => {
                            let mut value_deps = HashSet::new();
                            deps.insert(*dependant, value_deps);
                        }
                        Some(value_deps) => {
                            value_deps.insert(*depends_on);
                        }
                    }

                    deps
                });

        add_transitive_deps(deps)
    }
}

struct Parser;

impl InputParser<Input> for Parser {
    fn parse(&self, filename: &str) -> Input {
        let file = read_to_string(filename).unwrap();

        let lines = file.lines();

        let deps: Deps = lines
            .clone()
            .take_while(|line| !line.is_empty())
            .map(|line| {
                let (depends_on, dependant) = line.split_once("|").unwrap();

                (depends_on.parse().unwrap(), dependant.parse().unwrap())
            })
            .collect();

        let updates: Updates = lines
            .skip(deps.len() + 1)
            .map(|line| {
                line.split(',')
                    .into_iter()
                    .map(|num| num.parse().unwrap())
                    .collect()
            })
            .collect();

        Input { deps, updates }
    }
}

type Result = u32;

struct Solver;

impl ProblemSolver<Input, Result> for Solver {
    fn solve(&self, input: Input) -> Result {
        let deps = input.deps.iter().fold(
            HashMap::<u32, HashSet<u32>>::new(),
            |mut deps, (depends_on, dependant)| {
                match deps.get_mut(dependant) {
                    None => {
                        let mut value_deps = HashSet::new();
                        deps.insert(*dependant, value_deps);
                    }
                    Some(value_deps) => {
                        value_deps.insert(*depends_on);
                    }
                }

                deps
            },
        );
    }
}

fn main() {
    Runner::new(Parser, Solver).run(&vec!["src/05/input_1.txt"]);
}
