use colored::Colorize;
use std::{
    io::{BufRead, Write},
    num,
};

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

    let mut current_line: usize = 0;
    let mut tree_grid: TreeGrid = TreeGrid::new();
    while reader.read_line(&mut line).unwrap() > 0 {
        if line.chars().last().unwrap() == '\n' {
            line.pop(); //Remove trailing new-line character
        }
        //Do stuff
        for c in line.chars() {
            let height: i8 = c.to_string().parse().unwrap();
            tree_grid.add_tree(height, current_line)
        }
        current_line += 1;
        line.clear(); //Clear line string
    }
    //Part 1
    //tree_grid.look_from_every_direction();
    tree_grid.look_from_every_direction();
    tree_grid.print_tree_grid();
    let amount_of_trees_visible = tree_grid.amount_of_trees_visible_outside_of_the_grid();
    writeln!(output_1_file, "{}", amount_of_trees_visible).unwrap();
    //Part 2
    writeln!(output_2_file, "{}", "To do").unwrap();
}

///A structure representing a tree with a height in 0..10, and the directions from which the Tree is visible in
///its TreeGrid owner.
struct Tree {
    height: i8, //A number in [0, 9]
    visible_from_top: bool,
    visible_from_bottom: bool,
    visible_from_left: bool,
    visible_from_right: bool,
}

