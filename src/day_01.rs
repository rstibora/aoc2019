use std::cmp::max;

use super::utils;

pub fn first_star(input: &Vec<String>) -> String {
    let input = utils::convert_input_to_numbers(input);
    let mut total_fuel_for_mass = 0;

    for item in input {
        total_fuel_for_mass += fuel_for_mass(item);
    }
    total_fuel_for_mass.to_string()
}

pub fn second_star(input: &Vec<String>) -> String {
    let input = utils::convert_input_to_numbers(input);
    let mut total_fuel_for_mass = 0;

    for item in input {
        total_fuel_for_mass += fuel_for_mass_recursive(item);
    }
    total_fuel_for_mass.to_string()
}

fn fuel_for_mass_recursive(mass: i32) -> i32 {
    let fuel_needed = fuel_for_mass(mass);
    if fuel_needed == 0 {
        return 0;
    }
    fuel_needed + fuel_for_mass_recursive(fuel_needed)
}

fn fuel_for_mass(mass: i32) -> i32 {
    max((mass as f64 / 3.) as i32 - 2, 0)
}
