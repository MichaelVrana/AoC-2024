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

struct TopSorter<'a, 'b> {
    dependency_map: &'a DependencyMap,
    pages_in_update: &'b HashSet<Page>,
    topological_order: Vec<Page>,
}

impl<'a, 'b> TopSorter<'a, 'b> {
    fn new(dependency_map: &'a DependencyMap, pages_in_update: &'b HashSet<Page>) -> Self {
        Self {
            dependency_map,
            pages_in_update,
            topological_order: Vec::new(),
        }
    }

    fn top_sort(mut self) -> Vec<Page> {
        for page_in_update in self.pages_in_update {
            self.visit(*page_in_update);

            if self.pages_in_update.len() == self.topological_order.len() {
                break;
            }
        }

        self.topological_order
    }

    fn visit(&mut self, page: Page) {
        if self.topological_order.contains(&page) {
            return;
        }

        if let Some(page_deps) = self.dependency_map.get(&page) {
            for page_dep in page_deps.intersection(&self.pages_in_update) {
                self.visit(*page_dep);
            }
        }

        self.topological_order.push(page);
    }
}

struct Solver;

impl ProblemSolver<Input, Result> for Solver {
    fn solve(&self, input: Input) -> Result {
        let dependency_map = input.get_dependency_map();
        let empty_set = HashSet::new();

        input
            .updates
            .iter()
            .map(|update| {
                let pages_in_update: HashSet<Page> = update.iter().map(|page| *page).collect();

                let mut updated = HashSet::<Page>::new();

                let constraints_satisfied = update.iter().all(|page_in_update| {
                    let deps = dependency_map.get(page_in_update).unwrap_or(&empty_set);

                    let deps_printed = deps
                        .intersection(&pages_in_update)
                        .all(|page_dep| updated.contains(page_dep));

                    updated.insert(*page_in_update);

                    deps_printed
                });

                if !constraints_satisfied {
                    let fixed_update = TopSorter::new(&dependency_map, &pages_in_update).top_sort();
                    *fixed_update.get(fixed_update.len() / 2).unwrap()
                } else {
                    0
                }
            })
            .sum()
    }
}

fn main() {
    Runner::new(Parser, Solver).run(&vec![
        "src/05/input_1.txt",
        "src/05/input_2.txt",
        "src/05/input_3.txt",
        "src/05/input_4.txt",
    ]);
}
