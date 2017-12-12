use advent::AdventSolver;
use failure::Error;
use itertools::Itertools;
use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(Default)]
pub struct Solver {
    instructions: Vec<isize>
}

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let file = BufReader::new(File::open("input/day05.txt")?);
        self.instructions = file.lines()
                                .map_results(|s| s.parse::<isize>().unwrap())
                                .collect::<Result<_,_>>()?;

        println!("Steps to escape (pt 1): {}",
                 self.execute(|i| i + 1));
        println!("Steps to escape (pt 2): {}",
                 self.execute(|i| if i >= 3 { i - 1 } else { i + 1 }));
        Ok(())
    }
}

impl Solver {
    // Execute the instructions (using the given instruction-modifying rule)
    // and return the number of steps required to escape.
    fn execute(&self, rule: fn(isize) -> isize) -> usize{
        // Using isize for the program counter just in case a jump takes it into
        // negative range (which would also qualify as "outside" the list).
        let mut instructions = self.instructions.clone();
        let mut pc: isize = 0;
        let mut steps_taken: usize = 0;
        while pc >= 0 && (pc as usize) < instructions.len() {
            let jmp = instructions[pc as usize];
            instructions[pc as usize] = rule(jmp);
            pc += jmp;
            steps_taken += 1;
        }
        steps_taken
    }
}
