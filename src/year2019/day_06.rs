use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::{Rc, Weak};

use crate::aoc_error::{AocResult, AocError};

pub fn first_star(input: &str) -> AocResult {
    let orbits = parse_orbits(input)?;
    let mut nodes: HashMap<String, Rc<RefCell<OrbitNode>>> = HashMap::new();

    for (target, orbitee) in orbits {
        if !nodes.contains_key(&target) {
            nodes.insert(target.clone(), Rc::new(RefCell::new(OrbitNode::new(target.clone()))));
        }
        if !nodes.contains_key(&orbitee) {
            nodes.insert(orbitee.clone(), Rc::new(RefCell::new(OrbitNode::new(orbitee.clone()))));
        }

        let target_node = nodes.get(&target).ok_or(AocError::new(String::from("Could not create a new target node")))?;
        let orbitee_node = nodes.get(&orbitee).ok_or(AocError::new(String::from("Could not create a new orbitee node")))?;
        if !target_node.borrow().orbitees.contains_key(&orbitee) {
            target_node.borrow_mut().orbitees.insert(orbitee.clone(), Rc::clone(orbitee_node));
            orbitee_node.borrow_mut().parent_orbit = Some(Rc::downgrade(target_node));
        }
    }

    let mut total_orbits = 0;
    for node in nodes.values() {
        total_orbits += distance_to_root(node);
    }

    Ok(total_orbits.to_string())
}

struct OrbitNode {
    name: String,
    parent_orbit: Option<Weak<RefCell<OrbitNode>>>,
    orbitees: HashMap<String, Rc<RefCell<OrbitNode>>>,
}

impl OrbitNode {
    fn new(name: String) -> Self {
        Self { name, parent_orbit: None, orbitees: HashMap::new() }
    }
}

fn distance_to_root(node: &Rc<RefCell<OrbitNode>>) -> u32 {
    match node.borrow().parent_orbit.as_ref().map(|r| r.upgrade()).flatten() {
        Some(parent_ref) => 1 + distance_to_root(&parent_ref),
        None => 0
    }
}

fn parse_orbits(input: &str) -> Result<Vec<(String, String)>, AocError> {
    let mut orbits = Vec::new();
    for line in input.lines() {
        let mut splits = line.split(")");
        let target = splits.next().ok_or(AocError::new(String::from("Could not parse orbit")))?;
        let orbitee = splits.next().ok_or(AocError::new(String::from("Could not parse orbit")))?;
        orbits.push((target.to_string(), orbitee.to_string()));
    }
    Ok(orbits)
}