use std::cell::RefCell;
use std::io::{BufRead, Write};
use std::rc::{Rc, Weak};

//Sources used: https://doc.rust-lang.org/book/ch15-06-reference-cycles.html

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
    let root_directory = RefCell::new(Rc::new(Directory::new(&String::from("/"), &Weak::new())));
    let mut history_counter = 0;
    //Our current directory, changed by the "$ cd (dir)" command
    let mut current_directory = RefCell::new(Rc::clone(&root_directory.borrow()));

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
                    current_directory = RefCell::new(Rc::clone(&root_directory.borrow()));
                } else if dir == ".." {
                    //Move out one level
                    let parent_directory = &current_directory.borrow().parent.borrow().upgrade();
                    let parent = match parent_directory {
                        Some(dir) => dir.name.clone(),
                        None => String::from("None"),
                    };
                    print!(
                        "Changing directory to parent ({}) of current directory ({})",
                        parent,
                        current_directory.borrow().name
                    );
                    if parent_directory.is_some() {
                        //If the directory has a parent, and therefore isn't root "/"
                        current_directory =
                            RefCell::new(Rc::clone(&parent_directory.as_ref().unwrap()));
                    }
                } else {
                    //Move in one level to dir
                    print!(
                        "Changing directory to child ({}) of current directory ({})",
                        dir,
                        current_directory.borrow().name
                    );
                    let directories = current_directory.borrow().directories.clone();
                    let directories_vec = directories.borrow();
                    let mut child_exists = false;
                    for i in 0..directories_vec.len() {
                        if directories_vec.get(i).unwrap().borrow().name == dir {
                            //We have found the child directory
                            current_directory =
                                RefCell::new(Rc::clone(&directories_vec.get(i).unwrap().borrow()));
                            child_exists = true;
                        }
                    }
                    if !child_exists {
                        panic!("Tried to cd into non-existing child directory ({}) of current directory ({}).",dir,current_directory.borrow().name);
                    }
                }
            } else if command == "ls" {
                //We're in list mode
                print!(
                    "Listing files and directories in current directory ({})",
                    current_directory.borrow().name
                );
            }
        } else {
            //We're getting the result of an ls command
            if line[..3].to_string() == "dir" {
                //We've found a directory
                print!(
                    "Found a child directory of current directory ({})",
                    current_directory.borrow().name
                );
                let dir_name = line[4..].to_string();
                if current_directory.borrow().has_directory(&dir_name) {
                    print!(", child ({}) is already known", dir_name);
                } else {
                    print!(", added new child ({})", dir_name);
                    Directory::add_directory(
                        current_directory.clone(),
                        &dir_name,
                        &current_directory.borrow(),
                    );
                }
            } else {
                //We've found a file
                print!("Found a file in ({})", current_directory.borrow().name);
                //Get file_size and file_name from line
                let white_space_index = line.find(" ").unwrap();
                let (word_1, word_2) = line.split_at(white_space_index);
                let file_size: i32 = word_1.parse().unwrap();
                let file_name: String = word_2.to_string()[1..].to_string();

                if current_directory.borrow().has_file(&file_name) {
                    print!(", child ({}) is already known", &file_name);
                } else {
                    print!(", added new child ({})", &file_name);
                    Directory::add_file(current_directory.clone(), &file_name, file_size);
                }
            }
        }
        println!();
        history_counter += 1;
        line.clear(); //Clear line string
    }
    Directory::calculate_total_size(root_directory.clone());
    Directory::print_file_system(&root_directory, 0);
    //Part 1
    let sum_of_directories_size_atmost_100000 =
        Directory::calculate_sum_of_directories_with_size_atmost_100000(root_directory.clone());
    writeln!(output_1_file, "{}", sum_of_directories_size_atmost_100000).unwrap();
    //Part 2
    let total_disk_space: i32 = 70000000;
    let used_disk_space: i32 = *root_directory.borrow().size.borrow();
    let required_unused_disk_space = 30000000;
    let max_used_disk_space = total_disk_space - required_unused_disk_space;
    let disk_space_that_needs_to_be_freed = used_disk_space - max_used_disk_space;
    let mut smallest_directory_size_greater_than_required_size = 0;
    if disk_space_that_needs_to_be_freed > 0 {
        println!("The total disk space is {}.", total_disk_space);
        println!("The used disk space is {}.", used_disk_space);
        println!(
            "The required unused disk space is {}.",
            required_unused_disk_space
        );
        print!(
            "The disk space that needs to be freed is {}",
            disk_space_that_needs_to_be_freed,
        );
        let smallest_size_dir = Directory::find_smallest_directory_size_greater_than(
            root_directory.clone(),
            disk_space_that_needs_to_be_freed,
        );
        smallest_directory_size_greater_than_required_size =
            *smallest_size_dir.borrow().size.borrow();
        println!(
            ", the smallest possible directory atleast that size is ({}, size={}).",
            smallest_size_dir.borrow().name,
            smallest_directory_size_greater_than_required_size
        );
    }
    writeln!(
        output_2_file,
        "{}",
        smallest_directory_size_greater_than_required_size
    )
    .unwrap();
}

#[derive(Debug)]
struct File {
    name: String,
    size: i32,
}

