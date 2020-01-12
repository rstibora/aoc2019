use super::aoc_error::AocError;

pub trait AdventOfCodeCalendar {
    fn run_day(&self, day: u32) -> (Result<String, AocError>, Result<String, AocError>);
}
