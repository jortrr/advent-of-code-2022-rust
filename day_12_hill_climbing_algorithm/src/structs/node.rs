//Node
#[derive(Copy, Clone)]
pub struct Node {
    //The identifier of the Node is its 2D index in the nodes 2D grid
    used: bool,
    shortest_path_length: Option<u16>,
    parent: Option<u16>, //The index of another Node
    index: u16,          //The index of this Node
    mark: char,          //A letter in [a, z] + {S, E}
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

    pub fn reset(&mut self) {
        self.used = false;
        self.shortest_path_length = None;
        self.parent = None;
    }

    pub fn used(&self) -> bool {
        self.used
    }

    pub fn shortest_path_length(&self) -> Option<u16> {
        self.shortest_path_length
    }

    pub fn parent(&self) -> Option<u16> {
        self.parent
    }

    pub fn set_parent(&mut self, parent_index: u16, parent_shortest_path_length: u16) {
        self.used = true;
        self.parent = Some(parent_index);
        self.shortest_path_length = Some(parent_shortest_path_length + 1);
    }

    pub fn set_as_starting_node(&mut self) {
        self.used = true;
        self.shortest_path_length = Some(0);
    }

    pub fn index(&self) -> u16 {
        self.index
    }

    pub fn mark(&self) -> char {
        self.mark
    }
}
