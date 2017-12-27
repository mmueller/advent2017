use failure::Error;
use std::collections::{HashMap,VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone,Copy,Debug)]
pub enum Operand {
    Register(char),
    Value(isize)
}

#[derive(Clone,Copy,Debug)]
pub enum Instruction {
    Add(Operand, Operand),
    JumpGZ(Operand, Operand),
    JumpNZ(Operand, Operand),
    Mod(Operand, Operand),
    Multiply(Operand, Operand),
    Receive(Operand),
    Set(Operand, Operand),
    Sub(Operand, Operand),
    Send(Operand),
}

pub struct Program<'a> {
    registers: HashMap<char, isize>,
    instructions: &'a Vec<Instruction>,
    message_queue: VecDeque<isize>,
    pc: usize,
    running: bool
}

impl<'a> Program<'a> {
    pub fn new(id: usize, instructions: &'a Vec<Instruction>) -> Self {
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

    pub fn next_instruction(&self) -> Option<Instruction> {
        if self.pc >= self.instructions.len() {
            None
        } else {
            Some(self.instructions[self.pc])
        }
    }

    // Execute one instruction. In the case the program sends a value ("snd"),
    // then the result is Some(value), otherwise the result is None.
    pub fn step(&mut self) -> Result<Option<isize>, Error> {
        let mut result: Option<isize> = None;
        let mut advance_pc: bool = true;
        let inst = self.next_instruction();
        if inst.is_none() {
            return Ok(None);
        }
        match inst.unwrap() {
            Instruction::Add(Operand::Register(r), op) => {
                let v1 = self.read_register(r);
                let v2 = self.operand_value(op);
                self.store(r, v1+v2);
            },
            Instruction::JumpGZ(op1, op2) => {
                let v1 = self.operand_value(op1);
                if v1 > 0 {
                    let v2 = self.operand_value(op2);
                    self.jump(v2)?;
                    advance_pc = false;
                }
            },
            Instruction::JumpNZ(op1, op2) => {
                let v1 = self.operand_value(op1);
                if v1 != 0 {
                    let v2 = self.operand_value(op2);
                    self.jump(v2)?;
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
            Instruction::Sub(Operand::Register(r), op) => {
                let v1 = self.read_register(r);
                let v2 = self.operand_value(op);
                self.store(r, v1-v2);
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
        }
        if self.pc >= self.instructions.len() {
            self.running = false;
        }
        Ok(result)
    }

    pub fn add_to_queue(&mut self, value: isize) {
        self.message_queue.push_back(value);
    }

    // False if program terminated or is waiting for a message.
    pub fn is_running(&self) -> bool {
        self.running
    }

    pub fn read_register(&self, r: char) -> isize {
        if self.registers.contains_key(&r) {
            self.registers[&r]
        } else {
            0
        }
    }

    pub fn operand_value(&self, o: Operand) -> isize {
        match o {
            Operand::Register(r) => self.read_register(r),
            Operand::Value(v) => v
        }
    }

    pub fn store(&mut self, r: char, v: isize) {
        //println!("Setting {} to {}", r, v);
        self.registers.insert(r, v);
    }

    pub fn jump(&mut self, offset: isize) -> Result<(), Error> {
        let tmp_pc = self.pc as isize + offset;
        if tmp_pc < 0 || tmp_pc > self.instructions.len() as isize {
            // Treats jumping just past end as ok, everything else is an error.
            return Err(format_err!("Jumped out of range: {}", tmp_pc));
        }
        self.pc = tmp_pc as usize;
        Ok(())
    }
}

pub fn parse_instructions(file: &str) -> Result<Vec<Instruction>, Error> {
    let f = BufReader::new(File::open(file)?);
    let mut result = Vec::new();
    for line in f.lines() {
        let line = line?;
        let tokens: Vec<&str> = line.split(" ").collect();
        let op1 = parse_operand(tokens[1]);
        let op2: Option<Operand> =
            if tokens.len() == 3 {
                Some(parse_operand(tokens[2]))
            } else {
                None
            };
        let instruction = match tokens[0] {
            "add" => Instruction::Add(op1, op2.unwrap()),
            "jgz" => Instruction::JumpGZ(op1, op2.unwrap()),
            "jnz" => Instruction::JumpNZ(op1, op2.unwrap()),
            "mod" => Instruction::Mod(op1, op2.unwrap()),
            "mul" => Instruction::Multiply(op1, op2.unwrap()),
            "rcv" => Instruction::Receive(op1),
            "set" => Instruction::Set(op1, op2.unwrap()),
            "snd" => Instruction::Send(op1),
            "sub" => Instruction::Sub(op1, op2.unwrap()),
            _     => return Err(format_err!(
                        "unrecognized instruction: {}", tokens[0]))
        };
        result.push(instruction);
    }
    Ok(result)
}

fn parse_operand(token: &str) -> Operand {
    let first_char = token.chars().next().unwrap();
    if first_char.is_alphabetic() {
        Operand::Register(first_char)
    } else {
        Operand::Value(token.parse::<isize>().unwrap())
    }
}
