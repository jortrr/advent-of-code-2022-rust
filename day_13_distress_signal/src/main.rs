//! Advent of Code 2022 day x
use std::{
    env, fs,
    io::{self, BufRead},
};

use structs::{packet::Packet, packet_pair::PacketPair, signal::Signal};

use crate::structs::packet::PacketData;

mod structs;

// File paths
static INPUT_PATH: &str = "src/puzzle/INPUT";
static EXAMPLE_INPUT_PATH: &str = "src/puzzle/EXAMPLE_INPUT";
static ANSWER_PART_ONE_PATH: &str = "src/puzzle/ANSWER_PART_ONE";
static ANSWER_PART_TWO_PATH: &str = "src/puzzle/ANSWER_PART_TWO";

//Program main
fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    //let list: &str = "[[4,[2],4],4,4]";
    //let packet_data = PacketData::parse_list(list).unwrap();
    //println!("packet_data: {:?}", packet_data);

    //Create our main Advent of Code puzzle data structure from the INPUT file
    let mut signal: Signal = parse(EXAMPLE_INPUT_PATH).unwrap();
    /*if !height_map.is_valid() {
        panic!(
            "The HeightMap parsed from {} is in an invalid state, this should never happen.",
            INPUT_PATH
        );
    }*/

    //Print the data structure that was created by parse()
    signal.print();

    //Solve part one of the Advent of Code puzzle
    let answer_part_one = solve_part_one(&mut signal);
    fs::write(ANSWER_PART_ONE_PATH, format!("{}", answer_part_one)).unwrap();

    //Solve part two of the Advent of Code puzzle
    let answer_part_two = solve_part_two(&mut signal);
    fs::write(ANSWER_PART_TWO_PATH, format!("{}", answer_part_two)).unwrap();
}

//Replace this struct
struct MainStruct {}

///Parse the INPUT file at the relative input_file_path into our main data structure, HeightMap
fn parse(input_file_path: &str) -> Result<Signal, std::io::Error> {
    let input_file: fs::File = fs::File::open(input_file_path)?;
    let reader: io::BufReader<fs::File> = io::BufReader::new(input_file);
    let mut signal: Signal = Signal::new();
    let mut packet_lines: Vec<String> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        //Remove leading trailing new-line characters
        let line: String = line?.trim().to_string();
        if line.is_empty() {
            continue;
        }
        packet_lines.push(line);
        if packet_lines.len() == 2 {
            let left_packet: Packet = Packet::from(&packet_lines.get(0).unwrap());
            let right_packet: Packet = Packet::from(&packet_lines.get(1).unwrap());
            signal.add_packet_pair(PacketPair::new(left_packet, right_packet));
            packet_lines.clear();
        }
    }

    Ok(signal)
}

///Solve part one of the Advent of Code 2022 puzzle, returns the puzzle answer
fn solve_part_one(signal: &mut Signal) -> u16 {
    0
}

///Solve part two of the Advent of Code 2022 puzzle, returns the puzzle answer
fn solve_part_two(sigal: &mut Signal) -> u16 {
    0
}
