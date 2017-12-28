use advent::AdventSolver;
use failure::Error;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead,BufReader};

#[derive(Default)]
pub struct Solver;

#[derive(Debug)]
struct Component {
    id: usize,
    port1: usize,
    port2: usize,
}

#[derive(Clone,Debug)]
struct Bridge<'a> {
    pub components: Vec<&'a Component>
}

impl<'a> Bridge<'a> {
    fn new() -> Self {
        Bridge {
            components: Vec::new()
        }
    }

    fn strength(&self) -> usize {
        self.components.iter()
                       .map(|c| c.port1 + c.port2)
                       .sum()
    }

    fn contains(&self, id: usize) -> bool {
        self.components.iter()
                       .find(|c| c.id == id)
                       .is_some()
    }
}

impl<'a> fmt::Display for Bridge<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[ ")?;
        for component in &self.components {
            write!(f, "{}/{} ", component.port1, component.port2)?;
        }
        write!(f, "]")
    }
}

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        let components = Self::read_components()?;
        let components_index = Self::index_components(&components);

        // Part 1:
        Self::solve_helper("Strongest", &components_index,
                           |b| (b.strength(), 0));
        // Part 2:
        Self::solve_helper("Longest", &components_index,
                           |b| (b.components.len(), b.strength()));
        Ok(())
    }
}

impl Solver {
    fn solve_helper(label: &str,
                    components_index: &HashMap<usize, Vec<&Component>>,
                    score: fn(&Bridge) -> (usize, usize)) {
        let best_bridge =
            Self::find_best_bridge(&components_index, 0, Bridge::new(), score);
        match best_bridge {
            Some(best_bridge) => {
                println!("{} bridge: {}", label, best_bridge);
                println!("Strength: {}", best_bridge.strength());
            },
            None => {
                println!("Failed to find any bridge.");
            }
        }
    }

    fn read_components() -> Result<Vec<Component>, Error> {
        let mut result: Vec<Component> = Vec::new();
        let f = BufReader::new(File::open("input/day24.txt")?);
        for (i, line) in f.lines().enumerate() {
            let line = line?;
            let ports = line.split("/")
                            .map(|s| s.parse::<usize>().unwrap())
                            .collect::<Vec<usize>>();
            result.push(Component {
                            id: i,
                            port1: ports[0],
                            port2: ports[1],
                        });
        }
        Ok(result)
    }

    // Build a map of port -> Vec[components that have a port with that value].
    fn index_components(components: &Vec<Component>)
            -> HashMap<usize, Vec<&Component>> {
        let mut result: HashMap<usize, Vec<&Component>> = HashMap::new();
        for component in components {
            if !result.contains_key(&component.port1) {
                result.insert(component.port1, vec![component]);
            } else {
                result.get_mut(&component.port1).unwrap().push(component);
            }
            if component.port1 != component.port2 {
                if !result.contains_key(&component.port2) {
                    result.insert(component.port2, vec![component]);
                } else {
                    result.get_mut(&component.port2).unwrap().push(component);
                }
            }
        }
        result
    }

    // Returns the best bridge possible with the given start, or None if
    // there are no components left that match the tail end of the prefix.
    fn find_best_bridge<'a>(
            components_index: &HashMap<usize, Vec<&'a Component>>,
            last_port: usize, prefix: Bridge<'a>,
            score: fn(&Bridge) -> (usize, usize))
            -> Option<Bridge<'a>> {
        let mut best_bridge = prefix.clone();
        match components_index.get(&last_port) {
            Some(components) => {
                for component in components {
                    if !prefix.contains(component.id) {
                        let mut new_prefix = prefix.clone();
                        new_prefix.components.push(component);
                        let new_last_port = if component.port1 == last_port {
                                                component.port2
                                            } else {
                                                component.port1
                                            };
                        let tmp_bridge = match Self::find_best_bridge(
                                                   components_index,
                                                   new_last_port,
                                                   new_prefix.clone(),
                                                   score) {
                            Some(b) => b,
                            None => new_prefix,
                        };
                        if score(&tmp_bridge) > score(&best_bridge) {
                            best_bridge = tmp_bridge;
                        }
                    }
                }
            },
            None => {}
        }
        Some(best_bridge)
    }
}
