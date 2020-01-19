use crate::aoc_error::{AocResult, AocError};
use super::intcode_computer::{IntcodeComputer, Input, utils};

pub fn first_star(input: &str) -> AocResult {
    // Input is a single line of numbers.
    let input = input.lines().next().ok_or(AocError::new(String::from("Could not parse a line")))?;
    let program = utils::parse_intcode_program(input)?;
    let mut computer = IntcodeComputer::new();
    computer.load_program(program);

    computer.input_buffer.push_front(String::from("1"));

    computer.run(Input::new())?;
    let output = computer.output_buffer.front().ok_or(AocError::new(String::from("No output in the buffer")))?;
    Ok(output.to_string())
}