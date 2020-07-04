use std::sync::mpsc;

use crate::aoc_error::{AocResult, AocError};
use super::intcode_computer::{IntcodeComputer, utils};

pub fn first_star(input: &str) -> AocResult {
    // Input is a single line of numbers.
    let input = input.lines().next().ok_or(AocError::new(String::from("Could not parse a line")))?;
    let program = utils::parse_intcode_program(input)?;

    let (input_sender, input_receiver) = mpsc::channel();
    let (output_sender, output_receiver) = mpsc::channel();

    let mut computer = IntcodeComputer::new(None);

    computer.start(program, Some(input_receiver), vec![output_sender])?;
    input_sender.send(1).map_err(|_mpsc_error| AocError::new(String::from("Could not send input")))?;

    computer.wait_for_result()?;
    let output = output_receiver.iter().collect::<Vec<i64>>().pop()
                    .ok_or(AocError::new(String::from("Output buffer empty")))?;

    Ok(output.to_string())
}

pub fn second_star(input: &str) -> AocResult {
    // Input is a single line of numbers.
    let input = input.lines().next().ok_or(AocError::new(String::from("Could not parse a line")))?;
    let program = utils::parse_intcode_program(input)?;

    let (input_sender, input_receiver) = mpsc::channel();
    let (output_sender, output_receiver) = mpsc::channel();

    let mut computer = IntcodeComputer::new(None);

    computer.start(program, Some(input_receiver), vec![output_sender])?;
    input_sender.send(5).map_err(|_mpsc_error| AocError::new(String::from("Could not send input")))?;

    computer.wait_for_result()?;
    let output = output_receiver.iter().collect::<Vec<i64>>().pop()
                    .ok_or(AocError::new(String::from("Output buffer empty")))?;

    Ok(output.to_string())
}