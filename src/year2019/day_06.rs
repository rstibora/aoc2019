use std::collections::HashMap;
use std::cell::{Ref, RefCell};
use std::rc::{Rc, Weak};

use crate::aoc_error::{AocResult, AocError};

pub fn first_star(input: &str) -> AocResult {
    let orbits = parse_orbits(input)?;
    let nodes = construct_orbit_tree(orbits)?;

    let mut total_orbits = 0;
    for node in nodes.values() {
        total_orbits += distance_to_root(node);
    }

    Ok(total_orbits.to_string())
}

pub fn second_star(input: &str) -> AocResult {
    let orbits = parse_orbits(input)?;
    let nodes = construct_orbit_tree(orbits)?;

    let path_to_me = dfs(&nodes, "COM", "YOU")?.ok_or(AocError::new(String::from("Could not find 'YOU' in the orbit map")))?;
    let path_to_santa = dfs(&nodes, "COM", "SAN")?.ok_or(AocError::new(String::from("Could not find 'SAN' in the orbit map")))?;
    let common_path = trim_common_path(path_to_me, path_to_santa);
    Ok((common_path.len() - 3).to_string())
}

fn dfs(nodes: &HashMap<String, Rc<RefCell<OrbitNode>>>, source_node: &str, target_node: &str) -> Result<Option<Vec<String>>, AocError> {
    let mut stack: Vec<String> = Vec::new();

    // TODO: ?
    if let Some(node) = nodes.get(source_node) {
        stack.push(node.borrow().name.clone());
    }

    while let Some(stack_top_node_name) = stack.pop() {
        let stack_top_node = nodes.get(&stack_top_node_name).ok_or(AocError::new(String::from("Could not get current node")))?.borrow();

        if stack_top_node_name == target_node {
            let mut path: Vec<String> = vec!();
            let mut current_node_name = stack_top_node_name.clone();

            while current_node_name != "COM" {
                path.push(current_node_name.clone());
                let current_node = nodes.get(&current_node_name).ok_or(AocError::new(String::from("Could not get current node")))?.borrow();
                let parent_orbit = current_node.parent_orbit.as_ref().ok_or(AocError::new(String::from("Could not get parent node")))?.upgrade()
                    .ok_or(AocError::new(String::from("Could not get parent node")))?;
                current_node_name = parent_orbit.borrow().name.clone();
            }
            path.push(current_node_name.clone());
            return Ok(Some(path.iter().rev().cloned().collect()));
        }

        for child in stack_top_node.orbitees.values() {
            stack.push(child.borrow().name.clone());
        }
    }
    Ok(None)
}

fn trim_common_path(path_a: Vec<String>, path_b: Vec<String>) -> Vec<String> {
    let mut result = vec!();

    let mut index = 0;
    while index < path_a.len() && index < path_b.len() && path_a[index] == path_b[index] {
        index += 1;
    }

    result.extend(path_a[index..path_a.len()].iter().rev().cloned());
    if index != 0 {
        result.push(path_a[index - 1].clone());
    }
    result.extend(path_b[index..path_b.len()].iter().cloned());
    result
}

fn construct_orbit_tree(orbits: Vec<(String, String)>) -> Result<HashMap<String, Rc<RefCell<OrbitNode>>>, AocError> {
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
    Ok(nodes)
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
