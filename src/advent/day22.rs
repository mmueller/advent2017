use advent::AdventSolver;
use failure::Error;
use std::fs::File;
use std::io::{BufRead,BufReader};
use util::grid::{Dir,IPos};
use util::infinite_grid::InfiniteGrid;

#[derive(Default)]
pub struct Solver;

#[derive(Clone)]
enum NodeState {
    Clean,
    Infected,
    Weakened,
    Flagged,
}

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let grid = Self::read_input()?;
        Self::run_part_1(grid.clone());
        Self::run_part_2(grid.clone());
        Ok(())
    }
}

impl Solver {
    fn read_input() -> Result<InfiniteGrid<NodeState>, Error> {
        let f = BufReader::new(File::open("input/day22.txt")?);
        let lines = f.lines()
                     .map(|line| line.unwrap())
                     .collect::<Vec<String>>();
        let height = lines.len();
        let width = lines[0].len();
        let origin_row_offset = 0 - ((height / 2) as isize);
        let origin_col_offset = 0 - ((width / 2) as isize);

        let mut grid = InfiniteGrid::new(NodeState::Clean);
        for row in 0..width {
            for (col, c) in lines[row].chars().enumerate() {
                let real_row: isize = row as isize + origin_row_offset;
                let real_col: isize = col as isize + origin_col_offset;
                grid[ipos!(real_row,real_col)] = match c {
                    '.' => NodeState::Clean,
                    '#' => NodeState::Infected,
                    _   => return Err(format_err!(
                                      "parse failed: {}", lines[row]))
                }
            }
        }
        Ok(grid)
    }

    fn run_part_1(mut grid: InfiniteGrid<NodeState>) {
        let mut pos = IPos::origin();
        let mut dir = Dir::Up;
        let mut infections_caused: usize = 0;
        for _ in 0..10000 {
            match grid[pos] {
                NodeState::Clean    => {
                    dir = dir.turn_left();
                    grid[pos] = NodeState::Infected;
                    infections_caused += 1;
                },
                NodeState::Infected => {
                    dir = dir.turn_right();
                    grid[pos] = NodeState::Clean;
                },
                _ => {}
            }
            pos = pos.neighbor(dir);
        }
        println!("Infections caused after 10,000 iterations: {}",
                 infections_caused);
    }

    fn run_part_2(mut grid: InfiniteGrid<NodeState>) {
        let mut pos = ipos!(0, 0);
        let mut dir = Dir::Up;
        let mut infections_caused: usize = 0;
        for _ in 0..10_000_000 {
            match grid[pos] {
                NodeState::Clean    => {
                    dir = dir.turn_left();
                    grid[pos] = NodeState::Weakened;
                },
                NodeState::Infected => {
                    dir = dir.turn_right();
                    grid[pos] = NodeState::Flagged;
                },
                NodeState::Weakened => {
                    grid[pos] = NodeState::Infected;
                    infections_caused += 1;
                },
                NodeState::Flagged => {
                    dir = dir.reverse();
                    grid[pos] = NodeState::Clean;
                },
            }
            pos = pos.neighbor(dir);
        }
        println!("Infections caused after 10,000 iterations: {}",
                 infections_caused);
    }
}
