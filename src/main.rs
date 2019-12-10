use std::fs::File;
use std::io::{BufRead, BufReader};

mod utils;
mod day_01;

fn main() {
    run_day(String::from("01"));
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

fn run_day(day: String) {
    let filename = format!("inputs/day_{}.txt", day);
    let input = parse_input(&filename);

    println!("Day {}: first star solution: {} second star solution {}", day, day_01::first_star(&input), day_01::second_star(&input));
}