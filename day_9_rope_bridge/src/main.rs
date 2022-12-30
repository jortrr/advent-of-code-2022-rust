use std::collections::HashSet;
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
        line = line.trim().to_string(); //Remove trailing new-line character
                                        //Do stuff

        line.clear(); //Clear line string
    }
    //Part 1
    writeln!(output_1_file, "{}", "To do").unwrap();
    //Part 2
    writeln!(output_2_file, "{}", "To do").unwrap();
}

enum Motion {
    Up,
    Down,
    Left,
    Right,
}

impl Motion {
    fn new(c: char) -> Motion {
        match c {
            'U' => Motion::Up,
            'D' => Motion::Down,
            'L' => Motion::Left,
            'R' => Motion::Right,
            _ => panic!("The character '{}' is not a valid Motion, c should be an element of {{'U', 'D', 'L', 'R'}}.", c),
        }
    }
}

//A Head or a Tail. Maintains a Position and a HashSet of visited Positions.
struct Knot {
    position: Position,
    visited_positions: HashSet<Position>,
}

impl Knot {
    fn new() -> Knot {
        let mut result = Knot {
            position: Position { x: 0, y: 0 },
            visited_positions: HashSet::new(),
        };
        result.visited_positions.insert(result.position); //Add starting Position to visited_positions
        result
    }

    ///Move this Tail knot to the Head knot, according to the movement described in the advent of code assignment
    fn move_to(&mut self, head: &Knot) {
        let delta_x = head.position.x - self.position.x;
        let delta_y = head.position.y - self.position.y;
        if delta_x == 0 && delta_y == 0 {
            //Head and Tail overlap, Tail does not need to move
            return;
        }
        if delta_x.abs() < 2 && delta_y.abs() < 2 {
            //Head and Tail are adjacent, Tail does not need to move
            return;
        }
        if delta_x.abs() > 2 || delta_y.abs() > 2 {
            //The distance between Head and Tail should never be greater than 2, panic if it is
            panic!("The distance between Head and Tail is greater than 2, this should never be the case.");
        }
        //We now know Tail needs to move to Head, the below code works for both diagonal and nondiagonal motions
        self.position.x += (delta_x as f64 / 2.0).ceil() as i32;
        self.position.x += (delta_x as f64 / 2.0).ceil() as i32;
        self.visited_positions.insert(self.position);
    }

    ///Take 1 step in the given direction
    fn go(&mut self, direction: &Motion) {
        match direction {
            Motion::Up => self.position.y += 1,
            Motion::Down => self.position.y -= 1,
            Motion::Left => self.position.x -= 1,
            Motion::Right => self.position.x += 1,
        }
        self.visited_positions.insert(self.position);
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy)] //Derive these traits so Position can be used in a HashSet
struct Position {
    x: i32,
    y: i32,
}

///A struct allowing us to maintain our required awareness of the head and tail Knots, and which allows us to print
///the grid of positions including the starting_position, and the positions of the head and tail.
struct PositionGrid {
    starting_position: Position,
    head: Knot,
    tail: Knot,
}

impl PositionGrid {
    fn new() -> PositionGrid {
        PositionGrid {
            starting_position: Position { x: 0, y: 0 },
            head: Knot::new(),
            tail: Knot::new(),
        }
    }

    fn print_grid(&self) {
        //TODO
    }

    fn print_tail_positions(&self) {
        //TODO
    }
}
