use advent::AdventSolver;
use failure::Error;

const GEN_A_INIT: u64 = 699;
const GEN_A_FACTOR: u64 = 16807;
const GEN_B_INIT: u64 = 124;
const GEN_B_FACTOR: u64 = 48271;

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        self.solve_part1();
        self.solve_part2();
        Ok(())
    }
}

impl Solver {
    fn solve_part1(&self) {
        let mut gen_a = Generator::new(GEN_A_INIT, GEN_A_FACTOR);
        let mut gen_b = Generator::new(GEN_B_INIT, GEN_B_FACTOR);
        let count = self.run_generators(&mut gen_a, &mut gen_b, 40_000_000);
        println!("Judge's count (part 1): {}", count);
    }

    fn solve_part2(&self) {
        let mut gen_a = Generator::new(GEN_A_INIT, GEN_A_FACTOR)
                                 .rule(|v| v % 4 == 0);
        let mut gen_b = Generator::new(GEN_B_INIT, GEN_B_FACTOR)
                                 .rule(|v| v % 8 == 0);
        let count = self.run_generators(&mut gen_a, &mut gen_b, 5_000_000);
        println!("Judge's count (part 2): {}", count);
    }

    // Run the two generators the specified number of times and return the
    // judge's output (i.e. how often the lowest 16 bits matched).
    fn run_generators(&self, gen_a: &mut Generator, gen_b: &mut Generator,
                      iterations: usize) -> usize {
        (0..iterations)
            .filter(|_| gen_a.next() & 0xffff == gen_b.next() & 0xffff)
            .map(|_| 1)
            .sum()
    }
}

struct Generator {
    factor: u64,
    last_value: u64,
    rule: fn(u64)->bool
}

impl Generator {
    pub fn new(initial_value: u64, factor: u64) -> Self {
        Generator {
            factor: factor,
            last_value: initial_value,
            rule: |_| true
        }
    }

    pub fn rule(mut self, rule: fn(u64) -> bool) -> Self {
        self.rule = rule;
        self
    }

    pub fn next(&mut self) -> u64 {
        loop {
            self.last_value = (self.last_value * self.factor) % 2147483647;
            if (self.rule)(self.last_value) {
                break;
            }
        }
        self.last_value
    }
}
