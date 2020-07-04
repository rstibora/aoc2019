use std::{fmt, thread};
use std::sync::{Arc, Barrier, mpsc};

use crate::aoc_error::AocError;

// TODO: make the computer generic with respect to the register type.
type RegisterType = i64;
pub type Program = Vec<RegisterType>;

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

type Address = RegisterType;
type Value = RegisterType;

#[derive(PartialEq)]
enum Parameter {
    PositionMode(Address),
    ImmediateMode(Value),
    RelativeMode(Value),
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Parameter::PositionMode(address) => write!(f, "PositionMode({})", address),
            Parameter::ImmediateMode(value) => write!(f, "ImmediateMode({})", value),
            Parameter::RelativeMode(value) => write!(f, "RelativeMode({})", value),
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
    Rbo(Parameter),
}

pub struct IntcodeComputer {
    thread_handle: Option<thread::JoinHandle<Result<RegisterType, IntcodeComputerError>>>,
    finish_barrier: Option<Arc<Barrier>>,
}

impl IntcodeComputer {
    pub fn new(finish_barrier: Option<Arc<Barrier>>) -> Self {
        IntcodeComputer { thread_handle: None, finish_barrier }
    }

    pub fn start(&mut self, program: Program, input_bus: Option<mpsc::Receiver<RegisterType>>,
                 output_buses: Vec<mpsc::Sender<RegisterType>>) -> Result<(), IntcodeComputerError> {
        if self.thread_handle.is_some() {
            return Err(IntcodeComputerError::new(String::from("Computer already running")));
        };

        let barrier = match &self.finish_barrier {
            Some(barrier) => Some(Arc::clone(barrier)),
            None => None,
        };
        self.thread_handle = Some(thread::spawn(move|| {
            let mut hardware = IntcodeHardware::new();
            hardware.run(program, input_bus, output_buses, barrier)
        }));
        Ok(())
    }

    pub fn wait_for_result(&mut self) -> Result<RegisterType, IntcodeComputerError> {
        match std::mem::replace(&mut self.thread_handle, None) {
            Some(thread_handle) => {
                let result = match thread_handle.join() {
                    Ok(result) => result,
                    Err(error) => Err(IntcodeComputerError::new(String::from("Could not join thread")))
                };
                self.thread_handle = None;
                result
            }
            None => Err(IntcodeComputerError::new(String::from("Computer is not running")))
        }
    }
}

pub struct IntcodeHardware {
    memory: Program,
    ip: usize,
    relative_base: RegisterType,
    input: Option<mpsc::Receiver<RegisterType>>,
    outputs: Vec<mpsc::Sender<RegisterType>>,
}

impl IntcodeHardware {
    pub fn new() -> Self {
        IntcodeHardware { memory: vec![99], ip: 0, relative_base: 0, input: None, outputs: vec![] }
    }

    pub fn run(&mut self, program: Program, input: Option<mpsc::Receiver<RegisterType>>,
               outputs: Vec<mpsc::Sender<RegisterType>>, finish_barrier: Option<Arc<Barrier>>) -> Result<RegisterType, IntcodeComputerError> {
        self.input = input;
        self.outputs = outputs;
        self.memory = program;

        let mut instruction = self.parse_instruction()?;
        while instruction != Instruction::Halt {
            self.execute_instruction(&instruction)?;
            instruction = self.parse_instruction()?;
        }

        // Wait in case of cooperative computation so that the mpsc::Channel is not droppped prematurely.
        if let Some(barrier) = &finish_barrier {
            barrier.wait();
        }

        Ok(self.memory[0])
    }

