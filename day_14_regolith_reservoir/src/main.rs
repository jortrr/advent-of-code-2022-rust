//! Advent of Code 2022 day 14
use std::{
    env, fs,
    io::{self, BufRead}, thread,
};

use regex::Regex;

use structs::cave::Cave;
mod structs;

// File paths
static INPUT_PATH: &str = "puzzle/INPUT";
static EXAMPLE_INPUT_PATH: &str = "puzzle/EXAMPLE_INPUT";
static ANSWER_PART_ONE_PATH: &str = "puzzle/ANSWER_PART_ONE";
static ANSWER_PART_TWO_PATH: &str = "puzzle/ANSWER_PART_TWO";

//Program main
fn main() {
    //Create our main Advent of Code puzzle data structure from the INPUT file
    let mut cave: Cave = parse(EXAMPLE_INPUT_PATH).unwrap();

    //Print the data structure that was created by parse()
    cave.print();

    //Solve part one of the Advent of Code puzzle
    let answer_part_one = solve_part_one(&mut cave);
    fs::write(ANSWER_PART_ONE_PATH, format!("{}", answer_part_one)).unwrap();

    //Solve part two of the Advent of Code puzzle
    let answer_part_two = solve_part_two(&mut cave);
    fs::write(ANSWER_PART_TWO_PATH, format!("{}", answer_part_two)).unwrap();
}

///Parse the INPUT file at the relative input_file_path into our main data structure, HeightMap
fn parse(input_file_path: &str) -> Result<Cave, std::io::Error> {
    let input_file: fs::File = fs::File::open(input_file_path)?;
    let reader: io::BufReader<fs::File> = io::BufReader::new(input_file);
    let re = Regex::new(r"(\d+),(\d+)").unwrap();
    let mut cave: Cave = Cave::new();

    for line in reader.lines() {
        let line = line.unwrap();
        println!("[{}]",line);
        let caps: Vec<regex::Captures> = re.captures_iter(&line).collect();

        for i in 0..caps.len()-1 {
            let a = &caps[i];
            let b = &caps[i+1];
            let x1 = a[1].parse::<u16>().unwrap();
            let y1 = a[2].parse::<u16>().unwrap();
            let x2 = b[1].parse::<u16>().unwrap();
            let y2 = b[2].parse::<u16>().unwrap();
            cave.add_rock_path(x1,y1,x2,y2);
        }
        println!();
    }
    Ok(cave)
}

///Solve part one of the Advent of Code 2022 puzzle, returns the puzzle answer
fn solve_part_one(cave: &mut Cave) -> u16 {
    let mut t = 0;
    while cave.simulate_sand() {
        println!("t = {}",t);
        cave.print();
        t+=1;
        thread::sleep(std::time::Duration::from_millis(1000));
    }
    t
}

///Solve part two of the Advent of Code 2022 puzzle, returns the puzzle answer
fn solve_part_two(cave: &mut Cave) -> u16 {
    0
}
