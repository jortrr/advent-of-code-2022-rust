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
    let mut badge_priority_sum = 0;

    //Create item_priority_map
    for (i, c) in item_types.chars().enumerate() {
        item_priority_map.insert(c, i as i32 + 1);
    }

    let mut rucksack_count = 0;
    let mut elf_index = 0;
    let mut elf_badge_map: HashMap<char, i32> = HashMap::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.chars().last().unwrap() == '\n' {
            line.pop(); //Remove trailing new-line character
        }
        //Do stuff
        //Part 1
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
        //Part 2
        for c in line.chars() {
            if elf_index == 0 {
                if !elf_badge_map.contains_key(&c) {
                    elf_badge_map.insert(c, elf_index);
                }
            } else if elf_index == 1 || elf_index == 2 {
                if let Some(v) = elf_badge_map.get(&c) {
                    if *v == elf_index - 1 {
                        elf_badge_map.insert(c, elf_index);
                    }
                    if elf_badge_map[&c] == 2 {
                        //Elf group badge found
                        let badge = c;
                        let badge_priority = item_priority_map.get(&badge).unwrap();
                        println!(
                            "Badge found for Rucksack[{}..{}]: (b: {}, p: {})",
                            rucksack_count - 2,
                            rucksack_count,
                            badge,
                            badge_priority
                        );
                        badge_priority_sum += badge_priority;
                        break;
                    }
                }
            }
        }
        if elf_index == 2 {
            elf_index = 0;
            elf_badge_map.clear();
        } else {
            elf_index += 1;
        }

        line.clear(); //Clear line string
    }
    //Part 1
    writeln!(output_1_file, "{}", shared_item_priority_sum).unwrap();
    //Part 2
    writeln!(output_2_file, "{}", badge_priority_sum).unwrap();
}
