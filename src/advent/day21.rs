use advent::AdventSolver;
use failure::Error;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(Default)]
pub struct Solver;

struct Rule {
    pattern: Vec<Vec<bool>>,
    pattern_size: usize,
    replacement: Vec<Vec<bool>>,
}

type RuleHashKey = (usize, u32);

impl Rule {
    fn parse(text: &str) -> Result<Self, Error> {
        let parts = text.split(" => ").collect::<Vec<&str>>();
        let lhs = parts[0];
        let rhs = parts[1];
        let rows = lhs.split("/").collect::<Vec<&str>>();
        let mut pattern: Vec<Vec<bool>> = Vec::new();
        for &r in &rows {
            let mut pattern_row: Vec<bool> = Vec::new();
            for c in r.chars() {
                pattern_row.push(match c {
                    '.' => false,
                    '#' => true,
                    _   => return Err(format_err!("parse failed: {}", text))
                });
            }
            pattern.push(pattern_row);
        }

        let rows = rhs.split("/").collect::<Vec<&str>>();
        let mut replacement: Vec<Vec<bool>> = Vec::new();
        for &r in &rows {
            let mut replacement_row: Vec<bool> = Vec::new();
            for c in r.chars() {
                replacement_row.push(match c {
                    '.' => false,
                    '#' => true,
                    _   => return Err(format_err!("parse failed: {}", text))
                });
            }
            replacement.push(replacement_row);
        }

        let size = pattern.len();
        Ok(Rule {
            pattern: pattern,
            pattern_size: size,
            replacement: replacement
        })
    }

    // Unique representation of the pattern part of this rule.
    fn pattern_hashes(&self) -> Vec<RuleHashKey> {
        let size = self.pattern_size;
        let mut hashes: Vec<u32> = Vec::new();
        for _ in 0..8 {
            hashes.push(0);
        }
        for row in 0..size {
            let desc_row = size - row - 1;
            for col in 0..size {
                let desc_col = size - col - 1;
                if self.pattern[row][col] {
                    hashes[0] |= 1 << (row*size      + col);
                    hashes[1] |= 1 << (col*size      + row);
                    hashes[2] |= 1 << (desc_row*size + col);
                    hashes[3] |= 1 << (col*size      + desc_row);
                    hashes[4] |= 1 << (row*size      + desc_col);
                    hashes[5] |= 1 << (desc_col*size + row);
                    hashes[6] |= 1 << (desc_row*size + desc_col);
                    hashes[7] |= 1 << (desc_col*size + desc_row);
                }
            }
        }
        hashes.iter()
              .map(|&h| (size, h))
              .collect::<Vec<RuleHashKey>>()
    }
}

struct Image {
    rows: Vec<Vec<bool>>,
}

impl Image {
    // Returns this image:
    //   .#.
    //   ..#
    //   ###
    fn default() -> Self {
        let mut rows: Vec<Vec<bool>> = Vec::new();
        rows.push(vec![false,  true, false]);
        rows.push(vec![false, false,  true]);
        rows.push(vec![ true,  true,  true]);
        Image {
            rows: rows
        }
    }

    // Returns a size x size image of all pixels turned off.
    fn empty(size: usize) -> Self {
        let mut rows: Vec<Vec<bool>> = Vec::new();
        for _ in 0..size {
            rows.push(vec![false; size]);
        }
        Image {
            rows: rows
        }
    }

    fn enhance(&self, rules: &HashMap<RuleHashKey, &Rule>) -> Image {
        let old_size = self.rows.len();
        let block_size = if old_size % 2 == 0 { 2 } else { 3 };
        let num_blocks = old_size / block_size;
        let new_size = num_blocks * (block_size + 1);
        let mut result: Image = Image::empty(new_size);

        for block_row in 0..num_blocks {
            let old_row_offset = block_row * block_size;
            let new_row_offset = block_row * (block_size + 1);
            for block_col in 0..num_blocks {
                let old_col_offset = block_col * block_size;
                let new_col_offset = block_col * (block_size + 1);
                let mut bitfield: u32 = 0;
                for row in 0..block_size {
                    for col in 0..block_size {
                        if self.rows[old_row_offset+row][old_col_offset+col] {
                            bitfield |= 1 << (row * block_size + col);
                        }
                    }
                }
                let rule = rules[&(block_size, bitfield)];
                for row in 0..(block_size+1) {
                    for col in 0..(block_size+1) {
                        if rule.replacement[row][col] {
                            result.rows[new_row_offset+row]
                                       [new_col_offset+col] = true;
                        }
                    }
                }
            }
        }

        result
    }

    fn lit_pixels(&self) -> usize {
        self.rows.iter()
                 .map(|ref row| row.iter()
                                .map(|b| *b as usize)
                                .sum::<usize>())
                 .sum()
    }
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.rows {
            for &value in row.iter() {
                match value {
                    true => { write!(f, "#")?; },
                    false => { write!(f, ".")?; },
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let rules = self.read_rules()?;
        let mut image = Image::default();
        let mut rules_map: HashMap<RuleHashKey, &Rule> = HashMap::new();

        for rule in &rules {
            for key in rule.pattern_hashes() {
                rules_map.insert(key, rule);
            }
        }

        println!("Starting image:");
        print!("{}", image);

        let mut lit_after_5 = 0;
        for i in 1..19 {
            image = image.enhance(&rules_map);
            if i <= 5 {
                print!("\nEnhancement {}:\n{}", i, image);
            }
            if i == 5 {
                lit_after_5 = image.lit_pixels();
                println!("\n(stopping printing because they get too big)\n");
            }
        }

        println!("Lit pixels after 5:  {}", lit_after_5);
        println!("Lit pixels after 18: {}", image.lit_pixels());

        Ok(())
    }
}

impl Solver {
    fn read_rules(&self) -> Result<Vec<Rule>, Error> {
        let f = BufReader::new(File::open("input/day21.txt")?);
        let mut rules: Vec<Rule> = Vec::new();

        for line in f.lines() {
            rules.push(Rule::parse(&line?)?);
        }

        Ok(rules)
    }
}
