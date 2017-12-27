use advent::AdventSolver;
use failure::Error;
use util::duet::{self,Instruction,Program};

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let instructions = duet::parse_instructions("input/day23_orig.txt")?;
        Self::run_part_1(&instructions);
        // The program I was given tries to count the number of non-primes
        // between 105700 and 122700 (with step 17, so 1001 iterations).
        // Second version is hand-optimized by me to use `mod` instruction when
        // testing primality, instead of the O(n^2) inner loop in the original.
        let instructions = duet::parse_instructions("input/day23_opt.txt")?;
        Self::run_part_2(&instructions);
        Ok(())
    }
}

impl Solver {
    fn run_part_1(instructions: &Vec<Instruction>) {
        let mut program = Program::new(0, &instructions);
        let mut multiply_calls: usize = 0;

        while program.is_running() {
            match program.next_instruction() {
                Some(Instruction::Multiply(_, _)) => multiply_calls += 1,
                _ => {}
            }
            match program.step() {
                Ok(_) => {},
                Err(e) => {
                    println!("Program exited with error: {}", e);
                    break;
                }
            }
        }

        println!("{} multiply calls.", multiply_calls);
    }

    fn run_part_2(instructions: &Vec<Instruction>) {
        let mut program = Program::new(0, &instructions);
        program.store('a', 1);

        while program.is_running() {
            match program.step() {
                Ok(_) => {},
                Err(e) => {
                    println!("Program exited with error: {}", e);
                    break;
                }
            }
        }

        println!("Program terminated with register h: {}",
                 program.read_register('h'));
    }
}

