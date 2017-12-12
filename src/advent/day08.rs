use advent::AdventSolver;
use failure::Error;
use regex::Regex;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead,BufReader};

lazy_static! {                                                                  
    static ref INSTRUCTION_RE: Regex =
        Regex::new(r"(\w+) (\w+) (-?\d+) if (\w+) ([<>=!]+) (-?\d+)").unwrap();
}

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let file = BufReader::new(File::open("input/day08.txt")?);
        let mut registers: HashMap<String, isize> = HashMap::new();
        let mut largest_value_ever: isize = 0;
        for line in file.lines() {
            let line = line?;
            let inst = Instruction::parse(&line)?;
            println!("{}", inst);
            
            // Check condition
            let actual_value: isize = match registers.get(&inst.cond.reg) {
                                   Some(value) => *value,
                                   None => 0
                               };
            let cond_result =
                match inst.cond.comp {
                    Comparator::EQ => actual_value == inst.cond.value,
                    Comparator::LE => actual_value <= inst.cond.value,
                    Comparator::LT => actual_value <  inst.cond.value,
                    Comparator::GE => actual_value >= inst.cond.value,
                    Comparator::GT => actual_value >  inst.cond.value,
                    Comparator::NE => actual_value != inst.cond.value,
                };
            if cond_result {
                // Execute command
                let original_value: isize =
                    match registers.get(&inst.reg) {
                        Some(value) => *value,
                        None => 0
                    };
                let new_value: isize =
                    match inst.op {
                        Operation::Dec(amount) => original_value - amount,
                        Operation::Inc(amount) => original_value + amount
                    };
                registers.insert(inst.reg, new_value);

                if new_value > largest_value_ever {
                    largest_value_ever = new_value;
                }

            }
        }

        // Find largest register
        match registers.iter().max_by_key(|i| i.1) {
            Some(pair) => println!("largest register value at end: {} {}",
                                   pair.0, pair.1),
            None => println!("registers empty?")
        }
        println!("largest register value during run: {}", largest_value_ever);
        Ok(())
    }
}

enum Operation {
    Inc(isize),
    Dec(isize),
}

enum Comparator {
    EQ,
    LE,
    LT,
    GE,
    GT,
    NE
}

struct Condition {
    reg: String,
    comp: Comparator,
    value: isize
}

struct Instruction {
    reg: String,
    op: Operation,
    cond: Condition
}

impl Instruction {
    pub fn parse(text: &str) -> Result<Instruction, Error> {
        let caps = match INSTRUCTION_RE.captures(text) {
                       Some(caps) => caps,
                       None => return Err(format_err!(
                                   "Couldn't parse instruction {}", text))
                   };
        let amount = caps[3].parse::<isize>()?;
        let op = match &caps[2] {
            "dec" => Operation::Dec(amount),
            "inc" => Operation::Inc(amount),
            _ => return Err(format_err!("Unrecognized instruction: {}", text))
        };

        let cond_reg = caps[4].to_string();
        let comp = match &caps[5] {
            "==" => Comparator::EQ,
            "<=" => Comparator::LE,
            "<"  => Comparator::LT,
            ">=" => Comparator::GE,
            ">"  => Comparator::GT,
            "!=" => Comparator::NE,
            _    => return Err(format_err!("Unrecognized comparator: {}", text))
        };
        let value = caps[6].parse::<isize>()?;

        Ok(Instruction {
            reg: caps[1].to_string(),
            op: op,
            cond: Condition {
                reg: cond_reg,
                comp: comp,
                value: value
            }
        })
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<Instruction: {} {} if {} {} {}>",
               self.reg, self.op,
               self.cond.reg, self.cond.comp, self.cond.value)
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Operation::Dec(amt) => write!(f, "dec {}", amt),
            Operation::Inc(amt) => write!(f, "inc {}", amt)
        }
    }
}

impl fmt::Display for Comparator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
                            Comparator::EQ => "==",
                            Comparator::LE => "<=",
                            Comparator::LT => "<",
                            Comparator::GE => ">=",
                            Comparator::GT => ">",
                            Comparator::NE => "!=",
                        })
    }
}
