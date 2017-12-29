use advent::AdventSolver;
use failure::Error;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::str::FromStr;
use util::infinite_tape::InfiniteTape;

pub struct Solver {
    machine: TuringMachine
}

#[derive(Clone,Copy)]
enum Dir {
    Left,
    Right,
}

impl FromStr for Dir {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "left" => Ok(Dir::Left),
            "right" => Ok(Dir::Right),
            _ => Err(format_err!("bad direction: {}", s)),
        }
    }
}

struct TuringMachine {
    rules: HashMap<char, TuringMachineRule>,
    tape: InfiniteTape<bool>,
    state: char,
    pos: isize,
    checksum_after: usize,
}

#[derive(Clone,Copy)]
struct TuringMachineRule {
    state: char,
    actions: [TuringMachineAction; 2],
}

#[derive(Clone,Copy)]
struct TuringMachineAction {
    write: bool,
    next_state: char,
    dir: Dir,
}

impl TuringMachine {
    fn step(&mut self) {
        let v = self.tape[self.pos] as usize;
        let rule = self.rules[&self.state];
        let action = rule.actions[v];
        self.tape[self.pos] = action.write;
        match action.dir {
            Dir::Left  => self.pos -= 1,
            Dir::Right => self.pos += 1,
        }
        self.state = action.next_state;
    }

    fn checksum(&self) -> usize {
        self.tape.vec
                 .iter()
                 .map(|v| *v as usize)
                 .sum()
    }

    fn parse(file: &str) -> Result<TuringMachine, Error> {
        let f = BufReader::new(File::open(file)?);
        let desc = f.lines().map(|line| line.unwrap()).collect::<Vec<String>>();
        let mut rules: HashMap<char, TuringMachineRule> = HashMap::new();

        let init_state = Regex::new(r"^Begin in state (.)\.$")?
                               .captures(&desc[0]).unwrap()[1]
                               .chars().next().unwrap();
        let checksum_after = Regex::new(r"^Perform a diag.*after (\d+) steps.")?
                                   .captures(&desc[1]).unwrap()[1]
                                   .parse::<usize>()?;

        let mut i = 3;
        while i < desc.len() {
            let rule = TuringMachineRule::parse(&desc[i..i+9])?;
            rules.insert(rule.state, rule);
            i += 10;
        }

        Ok(TuringMachine {
            rules: rules,
            tape: InfiniteTape::new(false),
            state: init_state,
            pos: 0,
            checksum_after: checksum_after,
        })
    }
}

impl TuringMachineRule {
    // Hey, no one asked you to look at this.
    fn parse(lines: &[String]) -> Result<TuringMachineRule, Error> {
        let state = Regex::new(r"^In state (.):$")?
                          .captures(&lines[0]).unwrap()[1]
                          .chars().next().unwrap();
        let write0 = Regex::new(r"Write the value (0|1).")?
                           .captures(&lines[2]).unwrap()[1]
                           .parse::<u8>()? != 0;
        let dir0 = Regex::new(r"Move one slot to the (left|right).")?
                         .captures(&lines[3]).unwrap()[1]
                         .parse::<Dir>()?;
        let next_state0 = Regex::new(r"Continue with state (.)")?
                                .captures(&lines[4]).unwrap()[1]
                                .chars().next().unwrap();
        let write1 = Regex::new(r"Write the value (0|1).")?
                           .captures(&lines[6]).unwrap()[1]
                           .parse::<u8>()? != 0;
        let dir1 = Regex::new(r"Move one slot to the (left|right).")?
                         .captures(&lines[7]).unwrap()[1]
                         .parse::<Dir>()?;
        let next_state1 = Regex::new(r"Continue with state (.)")?
                                .captures(&lines[8]).unwrap()[1]
                                .chars().next().unwrap();
        let action0 = TuringMachineAction {
            write: write0,
            next_state: next_state0,
            dir: dir0,
        };
        let action1 = TuringMachineAction {
            write: write1,
            next_state: next_state1,
            dir: dir1,
        };

        Ok(TuringMachineRule {
            state: state, 
            actions: [action0, action1]
        })
    }
}

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        eprint!("Running Turing machine");
        for i in 0..self.machine.checksum_after {
            self.machine.step();
            if i % 10000 == 0 {
                eprint!(".");
            }
        }
        eprint!("\n");
        println!("After {} steps, checksum: {}",
                 self.machine.checksum_after,
                 self.machine.checksum());
        Ok(())
    }
}

impl Default for Solver {
    fn default() -> Solver {
        Solver {
            machine: TuringMachine::parse("input/day25.txt").unwrap(),
        }
    }
}
