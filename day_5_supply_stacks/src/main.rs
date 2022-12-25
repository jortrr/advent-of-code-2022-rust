use regex::Regex;
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

    let crate_mover_part_1 = "CrateMover 9000";
    let crate_mover_part_2 = "CrateMover 9001";
    let mut we_want_the_crates: bool = true;
    let mut stacks_of_crates_part_1: HashMap<i32, Vec<char>> = HashMap::new();
    let mut stacks_of_crates_part_2: HashMap<i32, Vec<char>> = HashMap::new();
    let mut procedure_counter = 0;
    // Create a regex to match the pattern "move number from number to number"
    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.chars().last().unwrap() == '\n' {
            line.pop(); //Remove trailing new-line character
        }
        //Do stuff
        //Let's implement the stacks of crates as a map of vectors
        //First we will need to read out the starting configuration, after all the queues have been read in,
        //they will need to be reversed.
        if we_want_the_crates {
            for (i, c) in line.chars().enumerate() {
                //println!("c: {}", c);
                //There could be a crate or stack here
                if c.is_numeric() {
                    //Its a stack, reverse the crates in stack_of_crates and go to the instruction part of the input
                    let length = stacks_of_crates_part_1.keys().len();
                    for i in 1..length + 1 {
                        stacks_of_crates_part_1
                            .get_mut(&(i as i32))
                            .unwrap()
                            .reverse();
                        stacks_of_crates_part_2.insert(
                            i as i32,
                            stacks_of_crates_part_1.get(&(i as i32)).unwrap().clone(),
                        );
                    }
                    we_want_the_crates = false;
                    print_stacks_of_crates(&stacks_of_crates_part_1, crate_mover_part_1);
                    print_stacks_of_crates(&stacks_of_crates_part_2, crate_mover_part_2);
                    break;
                } else if c.is_alphabetic() {
                    //Its a crate, add it to the stack
                    let stack: i32 = (i as i32 - 1) / 4 + 1;
                    //println!("stack: {}", stack);
                    if !stacks_of_crates_part_1.contains_key(&stack) {
                        stacks_of_crates_part_1.insert(stack, vec![c]);
                    } else {
                        stacks_of_crates_part_1.get_mut(&stack).unwrap().push(c);
                    }
                }
            }
        } else if !line.is_empty() {
            println!("procedure[{}]: {}", procedure_counter, line);
            procedure_counter += 1;
            //Crate stack configuration has been read in
            let numbers: Vec<i32> = regex
                .captures(&line)
                .unwrap()
                .iter()
                .skip(1)
                .map(|c| c.unwrap().as_str().parse::<i32>().unwrap())
                .collect();
            let amount_of_crates = numbers[0];
            let stack_source = numbers[1];
            let stack_target = numbers[2];
            //Part 1
            //Move the crates to the stack_target from the stack_source
            for _ in 0..amount_of_crates {
                let target_crate = stacks_of_crates_part_1
                    .get_mut(&stack_source)
                    .unwrap()
                    .pop()
                    .unwrap();
                stacks_of_crates_part_1
                    .get_mut(&stack_target)
                    .unwrap()
                    .push(target_crate);
            }
            //Part 2
            let crates_at_stack_source = stacks_of_crates_part_2.get_mut(&stack_source).unwrap();
            let mut crates = crates_at_stack_source
                .split_off(crates_at_stack_source.len() - amount_of_crates as usize);
            stacks_of_crates_part_2
                .get_mut(&stack_target)
                .unwrap()
                .append(&mut crates);
        }

        line.clear(); //Clear line string
    }
    print_stacks_of_crates(&stacks_of_crates_part_1, crate_mover_part_1);
    print_stacks_of_crates(&stacks_of_crates_part_2, crate_mover_part_2);
    //Create the output for part 1
    let top_crates_part_1 = find_top_crates(stacks_of_crates_part_1);
    let top_crates_part_2 = find_top_crates(stacks_of_crates_part_2);
    //Part 1
    writeln!(output_1_file, "{}", top_crates_part_1).unwrap();
    //Part 2
    writeln!(output_2_file, "{}", top_crates_part_2).unwrap();
}

fn print_stacks_of_crates(stacks_of_crates_part_1: &HashMap<i32, Vec<char>>, crate_mover: &str) {
    println!("Stacks of crates ({})", crate_mover);
    let amount_of_crates = stacks_of_crates_part_1.keys().len();
    for i in 1..amount_of_crates + 1 {
        print!("[{}]: ", i);
        let crates = stacks_of_crates_part_1.get(&(i as i32)).unwrap();
        for c in crates {
            print!("[{}] ", c);
        }
        println!();
    }
}

fn find_top_crates(stacks_of_crates_part_1: HashMap<i32, Vec<char>>) -> String {
    let mut top_crates = String::new();
    for i in 1..stacks_of_crates_part_1.keys().len() + 1 {
        let top_crate = stacks_of_crates_part_1
            .get(&(i as i32))
            .unwrap()
            .last()
            .unwrap()
            .to_string();
        top_crates += top_crate.as_str();
    }
    top_crates
}
