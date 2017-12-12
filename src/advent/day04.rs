use advent::AdventSolver;
use failure::Error;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Provides sorted().
use itertools::Itertools;

#[derive(Default)]
pub struct Solver {
    passphrases: Vec<String>
}

impl Solver {
    fn count_valid(&self, transform: fn(&str)->String) -> usize {
         self.passphrases.iter()
                         .filter(|pass| Self::is_valid(pass, transform))
                         .map(|_| 1)
                         .sum()
    }

    fn is_valid(passphrase: &str, transform: fn(&str)->String) -> bool {
        let words: Vec<String> = passphrase.split(" ")
                                           .map(|word| transform(word))
                                           .collect();
        let mut seen_words: HashSet<&str> = HashSet::new();
        for word in &words {
            if seen_words.contains(word as &str) {
                return false;
            }
            seen_words.insert(word as &str);
        }
        true
    }
}

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let file = BufReader::new(File::open("input/day04.txt")?);
        self.passphrases = file.lines().collect::<Result<_,_>>()?;
        let num_valid: usize         = self.count_valid(|s| s.to_string());
        let num_valid_anagram: usize = self.count_valid(|s| s.chars()
                                                             .sorted()
                                                             .iter()
                                                             .collect());
        println!("Valid passphrases (identity): {}", num_valid);
        println!("Valid passphrases (anagram):  {}", num_valid_anagram);
        Ok(())
    }
}
