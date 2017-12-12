use advent::AdventSolver;
use failure::Error;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs::File;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::io::Read;

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let mut f = File::open("input/day06.txt")?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;

        let init_banks: Vec<usize> = contents.trim()
                                             .split_whitespace()
                                             .map(|s| s.parse::<usize>())
                                             .collect::<Result<_,_>>()?;

        let mut seen_configurations: HashMap<u64, usize> = HashMap::new();
        let mut memory = Memory::new(&init_banks);
        let mut rebalancings: usize = 0;
        let mut hash = memory.get_hash();
        while !seen_configurations.contains_key(&hash) {
            seen_configurations.insert(hash, rebalancings);
            memory.rebalance();
            println!("{}", memory);
            rebalancings += 1;
            hash = memory.get_hash();
        }

        println!("Duplicate configuration after {} rebalancings.", rebalancings);
        println!("Cycle size was {}.", rebalancings - seen_configurations[&hash]);
        Ok(())
    }
}

#[derive(Hash)]
struct Memory {
    banks: Vec<usize>
}

impl Memory {
    pub fn new(initial_counts: &Vec<usize>) -> Self {
        Memory {
            banks: initial_counts.clone()
        }
    }

    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }

    pub fn clear_bank(&mut self, bank_id: usize) {
        self.banks[bank_id] = 0;
    }

    pub fn add_block(&mut self, bank_id: usize) {
        self.banks[bank_id] += 1;
    }

    pub fn rebalance(&mut self) {
        let mut cur_index: usize =
            self.banks.iter()
                      .enumerate()
                      // Sort by # of blocks, i as tiebreaker (lower is better)
                      .max_by_key(|&(i, count)| (count, self.banks.len()-i))
                      .unwrap().0;
        let mut blocks_to_distribute = self.banks[cur_index];
        self.clear_bank(cur_index);
        while blocks_to_distribute > 0 {
            cur_index = (cur_index+1) % self.banks.len();
            self.add_block(cur_index);
            blocks_to_distribute -= 1;
        }
    }
}

impl fmt::Display for Memory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Memory: ")?;
        for count in &self.banks {
            write!(f, "{:3}", count)?;
        }
        write!(f, ">")
    }
}
