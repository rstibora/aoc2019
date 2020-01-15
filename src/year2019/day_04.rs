
use crate::aoc_error::{AocError, AocResult};

const NUMBER_LENGHT: usize = 6;
type Number = [u32; NUMBER_LENGHT];

pub fn first_star(input: &str) -> AocResult {
    let first_line = input.lines().next().ok_or(
        AocError::new(String::from("Not enough lines in the input")))?;

    let lower_bound: String = first_line.chars().take(6).collect();
    let upper_bound: String = first_line.chars().skip(7).take(6).collect();

    let lower_bound = to_u32(&parse_number(&lower_bound[..]));
    let upper_bound = to_u32(&parse_number(&upper_bound[..]));

    Ok(generate_numbers_with_check_function(&lower_bound, &upper_bound, check_has_double).len().to_string())
}

pub fn second_star(input: &str) -> AocResult {
    let first_line = input.lines().next().ok_or(
        AocError::new(String::from("Not enough lines in the input")))?;

    let lower_bound: String = first_line.chars().take(6).collect();
    let upper_bound: String = first_line.chars().skip(7).take(6).collect();

    let lower_bound = to_u32(&parse_number(&lower_bound[..]));
    let upper_bound = to_u32(&parse_number(&upper_bound[..]));

    Ok(generate_numbers_with_check_function(&lower_bound, &upper_bound, check_has_isolated_double).len().to_string())
}

fn generate_numbers_with_check_function(lower_bound: &u32, upper_bound: &u32, check_function: impl Fn(&Number) -> bool) -> Vec<Number> {
    let mut possible_numbers: Vec<Number> = Vec::new();
    let mut number = [0; 6];
    while &to_u32(&number) <= upper_bound {
        if check_function(&number) && &to_u32(&number) >= lower_bound {
            possible_numbers.push(number);
        }
        number = increment(&number, NUMBER_LENGHT - 1);
    }
    possible_numbers
}

fn parse_number(string_number: &str) -> Number {
    let mut number = [0; 6];
    for (idx, string_char) in string_number.chars().enumerate() {
        number[idx] = string_char.to_digit(10).unwrap();
    }
    number
}

fn check_has_double(number: &Number) -> bool {
    for i in 1..NUMBER_LENGHT {
        if number[i-1] == number[i] {
            return true;
        }
    }
    false
}

fn check_has_isolated_double(number: &Number) -> bool {
    for i in 1..NUMBER_LENGHT {
        if number[i-1] == number[i] {
            match i {
                1 => {
                    if number[i+1] != number[i] {
                        return true;
                    }
                },
                _ if i == NUMBER_LENGHT - 1 => {
                    if number[i-2] != number[i-1] {
                        return true;
                    }
                },
                _ => {
                    if number[i-2] != number[i-1] && number[i+1] != number[i] {
                        return true;
                    }
                }
            }
        }
    }
    false
}

fn increment(number: &Number, index: usize) -> Number {
    let mut number = number.clone();
    if number[index] == 9 {
        if index > 0 {
            number = increment(&number, index - 1);
            number[index] = number[index - 1];
        } else {
            number[index] = number[index] + 1;
        }
    } else {
        number[index] = number[index] + 1;
    }
    number
}

fn to_u32(number: &Number) -> u32 {
    number[5] + number[4] * 10 + number[3] * 100 + number[2] * 1000 + number[1] * 10000 + number[0] * 100000
}
