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

    while reader.read_line(&mut line).unwrap() > 0 {
        //Remove trailing new-line character
        line = line.trim().to_string();
        //Do stuff

        line.clear(); //Clear line string
    }
    //Part 1
    writeln!(output_1_file, "{}", "To do").unwrap();
    //Part 2
    writeln!(output_2_file, "{}", "To do").unwrap();
}

///A CPU has a list of instructions it needs to execute
struct CPU {
    scheduled_instructions: Vec<Instruction>,
}

impl CPU {
    fn schedule(&mut self, instruction: &str) {
        self.scheduled_instructions
            .push(Instruction::new(instruction));
    }

    ///Removes done Instructions from scheduled_instructions
    fn clean() {}
}

#[derive(Clone)]
enum InstructionType {
    noop,
    addx,
}

///An Instruction can do something to a CPU
#[derive(Clone)]
struct Instruction {
    instruction: InstructionType,
    process: Process,
    done: bool,
}

impl Instruction {
    fn run(&mut self, cpu: &mut CPU) {
        self.done = self.process.poll();
        if self.done {
            match self.instruction {
                InstructionType::noop => (), //TODO
                InstructionType::addx => (), //TODO
            }
        }
    }

    fn new(instruction: &str) -> Instruction {
        //TODO
    }
}

#[derive(Clone)]
struct Process {
    cycles: u32,
}

impl Process {
    fn new(cycles: u32) -> Process {
        Process { cycles }
    }

    ///Polls the process, decrementing the cycles field by one unless cycles is equal to 0. Will return true if cycles
    ///equals 0, meaning the process is completed, and false otherwise.
    fn poll(&mut self) -> bool {
        if self.cycles == 0 {
            return true;
        }
        self.cycles -= 1;
        false
    }
}
