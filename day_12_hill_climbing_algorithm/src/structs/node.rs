//Node
#[derive(Copy, Clone)]
pub struct Node {
    //The identifier of the Node is its 2D index in the nodes 2D grid
    pub used: bool,
    pub shortest_path_length: Option<u16>,
    pub parent: Option<u16>, //The index of another Node
    pub index: u16,          //The index of this Node
    pub mark: char,          //A letter in [a, z] + {S, E}
}

impl Node {
    pub fn new(mark: char, index: u16) -> Node {
        Node {
            used: false,
            shortest_path_length: None,
            parent: None,
            index,
            mark,
        }
    }
}
