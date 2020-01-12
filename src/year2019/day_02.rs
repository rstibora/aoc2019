use crate::aoc_error::{AocResult, AocError};
use crate::utils;

pub fn first_star(input: &str) -> AocResult {
    // Input is a single line of numbers.
    let input = input.lines().next().ok_or(AocError::new("Could not parse a line"))?;
    let program = parse_intcode_program(input)?;
    let output = run_program_with_inputs(program, 12, 2);
    Ok(output.to_string())
}

pub fn second_star(input: &str) -> AocResult {
    const EXPECTED_VALUE: i32 = 19690720;

    let input = input.lines().next().ok_or(AocError::new("Could not parse a line"))?;
    let program = parse_intcode_program(input)?;
    for noun in 0..99 {
        for verb in 0..99 {
            if run_program_with_inputs(program.clone(), noun, verb) == EXPECTED_VALUE {
                return Ok((100 * noun + verb).to_string())
            }
        }
    }
    panic!();
}

fn run_program_with_inputs(mut program: Vec<i32>, arg_a: i32, arg_b: i32) -> i32 {
    let mut instruction_idx = 0;

    program[1] = arg_a;
    program[2] = arg_b;

    loop {
        let addr_op_a = handle_negative_address(program[instruction_idx + 1], program.len());
        let addr_op_b = handle_negative_address(program[instruction_idx + 2], program.len());
        let addr_op_result = handle_negative_address(program[instruction_idx + 3], program.len());

        match program[instruction_idx] {
            1 => program[addr_op_result] = program[addr_op_a] + program[addr_op_b],
            2 => program[addr_op_result] = program[addr_op_a] * program[addr_op_b],
            99 => break,
            _ => panic!(),
        }

        instruction_idx += 4;
    }
    program[0]
}

fn handle_negative_address(address: i32, program_lenght: usize) -> usize {
    if address < 0 {
        (program_lenght as i32 + address) as usize
    } else {
        address as usize
    }
}

fn parse_intcode_program(program_as_string: &str) -> Result<Vec<i32>, AocError> {
    let mut program: Vec<i32> = Vec::new();
    for item in program_as_string.split(",") {
        match item.parse() {
            Ok(value) => program.push(value),
            Err(error) => return Err(AocError::new(&format!("Could not parse intcode program: {}", error.to_string()))),
        }
    }
    Ok(program)
}
