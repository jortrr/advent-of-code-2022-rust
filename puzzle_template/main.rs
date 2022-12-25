use regex::Regex;
use std::io::{BufRead, Write};

fn main() {
    //---Copy this to every puzzle program main---
    // File paths
    let relative_puzzle_path = "puzzle/";
    let input_file_path = format!("{}{}", relative_puzzle_path, "INPUT");
    let output_1_path = format!("{}{}", relative_puzzle_path, "OUTPUT_PART_ONE");
    let output_2_path = format!("{}{}", relative_puzzle_path, "OUTPUT_PART_TWO");

    //Open file in Rust
    let input_file = std::fs::File::open(input_file_path).unwrap();
    let mut output_1_file = std::fs::File::create(output_1_path).unwrap();
    let mut output_2_file = std::fs::File::create(output_2_path).unwrap();
    let mut reader = std::io::BufReader::new(input_file);
    let mut line = String::new();
    //---End---

    // Create a regex to match the pattern "number-number,number-number"
    let regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.chars().last().unwrap() == '\n' {
            line.pop(); //Remove trailing new-line character
        }
        let numbers: Vec<i32> = regex
            .captures(&line)
            .unwrap()
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().parse::<i32>().unwrap())
            .collect();
        //Do stuff

        line.clear(); //Clear line string
    }
    //Part 1
    writeln!(output_1_file, "{}", "To do").unwrap();
    //Part 2
    writeln!(output_2_file, "{}", "To do").unwrap();
}
