mod aoc_error;
mod interface;
mod utils;
mod year2019;

use interface::AdventOfCodeCalendar;

fn main() {
    // TODO: get rid of unwrap and expect calls (do proper error handling).
    // TODO: pass proper slice types instead of Vectors.
    let day = 11;
    let calendar = year2019::calendar::Calendar2019::new();
    let (first_star, second_star) = calendar.run_day(day, None);
    let first_star = first_star.unwrap_or_else(|error| error.to_string());
    let second_star = second_star.unwrap_or_else(|error| error.to_string());
    println!("Day {}: first star result: {}, second star result: {}", day, first_star, second_star);
}