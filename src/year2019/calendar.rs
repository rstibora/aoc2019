use crate::interface::AdventOfCodeCalendar;
use crate::utils::file_handling;
use crate::aoc_error::AocError;

use super::configuration;
use super::{day_01, day_02, day_03, day_04, day_05, day_06, day_07, day_08, day_09, day_10,
            day_11, day_12, day_13};

pub struct Calendar2019;

impl Calendar2019 {
    pub fn new() -> Calendar2019 {
        Calendar2019 {}
    }
}

impl Default for Calendar2019 {
    fn default() -> Self {
        Calendar2019::new()
    }
}

impl AdventOfCodeCalendar for Calendar2019 {
    fn run_day(&self, day_number: u32, input: Option<&str>) -> (Result<String, AocError>, Result<String, AocError>) {
        let loaded_input: String;
        let input = match input {
            Some(input) => input,
            None => {
                match file_handling::get_input_for_day(day_number, None, configuration::get_inputs_folder_path()) {
                    Ok(input) => {
                        loaded_input = input;
                        &loaded_input
                    }
                    Err(error) => {
                        let message = format!("IO error: {}", error.to_string());
                        return (Err(AocError::new(message.clone())), Err(AocError::new(message)))
                    }
                }
            }
        };

        match day_number {
            1 => (day_01::first_star(input), day_01::second_star(input)),
            2 => (day_02::first_star(input), day_02::second_star(input)),
            3 => (day_03::first_star(input), day_03::second_star(input)),
            4 => (day_04::first_star(input), day_04::second_star(input)),
            5 => (day_05::first_star(input), day_05::second_star(input)),
            6 => (day_06::first_star(input), day_06::second_star(input)),
            7 => (day_07::first_star(input), day_07::second_star(input)),
            8 => (day_08::first_star(input), day_08::second_star(input)),
            9 => (day_09::first_star(input), day_09::second_star(input)),
            10 => (day_10::first_star(input), day_10::second_star(input)),
            11 => (day_11::first_star(input), day_11::second_star(input)),
            12 => (day_12::first_star(input), day_12::second_star(input)),
            13 => (day_13::first_star(input), Err(AocError::new("Not implemented".to_string()))),
            _ => {
                let message = format!("Day {} not implemented", day_number);
                (Err(AocError::new(message.clone())), Err(AocError::new(message)))
            },
        }
    }
}
