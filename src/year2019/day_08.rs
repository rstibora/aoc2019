use crate::aoc_error::{AocError, AocResult};
use crate::utils::input_conversion;

const IMAGE_SIZE: (usize, usize) = (25, 6);

pub fn first_star(input: &str) -> AocResult {
    let input: Vec<String> = input_conversion::input_to_lines(input).map_err(|_err| AocError::new(String::from("Could not get input lines")))?;
    let input_image = input[0].chars().map(|ch| ch.to_digit(10)
                        .ok_or_else(|| AocError::new(String::from("Could not convert input to u32 image"))))
                        .collect::<Result<Vec<u32>, AocError>>()?;
    let mut processed_layers = input_image.chunks_exact(IMAGE_SIZE.0 * IMAGE_SIZE.1).map(analyze_layer).collect::<Vec<(u32, u32)>>();
    processed_layers.sort_unstable_by(|a, b| a.cmp(b));
    Ok(processed_layers[0].1.to_string())
}

pub fn second_star(input: &str) -> AocResult {
    let input: Vec<String> = input_conversion::input_to_lines(input).map_err(|_err| AocError::new(String::from("Could not get input lines")))?;
    let input_image = input[0].chars().map(|ch| ch.to_digit(10)
                        .ok_or_else(|| AocError::new(String::from("Could not convert input to u32 image"))))
                        .collect::<Result<Vec<u32>, AocError>>()?;
    let mut output_image: Vec<u32> = vec![2; IMAGE_SIZE.0 * IMAGE_SIZE.1];
    for chunk in input_image.chunks_exact(IMAGE_SIZE.0 * IMAGE_SIZE.1) {
        for (layer_pixel, composite_pixel) in chunk.iter().zip(output_image.iter_mut()) {
            if *composite_pixel == 2 {
                *composite_pixel = *layer_pixel;
            }
        }
    }

    let mut printable_output_image = String::new();
    for (idx, output_pixel) in output_image.iter().enumerate() {
        if idx % IMAGE_SIZE.0 == 0 {
            printable_output_image.push('\n');
        }
        let printable_pixel = match *output_pixel {
            0 => '⬛',
            1 => '⬜',
            _ => return Err(AocError::new(String::from("Unexpected signal in the image"))),
        };
        printable_output_image.push(printable_pixel);
    }
    Ok(printable_output_image)
}

/// Returns (number of zeros, number of ones * number of twos) for the given image layer
///
/// # Arguments
/// - `layer` Image layer to processed.
///
fn analyze_layer(layer: &[u32]) -> (u32, u32) {
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
