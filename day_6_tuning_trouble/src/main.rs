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

    let mut amount_of_characters = 0;
    let mut last_four_characters: String = String::new();
    let mut last_fourteen_characters: String = String::new();
    let mut index_after_start_of_packet_marker = 0;
    let mut index_after_start_of_message_marker = 0;

    while reader.read_line(&mut line).unwrap() > 0 {
        if line.chars().last().unwrap() == '\n' {
            line.pop(); //Remove trailing new-line character
        }
        //Do stuff
        for c in line.chars() {
            amount_of_characters += 1;
            last_four_characters += c.to_string().as_str();
            last_fourteen_characters += c.to_string().as_str();
            if last_four_characters.len() > 4 {
                last_four_characters.remove(0);
            }
            if last_fourteen_characters.len() > 14 {
                last_fourteen_characters.remove(0);
            }
            //Part 1
            if last_four_characters.len() == 4 && index_after_start_of_packet_marker == 0 {
                if distinct(&last_four_characters[..]) {
                    //First start-of-packet marker found
                    index_after_start_of_packet_marker = amount_of_characters;
                    println!(
                        "Found first start-of-packet marker ({}) after processing ({}) characters.",
                        last_four_characters, index_after_start_of_packet_marker
                    );
                }
            }
            //Part 2
            if last_fourteen_characters.len() == 14 && index_after_start_of_message_marker == 0 {
                if distinct(&last_fourteen_characters[..]) {
                    //First start-of-message marker found
                    index_after_start_of_message_marker = amount_of_characters;
                    println!(
                        "Found first start-of-message marker ({}) after processing ({}) characters.",
                        last_fourteen_characters, index_after_start_of_message_marker
                    );
                }
            }
        }

        line.clear(); //Clear line string
    }
    //Part 1
    writeln!(output_1_file, "{}", index_after_start_of_packet_marker).unwrap();
    //Part 2
    writeln!(output_2_file, "{}", index_after_start_of_message_marker).unwrap();
}

fn distinct(some_string: &str) -> bool {
    if some_string.len() <= 1 {
        return true;
    }
    for i in 1..some_string.len() {
        if some_string.chars().nth(0) == some_string.chars().nth(i) {
            return false;
        }
    }
    return distinct(&some_string[1..]);
}
