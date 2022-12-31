use std::collections::HashSet;
use std::hash::{Hash, Hasher};
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
        PositionGrid::print_header(&line[..]);
        let direction = line.chars().nth(0).unwrap();
        let iterations = &line[2..];
        //Convert the direction and iterations to the right types
        let direction: Direction = Direction::new(direction);
        let iterations: u32 = iterations.parse().unwrap();
        for _ in 0..iterations {
            position_grid.head.go(&direction);
            //Part 1
            position_grid.tail.move_to(&position_grid.head.position);
            position_grid.update_grid_corners(&position_grid.head.position.clone());
            position_grid.update_grid_corners(&position_grid.tail.position.clone());
            //position_grid.print_grid();
            //Part 2
            let mut head_knot_position: Position = position_grid.head.position;
            for knot in &mut position_grid.nine_tail_knots {
                knot.move_to(&head_knot_position);
                head_knot_position = knot.position;
            }
            for i in 0..position_grid.nine_tail_knots.len() {
                position_grid.update_grid_corners(
                    &position_grid
                        .nine_tail_knots
                        .get(i)
                        .unwrap()
                        .position
                        .clone(),
                );
            }
            //position_grid.print_grid_ten_knots();
        }

        line.clear(); //Clear line string
    }
    //Part 1
    position_grid.print_visited_positions(&position_grid.tail, "Tail");
    writeln!(
        output_1_file,
        "{}",
        position_grid.tail.visited_positions.len()
    )
    .unwrap();
    //Part 2
    position_grid.print_visited_positions(&position_grid.nine_tail_knots.last().unwrap(), "9 Knot");
    writeln!(
        output_2_file,
        "{}",
        position_grid
            .nine_tail_knots
            .last()
            .unwrap()
            .visited_positions
            .len()
    )
    .unwrap();
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
    fn new(symbol: char) -> Knot {
        let mut result = Knot {
            position: Position { x: 0, y: 0, symbol },
            visited_positions: HashSet::new(),
        };
        result.visited_positions.insert(result.position); //Add starting Position to visited_positions
        result
    }

    ///Move this Tail knot to the Head knot position, according to the movement described in the advent of code assignment.
    /// Returns the new Position of the Knot.
    fn move_to(&mut self, head_position: &Position) -> Position {
        let delta_x = head_position.x - self.position.x;
        let delta_y = head_position.y - self.position.y;
        if delta_x == 0 && delta_y == 0 {
            //Head and Tail overlap, Tail does not need to move
            return self.position;
        }
        if delta_x.abs() < 2 && delta_y.abs() < 2 {
            //Head and Tail are adjacent, Tail does not need to move
            return self.position;
        }
        if delta_x.abs() > 2 || delta_y.abs() > 2 {
            //The distance between Head and Tail should never be greater than 2, panic if it is
            panic!("The distance between Head and Tail is greater than 2, this should never be the case.");
        }
        //We now know Tail needs to move to Head, the below code works for both diagonal and nondiagonal motions
        self.position.x += round_up(delta_x as f64 / 2.0);
        self.position.y += round_up(delta_y as f64 / 2.0);
        self.visited_positions.insert(self.position);
        return self.position;
    }

    ///Take 1 step in the given direction.
    /// Returns the new Position of the Knot.
    fn go(&mut self, direction: &Direction) -> Position {
        match direction {
            Direction::Up => self.position.y += 1,
            Direction::Down => self.position.y -= 1,
            Direction::Left => self.position.x -= 1,
            Direction::Right => self.position.x += 1,
        }
        self.visited_positions.insert(self.position);
        return self.position;
    }
}

#[derive(Eq, Clone, Copy, Debug)] //Derive these traits so Position can be used in a HashSet
struct Position {
    x: i32,
    y: i32,
    symbol: char,
}

impl Position {
    fn new(x: i32, y: i32, symbol: char) -> Position {
        Position { x, y, symbol }
    }
}

