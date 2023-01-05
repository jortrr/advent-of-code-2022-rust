use std::collections::VecDeque;
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

    let mut height_map = HeightMap::new();
    let mut current_line = 0;
    while reader.read_line(&mut line).unwrap() > 0 {
        //Remove trailing new-line character
        line = line.trim().to_string();
        //Do stuff
        if current_line >= height_map.nodes.len() {
            height_map.nodes.push(Vec::new());
        }
        for c in line.as_bytes().iter() {
            height_map
                .nodes
                .get_mut(current_line)
                .unwrap()
                .push(Node::new(*c));
        }
        current_line += 1;
        line.clear(); //Clear line string
    }
    height_map.print_height_map();
    //Part 1
    writeln!(output_1_file, "{}", "To do").unwrap();
    //Part 2
    writeln!(output_2_file, "{}", "To do").unwrap();
}

struct HeightMap {
    nodes: Vec<Vec<Node>>,  //A 2D grid of Nodes
    queue: VecDeque<Point>, //A queue containing the identifiers of the Nodes to be processed
}

impl HeightMap {
    fn new() -> HeightMap {
        HeightMap {
            nodes: Vec::new(),
            queue: VecDeque::new(),
        }
    }

    ///Run the breadth-first search algorithm on the HeightMap to find the shortest path from S to E.
    ///Source used: https://cp-algorithms.com/graph/breadth-first-search.html
    fn run_breadth_first_search_algorithm(&mut self) {
        let starting_node_point: Point = self.find_starting_node().unwrap();
        let starting_node = self.get_node_from_point(&starting_node_point);
        starting_node.used = true;
        starting_node.shortest_path_length = Some(0);

        self.queue.push_back(starting_node_point);
        while !self.queue.is_empty() {
            //Run the BFS algorithm
            let current_node: Point = self.queue.pop_front().unwrap();
            self.process_node(&current_node);
        }
    }

    fn process_node(&mut self, identifier: &Point) {
        let height: u16 = self.nodes.len() as u16;
        let width: u16 = self.nodes.get(identifier.y as usize).unwrap().len() as u16;
        //Check whether there is an edge between node and its neighbours (the difference between te elevations is atmost 1) and update neighbour.shortest_path_length and add to queue when needed
        //Check top
        if identifier.y > 0 {
            self.process_neighbour_node(identifier, 0, -1);
        }
        //Check bottom
        if identifier.y < height - 1 {
            self.process_neighbour_node(identifier, 0, 1);
        }
        //Check left
        if identifier.x > 0 {
            self.process_neighbour_node(identifier, -1, 0);
        }
        //Check right
        if identifier.x < width - 1 {
            self.process_neighbour_node(identifier, 1, 0);
        }
        //Set node.used to true
        let node = self.get_node_from_point(identifier);
        node.used = true;
    }

    ///Add the neigbours to queue and set neighbour.shortest_path_length = node.shortest_path_length+1 if there is an edge between the neighbouring nodes and neighbour.used==false.
    fn process_neighbour_node(
        &mut self,
        identifier: &Point,
        x_translation: i32,
        y_translation: i32,
    ) {
        let other = Point {
            x: (identifier.x as i32 + x_translation) as u16,
            y: (identifier.y as i32 + y_translation) as u16,
        };
        let node = self.get_node_immut(identifier.x, identifier.y);
        let shortest_path_length = node.shortest_path_length.unwrap() + 1;
        let other_node = self.get_node_immut(other.x, other.y);
        if self.have_an_edge(&node, &other_node) && !other_node.used {
            self.queue.push_back(other.clone());
        }
        let other_node = self.get_node(other.x, other.y);
        other_node.shortest_path_length = Some(shortest_path_length);
    }

    ///Returns true if there is an edge between Nodes a and b
    fn have_an_edge(&self, a: &Node, b: &Node) -> bool {
        HeightMap::panic_if_mark_invalid(a.mark);
        HeightMap::panic_if_mark_invalid(b.mark);
        let a_mark = HeightMap::transform_starting_and_goal_marks(a.mark);
        let b_mark = HeightMap::transform_starting_and_goal_marks(b.mark);
        a_mark.abs_diff(b_mark) < 2
    }

    fn transform_starting_and_goal_marks(mark: u8) -> u8 {
        let mut result = mark;
        if result == b'S' {
            result = b'a';
        }
        if result == b'E' {
            result = b'z';
        }
        result
    }

    fn panic_if_mark_invalid(mark: u8) {
        if mark < b'a' && mark != b'E' && mark != b'S' || mark > b'z' {
            panic!(
                "The mark of the Node is not in [a,z]+{{S,E}}. mark = {}",
                mark
            );
        }
    }

    fn get_node(&mut self, x: u16, y: u16) -> &mut Node {
        self.nodes
            .get_mut(x as usize)
            .unwrap()
            .get_mut(y as usize)
            .unwrap()
    }

    fn get_node_immut(&self, x: u16, y: u16) -> &Node {
        self.nodes.get(x as usize).unwrap().get(y as usize).unwrap()
    }

    fn get_node_from_point(&mut self, identifier: &Point) -> &mut Node {
        self.get_node(identifier.x, identifier.y)
    }

    ///Returns the identifier of the Node with mark 'S', or returns an error.
    fn find_starting_node(&self) -> Result<Point, &str> {
        let result = self.find_node_by_mark(b'S');
        match result {
            Ok(value) => Ok(value),
            Err(_) => Err("The HeightMap does not contain a Node with mark 'S', no starting node could be found.")
        }
    }

    ///Returns the identifier of the Node with mark 'E', or returns an error.
    fn find_goal_node(&self) -> Result<Point, &str> {
        let result = self.find_node_by_mark(b'E');
        match result {
            Ok(value) => Ok(value),
            Err(_) => Err(
                "The HeightMap does not contain a Node with mark 'E', no goal node could be found.",
            ),
        }
    }

    ///Returns the first occurence of the Node with parameter mark, or returns an error.
    fn find_node_by_mark(&self, mark: u8) -> Result<Point, &str> {
        for (y, node_vec) in self.nodes.iter().enumerate() {
            for (x, node) in node_vec.iter().enumerate() {
                if node.mark == b'S' {
                    return Ok(Point {
                        x: x as u16,
                        y: y as u16,
                    });
                }
            }
        }
        Err("The HeightMap does not contain a Node with the specified mark")
    }

    fn print_height_map(&self) {
        println!("HeightMap:");
        for node_vec in &self.nodes {
            for node in node_vec {
                print!("{}", node.mark as char);
            }
            println!();
        }
    }
}

#[derive(Clone)]
struct Point {
    x: u16,
    y: u16,
}

struct Node {
    //The identifier of the Node is its 2D index in the nodes 2D grid
    used: bool,
    shortest_path_length: Option<u16>,
    parent: Option<Point>, //The identifier of another Node
    mark: u8,              //A letter in [a, z] + {S, E}
}

impl Node {
    fn new(mark: u8) -> Node {
        Node {
            used: false,
            shortest_path_length: None,
            parent: None,
            mark,
        }
    }
}
