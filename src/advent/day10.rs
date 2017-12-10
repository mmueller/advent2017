use advent::AdventSolver;
use failure::Error;
use std::fs::File;
use std::io::Read;
use std::num::Wrapping;

#[derive(Default)]
pub struct Solver;

const KNOT_HASH_SIZE: usize = 256;

const SUFFIX: [u8; 5] = [17, 31, 73, 47, 23];

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let mut f = File::open("input/day10.txt")?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        let contents = contents.trim();
        self.solve_part1(contents)?;
        self.solve_part2(contents)?;
        Ok(())
    }
}

impl Solver {
    fn solve_part1(&self, contents: &str) -> Result<(), Error> {
        // Interpret contents as comma-separated integer lengths
        let lengths = contents.trim()
                              .split(",")
                              .map(|s| s.parse::<u8>().unwrap())
                              .collect::<Vec<u8>>();
        let mut hash = KnotHash::new();
        for length in lengths {
            hash.update(length);
        }

        println!("First two numbers: {} x {} = {}",
                 hash.vec[0], hash.vec[1],
                 hash.vec[0] as usize * hash.vec[1] as usize);
        Ok(())
    }

    fn solve_part2(&self, contents: &str) -> Result<(), Error> {
        // Interpret contents as bytes, run 64 rounds
        let mut hash = KnotHash::new();
        for _ in 0..64 {
            for byte in contents.bytes() {
                hash.update(byte);
            }
            // For some reason we're asked to append these values, too.
            for value in SUFFIX.iter() {
                hash.update(*value);
            }
        }

        for byte in hash.value().iter() {
            print!("{:02x}", *byte);
        }
        println!("");

        Ok(())
    }
}

struct KnotHash {
    current_position: Wrapping<u8>,
    skip_size: Wrapping<u8>,
    vec: Vec<u8>
}

impl KnotHash {
    pub fn new() -> KnotHash {
        let mut vec: Vec<u8> = Vec::with_capacity(KNOT_HASH_SIZE);
        for i in 0..KNOT_HASH_SIZE {
            vec.push(i as u8);
        }
        KnotHash {
            current_position: Wrapping(0),
            skip_size: Wrapping(0),
            vec: vec
        }
    }

    pub fn update(&mut self, input: u8) {
        // This Wrapping business is clunky. 😕
        let input = Wrapping(input);
        let mut start: Wrapping<u8> = self.current_position;
        let mut end: Wrapping<u8> = start + input - Wrapping(1);
        while start != end && start - Wrapping(1) != end {
            self.vec.swap(start.0 as usize, end.0 as usize);
            start += Wrapping(1);
            end -= Wrapping(1);
        }
        self.current_position += input + self.skip_size;
        self.skip_size += Wrapping(1);
    }

    pub fn value(&self) -> [u8; 16] {
        let mut result = [0; 16];
        for i in 0..16 {
            let mut value: u8 = 0;
            for j in i*16..(i+1)*16 {
                value ^= self.vec[j];
            }
            result[i] = value;
        }
        result
    }
}
