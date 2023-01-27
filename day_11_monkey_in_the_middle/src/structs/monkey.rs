use super::item::Item;

#[derive(Debug, Clone, Copy)]
pub enum Factor {
    Old,
    Number(u32),
}

#[derive(Debug, Clone, Copy)]
pub enum Operation {
    Add(u32),
    Multiply(Factor),
}

impl Operation {
    pub fn new(operation: &str, operand: &str) -> Operation {
        match operation {
            "+" => Operation::Add(operand.to_string().parse().unwrap()),
            "*" => match operand {
                "old" => Operation::Multiply(Factor::Old),
                _ => Operation::Multiply(Factor::Number(operand.to_string().parse().unwrap())),
            },
            _ => panic!(
                "The operation ({}) is not a valid operation, an operation is either '+' or '*'.",
                operation
            ),
        }
    }
}

#[derive(Clone)]
pub struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test_divisor: u32,
    monkey_if_test_is_true: u32,
    monkey_if_test_is_false: u32,
    amount_of_items_inspected: u32,
}

impl Monkey {
    ///Tests whether an Item is divisible by the Monkeys test_divisor, returns the index of the Monkey the item needs
    ///to be thrown to.
    pub fn test(&self, item: &Item) -> u32 {
        let test_holds: bool = item.test(self.test_divisor as u64);
        match test_holds {
            true => self.monkey_if_test_is_true,
            false => self.monkey_if_test_is_false,
        }
    }

    pub fn new(
        items: Vec<Item>,
        operation: Operation,
        test_divisor: u32,
        monkey_if_test_is_true: u32,
        monkey_if_test_is_false: u32,
        amount_of_items_inspected: u32,
    ) -> Monkey {
        Monkey {
            items,
            operation,
            test_divisor,
            monkey_if_test_is_true,
            monkey_if_test_is_false,
            amount_of_items_inspected,
        }
    }

    pub fn print_items(&self) {
        let amount_of_items = self.items.len();
        for j in 0..amount_of_items {
            print!("{}", self.items.get(j).unwrap().worry_level());
            if j < amount_of_items - 1 {
                print!(", ");
            }
        }
    }

    ///Play a round of Monkey in the Middle, by inspecting all the Monkeys items, and applying the Monkeys operation to the items
    pub fn play_round(&mut self, worry_level_divisor: f64) {
        for j in 0..self.items.len() {
            let item = &mut self.items[j];
            item.inspect(&self.operation);
            self.amount_of_items_inspected += 1;
            item.get_bored_with_item(worry_level_divisor);
            let item = &self.items[j];
            let recipient_monkey_index = self.test(item) as usize;
            let item = &mut self.items[j];
            item.set_monkey_index(recipient_monkey_index);
        }
    }

    pub fn pop_item(&mut self) -> Option<Item> {
        self.items.pop()
    }

    pub fn push_item(&mut self, item: Item) {
        self.items.push(item);
    }

    ///Keep the remainder of the worry_levels of the items of this monkey after moduloing with least_common_multiple
    pub fn modulo_item_worry_level(&mut self, least_common_multiple: u64) {
        for item in &mut self.items {
            item.modulo_worry_level(least_common_multiple);
        }
    }

    ///Print the MonkeyInTheMiddle Monkey
    pub fn print(&self) {
        print!("\tItems: ");
        self.print_items();
        println!();
        print!("\tOperation: new = old -> ");
        println!("{:?}", self.operation);
        println!("\tTest: divisible by {}", self.test_divisor);
        println!(
            "\t\tIf true: throw to monkey {}",
            self.monkey_if_test_is_true
        );
        println!(
            "\t\tIf false: throw to monkey {}",
            self.monkey_if_test_is_false
        );
    }

    pub fn amount_of_items_inspected(&self) -> u32 {
        self.amount_of_items_inspected
    }

    pub fn test_divisor(&self) -> u32 {
        self.test_divisor
    }
}
