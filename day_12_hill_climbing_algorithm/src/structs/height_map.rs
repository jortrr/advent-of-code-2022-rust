use colored::Colorize;

use super::node::Node;
use super::point::Point;
use std::collections::VecDeque;

//Heightmap
pub struct HeightMap {
    nodes: Vec<Vec<Node>>,  //A 2D grid of Nodes
    queue: VecDeque<Point>, //A queue containing the identifiers of the Nodes to be processed
}

impl HeightMap {
    pub fn new() -> HeightMap {
        HeightMap {
            nodes: Vec::new(),
            queue: VecDeque::new(),
        }
    }

    pub fn add_node(&mut self, row: usize, mark: u8) {
        while self.nodes.len() <= row {
            self.nodes.push(Vec::new());
        }
        self.nodes.get_mut(row).unwrap().push(Node::new(mark));
    }

    ///Run the breadth-first search algorithm on the HeightMap to find the shortest path from S to E.
    ///Source used: https://cp-algorithms.com/graph/breadth-first-search.html
    ///Returns the distance from start to goal
    pub fn run_breadth_first_search_algorithm(
        &mut self,
        start_mark: u8,
        goal: Point,
    ) -> Option<(u16, Point)> {
        let starting_node = self.get_node_from_point(&goal);
        starting_node.used = true;
        starting_node.shortest_path_length = Some(0);

        self.queue.push_back(goal);
        while !self.queue.is_empty() {
            //Run the BFS algorithm
            let current_node: Point = self.queue.pop_front().unwrap();
            if self.get_node_from_point(&current_node).mark == start_mark {
                //println!("Goal found on Node {:?}", current_node);
                return Some((
                    self.get_node_from_point(&current_node)
                        .shortest_path_length
                        .unwrap(),
                    current_node,
                ));
            }
            println!("Running the BFS algorithm on Node {:?}", current_node);
            self.process_node(&current_node);
            println!();
        }
        None
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
        print!("Processing neighbour Node {:?}", other);
        let node = self.get_node_immut(identifier.x, identifier.y);
        let shortest_path_length = node.shortest_path_length.unwrap() + 1;
        let other_node = self.get_node_immut(other.x, other.y);
        if self.have_an_edge(&other_node, &node) && !other_node.used {
            self.queue.push_back(other.clone());
            println!(", added Node to self.queue.");
            let other_node = self.get_node(other.x, other.y);
            other_node.shortest_path_length = Some(shortest_path_length);
            other_node.used = true;
            other_node.parent = Some(identifier.clone());
        } else {
            println!();
        }
    }

    ///Returns true if there is an edge from Node a and to Node b
    fn have_an_edge(&self, a: &Node, b: &Node) -> bool {
        HeightMap::panic_if_mark_invalid(a.mark);
        HeightMap::panic_if_mark_invalid(b.mark);
        let a_mark = HeightMap::transform_starting_and_goal_marks(a.mark);
        let b_mark = HeightMap::transform_starting_and_goal_marks(b.mark);
        a_mark + 1 >= b_mark
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
            .get_mut(y as usize)
            .unwrap()
            .get_mut(x as usize)
            .unwrap()
    }

    fn get_node_immut(&self, x: u16, y: u16) -> &Node {
        self.nodes.get(y as usize).unwrap().get(x as usize).unwrap()
    }

    fn get_node_from_point(&mut self, identifier: &Point) -> &mut Node {
        self.get_node(identifier.x, identifier.y)
    }

    ///Returns the identifier of the Node with mark 'E', or returns an error.
    pub fn find_goal_node(&self) -> Result<Point, &str> {
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
                if node.mark == mark {
                    return Ok(Point {
                        x: x as u16,
                        y: y as u16,
                    });
                }
            }
        }
        Err("The HeightMap does not contain a Node with the specified mark")
    }

    pub fn print_height_map(&self) {
        println!("HeightMap:");
        for node_vec in &self.nodes {
            for node in node_vec {
                print!("{}", node.mark as char);
            }
            println!();
        }
        println!();
    }

    pub fn print_distance_map(&self) {
        println!("HeightMap distances:");
        for node_vec in &self.nodes {
            for node in node_vec {
                match node.shortest_path_length {
                    Some(d) => print!("[{}]\t", d),
                    None => print!("[ ]\t"),
                }
            }
            println!();
        }
    }

    pub fn print_reachability_map(&self) {
        println!("HeightMap reachability:");
        for node_vec in &self.nodes {
            for node in node_vec {
                match node.shortest_path_length {
                    Some(_) => print!("{}", format!("{}", node.mark as char).on_green()),
                    None => print!("{}", format!("{}", node.mark as char).on_red()),
                }
            }
            println!();
        }
    }

    pub fn reset(&mut self) {
        for node_vec in &mut self.nodes {
            for node in node_vec {
                node.used = false;
                node.shortest_path_length = None;
            }
        }
        self.queue.clear();
    }

    pub fn print_shortest_path(&self, start: &Point) {
        println!("Shortest path to E:");
        let mut parents: Vec<Point> = vec![start.clone()];
        let mut current_node = self.get_node_immut(start.x, start.y);
        while current_node.parent.is_some() {
            parents.push(current_node.parent.as_ref().unwrap().clone());
            current_node = self.get_node_immut(
                current_node.parent.as_ref().unwrap().x,
                current_node.parent.as_ref().unwrap().y,
            );
        }
        for (j, node_vec) in self.nodes.iter().enumerate() {
            for (i, node) in node_vec.iter().enumerate() {
                let mut mark = format!("{}", node.mark as char).bright_white();
                let current_point = Point {
                    x: i as u16,
                    y: j as u16,
                };
                if parents.contains(&current_point) {
                    let index_of_next_parent =
                        parents.iter().position(|x| x == &current_point).unwrap() + 1;
                    if index_of_next_parent < parents.len() {
                        let next_parent = parents.get(index_of_next_parent).unwrap();
                        if next_parent.x < current_point.x {
                            mark = format!("<").red();
                        } else if next_parent.x > current_point.x {
                            mark = format!(">").red();
                        } else if next_parent.y > current_point.y {
                            mark = format!("v").red();
                        } else if next_parent.y < current_point.y {
                            mark = format!("^").red();
                        } else {
                            mark = mark.red();
                        }
                    }
                }
                if node.mark as char == 'E' {
                    mark = mark.bright_green();
                }
                print!("{}", mark);
            }
            println!();
        }
    }
}
