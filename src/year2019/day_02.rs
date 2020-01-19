use crate::aoc_error::{AocResult, AocError};
use super::intcode_computer::{IntcodeComputer, Input, utils};

pub fn first_star(input: &str) -> AocResult {
    // Input is a single line of numbers.
    let input = input.lines().next().ok_or(AocError::new(String::from("Could not parse a line")))?;
    let program = utils::parse_intcode_program(input)?;
    let mut computer = IntcodeComputer::new();
    computer.load_program(program);

    let mut input = Input::new();
    input.insert(1, 12);
    input.insert(2, 2);

    let output = computer.run(input)?;
    Ok(output.to_string())
}

pub fn second_star(input: &str) -> AocResult {
    const EXPECTED_VALUE: i32 = 19690720;

    let input = input.lines().next().ok_or(AocError::new(String::from("Could not parse a line")))?;
    let program = utils::parse_intcode_program(input)?;
    let mut computer = IntcodeComputer::new();
    computer.load_program(program.clone());

    for noun in 0..99 {
        for verb in 0..99 {
            let mut input = Input::new();
            input.insert(1, noun);
            input.insert(2, verb);
            computer.restart();
            computer.load_program(program.clone());
            if computer.run(input)? == EXPECTED_VALUE {
                return Ok((100 * noun + verb).to_string());
            }
        }
    }
    Err(AocError::new(String::from("Did not reach the expected value")))
}
