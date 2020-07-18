use std::cmp;
use std::collections::{HashMap, HashSet};

use num::integer;

use crate::aoc_error::{AocError, AocResult};

pub fn first_star(input: &str) -> AocResult {
    let asteroid_map = convert_input_to_asteroid_map(input)?;
    let visible_asteroids_map = calculate_asteroid_visibility(&asteroid_map);
    let max_visible_asteroids = visible_asteroids_map.values().map(HashMap::len).max()
                                    .ok_or_else(|| AocError::new(String::from("Invalid station position")))?;
    Ok(max_visible_asteroids.to_string())
}

pub fn second_star(input: &str) -> AocResult {
    // Find the position of the station.
    let mut asteroid_map = convert_input_to_asteroid_map(input)?;
    let visible_asteroids_map = calculate_asteroid_visibility(&asteroid_map);
    let (station_position, _) = visible_asteroids_map.iter().max_by_key(|(_, visible_asteroids)| visible_asteroids.len())
        .ok_or_else(|| AocError::new(String::from("Invalid station position")))?;

    // Shoot them asteroids.
    // let station_position = &(8, 3);
    let mut destroyed_targets = 0;
    let mut targets = acquire_targets(station_position, &asteroid_map);
    let last_destroyed_asteroid_position = loop {
        if targets.is_empty() {
            targets = acquire_targets(station_position, &asteroid_map)
        }

        match targets.pop() {
            Some(position) => {
                destroyed_targets += 1;
                asteroid_map.remove(&position);
                if destroyed_targets == 200 {
                    break Ok(position)
                }
            },
            None => break Err(AocError::new(String::from("Got invalid target")))
        };
    }?;
    Ok((last_destroyed_asteroid_position.0 * 100 + last_destroyed_asteroid_position.1).to_string())
}

const ASTEROID: char = '#';

type SpaceCoordinate = (i32, i32);
type Direction = (i32, i32);
type AsteroidMap = HashSet<SpaceCoordinate>;
type VisibilityMap = HashMap<SpaceCoordinate, HashMap<Direction, SpaceCoordinate>>;

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

fn calculate_asteroid_visibility(asteroid_map: &AsteroidMap) -> VisibilityMap {
    let mut visible_asteroids_map = HashMap::new();
    for source_position in asteroid_map {
        for target_position in asteroid_map {
            if source_position == target_position {
                continue;
            }

            let direction = (target_position.0 - source_position.0,
                             target_position.1 - source_position.1);
            visible_asteroids_map.entry(source_position.to_owned()).or_insert_with(HashMap::new)
                .entry(normalize_direction(direction))
                // Keep only the closest asteroid's position in the given direction.
                .and_modify(|existing_position: &mut (i32, i32)| {
                    let existing_distance = (existing_position.0 - source_position.0).abs() + (existing_position.1 - source_position.1).abs();
                    if existing_distance > direction.0.abs() + direction.1.abs() {
                        existing_position.0 = target_position.0;
                        existing_position.1 = target_position.1;
                    }
                })
                .or_insert_with(|| target_position.to_owned());
        }
    }
    visible_asteroids_map
}

fn acquire_targets(station_position: &SpaceCoordinate, asteroid_map: &AsteroidMap) -> Vec<SpaceCoordinate> {
    let mut visible_asteroids = calculate_asteroid_visibility(asteroid_map)[station_position]
                                    .values().cloned().collect::<Vec<SpaceCoordinate>>();
    visible_asteroids.sort_by(|a, b| {
        let a_station_asteroid_direction = (a.0 - station_position.0, a.1 - station_position.1);
        let b_station_asteroid_direciton = (b.0 - station_position.0, b.1 - station_position.1);
        clockwise_angle(a_station_asteroid_direction)
            .partial_cmp(&clockwise_angle(b_station_asteroid_direciton)).unwrap_or(cmp::Ordering::Equal)
    });
    visible_asteroids.reverse();
    visible_asteroids
}

fn normalize_direction(direction: SpaceCoordinate) -> SpaceCoordinate {
    let gcd = integer::gcd(direction.0, direction.1);
    if gcd == 0 {
        return direction;
    }
    (direction.0 / gcd, direction.1 / gcd)
}

fn clockwise_angle(direction_vector: SpaceCoordinate) -> f32 {
    // Hacky hack is hacky so that 0. is returned for (0, y).
    if direction_vector.0 == 0 {
        return 0.
    }

    let norm = (direction_vector.0.abs() + direction_vector.1.abs()) as f32;
    let normalized_vector = (direction_vector.0 as f32 / norm, direction_vector.1 as f32 / norm);
    let mut angle = normalized_vector.1.atan2(normalized_vector.0) + std::f32::consts::PI / 2.;

    if angle < 0. {
        angle += 2. * std::f32::consts::PI;
    }
    angle
}

#[cfg(test)]
mod tests {
    use assert_approx_eq::assert_approx_eq;
    use super::*;

    #[test]
    fn test_clockwise_angle() {
        assert_approx_eq!(clockwise_angle((0, 1)), 0.);
        assert_approx_eq!(clockwise_angle((1, 0)), std::f32::consts::PI / 2.);
        assert_approx_eq!(clockwise_angle((2, 2)), std::f32::consts::PI * (3./4.));
        assert_approx_eq!(clockwise_angle((-2, 2)), std::f32::consts::PI * (5./4.));
        assert_approx_eq!(clockwise_angle((-2, 0)), std::f32::consts::PI * (3./2.));
    }
}
