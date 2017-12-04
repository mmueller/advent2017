use std::fs::File;
use std::io::Read;

pub fn solve() {
    let mut f = File::open("input/day2.txt").expect("file not found");          
    let mut contents = String::new();                                           
    f.read_to_string(&mut contents)                                             
     .expect("unable to read input file");
    let mut result : u32 = 0;
    let mut result2 : u32 = 0;
    for line in contents.split("\n") {
        let values: Vec<u32> = line.split("\t")
                                   .filter(|v| !v.is_empty())
                                   .map(|v| v.parse::<u32>().unwrap())
                                   .collect();
        if !values.is_empty() {
            // Max-min version:
            result += values.iter().max().unwrap() -
                      values.iter().min().unwrap();
            // Divisibility version (can we do better than O(n^2)?):
            for i in 0..values.len() {
                let mut found: bool = false;
                for j in 0..values.len() {
                    if i == j {
                        continue;
                    }
                    if values[i] % values[j] == 0 {
                        result2 += values[i] / values[j];
                        found = true;
                        break;
                    }
                }
                if found {
                    break;
                }
            }
        }
    }
    println!("checksum: {}", result);
    println!("checksum2: {}", result2);
}
