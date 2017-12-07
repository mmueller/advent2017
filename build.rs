use std::fs::{File,read_dir};
use std::io::Write;
use std::path::Path;

fn main() {
    let advent_path = Path::new("src/advent/");
    let dest_path = Path::new("src/advent/_generated_advent_mod.rs");
    let mut f = File::create(&dest_path).unwrap();

    let mut days: Vec<String> = Vec::new();
    for entry in read_dir(advent_path).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        match path.file_stem() {
            Some(filename) => {
                let filename = filename.to_str().unwrap();
                if filename.starts_with("day") {
                    days.push(filename.to_string());
                }
            },
            None => { }
        }
    }

    f.write(br###"
// GENERATED BY build.rs
// DO NOT EDIT BY HAND, CHANGES WILL BE LOST.
"###).unwrap();
    for module in &days {
        write!(f, "pub mod {};\n", module).unwrap();
    }

    f.write(br###"
// Execute the solution for a particular day (1..25).
pub fn solve(day: usize) -> Result<(), Error> {
    let mut solver: Box<AdventSolver> = match day {
"###).unwrap();
    for module in &days {
        let day_num = module[3..].parse::<usize>().unwrap();
        write!(f, "         {} => Box::new({}::Solver::default()),\n",
               day_num, module).unwrap();
    }
    f.write(br###"
        _ => { return Err(format_err!("Invalid day specified: {}", day)); }
    };
    solver.solve()
}
    "###).unwrap();
}