    fn parse_instruction(&self) -> Result<Instruction, IntcodeComputerError> {
        let opcode = self.memory[self.ip].to_string();
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
                let result_address = self.parse_address(3, &parameter_modes)?;
                Ok(Instruction::Add(first_parameter, second_parameter, result_address))
            },
            2 => {
                let first_parameter = self.parse_parameter(1, &parameter_modes)?;
                let second_parameter = self.parse_parameter(2, &parameter_modes)?;
                let result_address = self.parse_address(3, &parameter_modes)?;
                Ok(Instruction::Mul(first_parameter, second_parameter, result_address))
            },
            3 => {
                let result_address = self.parse_address(1, &parameter_modes)?;
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
                let result_address = self.parse_address(3, &parameter_modes)?;
                Ok(Instruction::Lst(first_parameter, second_parameter, result_address))
            },
            8 => {
                let first_parameter = self.parse_parameter(1, &parameter_modes)?;
                let second_parameter = self.parse_parameter(2, &parameter_modes)?;
                let result_address = self.parse_address(3, &parameter_modes)?;
                Ok(Instruction::Eqs(first_parameter, second_parameter, result_address))
            },
            9 => {
                let parameter = self.parse_parameter(1, &parameter_modes)?;
                Ok(Instruction::Rbo(parameter))
            },
            unknown_opcode => Err(IntcodeComputerError::new(String::from(format!("Unknown opcode {}", unknown_opcode))))
        }
    }

    fn parse_parameter(&self, parameter_position: usize, parameter_modes: &Vec<char>) -> Result<Parameter, IntcodeComputerError> {
        let parameter_address = self.ip + parameter_position;
        match parameter_modes.get(parameter_position - 1).map(|x| char::to_digit(*x, 10)).flatten().unwrap_or(0) {
            0 => Ok(Parameter::PositionMode(self.memory[parameter_address])),
            1 => Ok(Parameter::ImmediateMode(self.memory[parameter_address])),
            2 => Ok(Parameter::RelativeMode(self.memory[parameter_address])),
            unknown_parameter_mode => Err(IntcodeComputerError::new(
                String::from(format!("Unknown parameter mode {}", unknown_parameter_mode))))
        }
    }

    fn parse_address(&self, parameter_position: usize, parameter_modes: &Vec<char>)  -> Result<Address, IntcodeComputerError> {
        let parameter_address = self.ip + parameter_position;
        match parameter_modes.get(parameter_position - 1).map(|x| char::to_digit(*x, 10)).flatten().unwrap_or(0) {
            0 => Ok(self.memory[parameter_address]),
            1 => Err(IntcodeComputerError::new(String::from("Output parameter can't be in immediate mode"))),
            2 => Ok(self.memory[parameter_address] as i64 + self.relative_base),
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
                self.store_value(value, *address);
                self.ip += 4;
            },
            Instruction::Mul(parameter_a, parameter_b, address) => {
                let value = self.load_parameter(parameter_a) * self.load_parameter(parameter_b);
                self.store_value(value, *address);
                self.ip += 4;
            },
            Instruction::Inp(address) => {
                let value = match &self.input {
                    Some(input) => input.recv()
                        .map_err(|mpsc_error| IntcodeComputerError::new(
                            String::from(format!("Could not read from the channel: {}", mpsc_error))))?,
                    None => return Err(IntcodeComputerError::new(String::from("Input not available"))),
                };
                self.store_value(value, *address);
                self.ip += 2;
            },
            Instruction::Out(parameter) => {
                let value = self.load_parameter(parameter);
                for output in &self.outputs {
                    output.send(value)
                        .map_err(|mpsc_error| IntcodeComputerError::new(
                            String::from(format!("Could not send a value to the channel: {}", mpsc_error))))?;
                }
                self.ip += 2;
            },
            Instruction::Jit(parameter, address) => {
                let value = self.load_parameter(parameter);
                let address = self.load_parameter(address);
                if value != 0 {
                    self.ip = address as usize;
                } else {
                    self.ip += 3;
                }
            },
            Instruction::Jif(parameter, address) => {
                let value = self.load_parameter(parameter);
                let address = self.load_parameter(address);
                if value == 0 {
                    self.ip = address as usize;
                } else {
                    self.ip += 3;
                }
            },
            Instruction::Lst(parameter_a, parameter_b, address) => {
                let value = match  self.load_parameter(parameter_a) < self.load_parameter(parameter_b) {
                    true => 1,
                    false => 0,
                };
                self.store_value(value, *address);
                self.ip += 4;
            },
            Instruction::Eqs(parameter_a, parameter_b, address) => {
                let value = match  self.load_parameter(parameter_a) == self.load_parameter(parameter_b) {
                    true => 1,
                    false => 0,
                };
                self.store_value(value, *address);
                self.ip += 4;
            },
            Instruction::Rbo(parameter) => {
                let value = self.load_parameter(parameter);
                self.relative_base = self.relative_base + value;
                self.ip += 2
            },
        };
        Ok(())
    }

    fn load_parameter(&self, parameter: &Parameter) -> RegisterType {
        match parameter {
            Parameter::ImmediateMode(value) => {
                value.to_owned()
            }
            Parameter::PositionMode(address) => {
                let address = *address as usize;
                if address >= self.memory.len() {
                    0
                } else {
                    self.memory[address]
                }
            }
            Parameter::RelativeMode(value) => {
                let address = (self.relative_base as RegisterType + value) as usize;
                if address >= self.memory.len() {
                    0
                } else {
                    self.memory[address]
                }
            }
        }
    }

    fn store_value(&mut self, value: RegisterType, address: Address) {
        let address = address as usize;
        if address >= self.memory.len() {
            self.memory.resize(address + 1, 0);
        }
        self.memory[address] = value
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
