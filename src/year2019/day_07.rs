use crate::aoc_error::{AocResult, AocError};
use super::intcode_computer::{IntcodeComputer, Input, utils};

use permutohedron::heap_recursive;

pub fn first_star(input: &str) -> AocResult {
    let input = input.lines().next().ok_or(AocError::new(String::from("Could not parse a line")))?;
    let program = utils::parse_intcode_program(input)?;

    let mut amp_computers = [IntcodeComputer::new(), IntcodeComputer::new(), IntcodeComputer::new(),
                             IntcodeComputer::new() ,IntcodeComputer::new()];

    let mut phase_setting = [0, 1, 2, 3, 4];
    let mut permutations = vec!();
    heap_recursive(&mut phase_setting, |permutation| { permutations.push(permutation.to_vec()) });

    let mut outputs = vec![0; permutations.len()];
    for permutation in permutations {
        let mut signal = [0; 5];
        for i in 0..5 {
            amp_computers[i].restart();
            amp_computers[i].load_program(program.clone());
            amp_computers[i].input_buffer.push_front(permutation[i]);
            amp_computers[i].input_buffer.push_front(signal[i]);
            amp_computers[i].run(Input::new())?;
            let output = amp_computers[i].output_buffer.pop_back()
                .ok_or(AocError::new(String::from("Output buffer should not be empty")))?;

            if i == 4 {
                outputs.push(output);
            } else {
                signal[i+1] = output;
            }
        }
    }
    let maximum_output = outputs.iter().max().ok_or(AocError::new(String::from("Could not get maximum value")))?;
    Ok(maximum_output.to_string())
}
