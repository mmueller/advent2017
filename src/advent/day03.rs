use advent::AdventSolver;
use failure::Error;

const TARGET: usize = 347991;

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        // Part 1: Simple spiral of 1, 2, 3...
        let mut spiral1 = Spiral::new();
        let mut pos: (isize, isize) = (0, 0);
        for value in 1..TARGET+1 {
            if value == TARGET {
                break;
            }
            pos = spiral1.append(value);
        }
        println!("{} occurs at ({}, {}), dist: {}",
                 TARGET, pos.0, pos.1, pos.0.abs()+pos.1.abs());

        // Part 2: Spiral using sum_of_neighbors at each position.
        let mut spiral2 = Spiral::new();
        let mut value: usize = 1;
        let mut pos: (isize, isize) = spiral2.append(1);
        while value <= TARGET {
            value = spiral2.sum_of_neighbors(pos);
            pos = spiral2.append(value);
        }
        println!("First value greater than target: {}", value);
        Ok(())
    }
}

struct Spiral {
    bounds: (isize, isize),
    cur_pos: (isize, isize),
    rows: Vec<Vec<usize>>
}

impl Spiral {
    pub fn new() -> Self {
        let mut rows = Vec::new();
        rows.push(vec![0]);

        Spiral {
            bounds: (0, 0),
            cur_pos: (0, 0),
            rows: rows
        }
    }

    // Add a new value onto the spiral. Returns the position at which the value
    // was added.
    pub fn append(&mut self, value: usize) -> (isize, isize) {
        // Write the value at the current position.
        let pos = self.cur_pos;
        self.set(pos, value);

        // Advance current position and grow if needed.
        self.cur_pos = self.next_pos(self.cur_pos);
        if self.cur_pos.0 > self.bounds.0 {
            let new_bound = self.cur_pos.0;
            self.grow(new_bound);
        }
        self.cur_pos
    }

    // Access a value at a position. Out of bounds will return None.
    pub fn get(&self, pos: (isize, isize)) -> Option<usize> {
        let index = (Self::pos_to_index(pos.0), Self::pos_to_index(pos.1));
        if index.1 < self.rows.len() {
            let row = &self.rows[index.1];
            if index.0 < row.len() {
                return Some(row[index.0]);
            }
        }
        None
    }

    // Write a value at a position. Doesn't grow spiral to fit.
    pub fn set(&mut self, pos: (isize, isize), value: usize) {
        let index = (Self::pos_to_index(pos.0), Self::pos_to_index(pos.1));
        self.rows[index.1][index.0] = value;
    }

    // Increase the spiral size so that its bounded at -bound..bound in both
    // dimensions. E.g. spirals start with bound == 0 (1x1), then grow to
    // bound=1 (3x3), bound=2 (5x5).
    fn grow(&mut self, bound: isize) {
        self.bounds = (bound, bound);
        let size: usize = (bound.abs() as usize) * 2 + 1;
        for row in &mut self.rows {
            if row.len() < size {
                row.resize(size, 0);
            }
        }
        while self.rows.len() < size {
            self.rows.push(vec![0; size]);
        }
    }

    fn next_pos(&self, pos: (isize, isize)) -> (isize, isize) {
        // Bottom edge
        if pos.1 == self.bounds.1 {
            // May exceed self.bounds, caller should grow if necessary
            (pos.0+1, pos.1)
        // Right edge
        } else if pos.0 == self.bounds.0 {
            if pos.1 > -self.bounds.1 {
                (pos.0, pos.1-1)
            } else {
                (pos.0-1, pos.1)
            }
        // Top edge
        } else if pos.1 == -self.bounds.1 {
            if pos.0 > -self.bounds.0 {
                (pos.0-1, pos.1)
            } else {
                (pos.0, pos.1+1)
            }
        // Left edge
        } else /* pos.0 == -self.bounds.0 */ {
            if pos.1 < self.bounds.1 {
                (pos.0, pos.1+1)
            } else {
                (pos.0+1, pos.1)
            }
        }
    }

    // Implement an indexing scheme that allows us to map negative & positive
    // positions into indexes in the range 0..∞. (Since the data type I want to
    // use (Vec) starts indexing at 0.)
    fn pos_to_index(pos: isize) -> usize {
        let result = if pos < 0 {
                         (-pos*2) - 1
                     } else {
                         pos * 2
                     };
        result as usize
    }

    // I never needed the inverse, but for reference it would look like this:
    #[allow(dead_code)]
    fn index_to_pos(index: usize) -> isize {
        let sindex = index as isize;
        if sindex % 2 == 1 {
            -(sindex+1)/2
        } else {
            sindex/2
        }
    }

    fn sum_of_neighbors(&self, pos: (isize, isize)) -> usize {
        let offsets: [(isize, isize); 8] = [
            (-1, -1), ( 0, -1), ( 1, -1),
            (-1,  0),           ( 1,  0),
            (-1,  1), ( 0,  1), ( 1,  1)
        ];
        let mut result: usize = 0;
        for offset in offsets.into_iter() {
            let neighbor_pos = (pos.0 + offset.0, pos.1 + offset.1);
            match self.get(neighbor_pos) {
                Some(value) => result += value,
                None => {}
            }
        }
        result
    }
}
