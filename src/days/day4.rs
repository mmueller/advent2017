use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

// Provides sorted().
use itertools::Itertools;

pub fn solve() {
    let file = BufReader::new(File::open("input/day4.txt")
                                   .expect("file not found"));
    let passphrases: Vec<String> = file.lines()
                                     .map(|s| s.unwrap())
                                     .collect();
    let num_valid: usize = count_valid(&passphrases, |s| s.to_string());
    let num_valid_anagram: usize = count_valid(&passphrases,
                                               |s| s.chars()
                                                    .sorted()
                                                    .iter()
                                                    .collect());
    println!("Valid passphrases (identity): {}", num_valid);
    println!("Valid passphrases (anagram):  {}", num_valid_anagram);
}

fn count_valid(passphrases: &Vec<String>, transform: fn(&str)->String) -> usize {
     passphrases.iter()
                .filter(|passphrase| is_valid(passphrase, transform))
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
