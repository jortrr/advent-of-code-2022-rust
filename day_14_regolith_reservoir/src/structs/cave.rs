use std::collections::{HashMap, HashSet};
use std::cmp::{min, max};

pub struct Cave {
    rocks: HashMap<u16, HashSet<u16>>,
    sand: HashMap<u16, HashSet<u16>>,
    min_x: u16,
    max_x: u16,
    min_y: u16,
    max_y: u16,
}

impl Cave {
    /// Draws a path of rocks from (x1,y1) to (x2,y2)
    pub fn add_rock_path(&mut self, x1: u16, y1:u16, x2: u16, y2: u16) {
        println!("Drawing a rock path from ({},{}) to ({},{}).",x1,y1,x2,y2);
        self.update_cave_borders(x1, y1);
        self.update_cave_borders(x2, y2);
        let min_x = min(x1,x2);
        let max_x = max(x1,x2);
        let min_y = min(y1,y2);
        let max_y = max(y1,y2);

        if x1 == x2 {
            if !self.rocks.contains_key(&x1) {
                self.rocks.insert(x1, HashSet::new());
            }
            let row = self.rocks.get_mut(&x1).unwrap();
            for y in min_y..=max_y {
                println!("\tInserted ({},{}).",x1,y);
                row.insert(y);
            }
        } else if y1 == y2 {
            for x in min_x..=max_x {
                if !self.rocks.contains_key(&x) {
                    self.rocks.insert(x, HashSet::new());
                }
                self.rocks.get_mut(&x).unwrap().insert(y1);
            }
        }
    }

    /// Updates min_x, max_x, min_y, max_y
    pub fn update_cave_borders(&mut self, x: u16, y: u16) {
        self.min_x = min(self.min_x, x);
        self.max_x = max(self.max_x, x);
        self.min_y = min(self.min_y, y);
        self.max_y = max(self.max_y, y);
    }

    /// Print the Cave as a 2d grid, with '#' for rocks, 'o' for sand, '.' for air and '+' for the source of the sand.
    pub fn print(&self, floor: bool) {
        println!("Cave (min_x: {}, max_x: {}, min_y: {}, max_y: {})",self.min_x,self.max_x,self.min_y,self.max_y);
        let mut max_y = self.max_y;
        if floor {
            max_y = max_y + 2;
        }
        for y in 0..=max_y {
            print!("{y}\t");
            for x in self.min_x..=self.max_x {
                if self.rocks.contains_key(&x) {
                    if self.rocks.get(&x).unwrap().contains(&y) {
                        print!("#"); //Rock
                        continue;
                    }
                } 
                if self.sand.contains_key(&x) {
                    if self.sand.get(&x).unwrap().contains(&y) {
                        print!("o"); //Sand
                        continue;
                    }
                }
                if x == 500 && y == 0 {
                    print!("+"); //Source of the sand
                    continue;
                }
                if y == max_y { //Floor
                    print!("#"); //Rock
                    continue;
                }
                print!("."); //Air
            }
            println!(); //New line
        }
    }

    pub fn new() -> Cave {
        Cave { rocks:HashMap::new(), sand: HashMap::new(), min_x: std::u16::MAX, max_x: 0, min_y: std::u16::MAX, max_y: 0 }
    }

    /// Returns true if (x,y) is occupied by either rock or sand.
    pub fn is_occupied(&self, x: u16, y: u16, floor: bool) -> bool {
        if floor {
            if y == self.max_y + 2 {
                return true;
            }
        }
        if self.rocks.contains_key(&x) {
            if self.rocks.get(&x).unwrap().contains(&y) {
                return true;
            }
        }
        if self.sand.contains_key(&x) {
            if self.sand.get(&x).unwrap().contains(&y) {
                return true;
            }
        }
        false
    }

    ///Simulate a piece of sand falling from the source of the sand and coming to rest according to the following rules:
    /// - 1: A unit of sand always falls down one step if possible. 
    /// - 2: If the tile immediately below is blocked (by rock or sand), the unit of sand attempts to instead move diagonally one step down and to the left. 
    /// - 3: If that tile is blocked, the unit of sand attempts to instead move diagonally one step down and to the right. 
    /// - 4: If all three possible destinations are blocked, the unit of sand comes to rest and no longer moves.
    /// - 5: If floor is true, there is a floor from (-inf, self.max_y+2) to (inf, self.max_y+2)
    /// 
    /// Returns true if the sand came to rest inside the borders of the cave, false if the sand moved outside of the borders
    /// of the cave, or if the sand is unable to leave the source.
    pub fn simulate_sand(&mut self, floor: bool) -> bool {
        let mut x = 500;
        let mut y = 0;
        loop {
            if self.is_occupied(x, y, floor) {
                return false;
            }
            if !floor && (x < self.min_x || x > self.max_x || y > self.max_y) {
                return false;
            }

            if !self.is_occupied(x, y+1, floor) { //1
                y = y+1;
                continue;
            }
            if !self.is_occupied(x-1, y+1, floor) { //2
                x=x-1;
                y=y+1;
                continue;
            }
            if !self.is_occupied(x+1, y+1, floor){ //3 
                x=x+1;
                y=y+1;
                continue;
            }
            // 4
            if !self.sand.contains_key(&x) {
                self.sand.insert(x, HashSet::new());
            }
            self.sand.get_mut(&x).unwrap().insert(y);
            self.update_cave_borders(x, self.max_y);
            return true;
        }
    }

    /// Removes all sand from the cave
    pub fn reset_sand(&mut self) {
        self.sand = HashMap::new();
    }
}