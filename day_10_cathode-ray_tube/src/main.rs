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

    let mut cpu = CPU::new();
    let interesting_cyles: [u32; 6] = [20, 60, 100, 140, 180, 220];
    let mut interesting_signal_strength_sum = 0;
    while reader.read_line(&mut line).unwrap() > 0 {
        //Remove trailing new-line character
        line = line.trim().to_string();
        //Do stuff
        //Part 1
        println!("Instruction: {}", line);
        let instruction = Instruction::new(&line);
        cpu.schedule(instruction);
        while cpu.process.as_ref().unwrap().state == ProcessState::Polling {
            cpu.tick();
            cpu.print_x_register();
            if interesting_cyles.contains(&cpu.cycle) {
                //Interesting cycle
                let signal_strength = cpu.calculate_signal_strength();
                interesting_signal_strength_sum += signal_strength;
                print!("\t Signal strength: {}", signal_strength);
            }
            println!();
        }
        cpu.execute_process();

        println!();
        line.clear(); //Clear line string
    }
    //Part 1
    println!(
        "The sum of the interesting signal strengths is: {}",
        interesting_signal_strength_sum
    );
    writeln!(output_1_file, "{}", interesting_signal_strength_sum).unwrap();
    //Part 2
    writeln!(output_2_file, "{}", "To do").unwrap();
}

///A CPU has a list of Processes that it needs poll and then execute
struct CPU {
    process: Option<Process>,
    x: i32,     //The X register
    cycle: u32, //The current cycle the CPU is on, starts at 0, gets incremented by tick()
}

impl CPU {
    fn new() -> CPU {
        CPU {
            process: None,
            x: 1,
            cycle: 0,
        }
    }

    fn schedule(&mut self, instruction: Instruction) {
        self.process = Some(Process::new(instruction));
    }

    ///Polls every Process in processes, and runs them if appropriate. Increments cycle.
    fn tick(&mut self) {
        let process = &mut self.process.as_mut().unwrap();
        if process.state == ProcessState::Polling {
            process.poll();
        }
        self.cycle += 1;
    }

    fn print_x_register(&self) {
        print!("[{}]\tX: {}", self.cycle, self.x);
    }

    fn calculate_signal_strength(&self) -> i32 {
        self.cycle as i32 * self.x
    }

    ///Executes self.process if self.process.state == ProcessState::Ready, panics otherwise.
    fn execute_process(&mut self) {
        let process = &mut self.process.as_mut().unwrap();
        if process.state == ProcessState::Ready {
            //Executes the Instruction on the CPU
            match process.instruction {
                Instruction::Noop => (),
                Instruction::Addx { v } => self.x += v,
            }
            process.state = ProcessState::Complete;
        } else {
            panic!("CPU.process.state ({:?}) should be equal to ProcessState::Ready, and it isn't, this should never happen.", process.state)
        }
    }
}

///An Instruction can do something to a CPU
enum Instruction {
    Noop,            //No operation
    Addx { v: i32 }, //Add v to the X register of the CPU
}

impl Instruction {
    fn new(instruction: &str) -> Instruction {
        let first_word = &instruction[0..4];
        match first_word {
            "noop" => Instruction::Noop,
            "addx" => {
                let second_word = &instruction[5..];
                let v: i32 = second_word.to_string().parse().unwrap();
                Instruction::Addx { v }
            }
            _ => panic!("The given first word ({}) is not a valid Instruction, a valid Instruction is 'noop' or 'addx'.",first_word),
        }
    }
}

#[derive(PartialEq, Debug)]
enum ProcessState {
    Polling,
    Ready,
    Complete,
}

struct Process {
    instruction: Instruction,
    cycles: u32,
    state: ProcessState,
}

impl Process {
    fn new(instruction: Instruction) -> Process {
        let mut cycles = 1; //For Noop instruction
        match instruction {
            Instruction::Addx { v: _ } => cycles = 2,
            _ => (), //Do nothing otherwise
        }
        Process {
            instruction,
            cycles,
            state: ProcessState::Polling,
        }
    }

    ///Polls the process, decrementing the cycles field by one unless cycles is equal to 0. Will return true if cycles
    ///equals 0, meaning the process is completed, and false otherwise. Sets complete field to true if complete.
    fn poll(&mut self) {
        if self.state == ProcessState::Polling {
            if self.cycles > 0 {
                self.cycles -= 1;
            }
            if self.cycles == 0 {
                self.state = ProcessState::Ready;
            }
        }
    }
}
