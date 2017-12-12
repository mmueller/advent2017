use advent::AdventSolver;
use failure::Error;
use std::cell::RefCell;
use std::collections::{HashMap,HashSet};
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::rc::Rc;
use regex::Regex;

#[derive(Default)]
pub struct Solver {
    all_nodes: Rc<RefCell<HashMap<String, TowerNode>>>
}

impl AdventSolver for Solver {
    fn solve(&mut self) -> Result<(), Error> {
        self.read_input()?;
        let root_node_id = self.find_root_id();
        println!("Root node: {}", root_node_id);
        //self.all_nodes.borrow()[&root_node_id].dump_tree();
        self.find_wrong_weight(&root_node_id);
        Ok(())
    }
}

// I'm horrified.

impl Solver {
    fn read_input(&self) -> Result<(), Error> {
        let re = Regex::new(r"(?x)
                     ^(?P<id>\w+)
                     \s
                     \((?P<weight>\d+)\)
                     (?:
                       \s
                       ->
                       \s
                       (?P<targets>[\w\s, ]+)
                     )?
                     $")?;
        let file = BufReader::new(File::open("input/day7.txt")?);
        for line in file.lines() {
            let line = line?;
            let caps = match re.captures(&line) {
                           Some(caps) => caps,
                           None => return Err(format_err!(
                                              "Didn't match regex: {}", line))
                       };
            let id = &caps["id"];
            let weight = caps["weight"].parse::<usize>()?;
            let targets: Vec<&str> = match caps.name("targets") {
                                         Some(m) => m.as_str().split(", ")
                                                     .collect(),
                                         None => Vec::new()
                                     };
            let mut node: TowerNode =
                TowerNode::new(id, weight, self.all_nodes.clone());
            for target in &targets {
                node.add_child_node(target);
            }
            self.all_nodes.borrow_mut().insert(id.to_string(), node);
        }
        Ok(())
    }

    fn find_root_id(&self) -> String {
        let mut possible_roots: HashSet<String> = HashSet::new();
        for node_id in self.all_nodes.borrow().keys() {
            possible_roots.insert(node_id.clone());
        }
        for (_, node) in self.all_nodes.borrow().iter() {
            for child in &node.children {
                possible_roots.remove(child);
            }
        }
        assert!(possible_roots.len() == 1);
        possible_roots.iter().next().unwrap().to_string()
    }

    fn find_wrong_weight(&self, root_node_id: &str) -> bool {
        let all_nodes = self.all_nodes.borrow();
        let root_node = &all_nodes[root_node_id];
        let children: Vec<&TowerNode> = root_node.children
                                                 .iter()
                                                 .map(|c| &all_nodes[c])
                                                 .collect();
        if children.len() < 3 {
            // Do we have to do this?
            return false;
        }
        let weight1 = children[0].total_weight();
        let mut weight2: Option<usize> = None;
        let mut weight2_index: Option<usize> = None;
        let mut bad_index: Option<usize> = None;
        // This is the worst code I've written in a long time.
        for i in 1..children.len() {
            let weight = children[i].total_weight();
            if weight != weight1 {
                if weight2.is_none() {
                    if i > 1 {
                        bad_index = Some(i);
                        break;
                    } else {
                        weight2 = Some(weight);
                        weight2_index = Some(i);
                    }
                } else {
                    bad_index = Some(0);
                    break;
                }
            } else if weight2.is_some() {
                bad_index = weight2_index;
                break;
            }
        }
        if bad_index.is_some() {
            let bad_index = bad_index.unwrap();
            if self.find_wrong_weight(&children[bad_index].id) {
                return true;
            }
            let bad_weight = children[bad_index].total_weight();
            let good_weight = if bad_weight == weight1 {
                                  weight2.unwrap()
                              } else {
                                  weight1
                              };
            let delta: isize = good_weight as isize - bad_weight as isize;

            println!("I guess {} should have been {} instead of {}.",
                     children[bad_index].id,
                     children[bad_index].weight as isize+ delta,
                     children[bad_index].weight);
            return true;
        }
        false
    }
}

struct TowerNode {
    id: String,
    weight: usize,
    children: HashSet<String>,
    all_nodes: Rc<RefCell<HashMap<String, TowerNode>>>
}

impl TowerNode {
    pub fn new(id: &str, weight: usize,
               all_nodes: Rc<RefCell<HashMap<String, TowerNode>>>) -> Self {
        TowerNode {
            id: id.to_string(),
            weight: weight,
            children: HashSet::new(),
            all_nodes: all_nodes
        }
    }

    pub fn add_child_node(&mut self, child: &str) {
        self.children.insert(child.to_string());
    }

    pub fn total_weight(&self) -> usize {
        let mut result = self.weight;
        for child in &self.children {
            result += self.all_nodes.borrow()[child].total_weight();
        }
        result
    }
    
    #[allow(dead_code)]
    pub fn dump_tree(&self) {
        self.dump_tree_inner(0);
    }

    #[allow(dead_code)]
    fn dump_tree_inner(&self, indent: usize) {
        let spacing: String = " ".repeat(indent);
        println!("{}{} ({}) ({})", spacing, self.id,
                 self.weight, self.total_weight());
        for child in &self.children {
            self.all_nodes.borrow()[child].dump_tree_inner(indent+2);
        }
    }
}
