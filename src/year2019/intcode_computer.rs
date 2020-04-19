use std::collections::{HashMap, VecDeque};
use std::fmt;

use crate::aoc_error::AocError;

// TODO: make the computer generic with respect to the register type.
type RegisterType = i32;
pub type Program = Vec<RegisterType>;
pub type Input = HashMap<usize, RegisterType>;

#[derive(Debug)]
pub struct IntcodeComputerError {
    message: String,
}

impl fmt::Display for IntcodeComputerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Intcode Computer Error: {}", self.message)
    }
}

impl IntcodeComputerError {
    pub fn new(message: String) -> IntcodeComputerError {
        IntcodeComputerError { message }
    }
}

impl From<IntcodeComputerError> for AocError {
    fn from(error: IntcodeComputerError) -> Self {
        let message = format!("Intcode Computer: {}", error.to_string());
        AocError::new(String::from(message))
    }
}

type Address = i32;
type Value = i32;

#[derive(PartialEq)]
enum Parameter {
    PositionMode(Address),
    ImmediateMode(Value),
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Parameter::PositionMode(address) => write!(f, "PositionMode({})", address),
            Parameter::ImmediateMode(value) => write!(f, "ImmediateMode({})", value),
        }
    }
}

#[derive(PartialEq)]
enum Instruction {
    Halt,
    Add(Parameter, Parameter, Address),
    Mul(Parameter, Parameter, Address),
    Inp(Address),
    Out(Parameter),
    Jit(Parameter, Parameter),
    Jif(Parameter, Parameter),
    Lst(Parameter, Parameter, Address),
    Eqs(Parameter, Parameter, Address),
}

pub struct IntcodeComputer {
    instruction_pointer: usize,
    program: Program,
    // TODO: hide behind an interface.
    pub input_buffer: VecDeque<RegisterType>,
    pub output_buffer: VecDeque<RegisterType>,
}

impl IntcodeComputer {
    pub fn new() -> IntcodeComputer {
        IntcodeComputer { instruction_pointer: 0, program: vec![99],
                          input_buffer: VecDeque::new(), output_buffer: VecDeque::new() }
    }

    pub fn load_program(&mut self, program: Program) {
        self.program = program;
    }

    pub fn restart(&mut self) {
        self.instruction_pointer = 0;
        self.input_buffer = VecDeque::new();
        self.output_buffer = VecDeque::new();
    }

    pub fn run(&mut self, input: Input) -> Result<RegisterType, IntcodeComputerError> {
        for (address, value) in input {
            self.program[address] = value;
        }

        let mut instruction = self.parse_instruction()?;
        while instruction != Instruction::Halt {
            self.execute_instruction(&instruction)?;
            instruction = self.parse_instruction()?;
        }
        Ok(self.program[0])
    }

    fn parse_instruction(&self) -> Result<Instruction, IntcodeComputerError> {
        let opcode = self.program[self.instruction_pointer].to_string();
        let instruction_code = match opcode.len() {
            1 => format!("0{}", opcode),
            _ => opcode[(opcode.len() - 2)..=(opcode.len() - 1)].to_owned()
        };
        let instruction_code = instruction_code.parse::<RegisterType>().map_err(
            |error| IntcodeComputerError::new(String::from(format!("Could not parse opcode: {}", error))))?;
        let parameter_modes = match opcode.len() {
            0..=2 => Vec::new(),
            non_default_modes => {
                let mut parameter_modes = opcode[..non_default_modes - 2].chars().collect::<Vec<char>>();
                parameter_modes.reverse();
                parameter_modes
            }
        };

        match instruction_code {
            99 => {
                Ok(Instruction::Halt)
            },
            1 => {
                let first_parameter = self.parse_parameter(1, &parameter_modes)?;
                let second_parameter = self.parse_parameter(2, &parameter_modes)?;
                let result_address = self.program[self.instruction_pointer + 3];
                Ok(Instruction::Add(first_parameter, second_parameter, result_address))
            },
            2 => {
                let first_parameter = self.parse_parameter(1, &parameter_modes)?;
                let second_parameter = self.parse_parameter(2, &parameter_modes)?;
                let result_address = self.program[self.instruction_pointer + 3];
                Ok(Instruction::Mul(first_parameter, second_parameter, result_address))
            },
            3 => {
                let result_address = self.program[self.instruction_pointer + 1];
                Ok(Instruction::Inp(result_address))
            },
            4 => {
                let parameter = self.parse_parameter(1, &parameter_modes)?;
                Ok(Instruction::Out(parameter))
            },
            5 => {
                let parameter = self.parse_parameter(1, &parameter_modes)?;
                let result_address = self.parse_parameter(2, &parameter_modes)?;
                Ok(Instruction::Jit(parameter, result_address))
            }
            6 => {
                let parameter = self.parse_parameter(1, &parameter_modes)?;
                let result_address = self.parse_parameter(2, &parameter_modes)?;
                Ok(Instruction::Jif(parameter, result_address))
            },
            7 => {
                let first_parameter = self.parse_parameter(1, &parameter_modes)?;
                let second_parameter = self.parse_parameter(2, &parameter_modes)?;
                let result_address = self.program[self.instruction_pointer + 3];
                Ok(Instruction::Lst(first_parameter, second_parameter, result_address))
            },
            8 => {
                let first_parameter = self.parse_parameter(1, &parameter_modes)?;
                let second_parameter = self.parse_parameter(2, &parameter_modes)?;
                let result_address = self.program[self.instruction_pointer + 3];
                Ok(Instruction::Eqs(first_parameter, second_parameter, result_address))
            },
            unknown_opcode => Err(IntcodeComputerError::new(String::from(format!("Unknown opcode {}", unknown_opcode))))
        }
    }

