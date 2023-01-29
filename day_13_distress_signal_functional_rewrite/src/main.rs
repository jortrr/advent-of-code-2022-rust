//! Advent of Code 2022 day x
use std::{
    fs,
    io::{self, BufRead},
};

use structs::{packet::Packet, signal::Signal};
mod enums;
mod structs;

// File paths
static INPUT_PATH: &str = "puzzle/INPUT";
static EXAMPLE_INPUT_PATH: &str = "puzzle/EXAMPLE_INPUT";
static ANSWER_PART_ONE_PATH: &str = "puzzle/ANSWER_PART_ONE";
static ANSWER_PART_TWO_PATH: &str = "puzzle/ANSWER_PART_TWO";

//Program main
fn main() {
    //Create our main Advent of Code puzzle data structure from the INPUT file
    let mut signal: Signal = parse(INPUT_PATH).unwrap();

    //Print the data structure that was created by parse()
    signal.print();

    //Solve part one of the Advent of Code puzzle
    let answer_part_one: usize = solve_part_one(&mut signal);
    fs::write(ANSWER_PART_ONE_PATH, format!("{}", answer_part_one)).unwrap();

    //Solve part two of the Advent of Code puzzle
    let answer_part_two = solve_part_two(&mut signal);
    fs::write(ANSWER_PART_TWO_PATH, format!("{}", answer_part_two)).unwrap();
}

///Parse the INPUT file at the relative `input_file_path` into our main data structure, `Signal`
fn parse(input_file_path: &str) -> Result<Signal, std::io::Error> {
    let input_file: fs::File = fs::File::open(input_file_path)?;
    let reader: io::BufReader<fs::File> = io::BufReader::new(input_file);

    //-Remove leading and trailing new-line characters
    //-Filter out empty lines
    //-Convert the packet_lines into Packets
    let packets: Vec<Packet> = reader
        .lines()
        .map(|l| l.unwrap().trim().to_string())
        .filter(|l| !l.is_empty())
        .map(|l| Packet::from(&l))
        .collect();

    let signal: Signal = Signal::new(packets);

    Ok(signal)
}

///Solve part one of the Advent of Code 2022 puzzle, returns the puzzle answer
fn solve_part_one(signal: &mut Signal) -> usize {
    println!("=== solve_part_one(signal) ===");
    let sum: usize = signal.compute_sum_of_ordered_pair_indices();
    println!("Sum of the indices of ordered pairs: {}", sum);
    sum
}

///Solve part two of the Advent of Code 2022 puzzle, returns the puzzle answer
fn solve_part_two(signal: &mut Signal) -> usize {
    println!("=== solve_part_two(signal) ===");
    let decoder_key: usize = signal.compute_decoder_key();
    println!("Decoder key: {}", decoder_key);
    decoder_key
}
