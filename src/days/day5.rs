use std::fs::File;
use std::io::Read;

pub fn solve() {
    let mut f = File::open("input/day5.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
     .expect("unable to read input file");

    let instructions: Vec<isize> =
        contents.split("\n")
                .filter(|s| !s.is_empty())
                .map(|s| s.parse::<isize>().unwrap())
                .collect();

    let steps1 = execute(instructions.clone(),
                         |i| i + 1);
    println!("Steps to escape (pt 1): {}", steps1);

    let steps2 = execute(instructions.clone(),
                         |i| if i >= 3 { i - 1 } else { i + 1 });
    println!("Steps to escape (pt 2): {}", steps2);
}

// Execute the instructions (using the given instruction-modifying rule)
// and return the number of steps required to escape.
fn execute(mut instructions: Vec<isize>, rule: fn(isize) -> isize) -> usize{
    // Using isize for the program counter just in case a jump takes it into
    // negative range (which would also qualify as "outside" the list).
    let mut pc: isize = 0;
    let mut steps_taken: usize = 0;
    while pc >= 0 && (pc as usize) < instructions.len() {
        let jmp = instructions[pc as usize];
        instructions[pc as usize] = rule(jmp);
        pc += jmp;
        steps_taken += 1;
    }
    steps_taken
}
