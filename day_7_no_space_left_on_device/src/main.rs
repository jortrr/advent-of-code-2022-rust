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

    //We will be filling in the details of the root_directory as we process more input
    let mut root_directory: Directory = {
        Directory {
            name: String::from("/"),
            size: 0,
            directories: Vec::new(),
            files: Vec::new(),
        }
    };
    let mut history_counter = 0;
    //Our current directory, changed by the "$ cd (dir)" command
    let mut current_directory: &Directory = &root_directory;
    //A stack data structure of our current directory, containing all of its ancestors
    //cd (dir) pushes (dir) to the stack. cd .. pops the stack. cd / pops all elements but the first from the stack.
    let mut directory_stack: Vec<&Directory> = vec![&root_directory];

    // Create a regex to match the pattern "number-number,number-number"
    let regex = Regex::new(r"(\d+)-(\d+),(\d+)-(\d+)").unwrap();
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.chars().last().unwrap() == '\n' {
            line.pop(); //Remove trailing new-line character
        }
        //Do stuff
        print!("history[{}]:\t{}", history_counter, line);
        let align_length = 24 - line.len();
        for _ in 0..align_length {
            //Align all the input interpretations at 24 spaces (3 or 6 tabs) to the right of the colon
            print!(" ");
        }

        if line.chars().nth(0).unwrap() == '$' {
            //We're in command mode
            let command = &line[2..4];
            if command == "cd" {
                //We're in change directory mode
                let dir = &line[5..];
                if dir == "/" {
                    //Change directory to root
                    print!("Changing directory to root ({}) directory", dir);
                    //current_directory = &root_directory;
                    while directory_stack.len() > 1 {
                        directory_stack.pop();
                    }
                } else if dir == ".." {
                    //Move out one level
                    print!(
                        "Changing directory to parent of current directory ({})",
                        current_directory.name
                    );
                    //directory_stack.pop();
                    //current_directory = directory_stack.last().unwrap();
                } else {
                    //Move in one level to dir
                    print!(
                        "Changing directory to ({}) subdirectory of current directory ({})",
                        dir, current_directory.name
                    );
                    /*if !current_directory.directories.iter().any(|d| d.name == dir) {
                        //dir has to be added to current_directory
                        current_directory.add_directory(dir.to_string());
                    }*/
                }
            } else if command == "ls" {
                //We're in list mode
                print!(
                    "Listing files and directories in current directory ({})",
                    current_directory.name
                );
            }
        } else {
            //We're getting the result of an ls command
            if line[..3].to_string() == "dir" {
                //We've found a directory
                print!("Found a directory in (current_directory)");
            } else {
                //We've found a file
                print!("Found a file in (current_directory)");
            }
        }

        println!();
        history_counter += 1;
        line.clear(); //Clear line string
    }
    //Part 1
    writeln!(output_1_file, "{}", "To do").unwrap();
    //Part 2
    writeln!(output_2_file, "{}", "To do").unwrap();
}

struct File {
    name: String,
    size: i32,
}

struct Directory {
    name: String,
    size: i32,
    directories: Vec<Directory>,
    files: Vec<File>,
}

impl Directory {
    ///Recursively calculate the size of all files and directories of this directory, call on the root "/" to calculate the size of all directories
    fn calculate_total_size(&mut self) -> i32 {
        let size_of_all_files: i32 = self.files.iter().map(|i| i.size).sum();
        let size_of_all_directories: i32 = self
            .directories
            .iter_mut()
            .map(|i| i.calculate_total_size())
            .sum();
        let total_size = size_of_all_files + size_of_all_directories;
        total_size
    }

    fn add_directory(&mut self, name: String) {
        self.directories.push(Directory {
            name,
            size: 0,
            directories: Vec::new(),
            files: Vec::new(),
        })
    }
}
