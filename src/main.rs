use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let filename = "inputs/day_01.txt";
    let file = File::open(&filename).unwrap();
    let reader = BufReader::new(&file);

    let mut total_module_mass = 0;
    for line in reader.lines() {
        let module_mass = line.expect("Not a number encountered!").parse::<f64>().unwrap();
        let module_mass = ((module_mass / 3.) as i32) - 2;
        total_module_mass += module_mass
    }

    println!("Start 1 solution: {}", total_module_mass)
}
