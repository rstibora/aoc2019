use std::cmp::max;

use crate::aoc_error::{AocError, AocResult};
use crate::utils::{input_conversion, performance::Cached};

pub fn first_star(input: &str) -> AocResult {
    // TODO: ugly, fix.
    let input = match input_conversion::input_to_lines(input) {
        Ok(input) => input,
        Err(_) => return Err(AocError::new(String::from("Could not convert to lines")))
    };
    let mut cached_fuel_for_mass = Cached::new(fuel_for_mass);
    let total_fuel_for_mass: i32 = input.into_iter()
                                    .map(|mass| cached_fuel_for_mass.calculate(mass).clone())
                                    .sum();
    Ok(total_fuel_for_mass.to_string())
}

pub fn second_star(input: &str) -> AocResult {
    // TODO: ugly, fix.
    let input = match input_conversion::input_to_lines(input) {
        Ok(input) => input,
        Err(_) => return Err(AocError::new(String::from("Could not convert to lines")))
    };
    let mut cached_fuel_for_mass_recursive = Cached::new(fuel_for_mass_recursive);
    let total_fuel_for_mass: i32 = input.into_iter()
                                    .map(|mass| cached_fuel_for_mass_recursive.calculate(mass).clone())
                                    .sum();
    Ok(total_fuel_for_mass.to_string())
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fuel_for_mass() {
        assert_eq!(fuel_for_mass(0), 0);
        assert_eq!(fuel_for_mass(100), 31);
    }

    #[test]
    fn test_fuel_for_mass_recursive() {
        assert_eq!(fuel_for_mass_recursive(0), 0);
        assert_eq!(fuel_for_mass_recursive(100), 39);
    }
}
