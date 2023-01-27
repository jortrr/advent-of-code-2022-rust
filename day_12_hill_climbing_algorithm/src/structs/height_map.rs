use colored::Colorize;

use super::node::Node;
use std::collections::VecDeque;

//Heightmap
pub struct HeightMap {
    nodes: Vec<Node>,     //A vector representation of a 2D grid of Nodes
    queue: VecDeque<u16>, //A queue containing the nodes vector indices of the Nodes to be processed
    row_length: u16,      //The amount of elements on each row in the 2D grid
}

impl HeightMap {
    pub fn new() -> HeightMap {
        HeightMap {
            nodes: Vec::new(),
            queue: VecDeque::new(),
            row_length: 0,
        }
    }

    ///Add a new Node to the HeightMap
    pub fn add_node(&mut self, mark: char) {
        let index: u16 = self.nodes.len() as u16;
        self.nodes.push(Node::new(mark, index));
    }

    ///Returns whether the HeightMap is in a valid state, meaning that every row in the 2D Node grid nodes has to have length exactly row_length
    pub fn is_valid(&self) -> bool {
        self.nodes.len() % self.row_length as usize == 0
    }

    pub fn set_row_length(&mut self, row_length: u16) {
        self.row_length = row_length;
    }

    ///Run the breadth-first search algorithm on the HeightMap to find the shortest path from S to E.
    ///Source used: <https://cp-algorithms.com/graph/breadth-first-search.html>
    ///Returns the distance from start to goal, starts at goal
    pub fn run_breadth_first_search_algorithm(
        &mut self,
        start_mark: char,
        goal_mark: char,
    ) -> Option<(u16, Node)> {
        let starting_node_index: u16 = self.get_node_index_by_mark(goal_mark).unwrap();
        let starting_node: &mut Node = self.nodes.get_mut(starting_node_index as usize).unwrap();
        starting_node.set_as_starting_node();

        self.queue.push_back(starting_node.index());
        while !self.queue.is_empty() {
            //Run the BFS algorithm
            let current_node_index: u16 = self.queue.pop_front().unwrap();
            let current_node: &Node = self.nodes.get(current_node_index as usize).unwrap();
            if current_node.mark() == start_mark {
                //Found a path from goal to start
                return Some((
                    current_node.shortest_path_length().unwrap(),
                    current_node.clone(),
                ));
            }
            //println!("Running the BFS algorithm on Node {:?}", current_node);
            self.visit_node(current_node.index());
            //println!();
        }
        None
    }

    ///Visits a node according to the BFS algorithm
    fn visit_node(&mut self, index: u16) {
        //Check whether there is an edge between node and its neighbours (the difference between te elevations is atmost 1) and update neighbour.shortest_path_length and add to queue when needed
        //Check top
        if index >= self.row_length {
            self.process_neighbour_node(index, index - self.row_length);
        }
        //Check bottom
        if index + self.row_length < self.nodes.len() as u16 {
            self.process_neighbour_node(index, index + self.row_length);
        }
        //Check left
        if index % self.row_length > 0 {
            self.process_neighbour_node(index, index - 1);
        }
        //Check right
        if index % self.row_length != self.row_length - 1 {
            self.process_neighbour_node(index, index + 1);
        }
    }

    ///Add the neigbours to queue and set neighbour.shortest_path_length = node.shortest_path_length()+1 if there is an edge between the neighbouring nodes and neighbour.used==false.
    fn process_neighbour_node(&mut self, node_index: u16, neighbour_index: u16) {
        //print!("Processing neighbour Node {:?}", other);
        let node: &Node = self.nodes.get(node_index as usize).unwrap();
        let shortest_path_length: u16 = node.shortest_path_length().unwrap() + 1;
        let neighbour: &Node = self.nodes.get(neighbour_index as usize).unwrap();
        if self.have_an_edge(&neighbour, &node) && !neighbour.used() {
            self.queue.push_back(neighbour.index());
            //println!(", added Node to self.queue.");
            let neighbour: &mut Node = self.nodes.get_mut(neighbour_index as usize).unwrap();
            neighbour.set_parent(node_index, shortest_path_length);
        } else {
            //println!();
        }
    }

