//! Advent of Code 2022 day 12
use std::{
    env, fs,
    io::{self, BufRead},
};
use structs::height_map::HeightMap;

mod structs;

// File paths
static INPUT_PATH: &str = "puzzle/INPUT";
static ANSWER_PART_ONE_PATH: &str = "puzzle/ANSWER_PART_ONE";
static ANSWER_PART_TWO_PATH: &str = "puzzle/ANSWER_PART_TWO";

//Program main
fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    //Create our main Advent of Code puzzle data structure from the INPUT file
    let mut height_map: HeightMap = parse(INPUT_PATH).unwrap();
    if !height_map.is_valid() {
        panic!(
            "The HeightMap parsed from {} is in an invalid state, this should never happen.",
            INPUT_PATH
        );
    }

    //Print the data structure that was created by parse()
    height_map.print();

    //Solve part one of the Advent of Code puzzle
    let distance_to_goal = solve_part_one(&mut height_map);
    fs::write(ANSWER_PART_ONE_PATH, format!("{}", distance_to_goal)).unwrap();

    //Solve part two of the Advent of Code puzzle
    height_map.reset();
    let distance_to_a = solve_part_two(&mut height_map);
    fs::write(ANSWER_PART_TWO_PATH, format!("{}", distance_to_a)).unwrap();
}

///Parse the INPUT file at the relative input_file_path into our main data structure, HeightMap
fn parse(input_file_path: &str) -> Result<HeightMap, std::io::Error> {
    let input_file: fs::File = fs::File::open(input_file_path)?;
    let reader: io::BufReader<fs::File> = io::BufReader::new(input_file);
    let mut height_map: HeightMap = HeightMap::new();

    for (row, line) in reader.lines().enumerate() {
        //Remove leading trailing new-line characters
        let line: String = line?.trim().to_string();
        if row == 0 {
            height_map.set_row_length(line.len() as u16);
        }
        //Add node to the HeightMap
        for c in line.chars() {
            height_map.add_node(c);
        }
    }
    Ok(height_map)
}

///Solve part one of the Advent of Code 2022 puzzle, returns the puzzle answer
fn solve_part_one(height_map: &mut HeightMap) -> u16 {
    let (distance_to_goal, start_point) = height_map
        .run_breadth_first_search_algorithm('S', 'E')
        .unwrap();
    height_map.print_reachability_map();
    println!(
        "The shortest distance from 'S' to 'E' is {}.",
        distance_to_goal
    );
    height_map.print_shortest_path_to_goal(&start_point);
    distance_to_goal
}

///Solve part two of the Advent of Code 2022 puzzle, returns the puzzle answer
fn solve_part_two(height_map: &mut HeightMap) -> u16 {
    let (distance_to_goal, start_point) = height_map
        .run_breadth_first_search_algorithm('a', 'E')
        .unwrap();
    //height_map.print_distance_map();
    println!(
        "The shortest distance from any 'a' to 'E' is {}.",
        distance_to_goal
    );
    height_map.print_shortest_path_to_goal(&start_point);
    distance_to_goal
}
