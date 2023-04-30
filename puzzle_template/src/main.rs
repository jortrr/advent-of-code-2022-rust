//! Advent of Code 2022 day x
use std::{
    env, fs,
    io::{self, BufRead},
};

mod structs;

// File paths
static INPUT_PATH: &str = "puzzle/INPUT";
static ANSWER_PART_ONE_PATH: &str = "puzzle/ANSWER_PART_ONE";
static ANSWER_PART_TWO_PATH: &str = "puzzle/ANSWER_PART_TWO";

//Program main
fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    //Create our main Advent of Code puzzle data structure from the INPUT file
    let mut main_struct: MainStruct = parse(INPUT_PATH).unwrap();
    /*if !height_map.is_valid() {
        panic!(
            "The HeightMap parsed from {} is in an invalid state, this should never happen.",
            INPUT_PATH
        );
    }*/

    //Print the data structure that was created by parse()
    main_struct.print();

    //Solve part one of the Advent of Code puzzle
    let answer_part_one = solve_part_one(&mut main_struct);
    fs::write(ANSWER_PART_ONE_PATH, format!("{}", answer_part_one)).unwrap();

    //Solve part two of the Advent of Code puzzle
    let answer_part_two = solve_part_two(&mut main_struct);
    fs::write(ANSWER_PART_TWO_PATH, format!("{}", answer_part_two)).unwrap();
}

//Replace this struct
struct MainStruct {}

///Parse the INPUT file at the relative input_file_path into our main data structure, HeightMap
fn parse(input_file_path: &str) -> Result<MainStruct, std::io::Error> {
    Ok(MainStruct {})
}

///Solve part one of the Advent of Code 2022 puzzle, returns the puzzle answer
fn solve_part_one(main_struct: &mut MainStruct) -> u16 {
    let input_file: fs::File = fs::File::open(input_file_path)?;
    let reader: io::BufReader<fs::File> = io::BufReader::new(input_file);

    0
}

///Solve part two of the Advent of Code 2022 puzzle, returns the puzzle answer
fn solve_part_two(main_struct: &mut MainStruct) -> u16 {
    let input_file: fs::File = fs::File::open(input_file_path)?;
    let reader: io::BufReader<fs::File> = io::BufReader::new(input_file);
    
    0
}