    ///Returns true if there is an edge from Node a to Node b
    fn have_an_edge(&self, a: &Node, b: &Node) -> bool {
        HeightMap::panic_if_mark_invalid(a.mark());
        HeightMap::panic_if_mark_invalid(b.mark());
        let a_mark = HeightMap::transform_starting_and_goal_marks(a.mark());
        let b_mark = HeightMap::transform_starting_and_goal_marks(b.mark());
        a_mark as u8 + 1 >= b_mark as u8
    }

    ///Sets 'S' to 'a' and 'E' to 'z', so HeightMap height calculation is possible
    fn transform_starting_and_goal_marks(mark: char) -> char {
        let mut result = mark;
        if result == 'S' {
            result = 'a';
        }
        if result == 'E' {
            result = 'z';
        }
        result
    }

    ///Panic if the mark is invalid
    fn panic_if_mark_invalid(mark: char) {
        if mark < 'a' && mark != 'E' && mark != 'S' || mark > 'z' {
            panic!(
                "The mark of the Node is not in [a,z]+{{S,E}}. mark = {}",
                mark
            );
        }
    }

    ///Returns the first occurence of the Node with parameter mark, or returns an error.
    fn get_node_index_by_mark(&self, mark: char) -> Result<u16, &str> {
        for node in self.nodes.iter() {
            if node.mark() == mark {
                return Ok(node.index());
            }
        }
        Err("The HeightMap does not contain a Node with the specified mark")
    }

    ///Prints a newline if the index is the rightmost Node on a row in the HeightMap 2D grid of Nodes
    pub fn print_line_at_row_end(&self, index: u16) {
        if (index + 1) % self.row_length == 0 {
            println!();
        }
    }

    ///Prints the HeightMap
    pub fn print(&self) {
        println!("HeightMap:");
        for (i, node) in self.nodes.iter().enumerate() {
            print!("{}", node.mark());
            self.print_line_at_row_end(i as u16);
        }
    }

    ///Prints distance to goal of each element of the HeightMap
    pub fn print_distance_map(&self) {
        println!("HeightMap distances:");
        for (i, node) in self.nodes.iter().enumerate() {
            match node.shortest_path_length() {
                Some(d) => print!("[{}]\t", d),
                None => print!("[ ]\t"),
            }
            self.print_line_at_row_end(i as u16);
        }
        println!();
    }

    ///Prints each element in green if they have a path to the goal, red otherwise
    pub fn print_reachability_map(&self) {
        println!("HeightMap reachability:");
        for (i, node) in self.nodes.iter().enumerate() {
            match node.shortest_path_length() {
                Some(_) => print!("{}", format!("{}", node.mark() as char).on_green()),
                None => print!("{}", format!("{}", node.mark() as char).on_red()),
            }
            self.print_line_at_row_end(i as u16);
        }
    }

    ///Resets the HeightMap to its original state, resets the BFS algorithm
    pub fn reset(&mut self) {
        for node in &mut self.nodes {
            node.reset();
        }
        self.queue.clear();
    }

    ///Print the shortest path from the starting_node to the BFS goal node in red
    pub fn print_shortest_path_to_goal(&self, starting_node: &Node) {
        println!("Shortest path from {} to E:", starting_node.mark());
        let mut parents: Vec<u16> = vec![starting_node.index()];
        let mut current_node: &Node = starting_node;
        while current_node.parent().is_some() {
            parents.push(current_node.parent().unwrap());
            current_node = self
                .nodes
                .get(current_node.parent().unwrap() as usize)
                .unwrap();
        }
        for (i, node) in self.nodes.iter().enumerate() {
            let mut mark = format!("{}", node.mark()).bright_white();
            if parents.contains(&node.index()) {
                let next_parent_index = node.parent();

                if next_parent_index.is_some() {
                    let next_parent_index = next_parent_index.unwrap();
                    if node.index() - 1 == next_parent_index {
                        mark = format!("<").red();
                    } else if node.index() + 1 == next_parent_index {
                        mark = format!(">").red();
                    } else if node.index() + self.row_length == next_parent_index {
                        mark = format!("v").red();
                    } else if node.index() - self.row_length == next_parent_index {
                        mark = format!("^").red();
                    } else {
                        mark = mark.red();
                    }
                }
            }
            if node.mark() == 'E' {
                mark = mark.bright_green();
            }
            print!("{}", mark);
            self.print_line_at_row_end(i as u16);
        }
    }
}
