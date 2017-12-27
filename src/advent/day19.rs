use advent::AdventSolver;
use util::grid::Dir;
use failure::Error;
use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(Default)]
pub struct Solver {
    circuit: Vec<Vec<char>>,
    width: usize,
    height: usize
}

#[derive(Clone,Copy,Debug)]
struct Pos {
    row: usize,
    col: usize
}

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        self.read_circuit()?;
        let dir = Dir::Down;
        let pos = self.find_start();
        let letters = self.trace_circuit(pos, dir);
        println!("Letters found: {}", letters.iter().collect::<String>());
        Ok(())
    }
}

impl Solver {
    fn read_circuit(&mut self) -> Result<(), Error> {
        let f = BufReader::new(File::open("input/day19.txt")?);
        for line in f.lines() {
            let line = line?;
            self.circuit.push(line.chars().collect::<Vec<char>>());
        }
        self.height = self.circuit.len();
        self.width = self.circuit[0].len();
        Ok(())
    }

    fn find_start(&self) -> Pos {
        Pos {
            row: 0,
            col: self.circuit[0].iter()
                                .enumerate()
                                .find(|&(_, &c)| c == '|')
                                .unwrap().0
        }
    }

    fn value_at(&self, pos: Pos) -> char {
        self.circuit[pos.row][pos.col]
    }

    // Naive: what is the postion adjacent to `pos` in direction `dir`.
    // Does not consider turning. Returns Some(pos) if neighbor is inside
    // the circuit bounds, or None if it would go out of bounds.
    fn neighbor(&self, pos: Pos, dir: Dir) -> Option<Pos> {
        match dir {
            Dir::Up    if pos.row > 0 => {
                Some(Pos { row: pos.row-1, col: pos.col })
            },
            Dir::Right if pos.col < self.width-1 => {
                Some(Pos { row: pos.row, col: pos.col+1 })
            },
            Dir::Down  if pos.row < self.height-1 => {
                Some(Pos { row: pos.row+1, col: pos.col })
            },
            Dir::Left  if pos.col > 0 => {
                Some(Pos { row: pos.row, col: pos.col-1 })
            },
            _ => { None }
        }
    }

    // Determine the next position along the circuit trace, which might
    // involve changing directions. As with neighbor, returns Some(pos, dir)
    // if the next position is in bounds, otherwise None.
    fn next_pos(&self, pos: Pos, dir: Dir) -> Option<(Pos, Dir)> {
        let mut dir = dir;
        if self.value_at(pos) == '+' {
            let right = dir.turn_right();
            if let Some(n) = self.neighbor(pos, right) {
                if self.value_at(n) != ' ' {
                    dir = right
                }
            }
            let left = dir.turn_left();
            if let Some(n) = self.neighbor(pos, left) {
                if self.value_at(n) != ' ' {
                    dir = left;
                }
            }
        }
        // The next position is ' ' when we have reached the end of the trace.
        if self.value_at(self.neighbor(pos, dir).unwrap()) == ' ' {
            None
        } else {
            Some((self.neighbor(pos, dir).unwrap(), dir))
        }
    }

    // Trace the path of the circuit starting at position `pos` and heading
    // in direction `dir`.
    fn trace_circuit(&self, pos: Pos, dir: Dir) -> Vec<char> {
        let mut result: Vec<char> = Vec::new();
        let mut pos = pos;
        let mut dir = dir;
        let mut trace_length = 1;
        loop {
            let v = self.value_at(pos);
            if v.is_alphabetic() {
                result.push(v);
            }
            match self.next_pos(pos, dir) {
                Some((new_pos, new_dir)) => {
                    trace_length += 1;
                    pos = new_pos;
                    dir = new_dir;
                },
                None => break
            }
        }
        println!("Trace length: {}", trace_length);
        result
    }
}
