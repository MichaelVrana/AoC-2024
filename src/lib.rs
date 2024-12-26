use std::fmt::Display;

pub trait InputParser<TProblem> {
    fn parse(&mut self, filename: &str) -> TProblem;
}

pub trait ProblemSolver<TProblem, TResult: Display> {
    fn solve(&mut self, input: TProblem) -> TResult;
}

pub struct Runner<'a, 'b, TProblem, TResult> {
    input_files: Vec<&'a str>,

    // TODO find a way to remove the dynamic dispatch
    parser: &'a mut dyn InputParser<TProblem>,
    solver: &'b mut dyn ProblemSolver<TProblem, TResult>,
}

impl<'a, 'b, TProblem, TResult: Display> Runner<'a, 'b, TProblem, TResult> {
    pub fn new(
        parser: &'a mut dyn InputParser<TProblem>,
        solver: &'b mut dyn ProblemSolver<TProblem, TResult>,
        input_files: Vec<&'a str>,
    ) -> Self {
        Runner {
            input_files,
            parser,
            solver,
        }
    }

    pub fn run(&mut self) {
        for filename in self.input_files.iter() {
            let input = self.parser.parse(filename);
            let result = self.solver.solve(input);

            println!("File {}: {}", filename, result)
        }
    }
}
