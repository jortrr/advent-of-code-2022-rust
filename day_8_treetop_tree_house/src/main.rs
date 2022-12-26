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
        if line.chars().last().unwrap() == '\n' {
            line.pop(); //Remove trailing new-line character
        }
        //Do stuff

        line.clear(); //Clear line string
    }
    //Part 1
    writeln!(output_1_file, "{}", "To do").unwrap();
    //Part 2
    writeln!(output_2_file, "{}", "To do").unwrap();
}

///A structure representing a tree with a height in 0..10, and the directions from which the Tree is visible in
///its TreeGrid owner.
struct Tree {
    height: u8, //A number in [0, 9]
    visible_from_top: bool,
    visible_from_bottom: bool,
    visible_from_left: bool,
    visible_from_right: bool,
}

impl Tree {
    fn new(height: u8) -> Tree {
        Tree {
            height,
            visible_from_top: false,
            visible_from_bottom: false,
            visible_from_left: false,
            visible_from_right: false,
        }
    }

    ///Returns whether the Tree is visible in its TreeGrid
    fn is_visible(&self) -> bool {
        let visible = self.visible_from_top
            || self.visible_from_bottom
            || self.visible_from_left
            || self.visible_from_right;
        visible
    }
}

///A structure representing a 2D grid of Trees
struct TreeGrid {
    grid: Vec<Vec<Tree>>,
}

impl TreeGrid {
    fn new() -> TreeGrid {
        TreeGrid { grid: Vec::new() }
    }

    ///Append a new Tree with field height to grid[row]
    fn add_tree(&mut self, height: u8, row: usize) {
        while row > self.grid.len() {
            //Make sure there are enough rows in the grid by extending it
            self.grid.push(Vec::new());
        }
        self.grid.get_mut(row).unwrap().push(Tree::new(height));
    }

    ///Scan the Tree line from the top. Updates for every Tree T ∈ TreeGrid T.visible_from_top. T.visible_from_top
    ///is true if and only if all Trees in a line between T and the top edge of the TreeGrid have a shorter height.
    fn look_from_top(&mut self) {}

    fn look_from_bottom(&mut self) {}

    fn look_from_left(&mut self) {}

    fn look_from_right(&mut self) {}

    fn look_from_every_direction(&mut self) {
        self.look_from_top();
        self.look_from_bottom();
        self.look_from_left();
        self.look_from_right();
    }
}
