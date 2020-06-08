use std::{fmt, thread};
use std::sync::mpsc;

use crate::aoc_error::AocError;

// TODO: make the computer generic with respect to the register type.
type RegisterType = i32;
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

pub struct IntcodeComputerBus {
    rx: Option<mpsc::Receiver<RegisterType>>,
    sx: Option<mpsc::Sender<RegisterType>>,
}

impl IntcodeComputerBus {
    pub fn new() -> Self {
        IntcodeComputerBus { rx: None, sx: None }
    }

    pub fn get_input(&mut self) -> Result<mpsc::Sender<RegisterType>, IntcodeComputerError> {
        match std::mem::replace(&mut self.sx, None) {
            None => {
                if self.rx.is_none() {
                    let (sx, rx) = mpsc::channel();
                    self.rx = Some(rx);
                    Ok(sx)
                } else {
                    Err(IntcodeComputerError::new(String::from("Bus input already in use")))
                }
            },
            Some(sx) => {
                Ok(sx)
            }
        }
    }

    pub fn get_output(&mut self) -> Result<mpsc::Receiver<RegisterType>, IntcodeComputerError> {
        match std::mem::replace(&mut self.rx, None) {
            None => {
                if self.sx.is_none() {
                    let (sx, rx) = mpsc::channel();
                    self.sx = Some(sx);
                    Ok(rx)
                } else {
                    Err(IntcodeComputerError::new(String::from("Bus output already in use")))
                }
            },
            Some(rx) => {
                Ok(rx)
            }
        }
    }
}

pub struct IntcodeComputer<'a> {
    thread_handle: Option<thread::JoinHandle<Result<RegisterType, IntcodeComputerError>>>,
    input_bus: Option<&'a mut IntcodeComputerBus>,
    output_bus: Option<&'a mut IntcodeComputerBus>,
}

impl<'a> IntcodeComputer<'a> {
    pub fn new(input_bus: Option<&'a mut IntcodeComputerBus>, output_bus: Option<&'a mut IntcodeComputerBus>) -> Self {
        IntcodeComputer { thread_handle: None, input_bus, output_bus }
    }

    pub fn start(&mut self, program: Program) -> Result<(), IntcodeComputerError> {
        if self.thread_handle.is_some() {
            return Err(IntcodeComputerError::new(String::from("Computer already running")));
        };

        let input = match &mut self.input_bus {
            None => None,
            Some(bus) => Some(bus.get_output()?),
        };
        let output = match &mut self.output_bus {
            None => None,
            Some(bus) => Some(bus.get_input()?),
        };
        self.thread_handle = Some(thread::spawn(move|| {
            let mut hardware = IntcodeHardware::new();
            hardware.run(program, input, output)
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
    input: Option<mpsc::Receiver<RegisterType>>,
    output: Option<mpsc::Sender<RegisterType>>,
}

impl IntcodeHardware {
    pub fn new() -> Self {
        IntcodeHardware { memory: vec![99], ip: 0, input: None, output: None }
    }

    pub fn run(&mut self, program: Program, input: Option<mpsc::Receiver<RegisterType>>,
               output: Option<mpsc::Sender<RegisterType>>) -> Result<RegisterType, IntcodeComputerError> {
        self.input = input;
        self.output = output;
        self.memory = program;

        let mut instruction = self.parse_instruction()?;
        while instruction != Instruction::Halt {
            self.execute_instruction(&instruction)?;
            instruction = self.parse_instruction()?;
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
                let result_address = self.memory[self.ip + 3];
                Ok(Instruction::Add(first_parameter, second_parameter, result_address))
            },
            2 => {
                let first_parameter = self.parse_parameter(1, &parameter_modes)?;
                let second_parameter = self.parse_parameter(2, &parameter_modes)?;
                let result_address = self.memory[self.ip + 3];
                Ok(Instruction::Mul(first_parameter, second_parameter, result_address))
            },
            3 => {
                let result_address = self.memory[self.ip + 1];
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
                let result_address = self.memory[self.ip + 3];
                Ok(Instruction::Lst(first_parameter, second_parameter, result_address))
            },
            8 => {
                let first_parameter = self.parse_parameter(1, &parameter_modes)?;
                let second_parameter = self.parse_parameter(2, &parameter_modes)?;
                let result_address = self.memory[self.ip + 3];
                Ok(Instruction::Eqs(first_parameter, second_parameter, result_address))
            },
            unknown_opcode => Err(IntcodeComputerError::new(String::from(format!("Unknown opcode {}", unknown_opcode))))
        }
    }

    fn parse_parameter(&self, parameter_position: usize, parameter_modes: &Vec<char>) -> Result<Parameter, IntcodeComputerError> {
        match parameter_modes.get(parameter_position - 1).map(|x| char::to_digit(*x, 10)).flatten().unwrap_or(0) {
            0 => Ok(Parameter::PositionMode(self.memory[self.ip + parameter_position])),
            1 => Ok(Parameter::ImmediateMode(self.memory[self.ip + parameter_position])),
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
                self.ip += 4;
            },
            Instruction::Mul(parameter_a, parameter_b, address) => {
                let value = self.load_parameter(parameter_a) * self.load_parameter(parameter_b);
                self.store_value(value, address);
                self.ip += 4;
            },
            Instruction::Inp(address) => {
                let value = match &self.input {
                    Some(input) => input.recv().map_err(|mpsc_error| IntcodeComputerError::new(String::from("Could not read from the channel")))?,
                    None => return Err(IntcodeComputerError::new(String::from("Input not available"))),
                };
                self.store_value(value, address);
                self.ip += 2;
            },
            Instruction::Out(parameter) => {
                let value = self.load_parameter(parameter);
                match &self.output {
                    Some(output) => output.send(value).map_err(|mpsc_error| IntcodeComputerError::new(String::from("Could not send a value to the channel")))?,
                    None => return Err(IntcodeComputerError::new(String::from("Output not available"))),
                }
                self.ip += 2;
            },
            Instruction::Jit(parameter, address) => {
                let value = self.load_parameter(parameter);
                let jump_address = self.load_parameter(address);
                if value != 0 {
                    self.ip = jump_address as usize;
                } else {
                    self.ip += 3;
                }
            },
            Instruction::Jif(parameter, address) => {
                let value = self.load_parameter(parameter);
                let jump_address = self.load_parameter(address);
                if value == 0 {
                    self.ip = jump_address as usize;
                } else {
                    self.ip += 3;
                }
            },
            Instruction::Lst(parameter_a, parameter_b, address) => {
                let value = match  self.load_parameter(parameter_a) < self.load_parameter(parameter_b) {
                    true => 1,
                    false => 0,
                };
                self.store_value(value, address);
                self.ip += 4;
            },
            Instruction::Eqs(parameter_a, parameter_b, address) => {
                let value = match  self.load_parameter(parameter_a) == self.load_parameter(parameter_b) {
                    true => 1,
                    false => 0,
                };
                self.store_value(value, address);
                self.ip += 4;
            },
        };
        Ok(())
    }

    fn load_parameter(&self, parameter: &Parameter) -> RegisterType {
        match parameter {
            Parameter::ImmediateMode(value) => value.to_owned(),
            Parameter::PositionMode(address) => self.memory[*address as usize]
        }
    }

    fn store_value(&mut self, value: RegisterType, address: &Address) {
        self.memory[*address as usize] = value
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
