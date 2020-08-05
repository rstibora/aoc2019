use std::collections::HashMap;
use std::sync::mpsc;

use crate::aoc_error::{AocError, AocResult};
use super::intcode_computer::{IntcodeComputer, utils};

pub fn first_star(input: &str) -> AocResult {
    let mut robot_direction = (0, 1);
    let mut robot_position = (0, 0);
    let mut colored_positions = HashMap::new();

    let program = utils::parse_intcode_program(input)?;
    let mut brain = IntcodeComputer::new(None);

    let (input_sender, input_receiver) = mpsc::channel();
    let (output_sender, output_receiver) = mpsc::channel();

    brain.start(program, Some(input_receiver), vec![output_sender])?;

    input_sender.send(0).map_err(|err| AocError::new(format!("Could not send input: {}", err)))?;
    loop {
        if let Ok(paint_instruction) = output_receiver.recv() {
            match paint_instruction {
                0 => colored_positions.insert(robot_position, 0),
                1 => colored_positions.insert(robot_position, 1),
                _ => return Err(AocError::new(String::from("Invalid paint instruction")))
            };
        } else {
            // return Err(AocError::new(String::from("Brain terminated unexpectedly")));
            break;
        }

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
            // println!("direction {:?}, position {:?}", robot_direction, robot_position);
        } else {
            // return Err(AocError::new(String::from("Brain terminated unexpectedly")));
            break;
        }

        let color = colored_positions.get(&robot_position).unwrap_or(&0);
        if let Err(_) = input_sender.send(color.to_owned()) {
            break;
        }
    }
    return Ok(colored_positions.len().to_string());
}
