use advent::AdventSolver;
use failure::Error;
use util::duet::{self,Program};

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    // Part 1 solution is uh, gone now. This solves part 2 only.
    fn solve(&mut self) -> Result<(), Error> {
        let instructions = duet::parse_instructions("input/day18.txt")?;
        let mut program0 = Program::new(0, &instructions);
        let mut program1 = Program::new(1, &instructions);
        let mut values_sent_by_program1: usize = 0;

        while program0.is_running() || program1.is_running() {
            match program0.step()? {
                Some(v) => {
                    program1.add_to_queue(v);
                },
                None => {}
            }
            match program1.step()? {
                Some(v) => {
                    values_sent_by_program1 += 1;
                    program0.add_to_queue(v);
                },
                None => {}
            }
        }
        println!("Program1 sent {} values to program0.",
                 values_sent_by_program1);
        Ok(())
    }
}
