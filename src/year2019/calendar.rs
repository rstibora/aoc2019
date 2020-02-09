use crate::interface::AdventOfCodeCalendar;
use crate::utils::file_handling;
use crate::aoc_error::AocError;

use super::configuration;
use super::{day_01, day_02, day_03, day_04, day_05, day_06};


pub struct Calendar2019;

impl Calendar2019 {
    pub fn new() -> Calendar2019 {
        Calendar2019 {}
    }
}

impl AdventOfCodeCalendar for Calendar2019 {
    fn run_day(&self, day_number: u32) -> (Result<String, AocError>, Result<String, AocError>) {
        let input = match file_handling::get_input_for_day(day_number, configuration::get_inputs_folder_path()) {
            Ok(input) => input,
            Err(error) => {
                let message = format!("IO error: {}", error.to_string());
                return (Err(AocError::new(message.clone())), Err(AocError::new(message)))
            }
        };

        match day_number {
            1 => (day_01::first_star(&input), day_01::second_star(&input)),
            2 => (day_02::first_star(&input), day_02::second_star(&input)),
            3 => (day_03::first_star(&input), day_03::second_star(&input)),
            4 => (day_04::first_star(&input), day_04::second_star(&input)),
            5 => (day_05::first_star(&input), day_05::second_star(&input)),
            6 => (day_06::first_star(&input), Err(AocError::new(String::from("Not implemented")))),
            _ => {
                let message = format!("Day {} not implemented", day_number);
                return (Err(AocError::new(message.clone())), Err(AocError::new(message)));
            },
        }
    }
}
