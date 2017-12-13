use advent::AdventSolver;
use failure::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
pub struct Solver {
    scanners: Vec<Option<usize>>
}

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let f = BufReader::new(File::open("input/day13.txt")?);
        for line in f.lines() {
            let line = line?;
            let parts = line.split(": ").collect::<Vec<&str>>();
            let depth = parts[0].parse::<usize>()?;
            let range = parts[1].parse::<usize>()?;
            while self.scanners.len() < depth {
                self.scanners.push(None);
            }
            self.scanners.push(Some(range));
        }

        println!("Severity with no delay (part 1): {}",
                 self.run_attempt(0, false));
        let mut delay = 0;
        loop {
            if self.run_attempt(delay, true) == 0 {
                println!("Didn't get caught with delay {}.", delay);
                break;
            }
            delay += 1;
        }

        Ok(())
    }
}

impl Solver {
    // Strangely, getting caught at depth 0 is not counted against you in part
    // 1 (for the severity calculation), but for part 2 you can't get caught
    // at all. So I use pass_fail = true for part 2. In pass_fail mode, result
    // is always 0 (pass) or 1 (fail).
    fn run_attempt(&mut self, delay: usize, pass_fail: bool) -> usize {
        let mut severity: usize = 0;
        for (depth, scanner) in self.scanners.iter().enumerate() {
            match scanner {
                &None => {},
                &Some(ref range) => {
                    let period = (range-1) * 2;
                    if (delay+depth) % period == 0 {
                        if pass_fail {
                            return 1;
                        }
                        severity += depth * range;
                    }
                }
            }
        }
        severity
    }
}
