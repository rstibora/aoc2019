
const NUMBER_LENGHT: usize = 6;
type Number = [u32; NUMBER_LENGHT];

pub fn first_star(input: &Vec<String>) -> String {
    let lower_bound: String = input[0].chars().take(6).collect();
    let upper_bound: String = input[0].chars().skip(7).take(6).collect();

    let lower_bound = to_u32(&parse_number(&lower_bound[..]));
    let upper_bound = to_u32(&parse_number(&upper_bound[..]));

    let mut possible_numbers: Vec<u32> = Vec::new();
    let mut number = [0, 0, 0, 0, 0, 0];
    while to_u32(&number) <= upper_bound {
        if check_has_double(&number) && to_u32(&number) >= lower_bound {
            possible_numbers.push(to_u32(&number));
        }
        number = increment(&number, NUMBER_LENGHT - 1);
    }
    possible_numbers.len().to_string()
}

pub fn second_star(input: &Vec<String>) -> String {
    let lower_bound: String = input[0].chars().take(6).collect();
    let upper_bound: String = input[0].chars().skip(7).take(6).collect();

    let lower_bound = to_u32(&parse_number(&lower_bound[..]));
    let upper_bound = to_u32(&parse_number(&upper_bound[..]));

    let mut possible_numbers: Vec<u32> = Vec::new();
    let mut number = [0, 0, 0, 0, 0, 0];
    while to_u32(&number) <= upper_bound {
        if check_has_isolated_double(&number) && to_u32(&number) >= lower_bound {
            possible_numbers.push(to_u32(&number));
        }
        number = increment(&number, NUMBER_LENGHT - 1);
    }
    possible_numbers.len().to_string()
}

fn parse_number(string_number: &str) -> Number {
    let mut number = [0, 0, 0, 0, 0, 0];
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