    fn parse_parameter(&self, parameter_position: usize, parameter_modes: &Vec<char>) -> Result<Parameter, IntcodeComputerError> {
        match parameter_modes.get(parameter_position - 1).map(|x| char::to_digit(*x, 10)).flatten().unwrap_or(0) {
            0 => Ok(Parameter::PositionMode(self.program[self.instruction_pointer + parameter_position])),
            1 => Ok(Parameter::ImmediateMode(self.program[self.instruction_pointer + parameter_position])),
            unknown_parameter_mode => Err(IntcodeComputerError::new(
                String::from(format!("Unknown parameter mode {}", unknown_parameter_mode))))
        }
    }

    fn execute_instruction(&mut self, instruction: &Instruction) -> Result<(), IntcodeComputerError>{
        match instruction {
            Instruction::Halt => {
                return Ok(())
            }
            Instruction::Add(parameter_a, parameter_b, address) => {
                let value = self.load_parameter(parameter_a) + self.load_parameter(parameter_b);
                self.store_value(value, address);
                self.instruction_pointer += 4;
            },
            Instruction::Mul(parameter_a, parameter_b, address) => {
                let value = self.load_parameter(parameter_a) * self.load_parameter(parameter_b);
                self.store_value(value, address);
                self.instruction_pointer += 4;
            },
            Instruction::Inp(address) => {
                let value = self.input_buffer.pop_back().ok_or(
                    IntcodeComputerError::new(String::from("IO error: reading empty input buffer"))
                )?;
                self.store_value(value, address);
                self.instruction_pointer += 2;
            },
            Instruction::Out(parameter) => {
                let value = self.load_parameter(parameter);
                self.output_buffer.push_front(value);
                self.instruction_pointer += 2;
            },
            Instruction::Jit(parameter, address) => {
                let value = self.load_parameter(parameter);
                let jump_address = self.load_parameter(address);
                if value != 0 {
                    self.instruction_pointer = jump_address as usize;
                } else {
                    self.instruction_pointer += 3;
                }
            },
            Instruction::Jif(parameter, address) => {
                let value = self.load_parameter(parameter);
                let jump_address = self.load_parameter(address);
                if value == 0 {
                    self.instruction_pointer = jump_address as usize;
                } else {
                    self.instruction_pointer += 3;
                }
            },
            Instruction::Lst(parameter_a, parameter_b, address) => {
                let value = match  self.load_parameter(parameter_a) < self.load_parameter(parameter_b) {
                    true => 1,
                    false => 0,
                };
                self.store_value(value, address);
                self.instruction_pointer += 4;
            },
            Instruction::Eqs(parameter_a, parameter_b, address) => {
                let value = match  self.load_parameter(parameter_a) == self.load_parameter(parameter_b) {
                    true => 1,
                    false => 0,
                };
                self.store_value(value, address);
                self.instruction_pointer += 4;
            },
        };
        Ok(())
    }

    fn load_parameter(&self, parameter: &Parameter) -> RegisterType {
        match parameter {
            Parameter::ImmediateMode(value) => value.to_owned(),
            Parameter::PositionMode(address) => self.program[*address as usize]
        }
    }

    fn store_value(&mut self, value: RegisterType, address: &Address) {
        self.program[*address as usize] = value
    }
}

pub mod utils {
    use super::*;

    pub fn parse_intcode_program(program_as_string: &str) -> Result<Program, IntcodeComputerError> {
        let mut program: Program = Vec::new();
        for item in program_as_string.split(",") {
            match item.parse() {
                Ok(value) => program.push(value),
                Err(error) => return Err(IntcodeComputerError::new(format!("Could not parse intcode program: {}", error.to_string()))),
            }
        }
        Ok(program)
    }
}