use std::fs::File;
use std::io::{BufRead, BufReader};

mod utils;
mod day_01;
mod day_02;
mod day_03;

fn main() {
    run_day(String::from("03"), false);
}

fn parse_input(filename: &String) -> Vec<String> {
    let file = File::open(&filename).unwrap();
    let reader = BufReader::new(&file);

    let mut result = Vec::new();
    for line in reader.lines() {
        result.push(line.unwrap());
    }
    result
}

fn run_day(day: String, test_input:bool) {
    let mut test_suffix = "";
    if test_input {
        test_suffix = "_test";
    }
    let filename = format!("inputs/day_{}{}.txt", day, test_suffix);
    let input = parse_input(&filename);

    let (first_star_solution, second_star_solution) = match &day[..] {
        "01" => (day_01::first_star(&input), day_01::second_star(&input)),
        "02" => (day_02::first_star(&input), day_02::second_star(&input)),
        "03" => (day_03::first_star(&input), day_03::second_star(&input)),
        _ => (String::from("N/A"), String::from("N/A")),
    };

    println!("Day {}: first star solution: {}, second star solution {}", day, first_star_solution, second_star_solution);
}