use advent::AdventSolver;
use failure::Error;
use regex::Regex;
use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(Default)]
pub struct Solver {
    particles: Vec<Particle>
}

#[derive(Debug)]
struct Particle {
    pos: [isize; 3],
    vel: [isize; 3],
    acc: [isize; 3],
}

impl Particle {
    fn acc_magnitude(&self) -> usize {
        (0..3).map(|dim| self.acc[dim].abs() as usize)
              .sum()
    }
}

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        self.read_particles()?;
        let min_acc = self.particles.iter()
                                    .enumerate()
                                    .min_by_key(|&(_, ref p)| p.acc_magnitude())
                                    .unwrap().0;
        println!("Particle {} has min acceleration:\n{:?}",
                 min_acc, self.particles[min_acc]);

        let mut num_particles = self.particles.len();
        for i in 1..10_000 {
            self.tick();
            if self.particles.len() != num_particles {
                num_particles = self.particles.len();
                println!("{}: Number of particles is now: {}",
                         i, num_particles);
            }
        }

        Ok(())
    }
}

impl Solver {
    fn read_particles(&mut self) -> Result<(), Error> {
        let f = BufReader::new(File::open("input/day20.txt")?);
        let re = Regex::new(r"(?x)
            p=<(?P<px>-?\d+),(?P<py>-?\d+),(?P<pz>-?\d+)>,\s
            v=<(?P<vx>-?\d+),(?P<vy>-?\d+),(?P<vz>-?\d+)>,\s
            a=<(?P<ax>-?\d+),(?P<ay>-?\d+),(?P<az>-?\d+)>
        ")?;
        for line in f.lines() {
            let line = line?;
            let caps = match re.captures(&line) {
                Some(caps) => caps,
                None => return Err(format_err!("parse failed: {}", line))
            };
            self.particles.push(
                Particle {
                    pos: [caps["px"].parse::<isize>()?,
                          caps["py"].parse::<isize>()?,
                          caps["pz"].parse::<isize>()?],
                    vel: [caps["vx"].parse::<isize>()?,
                          caps["vy"].parse::<isize>()?,
                          caps["vz"].parse::<isize>()?],
                    acc: [caps["ax"].parse::<isize>()?,
                          caps["ay"].parse::<isize>()?,
                          caps["az"].parse::<isize>()?],
                }
            );
        }
        Ok(())
    }

    fn tick(&mut self) {
        for p in self.particles.iter_mut() {
            for dim in 0..3 {
                p.vel[dim] += p.acc[dim];
                p.pos[dim] += p.vel[dim];
            }
        }
        self.collision_detect();
    }

    fn collision_detect(&mut self) {
        let num_particles = self.particles.len();
        let mut dead_positions: Vec<[isize; 3]> = Vec::new();
        for i in 0..num_particles {
            let particle_i = &self.particles[i];
            for j in i+1..num_particles {
                if particle_i.pos == self.particles[j].pos {
                    dead_positions.push(particle_i.pos);
                    break;
                }
            }
        }
        self.particles.retain(|ref p| !dead_positions.contains(&p.pos));
    }
}
