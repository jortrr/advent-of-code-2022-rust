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

    //Print the data structure that was created by parse()
    height_map.print_height_map();

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
        //Add node to the HeightMap
        for c in line.as_bytes() {
            height_map.add_node(row, *c);
        }
    }
    Ok(height_map)
}

///Solve part one of the Advent of Code 2022 puzzle, returns the puzzle answer
fn solve_part_one(height_map: &mut HeightMap) -> u16 {
    let goal_point = height_map.find_goal_node().unwrap();
    let (distance_to_goal, start_point) = height_map
        .run_breadth_first_search_algorithm(b'S', goal_point)
        .unwrap();
    height_map.print_distance_map();
    println!(
        "The shortest distance from 'S' to 'E' is {}.",
        distance_to_goal
    );
    height_map.print_shortest_path(&start_point);
    distance_to_goal
}

///Solve part two of the Advent of Code 2022 puzzle, returns the puzzle answer
fn solve_part_two(height_map: &mut HeightMap) -> u16 {
    let goal_point = height_map.find_goal_node().unwrap();
    let (distance_to_goal, start_point) = height_map
        .run_breadth_first_search_algorithm(b'a', goal_point)
        .unwrap();
    height_map.print_distance_map();
    height_map.print_reachability_map();
    println!(
        "The shortest distance from any 'a' to 'E' is {}.",
        distance_to_goal
    );
    height_map.print_shortest_path(&start_point);
    distance_to_goal
}
