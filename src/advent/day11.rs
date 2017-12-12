use advent::AdventSolver;
use failure::Error;
use std::cmp::max;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

#[derive(Default)]
pub struct Solver;

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let mut f = File::open("input/day11.txt")?;
        let mut contents = String::new();
        f.read_to_string(&mut contents)?;
        let directions: Vec<&str> = contents.trim()
                                            .split(",")
                                            .collect();
        
        let origin = HexPoint::origin();
        let mut position = origin;
        let mut furthest_distance: usize = 0;
        for dir in directions {
            let dir = Direction::from_str(dir)?;
            position = position.neighbor(dir);
            furthest_distance = max(furthest_distance,
                                    position.manhattan_distance(&origin));
        }

        println!("Ended at position:    {}", position);
        println!("Distance from origin: {}",
                 position.manhattan_distance(&origin));
        println!("Furthest distance:    {}", furthest_distance);
        Ok(())
    }
}

// Directions in a flat-topped hex space.
enum Direction {
    N,
    NE,
    SE,
    S,
    SW,
    NW
}

impl FromStr for Direction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "n"  => Ok(Direction::N),
            "ne" => Ok(Direction::NE),
            "se" => Ok(Direction::SE),
            "s"  => Ok(Direction::S),
            "sw" => Ok(Direction::SW),
            "nw" => Ok(Direction::NW),
            _    => Err(format_err!("bad direction: {}", s))
        }
    }
}

// Axial-coordinate point in hex space.
// For what it's worth, I'm choosing these axes:
//
//   +y           -z
//      ↖ _____ ↗
//       ╱     ╲
//      ╱       ╲
// -x ← ╲       ╱ →  +x
//       ╲_____╱
//      ↙       ↘
//    +z          -y
//
// z will always be calculated from x and y.
#[derive(Clone, Copy)]
struct HexPoint {
    x: isize,
    y: isize,
    z: isize
}

impl HexPoint {
    pub fn new(x: isize, y: isize) -> Self {
        HexPoint {
            x: x,
            y: y,
            z: 0 - x - y
        }
    }

    pub fn origin() -> Self {
        HexPoint {
            x: 0,
            y: 0,
            z: 0
        }
    }

    pub fn neighbor(&self, dir: Direction) -> HexPoint {
        match dir {
            Direction::N  => HexPoint::new(self.x,   self.y+1),
            Direction::NE => HexPoint::new(self.x+1, self.y  ),
            Direction::SE => HexPoint::new(self.x+1, self.y-1),
            Direction::S  => HexPoint::new(self.x,   self.y-1),
            Direction::SW => HexPoint::new(self.x-1, self.y  ),
            Direction::NW => HexPoint::new(self.x-1, self.y+1)
        }
    }

    pub fn manhattan_distance(&self, other: &HexPoint) -> usize {
        let cube_distance: isize = (self.x-other.x).abs() + 
                                   (self.y-other.y).abs() +
                                   (self.z-other.z).abs();
        (cube_distance as usize)/2
    }
}

impl fmt::Display for HexPoint {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "<HexPoint ({}, {}, {})>", self.x, self.y, self.z)
    }
}
