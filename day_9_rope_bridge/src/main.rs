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

    let mut position_grid: PositionGrid = PositionGrid::new();
    PositionGrid::print_header("Initial State");
    position_grid.print_grid();
    while reader.read_line(&mut line).unwrap() > 0 {
        //Remove trailing new-line character
        line = line.trim().to_string();
        //Part 1
        PositionGrid::print_header(&line[..]);
        let direction = line.chars().nth(0).unwrap();
        let iterations = line.chars().nth(2).unwrap();
        //Convert the direction and iterations chars to the right types
        let direction: Direction = Direction::new(direction);
        let iterations: u32 = iterations.to_digit(10).unwrap();
        for _ in 0..iterations {
            position_grid.head.go(&direction);
            position_grid.tail.move_to(&position_grid.head);
            position_grid.update_grid_corners();
            position_grid.print_grid();
        }

        line.clear(); //Clear line string
    }
    //Part 1
    writeln!(output_1_file, "{}", "To do").unwrap();
    //Part 2
    writeln!(output_2_file, "{}", "To do").unwrap();
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("The character '{}' is not a valid Direction, c should be an element of {{'U', 'D', 'L', 'R'}}.", c),
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
    fn go(&mut self, direction: &Direction) {
        match direction {
            Direction::Up => self.position.y += 1,
            Direction::Down => self.position.y -= 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        }
        self.visited_positions.insert(self.position);
    }
}

#[derive(Eq, Hash, PartialEq, Clone, Copy, Debug)] //Derive these traits so Position can be used in a HashSet
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
    grid_corner_bottom_left: Position,
    grid_corner_top_right: Position,
}

impl PositionGrid {
    fn new() -> PositionGrid {
        PositionGrid {
            starting_position: Position { x: 0, y: 0 },
            head: Knot::new(),
            tail: Knot::new(),
            grid_corner_bottom_left: Position { x: 0, y: 0 },
            grid_corner_top_right: Position { x: 0, y: 0 },
        }
    }

    ///Prints the PositionGrid in the format specified in the Advent of Code assignment. Prints an 's' and the starting_position,
    /// an 'H' at the head, a 'T' at the tail, and '.'s at unoccupied positions.
    ///Make sure to update the grid corners by calling update_grid_corners() first.
    fn print_grid(&self) {
        for y in (self.grid_corner_bottom_left.y..self.grid_corner_top_right.y + 1).rev() {
            for x in self.grid_corner_bottom_left.x..self.grid_corner_top_right.x + 1 {
                let grid_position = Position { x, y };
                //println!("grid_position = {:?}", grid_position);
                let mut position_symbol = ".";
                if grid_position == self.starting_position {
                    position_symbol = "s";
                }
                if grid_position == self.tail.position {
                    position_symbol = "T";
                }
                if grid_position == self.head.position {
                    position_symbol = "H";
                }
                print!("{}", position_symbol);
            }
            if self.starting_position == self.tail.position && y == self.starting_position.y {
                if self.head.position == self.tail.position {
                    println!("\t(H covers T, s)");
                } else {
                    println!("\t(T covers s)");
                }
            } else if self.head.position == self.tail.position && y == self.head.position.y {
                println!("\t(H covers T)");
            } else {
                println!();
            }
        }
        println!();
    }

    fn print_tail_positions(&self) {
        //TODO
    }

    ///Find the extreme values of the visited_positions of the Head and Tail knot, so we can know the size of the grid we need to draw.
    ///Updates grid_corner_bottom_right and grid_corner_top_left.
    fn update_grid_corners(&mut self) {
        let mut x_min = 0;
        let mut x_max = 0;
        let mut y_min = 0;
        let mut y_max = 0;
        let knots = [&self.head, &self.tail];
        for knot in knots {
            for position in knot.visited_positions.iter() {
                x_min = x_min.min(position.x);
                x_max = x_max.max(position.x);
                y_min = y_min.min(position.y);
                y_max = y_max.max(position.y);
            }
        }
        self.grid_corner_bottom_left = Position { x: x_min, y: y_min };
        self.grid_corner_top_right = Position { x: x_max, y: y_max };
    }

    ///Print a PositionGrid header to the terminal in the following format: '== {header_text} =='
    fn print_header(header_text: &str) {
        println!("== {} ==", header_text);
        println!();
    }
}
