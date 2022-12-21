use std::io::{BufRead, Write};

fn main() {
    // File paths
    let relative_puzzle_path = "puzzle/";
    let input_file_path = format!("{}{}", relative_puzzle_path, "INPUT");
    let output_file_path = format!("{}{}", relative_puzzle_path, "OUTPUT");

    //Open file in Rust
    let input_file = std::fs::File::open(input_file_path).unwrap();
    let mut output_file = std::fs::File::create(output_file_path).unwrap();
    let mut reader = std::io::BufReader::new(input_file);
    let mut line = String::new();
    let mut most_calories = 0;
    let mut current_calories = 0;

    //Read every line of input_file
    while reader.read_line(&mut line).unwrap() > 0 {
        //println!("line: {}", line);
        line.pop();
        if line.is_empty() {
            if current_calories > most_calories {
                most_calories = current_calories;
            }
            current_calories = 0;
        } else {
            let inventory_item = line.parse::<i32>().unwrap();
            current_calories += inventory_item;
        }
        line.clear();
    }
    println!("{}", most_calories);
    writeln!(output_file, "{}", most_calories).unwrap();
}
