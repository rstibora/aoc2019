pub fn convert_input_to_numbers(input: &Vec<String>) -> Vec<i32> {
    let mut result = Vec::new();
    for line in input {
        let number: i32 = line.parse().unwrap();
        result.push(number);
    }
    result
}