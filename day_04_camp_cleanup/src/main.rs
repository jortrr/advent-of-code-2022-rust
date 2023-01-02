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
    let mut amount_of_fully_contained_ranges = 0;
    let mut amount_of_overlapping_ranges = 0;
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
        //Part 1
        if numbers[0] >= numbers[2] && numbers[1] <= numbers[3] {
            //First range is fuly contained in second range
            amount_of_fully_contained_ranges += 1;
        } else if numbers[0] <= numbers[2] && numbers[1] >= numbers[3] {
            //Second range is fully contained in first range
            amount_of_fully_contained_ranges += 1;
        }
        //Part 2
        if (numbers[0] >= numbers[2] && numbers[0] <= numbers[3])
            || (numbers[1] >= numbers[2] && numbers[1] <= numbers[3])
        {
            //First range endpoint is in second range
            amount_of_overlapping_ranges += 1;
        } else if (numbers[2] >= numbers[0] && numbers[2] <= numbers[1])
            || (numbers[3] >= numbers[0] && numbers[3] <= numbers[1])
        {
            //Second range endpoint is first range
            amount_of_overlapping_ranges += 1;
        }
        line.clear(); //Clear line string
    }
    //Part 1
    writeln!(output_1_file, "{}", amount_of_fully_contained_ranges).unwrap();
    //Part 2
    writeln!(output_2_file, "{}", amount_of_overlapping_ranges).unwrap();
}
