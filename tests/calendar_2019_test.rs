use aoc_framework::year2019::{calendar::Calendar2019, configuration};
use aoc_framework::utils::file_handling;

use aoc_framework::interface::AdventOfCodeCalendar;

fn run_test_for_day(day: u32) {
    let calendar = Calendar2019 {};
    let inputs_with_results = file_handling::get_test_inputs_with_results_for_day(day, configuration::get_inputs_folder_path()).unwrap();

    // TODO: allow to actually run non-default inputs (currently the only input being tested is the one of the original puzzle).
    for input_with_result in inputs_with_results {
        if let Some(solution) = input_with_result.first_star_solution {
            assert_eq!(solution, calendar.run_day(day).0.unwrap());
        }

        if let Some(solution) = input_with_result.second_star_solution {
            assert_eq!(solution, calendar.run_day(day).1.unwrap());
        }
    }
}

#[test]
fn day_one() {
    run_test_for_day(1);
}

#[test]
fn day_two() {
    run_test_for_day(2);
}

#[test]
fn day_three() {
    run_test_for_day(3);
}

#[test]
fn day_four() {
    run_test_for_day(4);
}

#[test]
fn day_five() {
    run_test_for_day(5);
}