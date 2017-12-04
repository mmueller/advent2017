use std::fs::File;
use std::io::Read;

pub fn solve() {
    let mut f = File::open("input/day1.txt").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents)
     .expect("unable to read input file");
    let captcha = contents.trim();
    println!("CAPTCHA solution 1: {}", solve_captcha(captcha, 1));
    println!("CAPTCHA solution 2: {}", solve_captcha(captcha, captcha.len()/2));
}

fn string_to_digits(string: &str) -> Vec<u8> {
    string.chars()
          .map(|c| c.to_string().parse::<u8>().unwrap())
          .collect()
}

fn solve_captcha(captcha: &str, offset: usize) -> usize {
    let mut result: usize = 0;
    let digits = string_to_digits(captcha);
    for i in 0..digits.len() {
        if digits[i] == digits[(i+offset)%digits.len()] {
            result += digits[i] as usize;
        }
    }
    result
}
