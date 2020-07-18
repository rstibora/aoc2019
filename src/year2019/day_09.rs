use std::sync::mpsc;

use crate::aoc_error::{AocError, AocResult};
use super::intcode_computer::{IntcodeComputer, utils};


pub fn first_star(input: &str) -> AocResult {
    Ok(run_with_input(input, 1)?)
}

pub fn second_star(input: &str) -> AocResult {
    Ok(run_with_input(input, 2)?)
}

fn run_with_input(input: &str, computer_input: i64) -> AocResult {
    let input = input.lines().next().ok_or_else(|| AocError::new(String::from("Could not parse a line")))?;
    let program = utils::parse_intcode_program(input)?;

    let (input_sender, input_receiver) = mpsc::channel();
    let (output_sender, output_receiver) = mpsc::channel();
    let mut computer = IntcodeComputer::new(None);

    computer.start(program, Some(input_receiver), vec![output_sender])?;
    input_sender.send(computer_input).map_err(|_mpsc_error| AocError::new(String::from("Could not send input")))?;

    computer.wait_for_result()?;
    let output = output_receiver.iter().next().ok_or_else(|| AocError::new(String::from("Did not get output")))?;
    Ok(output.to_string())
}