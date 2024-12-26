use std::{fmt::Display, marker::PhantomData};

pub trait InputParser<TProblem> {
    fn parse(&self, filename: &str) -> TProblem;
}

pub trait ProblemSolver<TProblem, TResult: Display> {
    fn solve(&self, input: TProblem) -> TResult;
}

pub struct Runner<TProblem, TResult, TInputParser, TProblemSolver>
where
    TResult: Display,
    TInputParser: InputParser<TProblem>,
    TProblemSolver: ProblemSolver<TProblem, TResult>,
{
    parser: TInputParser,
    solver: TProblemSolver,

    phantom_problem: PhantomData<TProblem>,
    phantom_result: PhantomData<TResult>,
}

impl<TProblem, TResult, TInputParser, TProblemSolver>
    Runner<TProblem, TResult, TInputParser, TProblemSolver>
where
    TResult: Display,
    TInputParser: InputParser<TProblem>,
    TProblemSolver: ProblemSolver<TProblem, TResult>,
{
    pub fn new(parser: TInputParser, solver: TProblemSolver) -> Self {
        Runner {
            parser,
            solver,

            phantom_problem: PhantomData,
            phantom_result: PhantomData,
        }
    }

    pub fn run(&self, input_files: &Vec<&'_ str>) {
        for filename in input_files.iter() {
            let input = self.parser.parse(filename);
            let result = self.solver.solve(input);

            println!("File {}: {}", filename, result)
        }
    }
}
