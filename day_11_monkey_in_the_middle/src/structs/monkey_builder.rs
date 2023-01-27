use super::{
    item::Item,
    monkey::{Monkey, Operation},
};

pub struct MonkeyBuilder {
    pub items: Vec<Item>,
    pub operation: Operation,
    pub test_divisor: u32,
    pub monkey_if_test_is_true: u32,
    pub monkey_if_test_is_false: u32,
    pub amount_of_items_inspected: u32,
}

impl MonkeyBuilder {
    pub fn new() -> MonkeyBuilder {
        MonkeyBuilder {
            items: Vec::new(),
            operation: Operation::Add(0),
            test_divisor: 0,
            monkey_if_test_is_true: 0,
            monkey_if_test_is_false: 0,
            amount_of_items_inspected: 0,
        }
    }

    pub fn build(self) -> Monkey {
        Monkey::new(
            self.items,
            self.operation,
            self.test_divisor,
            self.monkey_if_test_is_true,
            self.monkey_if_test_is_false,
            self.amount_of_items_inspected,
        )
    }
}
