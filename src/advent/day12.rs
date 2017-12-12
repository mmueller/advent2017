use advent::AdventSolver;
use failure::Error;
use regex::Regex;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

lazy_static! {                                                                  
    static ref PIPE_RE: Regex =
        Regex::new(r"^(\d+) <-> ([\d, ]+)$").unwrap();
}

#[derive(Clone)]
struct Pipe {
    program_id: usize,
    targets: Vec<usize>
}

#[derive(Default)]
pub struct Solver {
    groups: Vec<HashSet<usize>>
}

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let pipes = self.read_pipes("input/day12.txt")?;
        self.build_groups(&pipes);
        self.report();
        Ok(())
    }
}

impl Solver {
    fn read_pipes(&mut self, path: &str) -> Result<Vec<Pipe>, Error> {
        let f = BufReader::new(File::open(path)?);
        let mut pipes = Vec::new();
        for line in f.lines() {
            let line = line?;
            let caps = match PIPE_RE.captures(&line) {
                           Some(caps) => caps,
                           None => return Err(format_err!(
                                              "Couldn't parse pipe: {}", line))
                       };
            let program_id = caps[1].parse::<usize>()?;
            let targets = caps[2].split(", ")
                                 .map(|s| s.parse::<usize>().unwrap())
                                 .collect::<Vec<usize>>();
            pipes.push(Pipe { program_id: program_id, targets: targets });
        }
        println!("Read {} pipes.", pipes.len());
        Ok(pipes)
    }

    fn build_groups(&mut self, pipes: &Vec<Pipe>) {
        for pipe in pipes {
            let mut p_group_index = self.find_or_create_group(pipe.program_id);
            for target in pipe.targets.iter().cloned() {
                match self.find_group(target) {
                    Some(t_group_index) => {
                        // If target is already in another group, merge that
                        // group into program_id's group.
                        if t_group_index != p_group_index {
                            p_group_index = self.merge_groups(p_group_index,
                                                              t_group_index);
                        }
                    },
                    None => {
                        // Target not in any group, add to program_id's group.
                        self.groups[p_group_index].insert(target);
                    }
                }
            }
        }
    }

    fn find_group(&self, program_id: usize) -> Option<usize> {
        self.groups.iter()
                   .enumerate()
                   .find(|&(_, group)| group.contains(&program_id))
                   .map(|(i, _)| i)
    }

    // Returns the index of the group containing program_id.
    fn find_or_create_group(&mut self, program_id: usize) -> usize {
        match self.find_group(program_id) {
            Some(i) => i,
            None => {
                self.groups.push(HashSet::new());
                let new_group_index = self.groups.len() - 1;
                self.groups[new_group_index].insert(program_id);
                new_group_index
            }
        }
    }

    // Merge all items from group index j into group index i, then remove
    // group index j. Returns the new index of group i.
    fn merge_groups(&mut self, i: usize, j: usize) -> usize {
        self.groups[i] = self.groups[i].union(&self.groups[j])
                                       .map(|p| *p)
                                       .collect::<HashSet<usize>>();
        self.groups.remove(j);
        if j < i {
            i - 1
        } else {
            i
        }
    }

    fn report(&self) {
        println!("Pipes form {} groups.", self.groups.len());
        for i in 0..self.groups.len() {
            if self.groups[i].contains(&0) {
                println!("Program 0's group contains {} programs.",
                         self.groups[i].len());
            }
        }
    }
}
