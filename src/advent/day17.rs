use advent::AdventSolver;
use failure::Error;

#[derive(Default)]
pub struct Solver;

const STEP_SIZE: usize = 337;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        // Part 1: Naive implementation.
        let mut buf: Vec<usize> = Vec::new();
        buf.push(0);
        let mut pos = 0;
        for i in 1..2018 {
            pos = (pos+1 + STEP_SIZE) % buf.len();
            if pos == buf.len()-1 {
                buf.push(i);
            } else {
                buf.insert(pos+1, i);
            }
        }
        let mut len = buf.len();
        println!("Thing after 2017: {}", buf[(pos+2)%len]);

        // Part 2: Zero never moves (it's always at pos 0).
        // No longer putting everything in the buffer, just simulating.
        // The value we want is always at pos==1, so just monitor that.
        let mut thing_after_zero = 0;
        for i in 2018..50_000_000 {
            pos = (pos+1 + STEP_SIZE) % len;
            if pos == 0 {
                thing_after_zero = i;
            }
            len += 1;
        }
        println!("Thing after zero: {}", thing_after_zero);
        Ok(())
    }
}
