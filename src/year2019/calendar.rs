use crate::interface::AdventOfCodeCalendar;
use crate::utils::file_handling;
use crate::aoc_error::AocError;

use super::day_01;

const INPUT_FOLDER: &str = "inputs";

pub struct Calendar2019;

impl Calendar2019 {
    pub fn new() -> Calendar2019 {
        Calendar2019 {}
    }
}

impl AdventOfCodeCalendar for Calendar2019 {
    fn run_day(&self, day_number: u32) -> (Result<String, AocError>, Result<String, AocError>) {
        let input = match file_handling::get_input_for_day(day_number, INPUT_FOLDER) {
            Ok(input) => input,
            Err(error) => {
                let message = format!("IO error: {}", error.to_string());
                return (Err(AocError::new(&message)), Err(AocError::new(&message)))
            }
        };

        match day_number {
            1 => (day_01::first_star(&input) , day_01::second_star(&input)),
            _ => {
                let message = format!("Day {} not implemented", day_number);
                return (Err(AocError::new(&message)), Err(AocError::new(&message)));
            },
        }
    }
}