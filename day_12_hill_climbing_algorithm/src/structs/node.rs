use super::point::Point;

//Node
pub struct Node {
    //The identifier of the Node is its 2D index in the nodes 2D grid
    pub used: bool,
    pub shortest_path_length: Option<u16>,
    pub parent: Option<Point>, //The identifier of another Node
    pub mark: u8,              //A letter in [a, z] + {S, E}
}

impl Node {
    pub fn new(mark: u8) -> Node {
        Node {
            used: false,
            shortest_path_length: None,
            parent: None,
            mark,
        }
    }
}
