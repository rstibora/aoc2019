
pub fn first_star(input: &Vec<String>) -> String {
    // Input is a single line of numbers.
    let input = input[0].split(",");
    let mut program: Vec<i32> = Vec::new();
    for item in input {
        program.push(item.parse().unwrap());
    }
    let mut instruction_idx = 0;

    // Restoring the program.
    program[1] = 12;
    program[2] = 2;

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

    program[0].to_string()
}

fn handle_negative_address(address: i32, program_lenght: usize) -> usize {
    if address < 0 {
        (program_lenght as i32 + address) as usize
    } else {
        address as usize
    }
}

