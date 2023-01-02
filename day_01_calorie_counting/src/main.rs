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

    //--- Day 1: Calorie Counting --- (https://adventofcode.com/2022/day/1)
    let mut top_three_most_calories: [i32; 3] = [0; 3];
    let mut current_calories = 0;

    //Read every line of input_file
    while reader.read_line(&mut line).unwrap() > 0 {
        //println!("line: {}", line);
        line.pop();
        if line.is_empty() {
            for i in 0..top_three_most_calories.len() {
                if current_calories > top_three_most_calories[i] {
                    top_three_most_calories[i] = current_calories;
                    top_three_most_calories.sort();
                    break;
                }
            }
            current_calories = 0;
        } else {
            let inventory_item = line.parse::<i32>().unwrap();
            current_calories += inventory_item;
        }
        line.clear();
    }
    //Part 1
    writeln!(output_1_file, "{}", top_three_most_calories[0]).unwrap();
    //Part 2
    let top_three_sum: i32 = top_three_most_calories.iter().sum();
    writeln!(output_2_file, "{}", top_three_sum).unwrap();
}
