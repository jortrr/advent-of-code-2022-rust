use super::monkey::{Factor, Operation};

#[derive(Clone)]
pub struct Item {
    worry_level: u64,
    monkey_index: usize,
}

impl Item {
    ///Inspect the Item, applies a Monkeys Operation to the Item
    pub fn inspect(&mut self, operation: &Operation) {
        match operation {
            Operation::Add(x) => self.worry_level += *x as u64,
            Operation::Multiply(factor) => match factor {
                Factor::Old => self.worry_level *= self.worry_level,
                Factor::Number(x) => self.worry_level *= *x as u64,
            },
        }
    }

    ///The Monkey gets bored with the Item after inspecting it. worry_level is divided by 3 because the Item
    /// wasn't damaged.
    pub fn get_bored_with_item(&mut self, worry_level_divisor: f64) {
        self.worry_level = (self.worry_level as f64 / worry_level_divisor).floor() as u64;
    }

    pub fn new(worry_level: u64, monkey_index: usize) -> Item {
        Item {
            worry_level,
            monkey_index,
        }
    }

    //Tests whether an Items worry_level is divisible by a Monkeys test_divisor
    pub fn test(&self, test_divisor: u64) -> bool {
        self.worry_level % test_divisor == 0
    }

    pub fn worry_level(&self) -> u64 {
        self.worry_level
    }

    pub fn monkey_index(&self) -> usize {
        self.monkey_index
    }

    pub fn set_monkey_index(&mut self, monkey_index: usize) {
        self.monkey_index = monkey_index;
    }

    ///Keep the remainder of the worry_level of this item after moduloing with least_common_multiple
    pub fn modulo_worry_level(&mut self, least_common_multiple: u64) {
        self.worry_level %= least_common_multiple;
    }
}
