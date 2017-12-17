use advent::AdventSolver;
use failure::Error;
use std::fs::File;
use std::io::Read;
use std::str;
use nom;

#[derive(Default)]
pub struct Solver {
    programs: Vec<String>,
    count: usize,
    offset: usize
}


#[derive(Debug)]
enum DanceMove<'a> {
    Spin(usize),
    Exchange(usize, usize),
    Partner(&'a str, &'a str)
}

named!(parse_dance_moves<&[u8], Vec<DanceMove>>,
    separated_list_complete!(char!(','),
    alt!(
        do_parse!(
            tag!("s") >>
            amount: map_res!(map_res!(nom::digit, str::from_utf8),
                             str::FromStr::from_str) >>
            (DanceMove::Spin(amount))
        ) |
        do_parse!(
            tag!("x") >>
            pos1: map_res!(map_res!(nom::digit, str::from_utf8),
                           str::FromStr::from_str) >>
            tag!("/") >>
            pos2: map_res!(map_res!(nom::digit, str::from_utf8),
                           str::FromStr::from_str) >>
            (DanceMove::Exchange(pos1, pos2))
        ) |
        do_parse!(
            tag!("p") >>
            program1: map_res!(nom::alpha, str::from_utf8) >>
            tag!("/") >>
            program2: map_res!(nom::alpha, str::from_utf8) >>
            (DanceMove::Partner(program1, program2))
        )
    ))
);


impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let mut contents: Vec<u8> = Vec::new();
        File::open("input/day16.txt")?
             .read_to_end(&mut contents)?;
        let dance_moves = parse_dance_moves(&contents).unwrap().1;

        self.programs = "abcdefghijklmnop".chars()
                                          .map(|c| c.to_string())
                                          .collect();
        let original_programs = self.programs.clone();
        self.count = self.programs.len();

        self.dance(&dance_moves);
        print!("After dancing once: {}", self.to_string());

        let mut cycle_length: Option<usize> = None;
        println!("Dancing up to 1 billion times...");
        for i in 1..1_000_000_000 {
            if self.programs == original_programs && self.offset == 0 {
                cycle_length = Some(i);
                break;
            }
            self.dance(&dance_moves);
        }

        match cycle_length {
            Some(c) => {
                println!("Found cycle: {}", c);
                let iters = 1_000_000_000 % c;
                println!("Only need {} iterations.", iters);
                // We know that self.programs is back in its original state.
                for _ in 0..iters {
                    self.dance(&dance_moves);
                }
                println!("After dancing {} times: {}", iters, self.to_string());
            },
            None => { 
                println!("Wow, actually danced a billion times: {}",
                         self.to_string());
            }
        }

        Ok(())
    }
}

impl Solver {
    fn dance(&mut self, dance_moves: &Vec<DanceMove>) {
        for dance_move in dance_moves {
            match dance_move {
                &DanceMove::Spin(amount) => {
                    self.offset = (self.offset+self.count-amount) % self.count;
                },
                &DanceMove::Exchange(pos1, pos2)  => {
                    let index1 = self.pos_to_index(pos1);
                    let index2 = self.pos_to_index(pos2);
                    self.programs.swap(index1, index2);
                },
                &DanceMove::Partner(p1, p2) => {
                    let index1 = self.find_program_index(p1);
                    let index2 = self.find_program_index(p2);
                    self.programs.swap(index1, index2);
                }
            }
        }
    }

    fn find_program_index(&self, name: &str) -> usize {
        self.programs.iter().enumerate().find(|&(_, p)| p == name).unwrap().0
    }

    fn pos_to_index(&self, pos: usize) -> usize {
        (self.offset + pos) % self.count
    }

    fn to_string(&self) -> String {
        let mut result: String = String::new();
        for i in self.offset..self.count {
            result.push_str(&self.programs[i]);
        }
        for i in 0..self.offset {
            result.push_str(&self.programs[i]);
        }
        result
    }
}
