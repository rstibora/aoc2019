use num::abs;
use crate::aoc_error::{AocError, AocResult};

pub fn first_star(input: &str) -> AocResult {
    let mut positions = parse_positions_from_input(input)?;
    let mut velocities: Vec<[i32; 3]> = vec![[0, 0, 0]; positions.len()];
    for _ in 0..1000 {
        // Update velocities by applying gravity.
        for updated_moon_index in 0..positions.len() {
            for observed_moon_index in 0..positions.len() {
                if updated_moon_index == observed_moon_index {
                    continue;
                }

                for coordinate in 0..3 {
                    velocities[updated_moon_index][coordinate] += distance_to_velocity_change(
                        positions[observed_moon_index][coordinate] - positions[updated_moon_index][coordinate]);
                }
            }
        }

        // Update positions by applying velocities.
        for moon_index in 0..positions.len() {
            for coordinate in 0..3 {
                positions[moon_index][coordinate] += velocities[moon_index][coordinate];
            }
        }
    }

    let system_energy = positions.iter().zip(velocities.iter())
                            .fold(0, |acc, (position, velocity)| {
                                acc + (abs(position[0]) + abs(position[1]) + abs(position[2]))
                                    * (abs(velocity[0]) + abs(velocity[1]) + abs(velocity[2]))});
    Ok(system_energy.to_string())
}

fn parse_positions_from_input(input: &str) -> Result<Vec<[i32; 3]>, AocError> {
    let mut positions = vec![];
    for line in input.lines() {
        let mut position_iterator = line.split(|c| c == '=' || c == ',' || c == '>').skip(1);
        let mut position = [0, 0, 0];
        for (coordinate_index, coordinate) in ('x'..='z').enumerate() {
            position[coordinate_index] = position_iterator.next()
                .ok_or_else(|| AocError::new(format!("Could not parse position coordinate {}", coordinate)))?
                .parse::<i32>()
                .map_err(|err| AocError::new(format!("Could not parse position coordinate {}: {}", coordinate, err)))?;
            position_iterator.next();
        }
        positions.push(position)
    }
    Ok(positions)
}

fn distance_to_velocity_change(distance: i32) -> i32 {
    match distance {
        0 => 0,
        x if x < 0 => -1,
        x if x > 0 => 1,
        _ => panic!()
    }
}