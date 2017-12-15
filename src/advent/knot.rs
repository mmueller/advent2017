use std::num::Wrapping;

const KNOT_HASH_SIZE: usize = 256;
const KNOT_HASH_ROUNDS: usize = 64;
const KNOT_HASH_SUFFIX: [u8; 5] = [17, 31, 73, 47, 23];

pub struct KnotHash {
    current_position: Wrapping<u8>,
    skip_size: Wrapping<u8>,
    vec: Vec<u8>
}

// Performs the 64-round hash using the lower-level functions in KnotHash.
pub fn knot_hash(input: &[u8]) -> KnotHash {
    let mut hash = KnotHash::new();
    for _ in 0..KNOT_HASH_ROUNDS {
        for &byte in input {
            hash.update(byte);
        }
        for &byte in KNOT_HASH_SUFFIX.iter() {
            hash.update(byte);
        }
    }
    hash
}

impl KnotHash {
    pub fn new() -> KnotHash {
        let mut vec: Vec<u8> = Vec::with_capacity(KNOT_HASH_SIZE);
        for i in 0..KNOT_HASH_SIZE {
            vec.push(i as u8);
        }
        KnotHash {
            current_position: Wrapping(0),
            skip_size: Wrapping(0),
            vec: vec
        }
    }

    pub fn update(&mut self, input: u8) {
        // This Wrapping business is clunky. 😕
        let input = Wrapping(input);
        let mut start: Wrapping<u8> = self.current_position;
        let mut end: Wrapping<u8> = start + input - Wrapping(1);
        while start != end && start - Wrapping(1) != end {
            self.vec.swap(start.0 as usize, end.0 as usize);
            start += Wrapping(1);
            end -= Wrapping(1);
        }
        self.current_position += input + self.skip_size;
        self.skip_size += Wrapping(1);
    }

    pub fn value(&self) -> [u8; 16] {
        let mut result = [0; 16];
        for i in 0..16 {
            let mut value: u8 = 0;
            for j in i*16..(i+1)*16 {
                value ^= self.vec[j];
            }
            result[i] = value;
        }
        result
    }

    // Requested in day 10.1
	pub fn product_of_first_two_bytes(&self) -> usize {
		self.vec[0] as usize * self.vec[1] as usize
	}
}

