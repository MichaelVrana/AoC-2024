pub trait InputParser<TProblem> {
    fn parse(filename: &str) -> TProblem;
}

pub trait Solver<TProblem, TResult: ToString> {
    fn solve(input: TProblem) -> TResult;
}

pub struct ProblemRunner<'a, TProblem> {
    filenames: Vec<String>,
    input_parser: InputParser<TProblem>,
}