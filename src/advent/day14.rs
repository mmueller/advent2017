use advent::AdventSolver;
use advent::knot::knot_hash;
use advent::union_find::UnionFind;
use failure::Error;
use std::cmp::{max,min};
use std::collections::BTreeSet;

const KEY: &str = "hfdlxzhv";

#[derive(Default)]
pub struct Solver {
    grid: Vec<Vec<bool>>
}

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        self.build_grid(KEY);
        let used_blocks: usize = self.grid.iter()
                                          .map(|row| row.iter()
                                                        .map(|&b| b as usize)
                                                        .sum::<usize>())
                                          .sum();
        println!("Total used: {}", used_blocks);

        println!("Total regions: {}", self.count_regions());
        Ok(())
    }
}

impl Solver {
    fn build_grid(&mut self, key: &str) {
        for i in 0..128 {
            let mut row: Vec<bool> = Vec::new();
            let hash = knot_hash(format!("{}-{}", key, i).as_bytes());
            for byte in hash.value().iter() {
                for bit in (0..8).rev() {
                    row.push((byte & (1 << bit)) != 0);
                }
            }
            self.grid.push(row);
        }
    }

    fn count_regions(&self) -> usize {
        // Pass 1: Naive labeling and build label equivalency set
        let mut equivalents = UnionFind::new(10000);
        let mut max_label: usize = 0;
        let mut regions: Vec<Vec<usize>> = Vec::new();
        for row in 0..128 {
            let mut region_row: Vec<usize> = Vec::new();
            for col in 0..128 {
                if self.grid[row][col] {
                    let mut new_label: usize = 0;
                    let mut left_neighbor: usize = 0;
                    let mut above_neighbor: usize = 0;
                    if col > 0 && self.grid[row][col-1] {
                        left_neighbor = region_row[col-1];
                        new_label = left_neighbor;
                    }
                    if row > 0 && self.grid[row-1][col] {
                        above_neighbor = regions[row-1][col];
                        new_label = above_neighbor;
                    }
                    if left_neighbor > 0 && above_neighbor > 0 {
                        let min_neighbor = min(left_neighbor, above_neighbor);
                        let max_neighbor = max(left_neighbor, above_neighbor);
                        if min_neighbor != max_neighbor {
                            equivalents.unite(max_neighbor, min_neighbor);
                        }
                        new_label = min_neighbor;
                    }
                    if new_label == 0 {
                        max_label += 1;
                        new_label = max_label;
                    }
                    region_row.push(new_label);
                } else {
                    region_row.push(0);
                }
            }
            regions.push(region_row);
        }

        // Pass 2: Use smallest equivalent label across grid (also track
        // distinct regions for the final result)
        let mut distinct_regions: BTreeSet<usize> = BTreeSet::new();
        for row in 0..128 {
            for col in 0..128 {
                let value: usize = regions[row][col];
                if value > 0 {
                    distinct_regions.insert(equivalents.rep(value));
                    regions[row][col] = equivalents.rep(value);
                }
            }
        }

        distinct_regions.len()
    }

    #[allow(dead_code)]
    fn dump_grid(&self) {
        for row in &self.grid {
            for &b in row {
                if b {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}
