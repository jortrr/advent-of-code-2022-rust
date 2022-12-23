use std::{
    collections::HashMap,
    io::{BufRead, Write},
};

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

    //--- Day 3: Rucksack Reorganization ---
    let item_types = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let mut item_priority_map: HashMap<char, i32> = HashMap::new();
    let mut shared_item_priority_sum = 0;

    //Create item_priority_map
    for (i, c) in item_types.chars().enumerate() {
        item_priority_map.insert(c, i as i32 + 1);
    }

    let mut rucksack_count = 0;
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.chars().last().unwrap() == '\n' {
            line.pop(); //Remove trailing new-line character
        }
        //Do stuff
        let second_rucksack_compartment_start = line.len() / 2;
        let (rucksack_compartment_1, rucksack_compartment_2) =
            line.split_at(second_rucksack_compartment_start);

        for c in rucksack_compartment_1.chars() {
            if rucksack_compartment_2.contains(c) {
                //Shared item found
                let shared_item = c;
                let shared_item_priority = item_priority_map.get(&shared_item).unwrap();
                println!(
                    "Rucksack[{}]:\tShared item (i: {}, p: {})\tfound in (1: {}, 2: {})",
                    rucksack_count,
                    shared_item,
                    shared_item_priority,
                    rucksack_compartment_1,
                    rucksack_compartment_2
                );
                rucksack_count += 1;
                shared_item_priority_sum += shared_item_priority;
                break;
            }
        }

        line.clear(); //Clear line string
    }
    //Part 1
    writeln!(output_1_file, "{}", shared_item_priority_sum).unwrap();
    //Part 2
    writeln!(output_2_file, "{}", "To do").unwrap();
}
