use advent::AdventSolver;
use failure::Error;
use std::str::Chars;
use std::fs::File;
use std::io::Read;

#[derive(Default)]
pub struct Solver {
    garbage_count: usize,
}

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let mut f = File::open("input/day9.txt")?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        println!("Total score: {}",
                 self.score_groups(&mut contents.trim().chars(), 0));
        println!("Amount of garbage: {}", self.garbage_count);
        Ok(())
    }
}

impl Solver {
    fn score_groups(&mut self, input: &mut Chars, depth: usize) -> usize {
        let mut result = 0;
        loop {
            match input.next() {
                Some('{') => {
                    result += self.score_groups(input, depth+1);
                },
                Some('}') => {
                    if depth == 0 {
                        println!("Unexpected group closing bracket.");
                    }
                    result += depth;
                    break;
                },
                Some('<') => {
                    self.consume_garbage(input);
                },
                Some(_)   => { },
                None      => {
                    if depth > 0 {
                        println!("ran out of input at depth {}", depth);
                    }
                    break;
                }
            }
        }
        result
    }

    // Assumes we've already encountered a '<', advances past closing '>'.
    fn consume_garbage(&mut self, input: &mut Chars) {
        let mut ignore_next: bool = false;
        loop {
            let c = input.next();
            if ignore_next {
                ignore_next = false;
                continue;
            }
            match c {
                Some('!') => ignore_next = true,
                Some('>') => break,
                None => { println!("ran out of input in garbage"); break },
                _ => { self.garbage_count += 1 }
            }
        }
    }
}