impl Tree {
    fn new(height: i8) -> Tree {
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
    fn add_tree(&mut self, height: i8, row: usize) {
        while row >= self.grid.len() {
            //Make sure there are enough rows in the grid by extending it
            self.grid.push(Vec::new());
        }
        self.grid.get_mut(row).unwrap().push(Tree::new(height));
    }

    ///Scan the Tree line from the top. Updates for every Tree T ∈ TreeGrid T.visible_from_top. T.visible_from_top
    ///is true if and only if all Trees in a line between T and the top edge of the TreeGrid have a shorter height.
    fn look_from_top(&mut self) {
        let width = self.get_width();
        let height = self.get_height();
        let mut highest_tree_height = -1;
        for i in 0..width {
            for j in 0..height {
                let tree = self.get_mut_tree(i, j);
                if tree.height > highest_tree_height {
                    //New highest tree found
                    tree.visible_from_top = true;
                    highest_tree_height = tree.height;
                }
                if highest_tree_height == 9 {
                    //Quit looking at trees when finding a 9, to speed things up
                    break;
                }
            }
            highest_tree_height = -1;
        }
    }

    ///Scan the Tree line from the bottom. Updates for every Tree T ∈ TreeGrid T.visible_from_bottom. T.visible_from_bottom
    ///is true if and only if all Trees in a line between T and the bottom edge of the TreeGrid have a shorter height.
    fn look_from_bottom(&mut self) {
        let width = self.get_width();
        let height = self.get_height();
        let mut highest_tree_height = -1;
        for i in 0..width {
            for j in 0..height {
                let current_height = height - 1 - j;
                let tree = self.get_mut_tree(i, current_height);
                if tree.height > highest_tree_height {
                    //New highest tree found
                    tree.visible_from_bottom = true;
                    highest_tree_height = tree.height;
                }
                if highest_tree_height == 9 {
                    //Quit looking at trees when finding a 9, to speed things up
                    break;
                }
            }
            highest_tree_height = -1;
        }
    }

    ///Scan the Tree line from the left. Updates for every Tree T ∈ TreeGrid T.visible_from_left. T.visible_from_left
    ///is true if and only if all Trees in a line between T and the left edge of the TreeGrid have a shorter height.
    fn look_from_left(&mut self) {
        let width = self.get_width();
        let height = self.get_height();
        let mut highest_tree_height = -1;
        for i in 0..height {
            for j in 0..width {
                let tree = self.get_mut_tree(j, i);
                if tree.height > highest_tree_height {
                    //New highest tree found
                    tree.visible_from_left = true;
                    highest_tree_height = tree.height;
                }
                if highest_tree_height == 9 {
                    //Quit looking at trees when finding a 9, to speed things up
                    break;
                }
            }
            highest_tree_height = -1;
        }
    }

    ///Scan the Tree line from the right. Updates for every Tree T ∈ TreeGrid T.visible_from_right. T.visible_from_right
    ///is true if and only if all Trees in a line between T and the right edge of the TreeGrid have a shorter height.
    fn look_from_right(&mut self) {
        let width = self.get_width();
        let height = self.get_height();
        let mut highest_tree_height = -1;
        for i in 0..height {
            for j in 0..width {
                let current_width = width - 1 - j;
                let tree = self.get_mut_tree(current_width, i);
                if tree.height > highest_tree_height {
                    //New highest tree found
                    tree.visible_from_right = true;
                    highest_tree_height = tree.height;
                }
                if highest_tree_height == 9 {
                    //Quit looking at trees when finding a 9, to speed things up
                    break;
                }
            }
            highest_tree_height = -1;
        }
    }

    ///Look at the Tree grid from every direction, and update the visibility of all the Trees in the grid (calls look_from_top(), look_from_...(), etc.)
    fn look_from_every_direction(&mut self) {
        self.look_from_top();
        self.look_from_bottom();
        self.look_from_left();
        self.look_from_right();
    }

    ///Returns the width of the grid of Trees
    fn get_width(&self) -> usize {
        let width = self.grid.get(0).unwrap().len();
        width
    }

    ///Returns the height of the grid of Trees
    fn get_height(&self) -> usize {
        let height = self.grid.len();
        height
    }

    ///Returns a mutable reference to the Tree in grid at position (x, y). (0, 0) is the top left element.
    ///x grows to the right. y grows downwards.
    fn get_mut_tree(&mut self, x: usize, y: usize) -> &mut Tree {
        self.grid.get_mut(y).unwrap().get_mut(x).unwrap()
    }

    ///Returns an immutable reference to the Tree in grid at position (x, y). (0, 0) is the top left element.
    ///x grows to the right. y grows downwards.
    fn get_tree(&self, x: usize, y: usize) -> &Tree {
        self.grid.get(y).unwrap().get(x).unwrap()
    }

    ///Returns the amount of Trees that are visible outside of the TreeGrid
    fn amount_of_trees_visible_outside_of_the_grid(&self) -> i32 {
        let mut amount_of_trees_visible = 0;
        let width = self.get_width();
        let height = self.get_height();
        for i in 0..height {
            for j in 0..width {
                let tree: &Tree = self.get_tree(j, i);
                if tree.is_visible() {
                    amount_of_trees_visible += 1;
                }
            }
        }
        amount_of_trees_visible
    }

    ///Prints out the TreeGrid, also prints out the width, height and total numbers of Trees in the TreeGrid
    fn print_tree_grid(&self) {
        let width = self.get_width();
        let height = self.get_height();
        let number_of_trees = width * height;
        println!(
            "TreeGrid(width:{}, height:{}, number_of_trees:{})",
            width, height, number_of_trees
        );
        let top = format!("top").on_red().bold().black();
        let left = format!("left").on_green().bold().black();
        let right = format!("right").on_blue().bold().black();
        println!(
            "{} {} {} {}",
            top,
            left,
            right,
            format!("bottom").truecolor(255, 165, 0).bold()
        );
        println!(
            "{} + {} = {}",
            top,
            left,
            format!("top-left").on_truecolor(255, 255, 0).bold().black()
        );
        println!(
            "{} + {} = {}",
            top,
            right,
            format!("top-right")
                .on_truecolor(255, 0, 255)
                .bold()
                .black()
        );
        println!(
            "{} + {} = {}",
            left,
            right,
            format!("left-right")
                .on_truecolor(0, 255, 255)
                .bold()
                .black()
        );
        println!(
            "{} + {} + {} = {}",
            top,
            left,
            right,
            format!("top-left-right")
                .on_truecolor(255, 255, 255)
                .bold()
                .black()
        );
        for i in 0..height {
            for j in 0..width {
                let tree = self.get_tree(j, i);
                if tree.is_visible() {
                    let mut colored_string = format!("{}", tree.height).bold().black();
                    let red: u8 = match tree.visible_from_top {
                        true => 255,
                        false => 0,
                    };
                    let green: u8 = match tree.visible_from_left {
                        true => 255,
                        false => 0,
                    };
                    let blue: u8 = match tree.visible_from_right {
                        true => 255,
                        false => 0,
                    };
                    colored_string = colored_string.on_truecolor(red, green, blue);
                    if tree.visible_from_bottom {
                        colored_string = colored_string.truecolor(255, 165, 0);
                    }
                    print!("{}", colored_string);
                } else {
                    print!("{}", tree.height);
                }
            }
            println!();
        }
    }
}
