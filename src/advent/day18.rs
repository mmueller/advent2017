use advent::AdventSolver;
use failure::Error;
use std::collections::{HashMap,VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Default)]
pub struct Solver;

#[derive(Clone,Copy,Debug)]
enum Operand {
    Register(char),
    Value(isize)
}

#[derive(Clone,Copy,Debug)]
enum Instruction {
    Add(Operand, Operand),
    JumpGZ(Operand, Operand),
    Mod(Operand, Operand),
    Multiply(Operand, Operand),
    Receive(Operand),
    Set(Operand, Operand),
    Send(Operand),
}

struct Program<'a> {
    registers: HashMap<char, isize>,
    instructions: &'a Vec<Instruction>,
    message_queue: VecDeque<isize>,
    pc: usize,
    running: bool
}

impl<'a> Program<'a> {
    fn new(id: usize, instructions: &'a Vec<Instruction>) -> Self {
        let mut registers = HashMap::new();
        registers.insert('p', id as isize);
        Program {
            registers: registers,
            instructions: instructions,
            message_queue: VecDeque::new(),
            pc: 0,
            running: true
        }
    }

    // Execute one instruction. In the case the program sends a value ("snd"),
    // then the result is Some(value), otherwise the result is None.
    fn step(&mut self) -> Result<Option<isize>, Error> {
        let mut result: Option<isize> = None;
        let mut advance_pc: bool = true;
        if self.pc >= self.instructions.len() {
            return Ok(None);
        }
        match self.instructions[self.pc] {
            Instruction::Add(Operand::Register(r), op) => {
                let v1 = self.read_register(r);
                let v2 = self.operand_value(op);
                self.store(r, v1+v2);
            },
            Instruction::JumpGZ(op1, op2) => {
                let v1 = self.operand_value(op1);
                if v1 > 0 {
                    let v2 = self.operand_value(op2);
                    let tmp_pc = self.pc as isize + v2;
                    if tmp_pc < 0 ||
                       tmp_pc >= self.instructions.len() as isize {
                        return Err(format_err!(
                            "Jumped out of range: {}", tmp_pc));
                    }
                    self.pc = tmp_pc as usize;
                    advance_pc = false;
                }
            },
            Instruction::Mod(Operand::Register(r), op) => {
                let v1 = self.read_register(r);
                let v2 = self.operand_value(op);
                self.store(r, v1 % v2);
            },
            Instruction::Multiply(Operand::Register(r), op) => {
                let v1 = self.read_register(r);
                let v2 = self.operand_value(op);
                self.store(r, v1 * v2);
            },
            Instruction::Receive(Operand::Register(r)) => {
                match self.message_queue.pop_front() {
                    Some(v) => {
                        self.store(r, v);
                        self.running = true;
                    },
                    None => {
                        self.running = false;
                        advance_pc = false;
                    }
                }
            },
            Instruction::Set(Operand::Register(r), op) => {
                let v = self.operand_value(op);
                self.store(r, v);
            },
            Instruction::Send(op) => {
                result = Some(self.operand_value(op));
            },
            bad_instruction => {
                // It's conceivable that one of the above instructions doesn't
                // match because the first operand is supposed to be a register
                // but a value was provided instead. If that happens, the
                // instructions are bad and we should give up.
                return Err(format_err!(
                    "Bad instruction: {:?}", bad_instruction));
            }
        }
        if advance_pc {
            self.pc += 1;
            if self.pc >= self.instructions.len() {
                self.running = false;
            }
        }
        Ok(result)
    }

    fn add_to_queue(&mut self, value: isize) {
        self.message_queue.push_back(value);
    }

    // False if program terminated or is waiting for a message.
    fn is_running(&self) -> bool {
        self.running
    }

    fn read_register(&self, r: char) -> isize {
        if self.registers.contains_key(&r) {
            self.registers[&r]
        } else {
            0
        }
    }

    fn operand_value(&self, o: Operand) -> isize {
        match o {
            Operand::Register(r) => self.read_register(r),
            Operand::Value(v) => v
        }
    }

    fn store(&mut self, r: char, v: isize) {
        //println!("Setting {} to {}", r, v);
        self.registers.insert(r, v);
    }
}

impl AdventSolver for Solver {
    // Part 1 solution is uh, gone now. This solves part 2 only.
    fn solve(&mut self) -> Result<(), Error> {
        let instructions = self.parse_instructions()?;
        let mut program0 = Program::new(0, &instructions);
        let mut program1 = Program::new(1, &instructions);
        let mut values_sent_by_program1: usize = 0;

        while program0.is_running() || program1.is_running() {
            match program0.step()? {
                Some(v) => {
                    program1.add_to_queue(v);
                },
                None => {}
            }
            match program1.step()? {
                Some(v) => {
                    values_sent_by_program1 += 1;
                    program0.add_to_queue(v);
                },
                None => {}
            }
        }
        println!("Program1 sent {} values to program0.",
                 values_sent_by_program1);
        Ok(())
    }
}

impl Solver {
    fn parse_instructions(&self) -> Result<Vec<Instruction>, Error> {
        let f = BufReader::new(File::open("input/day18.txt")?);
        let mut result = Vec::new();
        for line in f.lines() {
            let line = line?;
            let tokens: Vec<&str> = line.split(" ").collect();
            let op1 = self.parse_operand(tokens[1]);
            let op2: Option<Operand> =
                if tokens.len() == 3 {
                    Some(self.parse_operand(tokens[2]))
                } else {
                    None
                };
            let instruction = match tokens[0] {
                "add" => Instruction::Add(op1, op2.unwrap()),
                "jgz" => Instruction::JumpGZ(op1, op2.unwrap()),
                "mod" => Instruction::Mod(op1, op2.unwrap()),
                "mul" => Instruction::Multiply(op1, op2.unwrap()),
                "rcv" => Instruction::Receive(op1),
                "set" => Instruction::Set(op1, op2.unwrap()),
                "snd" => Instruction::Send(op1),
                _ => return Err(format_err!(
                        "unrecognized instruction: {}", tokens[0]))
            };
            result.push(instruction);
        }
        Ok(result)
    }

    fn parse_operand(&self, token: &str) -> Operand {
        let first_char = token.chars().next().unwrap();
        if first_char.is_alphabetic() {
            Operand::Register(first_char)
        } else {
            Operand::Value(token.parse::<isize>().unwrap())
        }
    }
}
