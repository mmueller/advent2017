use advent::AdventSolver;
use advent::knot::{knot_hash,KnotHash};
use failure::Error;
use std::fs::File;
use std::io::Read;

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let mut f = File::open("input/day10.txt")?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        let contents = contents.trim();
        self.solve_part1(contents)?;
        self.solve_part2(contents)?;
        Ok(())
    }
}

impl Solver {
    fn solve_part1(&self, contents: &str) -> Result<(), Error> {
        // Interpret contents as comma-separated integer lengths
        let lengths = contents.trim()
                              .split(",")
                              .map(|s| s.parse::<u8>().unwrap())
                              .collect::<Vec<u8>>();
        let mut hash = KnotHash::new();
        for length in lengths {
            hash.update(length);
        }

        println!("Product of first two: {}",
                 hash.product_of_first_two_bytes());
        Ok(())
    }

    fn solve_part2(&self, contents: &str) -> Result<(), Error> {
        // Interpret contents as bytes, run 64 rounds
        let hash = knot_hash(contents.as_bytes());
        for byte in hash.value().iter() {
            print!("{:02x}", *byte);
        }
        println!("");

        Ok(())
    }
}
