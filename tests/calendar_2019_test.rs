use std::{io, fs, ffi::OsStr, path::PathBuf};

use aoc_framework::year2019::{calendar::Calendar2019, configuration};
use aoc_framework::utils::file_handling;

use aoc_framework::interface::AdventOfCodeCalendar;

fn run_test_for_day(day: u32) {
    let calendar = Calendar2019 {};
    let inputs_with_results = get_test_inputs_with_results_for_day(day, configuration::get_inputs_folder_path()).unwrap();

    for input_with_result in inputs_with_results {
        if let Some(solution) = input_with_result.first_star_solution {
            assert_eq!(solution, calendar.run_day(day, Some(&input_with_result.input)).0.unwrap());
        }

        if let Some(solution) = input_with_result.second_star_solution {
            assert_eq!(solution, calendar.run_day(day, Some(&input_with_result.input)).1.unwrap());
        }
    }
}

pub struct InputWithResult {
    pub input: String,
    pub first_star_solution: Option<String>,
    pub second_star_solution: Option<String>,
}

fn get_test_inputs_with_results_for_day(day_number: u32, folder: &str) -> Result<Vec<InputWithResult>, io::Error> {
    let mut inputs = Vec::new();
    for mut file_path in list_test_input_file_names(day_number, folder)? {
        file_path.set_extension(file_handling::get_input_extension());
        let input = fs::read_to_string(&file_path)?;

        file_path.set_extension(get_test_result_suffix());
        let results = fs::read_to_string(&file_path)?;

        let mut results = results.lines().map(str::to_string);
        // Allow to have solutions only for the second star.
        let first_star_solution = match results.next() {
            Some(empty) if empty == String::from("") => None,
            None => None,
            Some(non_empty) => Some(non_empty),
        };
        let second_star_solution = results.next();

        inputs.push(InputWithResult { input, first_star_solution, second_star_solution });
    }
    Ok(inputs)
}

fn list_test_input_file_names(day_number: u32, folder: &str) -> Result<Vec<PathBuf>, io::Error> {
    let mut filenames = Vec::new();

    for item in fs::read_dir(folder)? {
        let path = item?.path();

        // Check that the current item is a file with the proper name and suffix (expected result file).
        let is_correct_day = path.file_name().and_then(OsStr::to_str).map(|x| {x.contains(&file_handling::get_input_filename(day_number, None))}).unwrap_or(false);
        let has_correct_extension = path.extension().and_then(OsStr::to_str).map(|x| { x == get_test_result_suffix() }).unwrap_or(false);
        if !is_correct_day || !has_correct_extension {
            continue;
        }

        let mut input_path = path.clone();
        input_path.set_extension("");
        filenames.push(input_path);
    }
    Ok(filenames)
}

pub fn get_test_result_suffix() -> String {
    String::from("result")
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

#[test]
fn day_six() {
    run_test_for_day(6);
}

#[test]
fn day_seven() {
    run_test_for_day(7);
}

#[test]
fn day_eight() {
    run_test_for_day(8);
}
