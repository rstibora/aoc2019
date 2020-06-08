use crate::aoc_error::{AocResult, AocError};
use super::intcode_computer::{IntcodeComputer, IntcodeComputerBus, utils};

pub fn first_star(input: &str) -> AocResult {
    // Input is a single line of numbers.
    let input = input.lines().next().ok_or(AocError::new(String::from("Could not parse a line")))?;
    let program = utils::parse_intcode_program(input)?;

    let mut input_bus = IntcodeComputerBus::new();
    let mut output_bus = IntcodeComputerBus::new();
    let input_sender = input_bus.get_input()?;
    let output_receiver = output_bus.get_output()?;

    let mut computer = IntcodeComputer::new(Some(&mut input_bus), Some(&mut output_bus));

    computer.start(program)?;
    input_sender.send(1).map_err(|_mpsc_error| AocError::new(String::from("Could not send input")))?;

    computer.wait_for_result()?;
    let output = output_receiver.iter().collect::<Vec<i32>>().pop()
                    .ok_or(AocError::new(String::from("Output buffer empty")))?;

    Ok(output.to_string())
}

pub fn second_star(input: &str) -> AocResult {
    // Input is a single line of numbers.
    let input = input.lines().next().ok_or(AocError::new(String::from("Could not parse a line")))?;
    let program = utils::parse_intcode_program(input)?;

    let mut input_bus = IntcodeComputerBus::new();
    let mut output_bus = IntcodeComputerBus::new();
    let input_sender = input_bus.get_input()?;
    let output_receiver = output_bus.get_output()?;

    let mut computer = IntcodeComputer::new(Some(&mut input_bus), Some(&mut output_bus));

    computer.start(program)?;
    input_sender.send(5).map_err(|_mpsc_error| AocError::new(String::from("Could not send input")))?;

    computer.wait_for_result()?;
    let output = output_receiver.iter().collect::<Vec<i32>>().pop()
                    .ok_or(AocError::new(String::from("Output buffer empty")))?;

    Ok(output.to_string())
}