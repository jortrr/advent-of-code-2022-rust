use std::collections::{HashMap, HashSet};
use std::cmp::{self, min, max};
use std::hash::Hash;

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
        self.update_cave_borders(x1, y1, x2, y2);
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
    pub fn update_cave_borders(&mut self, x1: u16, y1: u16, x2: u16, y2: u16) {
        self.min_x = min(self.min_x, x1);
        self.min_x = min(self.min_x, x2);
        self.max_x = max(self.max_x, x1);
        self.max_x = max(self.max_x, x2);

        self.min_y = min(self.min_y, y1);
        self.min_y = min(self.min_y, y2);
        self.max_y = max(self.max_y, y1);
        self.max_y = max(self.max_y, y2);
    }

    /// Print the Cave as a 2d grid, with '#' for rocks, 'o' for sand, '.' for air and '+' for the source of the sand.
    pub fn print(&self) {
        println!("Cave (min_x: {}, max_x: {}, min_y: {}, max_y: {})",self.min_x,self.max_x,self.min_y,self.max_y);
        for y in 0..=self.max_y {
            print!("{y}\t");
            for x in self.min_x..=self.max_x {
                if x == 500 && y == 0 {
                    print!("+"); //Source of the sand
                    continue;
                }
                else if self.rocks.contains_key(&x) {
                    if self.rocks.get(&x).unwrap().contains(&y) {
                        print!("#"); //Rock
                        continue;
                    }
                } else if self.sand.contains_key(&x) {
                    if self.sand.get(&y).unwrap().contains(&y) {
                        print!("o"); //Sand
                        continue;
                    }
                }
                print!("."); //Air
            }
            println!(); //New line
        }
    }

    pub fn new() -> Cave {
        Cave { rocks:HashMap::new(), sand: HashMap::new(), min_x: std::u16::MAX, max_x: 0, min_y: std::u16::MAX, max_y: 0 }
    }
}