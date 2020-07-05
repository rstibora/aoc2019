use num::integer;

use std::collections::{HashMap, HashSet};
use crate::aoc_error::{AocError, AocResult};

pub fn first_star(input: &str) -> AocResult {
    let mut visible_asteroids_map = HashMap::new();
    let asteroid_map = convert_input_to_asteroid_map(input)?;
    for source_position in &asteroid_map {
        for target_position in &asteroid_map {
            if source_position == target_position {
                continue;
            }

            let direction = (target_position.0 - source_position.0,
                             target_position.1 - source_position.1);
            visible_asteroids_map.entry(source_position).or_insert(HashSet::new())
                .insert(normalize_direction(direction));
        }
    }

    let max_visible_asteroids = visible_asteroids_map.values().map(HashSet::len).max().unwrap();
    Ok(max_visible_asteroids.to_string())
}

pub fn second_star(input: &str) -> AocResult {
    Err(AocError::new(String::from("Not implemented")))
}

const ASTEROID: char = '#';

type AsteroidMap = HashSet<(i32, i32)>;

fn convert_input_to_asteroid_map(input: &str) -> Result<AsteroidMap, AocError> {
    let mut asteroid_map = AsteroidMap::new();
    for (idx_y, line) in input.lines().enumerate() {
        for (idx_x, asteroid_reading) in line.chars().enumerate() {
            if asteroid_reading == ASTEROID {
                asteroid_map.insert((idx_x as i32, idx_y as i32));
            }
        }
    }
    Ok(asteroid_map)
}

fn normalize_direction(direction: (i32, i32)) -> (i32, i32) {
    let gcd = integer::gcd(direction.0, direction.1);
    if gcd == 0 {
        return direction;
    }
    (direction.0 / gcd, direction.1 / gcd)
}