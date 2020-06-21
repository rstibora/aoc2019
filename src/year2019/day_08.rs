use crate::aoc_error::{AocError, AocResult};
use crate::utils::input_conversion;

const IMAGE_SIZE: (usize, usize) = (25, 6);

pub fn first_star(input: &str) -> AocResult {
    let input: Vec<String> = input_conversion::input_to_lines(input).map_err(|_err| AocError::new(String::from("Could not get input lines")))?;
    let input_image = input[0].chars().map(|ch| ch.to_digit(10)
                        .ok_or(AocError::new(String::from("Could not convert input to u32 image"))))
                        .collect::<Result<Vec<u32>, AocError>>()?;
    let mut processed_layers = input_image.chunks_exact(IMAGE_SIZE.0 * IMAGE_SIZE.1).map(process_layer).collect::<Vec<(u32, u32)>>();
    processed_layers.sort_unstable_by(|a, b| a.cmp(b));
    Ok(processed_layers[0].1.to_string())
}

/// Return (number of zeros, number of ones * number of twos) for the given image layer
///
/// # Arguments
/// - `layer` Image layer to processed.
///
fn process_layer(layer: &[u32]) -> (u32, u32) {
    let mut zeros_count = 0;
    let mut ones_count = 0;
    let mut twos_count = 0;

    for pixel in layer {
        match pixel {
            0 => zeros_count += 1,
            1 => ones_count += 1,
            2 => twos_count += 1,
            _ => (),
        };
    }
    (zeros_count, ones_count * twos_count)
}
