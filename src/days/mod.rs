// Nothing interesting in here except for the repetitive module declarations
// and mapping via the solve() method.  If only Rust had the metaprogramming
// power of Ruby...

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

// Execute the solution for a particular day (1..25).
pub fn solve(day: usize) {
    match day {
        1 => day1::solve(),
        2 => day2::solve(),
        3 => day3::solve(),
        4 => day4::solve(),
        5 => day5::solve(),
        6 => day6::solve(),
        _ => println!("Unsupported day: {}", day),
    }
}
