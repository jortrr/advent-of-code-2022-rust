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
        let node = self.get_node_from_point(identifier);
        //Check whether there is an edge between node and its neighbours (the difference between te elevations is atmost 1)
        //TODO
        //Add the neigbours to queue and set neighbour.shortest_path_length = node.shortest_path_length+1 if neighbour.used==false.
        //TODO
        //Set node.used to true
        node.used = true;
    }

    fn get_node(&mut self, x: u16, y: u16) -> &mut Node {
        self.nodes
            .get_mut(x as usize)
            .unwrap()
            .get_mut(y as usize)
            .unwrap()
    }

    fn get_node_from_point(&mut self, identifier: &Point) -> &mut Node {
        self.get_node(identifier.x, identifier.y)
    }

    ///Returns the identifier of the Node with mark 'S', or returns an error.
    fn find_starting_node(&self) -> Result<Point, &str> {
        self.find_node_by_mark(b'S')
        //Err("The HeightMap does not contain a Node with mark 'S', no starting node could be found.") //TODO
    }

    fn find_goal_node(&self) -> Result<Point, &str> {
        self.find_node_by_mark(b'E')
    }

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
        Err("The HeightMap does not contain a Node with mark 'TODO'")
    }
}

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
