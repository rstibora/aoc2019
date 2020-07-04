use std::sync::{Arc, Barrier, mpsc};

use crate::aoc_error::{AocResult, AocError};
use super::intcode_computer::{IntcodeComputer, utils};

use permutohedron::heap_recursive;

// TODO: get rid of unwraps.
pub fn first_star(input: &str) -> AocResult {
    let input = input.lines().next().ok_or(AocError::new(String::from("Could not parse a line")))?;
    let program = utils::parse_intcode_program(input)?;

    let mut phase_setting = [0, 1, 2, 3, 4];
    let mut permutations = vec!();
    heap_recursive(&mut phase_setting, |permutation| { permutations.push(permutation.to_vec()) });

    let mut outputs = vec![0; permutations.len()];
    for permutation in permutations {

        let mut senders = vec![];
        let mut receivers = vec![];
        for _ in 0..5 {
            let (sx, rx) = mpsc::channel();
            senders.push(sx);
            receivers.push(rx);
        }

        // Send in the initial configuration and signal.
        for i in 0..5 {
            senders[i].send(permutation[i]).map_err(|_mpsc_error| AocError::new(String::from("Could not input configuration")))?;
        }
        senders[0].send(0).map_err(|_mpsc_error| AocError::new(String::from("Could not input the initial signal")))?;

        let sync_barrier = Arc::new(Barrier::new(6));
        let mut amp_computers = [
            IntcodeComputer::new(Some(sync_barrier.clone())), IntcodeComputer::new(Some(sync_barrier.clone())),
            IntcodeComputer::new(Some(sync_barrier.clone())), IntcodeComputer::new(Some(sync_barrier.clone())),
            IntcodeComputer::new(Some(sync_barrier.clone())),
        ];

        // Shift senders back by one, so that the sending computer is the one before the receiving.
        senders.rotate_left(1);

        let mut sx_rx: Vec<(mpsc::Sender<i64>, mpsc::Receiver<i64>)> = senders.drain(..).zip(receivers.drain(..)).rev().collect();
        for i in 0..4 {
            let (sender, receiver) = sx_rx.pop().unwrap();
            amp_computers[i].start(program.clone(), Some(receiver), vec![sender])?;
        }
        let (_, receiver) = sx_rx.pop().unwrap();
        let (output_sender, output_receiver) = mpsc::channel();
        amp_computers[4].start(program.clone(), Some(receiver), vec![output_sender])?;

        sync_barrier.wait();

        for i in 0..5 {
            amp_computers[i].wait_for_result()?;
        }
        outputs.push(output_receiver.recv().map_err(|_mpsc_error| AocError::new(String::from("Did not receive output")))?);
    }
    let maximum_output = outputs.iter().max().ok_or(AocError::new(String::from("Could not get maximum value")))?;
    Ok(maximum_output.to_string())
}

pub fn second_star(input: &str) -> AocResult {
    let input = input.lines().next().ok_or(AocError::new(String::from("Could not parse a line")))?;
    let program = utils::parse_intcode_program(input)?;

    let mut phase_setting = [5, 6, 7, 8, 9];
    let mut permutations = vec!();
    heap_recursive(&mut phase_setting, |permutation| { permutations.push(permutation.to_vec()) });

    let mut outputs = vec![0; permutations.len()];
    for permutation in permutations {

        let mut senders = vec![];
        let mut receivers = vec![];
        for _ in 0..5 {
            let (sx, rx) = mpsc::channel();
            senders.push(sx);
            receivers.push(rx);
        }

        // Send in the initial configuration and signal.
        for i in 0..5 {
            senders[i].send(permutation[i]).map_err(|_mpsc_error| AocError::new(String::from("Could not input configuration")))?;
        }
        senders[0].send(0).map_err(|_mpsc_error| AocError::new(String::from("Could not input the initial signal")))?;

        let sync_barrier = Arc::new(Barrier::new(6));
        let mut amp_computers = [
            IntcodeComputer::new(Some(sync_barrier.clone())), IntcodeComputer::new(Some(sync_barrier.clone())),
            IntcodeComputer::new(Some(sync_barrier.clone())), IntcodeComputer::new(Some(sync_barrier.clone())),
            IntcodeComputer::new(Some(sync_barrier.clone())),
        ];

        // Shift senders back by one, so that the sending computer is the one before the receiving.
        senders.rotate_left(1);

        let mut sx_rx: Vec<(mpsc::Sender<i64>, mpsc::Receiver<i64>)> = senders.drain(..).zip(receivers.drain(..)).rev().collect();
        for i in 0..4 {
            let (sender, receiver) = sx_rx.pop().unwrap();
            amp_computers[i].start(program.clone(), Some(receiver), vec![sender])?;
        }
        let (sender, receiver) = sx_rx.pop().unwrap();
        let (output_sender, output_receiver) = mpsc::channel();
        amp_computers[4].start(program.clone(), Some(receiver), vec![sender, output_sender])?;

        sync_barrier.wait();

        for i in 0..5 {
            amp_computers[i].wait_for_result()?;
        }

        let result = output_receiver.iter().last();
        outputs.push(result.ok_or(AocError::new(String::from("Did not get output")))?);
    }
    let maximum_output = outputs.iter().max().ok_or(AocError::new(String::from("Could not get maximum value")))?;
    Ok(maximum_output.to_string())
}
