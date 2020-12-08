use num::{abs, integer::lcm};
use crate::aoc_error::{AocError, AocResult};

pub fn first_star(input: &str) -> AocResult {
    let mut positions = parse_positions_from_input(input)?;
    let mut velocities: Vec<[i32; 3]> = vec![[0, 0, 0]; positions.len()];
    simulate(1000, &mut positions, &mut velocities, 0..3);

    let system_energy = positions.iter().zip(velocities.iter())
                            .fold(0, |acc, (position, velocity)| {
                                acc + (abs(position[0]) + abs(position[1]) + abs(position[2]))
                                    * (abs(velocity[0]) + abs(velocity[1]) + abs(velocity[2]))});
    Ok(system_energy.to_string())
}

pub fn second_star(input: &str) -> AocResult {
    let original_positions = parse_positions_from_input(input)?;
    let original_velocities: Vec<[i32; 3]> = vec![[0, 0, 0]; original_positions.len()];

    let mut num_steps_per_dimension: [i64; 3] = [0; 3];
    for dimension in 0..3 {
        let mut positions = original_positions.to_owned();
        let mut velocities = original_velocities.to_owned();

        let mut num_steps = 1;
        simulate(1, &mut positions, &mut velocities, dimension..dimension + 1);
        num_steps_per_dimension[dimension] = loop {
            while positions != original_positions || velocities != original_velocities {
                num_steps += 1;
                simulate(1, &mut positions, &mut velocities, dimension..dimension + 1);
            }
            break num_steps
        }
    }
    let result = lcm(lcm(num_steps_per_dimension[0], num_steps_per_dimension[1]), num_steps_per_dimension[2]);
    Ok(result.to_string())
}

fn simulate(iterations: u64, positions: &mut Vec<[i32; 3]>, velocities: &mut Vec<[i32; 3]>, dimensions: std::ops::Range<usize>) {
    let num_moons = positions.len();
    for _ in 0..iterations {
        // Update velocities by applying gravity.
        for moon_a_index in 0..num_moons - 1 {
            for moon_b_index in moon_a_index + 1..num_moons {
                for coordinate in dimensions.to_owned() {
                    let distance_signum = num::signum(positions[moon_b_index][coordinate] - positions[moon_a_index][coordinate]);
                    velocities[moon_a_index][coordinate] += distance_signum;
                    velocities[moon_b_index][coordinate] -= distance_signum;
                }
            }
        }

        // Update positions by applying velocities.
        for moon_index in 0..positions.len() {
            for coordinate in dimensions.to_owned() {
                positions[moon_index][coordinate] += velocities[moon_index][coordinate];
            }
        }
    }
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
