use failure::Error;

trait AdventSolver {
    fn solve(&mut self) -> Result<(), Error>;
}

include!("_generated_advent_mod.rs");
