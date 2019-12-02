use std::fs::File;
use std::io::{BufRead, BufReader};
use std::cmp::max;

fn main() {
    let filename = "inputs/day_01.txt";
    let file = File::open(&filename).unwrap();
    let reader = BufReader::new(&file);

    let mut total_fuel_for_mass = 0;
    let mut total_fuel_for_fuel_and_mass = 0;
    for line in reader.lines() {
        let module_mass = line.expect("Not a number encountered!").parse::<i32>().unwrap();
        total_fuel_for_mass += fuel_for_mass(module_mass);
        total_fuel_for_fuel_and_mass += fuel_for_mass_recursive(module_mass);
    }

    println!("Star 1 solution: {}", total_fuel_for_mass);
    println!("Star 2 solution: {}", total_fuel_for_fuel_and_mass);
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
