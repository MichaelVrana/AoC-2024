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

type Page = u32;

type DependencyMap = HashMap<Page, HashSet<Page>>;

impl Input {
    fn get_dependency_map(&self) -> DependencyMap {
        self.deps
            .iter()
            .fold(DependencyMap::new(), |mut deps, (depends_on, dependant)| {
                match deps.get_mut(dependant) {
                    None => {
                        let mut value_deps = HashSet::new();
                        value_deps.insert(*depends_on);
                        deps.insert(*dependant, value_deps);
                    }
                    Some(value_deps) => {
                        value_deps.insert(*depends_on);
                    }
                }

                deps
            })
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

fn check_dependencies_rec(
    dependency_map: &DependencyMap,
    pages_to_update: &HashSet<u32>,
    updated: &HashSet<u32>,
    page_deps: &HashSet<Page>,
) -> bool {
    page_deps.intersection(pages_to_update).all(|page_dep| {
        if !updated.contains(page_dep) {
            return false;
        }

        match dependency_map.get(page_dep) {
            Some(child_deps) => {
                check_dependencies_rec(dependency_map, pages_to_update, updated, child_deps)
            }
            None => true,
        }
    })
}

impl ProblemSolver<Input, Result> for Solver {
    fn solve(&self, input: Input) -> Result {
        let dependency_map = input.get_dependency_map();

        input
            .updates
            .iter()
            .map(|update| {
                let pages_to_update: HashSet<Page> = update.iter().map(|page| *page).collect();
                let mut updated = HashSet::<Page>::new();

                let constraints_satisfied = update.iter().all(|page_in_update| {
                    let result = match dependency_map.get(page_in_update) {
                        Some(page_deps) => check_dependencies_rec(
                            &dependency_map,
                            &pages_to_update,
                            &updated,
                            page_deps,
                        ),
                        None => true,
                    };

                    updated.insert(*page_in_update);

                    result
                });

                if constraints_satisfied {
                    let result = *update.get(update.len() / 2).unwrap();

                    result
                } else {
                    0
                }
            })
            .sum()
    }
}

fn main() {
    Runner::new(Parser, Solver).run(&vec!["src/05/input_1.txt", "src/05/input_2.txt"]);
}
