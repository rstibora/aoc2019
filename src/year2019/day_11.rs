use std::collections::HashMap;
use std::sync::mpsc;

use crate::aoc_error::{AocError, AocResult};
use super::intcode_computer::{IntcodeComputer, utils};

fn run_robot_with_initial_tile(input: &str, initial_tile: i64) -> Result<HashMap<(i32, i32), i64>, AocError>{
    let mut robot_direction = (0, 1);
    let mut robot_position = (0, 0);
    let mut colored_positions = HashMap::new();

    let program = utils::parse_intcode_program(input)?;
    let mut brain = IntcodeComputer::new(None);

    let (input_sender, input_receiver) = mpsc::channel();
    let (output_sender, output_receiver) = mpsc::channel();

    brain.start(program, Some(input_receiver), vec![output_sender])?;

    input_sender.send(initial_tile).map_err(|err| AocError::new(format!("Could not send input: {}", err)))?;
    while let Ok(paint_instruction) = output_receiver.recv() {
        match paint_instruction {
            0 => colored_positions.insert(robot_position, 0),
            1 => colored_positions.insert(robot_position, 1),
            _ => return Err(AocError::new(String::from("Invalid paint instruction")))
        };

        if let Ok(direction_instruction) = output_receiver.recv() {
            robot_direction = match direction_instruction {
                1 => {
                    // Turn right.
                    match robot_direction {
                        (0, 1) => Ok((1, 0)),
                        (1, 0) => Ok((0, -1)),
                        (0, -1) => Ok((-1, 0)),
                        (-1, 0) => Ok((0, 1)),
                        _ => Err(AocError::new(String::from("Invalid direction")))
                    }
                },
                0 => {
                    // Turn left.
                    match robot_direction {
                        (0, 1) => Ok((-1, 0)),
                        (-1, 0) => Ok((0, -1)),
                        (0, -1) => Ok((1, 0)),
                        (1, 0) => Ok((0, 1)),
                        _ => Err(AocError::new(String::from("Invalid direction")))
                    }
                },
                _ => Err(AocError::new(String::from("Invalid direction change")))
            }?;

            robot_position.0 += robot_direction.0;
            robot_position.1 += robot_direction.1;
        } else {
            return Err(AocError::new(String::from("Brain terminated unexpectedly when moving")));
        }

        let color = colored_positions.get(&robot_position).unwrap_or(&0);
        input_sender.send(color.to_owned())
                    .map_err(|err| AocError::new(format!("Brain terminated when receiveing input: {}", err)))?;
    }
    Ok(colored_positions)
}

pub fn first_star(input: &str) -> AocResult {
    let colored_positions = run_robot_with_initial_tile(input, 0)?;
    Ok(colored_positions.len().to_string())
}

pub fn second_star(input: &str) -> AocResult {
    let colored_positions = run_robot_with_initial_tile(input, 1)?;
    let mut x_positions = colored_positions.keys().map(|position| position.0).collect::<Vec<i32>>();
    let mut y_positions = colored_positions.keys().map(|position| position.1).collect::<Vec<i32>>();
    x_positions.sort_unstable();
    y_positions.sort_unstable();

    let mut output_string = String::from("\n");
    for y in (y_positions[0]..=y_positions[y_positions.len() - 1]).rev() {
        for x in x_positions[0]..=x_positions[x_positions.len() - 1] {
            let mut color = '.';
            if colored_positions.get(&(x, y)).unwrap_or(&0) == &1 {
                color = '#'
            }
            output_string.push(color);
        }
        output_string.push('\n')
    }
    Ok(output_string)
}
