use std::collections::HashMap;
use std::sync::mpsc;

use crate::aoc_error::{AocError, AocResult};
use super::intcode_computer::{IntcodeComputer, utils};

const TILE_BLOCK: i64 = 2;


pub fn first_star(input: &str) -> AocResult {
    let program = utils::parse_intcode_program(input)?;
    let mut cabinet = IntcodeComputer::new(None);

    let mut tiles = HashMap::new();

    let (output_sender, output_receiver) = mpsc::channel();

    cabinet.start(program, None, vec![output_sender])?;


    loop {
        let x_pos = match output_receiver.recv() {
            Ok(x_pos) => x_pos,
            Err(_) => break
        };
        let y_pos = match output_receiver.recv() {
            Ok(y_pos) => y_pos,
            Err(err) => return Err(AocError::new(format!("Error reading y coordinate: {}", err)))
        };
        let tile_id = match output_receiver.recv() {
            Ok(tile_id) => tile_id,
            Err(err) => return Err(AocError::new(format!("Error reading tile id: {}", err)))
        };
        tiles.insert((x_pos, y_pos), tile_id);
    }

    Ok(tiles.values().filter(|&&tile| {tile == TILE_BLOCK}).into_iter().count().to_string())
}