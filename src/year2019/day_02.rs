use crate::aoc_error::{AocResult, AocError};
use super::intcode_computer::{IntcodeComputer, utils};

pub fn first_star(input: &str) -> AocResult {
    // Input is a single line of numbers.
    let input = input.lines().next().ok_or_else(|| AocError::new(String::from("Could not parse a line")))?;
    let mut program = utils::parse_intcode_program(input)?;
    let mut computer = IntcodeComputer::new(None);
    program[1] = 12;
    program[2] = 2;

    computer.start(program, None, vec![])?;
    let output = computer.wait_for_result()?;
    Ok(output.to_string())
}

pub fn second_star(input: &str) -> AocResult {
    const EXPECTED_VALUE: i64 = 19690720;

    let input = input.lines().next().ok_or_else(|| AocError::new(String::from("Could not parse a line")))?;
    let mut program = utils::parse_intcode_program(input)?;

    for noun in 0..99 {
        program[1] = noun;
        for verb in 0..99 {
            program[2] = verb;
            let mut computer = IntcodeComputer::new(None);
            computer.start(program.clone(), None, vec![])?;
            let output = computer.wait_for_result()?;
            if output == EXPECTED_VALUE {
                return Ok((100 * noun + verb).to_string());
            }
        }
    }
    Err(AocError::new(String::from("Did not reach the expected value")))
}