impl PartialEq for Position {
    //Implement PartialEq to te able to compare Positions purely based on their x and y values
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Hash for Position {
    //Implement Hash to be able to use it in HashSets purely by considering the x and y values of a Position.

    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

///A struct allowing us to maintain our required awareness of the head and tail Knots, and which allows us to print
///the grid of positions including the starting_position, and the positions of the head and tail.
struct PositionGrid {
    starting_position: Position,
    head: Knot,
    tail: Knot,
    nine_tail_knots: [Knot; 9],
    grid_corner_bottom_left: Position,
    grid_corner_top_right: Position,
}

impl PositionGrid {
    fn new() -> PositionGrid {
        PositionGrid {
            starting_position: Position::new(0, 0, 's'),
            head: Knot::new('H'),
            tail: Knot::new('T'),
            nine_tail_knots: [
                Knot::new('1'),
                Knot::new('2'),
                Knot::new('3'),
                Knot::new('4'),
                Knot::new('5'),
                Knot::new('6'),
                Knot::new('7'),
                Knot::new('8'),
                Knot::new('9'),
            ],
            grid_corner_bottom_left: Position::new(0, 0, '.'),
            grid_corner_top_right: Position::new(0, 0, '.'),
        }
    }

    ///Prints the PositionGrid in the format specified in the Advent of Code assignment. Prints an 's' and the starting_position,
    /// an 'H' at the head, a 'T' at the tail, and '.'s at unoccupied positions.
    ///Make sure to update the grid corners by calling update_grid_corners() first.
    fn print_grid(&self) {
        let positions = [
            &self.head.position,
            &self.tail.position,
            &self.starting_position,
        ];
        self.print_grid_with_symbols(&positions[..]);
    }

    ///Prints the PositionGrid in the format specified in the Advent of Code assignment. Prints an 's' and the starting_position,
    /// an 'H' at the head, '1'..'9' for the tail, and '.'s at unoccupied positions.
    ///Make sure to update the grid corners by calling update_grid_corners() first.
    fn print_grid_ten_knots(&self) {
        //Add all Positions with a symbol to positions
        let mut positions: Vec<&Position> = vec![&self.head.position];
        for knot in &self.nine_tail_knots {
            positions.push(&knot.position);
        }
        positions.push(&self.starting_position);
        self.print_grid_with_symbols(&positions[..]);
    }

    ///Prints the PositionGrid in the format specified in the Advent of Code assignment. Prints the symbol associated with the Positions in
    ///positions.
    ///The order of a Position in positions determines which Position gets printed when Positions overlap, earlier Positions will get printed over later Positions,
    /// when their coordinates are the same.
    ///Make sure to update the grid corners by calling update_grid_corners() first.
    fn print_grid_with_symbols(&self, positions: &[&Position]) {
        for y in (self.grid_corner_bottom_left.y..self.grid_corner_top_right.y + 1).rev() {
            for x in self.grid_corner_bottom_left.x..self.grid_corner_top_right.x + 1 {
                let grid_position = Position::new(x, y, '.');
                let mut position_symbol = grid_position.symbol;
                for position in positions {
                    if grid_position == **position {
                        position_symbol = position.symbol;
                        break;
                    }
                }
                print!("{}", position_symbol);
            }
            println!();
            //Print a message when H, T or s are covered by eachother on the corresponding line, else print a newline.
            /*if self.starting_position == self.tail.position && y == self.starting_position.y {
                if self.head.position == self.tail.position {
                    println!("\t(H covers T, s)");
                } else {
                    println!("\t(T covers s)");
                }
            } else if self.head.position == self.tail.position && y == self.head.position.y {
                println!("\t(H covers T)");
            } else {
                println!();
            }*/
            //TODO
        }
        println!();
    }

    ///Prints the PositionGrid with a '#' symbol at every Position in knot.visited_positions. Prints a 's' at starting_position.
    fn print_visited_positions(&self, knot: &Knot, knot_name: &str) {
        println!("Positions the {} visited:", knot_name);
        for y in (self.grid_corner_bottom_left.y..self.grid_corner_top_right.y + 1).rev() {
            for x in self.grid_corner_bottom_left.x..self.grid_corner_top_right.x + 1 {
                let grid_position = Position::new(x, y, '.');
                let mut position_symbol = grid_position.symbol;
                if grid_position == self.starting_position {
                    position_symbol = self.starting_position.symbol;
                } else if knot.visited_positions.contains(&grid_position) {
                    position_symbol = '#';
                }
                print!("{}", position_symbol);
            }
            println!();
        }
        println!(
            "The {} visited {} Positions.",
            knot_name,
            knot.visited_positions.len()
        );
    }

    ///Find the extreme values of the visited_positions of the Head and Tail knot by supplying the current Position of these Knots, so we can know the size of the grid we need to draw.
    ///Updates grid_corner_bottom_right and grid_corner_top_left.
    fn update_grid_corners(&mut self, knot_position: &Position) {
        self.grid_corner_bottom_left.x = self.grid_corner_bottom_left.x.min(knot_position.x);
        self.grid_corner_bottom_left.y = self.grid_corner_bottom_left.y.min(knot_position.y);
        self.grid_corner_top_right.x = self.grid_corner_top_right.x.max(knot_position.x);
        self.grid_corner_top_right.y = self.grid_corner_top_right.y.max(knot_position.y);
    }

    ///Print a PositionGrid header to the terminal in the following format: '== {header_text} =='
    fn print_header(header_text: &str) {
        println!("== {} ==", header_text);
        println!();
    }
}

///Rounds up both positive and negative f64 values to an i32.
/// -0.7 => -1
/// 1.2 => 2
fn round_up(value: f64) -> i32 {
    if value < 0.0 {
        value.floor() as i32
    } else {
        value.ceil() as i32
    }
}