#[derive(Debug)]
struct Directory {
    name: String,
    size: RefCell<i32>,
    directories: RefCell<Vec<RefCell<Rc<Directory>>>>,
    files: RefCell<Vec<File>>,
    parent: RefCell<Weak<Directory>>,
}

impl Directory {
    ///Creates a new Directory with a weak reference (non-ownership) to a parent Directory
    fn new(name: &String, parent: &Weak<Directory>) -> Directory {
        Directory {
            name: name.clone(),
            size: RefCell::new(0),
            directories: RefCell::new(Vec::new()),
            files: RefCell::new(Vec::new()),
            parent: RefCell::new(parent.clone()),
        }
    }

    ///Returns true if this Directory has a direct child File named file_name
    fn has_file(&self, file_name: &String) -> bool {
        for file in self.files.borrow().iter() {
            if file.name == *file_name {
                return true;
            }
        }
        return false;
    }

    ///Returns true if this Directory has a direct child Directory named dir_name
    fn has_directory(&self, dir_name: &String) -> bool {
        for dir in self.directories.borrow().iter() {
            if dir.borrow().name == *dir_name {
                return true;
            }
        }
        return false;
    }

    ///Adds a File{name: file_name, size: file_size} to the files field of this Directory
    fn add_file(root: RefCell<Rc<Directory>>, file_name: &String, file_size: i32) {
        root.borrow_mut().files.borrow_mut().push(File {
            name: file_name.clone(),
            size: file_size,
        });
    }

    ///Adds a Directory{name: dir_name, ..., parent: Rc::downgrade(parent)} to the directories field of this Directory
    fn add_directory(root: RefCell<Rc<Directory>>, dir_name: &String, parent: &Rc<Directory>) {
        root.borrow_mut()
            .directories
            .borrow_mut()
            .push(RefCell::new(Rc::new(Directory::new(
                &dir_name,
                &Rc::downgrade(parent),
            ))));
    }

    ///Recursively calculate the size of all files and directories of this directory, call on the root "/" to calculate the size of all directories
    fn calculate_total_size(root: RefCell<Rc<Directory>>) -> i32 {
        let size_of_all_files: i32 = root.borrow().files.borrow().iter().map(|i| i.size).sum();
        let mut size_of_all_directories: i32 = 0;
        for child_dir in root.borrow().directories.borrow().iter() {
            size_of_all_directories += Directory::calculate_total_size(child_dir.clone());
        }
        let total_size = size_of_all_files + size_of_all_directories;
        root.borrow_mut().size.replace(total_size);
        total_size
    }

    ///Prints out a file system (all Directories and Files) recursively, starting at the root Directory
    fn print_file_system(root: &RefCell<Rc<Directory>>, indent_level: i32) {
        print_indent_level(indent_level);
        let parent = match root.borrow().parent.borrow().upgrade() {
            Some(dir) => dir.name.clone(),
            None => String::from("None"),
        };
        println!(
            "- {} (dir, size={}) (parent: {})",
            root.borrow().name,
            root.borrow().size.borrow(),
            parent
        ); //Print root directory
        for i in 0..root.borrow().directories.borrow().len() {
            //Recursively print all directories of root
            Directory::print_file_system(
                root.borrow().directories.borrow().get(i).unwrap(),
                indent_level + 1,
            )
        }
        for i in 0..root.borrow().files.borrow().len() {
            //Print all files of root
            print_indent_level(indent_level + 1);
            println!(
                "- {} (file, size={})",
                root.borrow().files.borrow().get(i).unwrap().name,
                root.borrow().files.borrow().get(i).unwrap().size
            );
        }
    }

    ///Recursively look for directories in root with a size of atmost 100000, returns the sum of the sizes of those directories.
    fn calculate_sum_of_directories_with_size_atmost_100000(root: RefCell<Rc<Directory>>) -> i32 {
        let mut sum: i32 = 0;
        let size = *root.borrow().size.borrow();
        if size <= 100000 {
            sum += size;
            println!(
                "Found directory ({}, size={}) with size atmost 100000.",
                root.borrow().name,
                root.borrow().size.borrow()
            );
        }
        for child_dir in root.borrow().directories.borrow().iter() {
            sum +=
                Directory::calculate_sum_of_directories_with_size_atmost_100000(child_dir.clone());
        }
        sum
    }

    fn find_smallest_directory_size_greater_than(
        root: RefCell<Rc<Directory>>,
        required_size: i32,
    ) -> RefCell<Rc<Directory>> {
        let mut smallest_size = *root.borrow().size.borrow();
        let mut smallest_size_dir = root.clone();
        for child_dir in root.borrow().directories.borrow().iter() {
            let smallest_child_dir = Directory::find_smallest_directory_size_greater_than(
                child_dir.clone(),
                required_size,
            );
            let smallest_child_size = *smallest_child_dir.borrow().size.borrow();
            if smallest_child_size < smallest_size && smallest_child_size >= required_size {
                smallest_size = smallest_child_size;
                smallest_size_dir = smallest_child_dir;
            }
        }
        smallest_size_dir
    }
}

///Prints spaces on the current line based on indent_level
fn print_indent_level(indent_level: i32) {
    for _ in 0..indent_level {
        print!("  ");
    }
}