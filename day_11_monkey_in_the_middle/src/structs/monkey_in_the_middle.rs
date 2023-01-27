use super::monkey::Monkey;

#[derive(Clone)]
pub struct MonkeyInTheMiddle {
    monkeys: Vec<Monkey>,
    round: u32,
    least_common_multiple: u64,
}

impl MonkeyInTheMiddle {
    pub fn new() -> MonkeyInTheMiddle {
        MonkeyInTheMiddle {
            monkeys: Vec::new(),
            round: 0,
            least_common_multiple: 0,
        }
    }

    pub fn play_round_part_1(&mut self) {
        self.play_round(3.0);
    }

    pub fn play_round_part_2(&mut self) {
        self.play_round(1.0);
        //Prevent the worry_levels from ballooning by moduloing them by the least common multiple of all the test_divisors
        //The lcm contains every test_divisor. Removing the part of an int that was cleanly divisible by a multiple of a divisor
        //has no influence on the rest, because the entire number needs to be divisible by the test_divisor, the rest and what got removed. Therefore
        //the least common multiple can be safely modulod away, to keep the numbers from ballooning, it seems.
        for monkey in &mut self.monkeys {
            monkey.modulo_item_worry_level(self.least_common_multiple);
        }
    }

    ///Play a round of Monkey in the Middle
    pub fn play_round(&mut self, worry_level_divisor: f64) {
        for monkey in &mut self.monkeys {
            //Inspect and test each item
            monkey.play_round(worry_level_divisor);

            //Transfer all the items to the next Monkey
            let mut item_option = monkey.pop_item();
            while item_option.is_some() {
                let item = item_option.unwrap();
                self.monkeys[item.monkey_index()].push_item(item);
                item_option = monkey.pop_item();
            }
        }
        self.round += 1;
    }

    pub fn print_round_result(&self) {
        println!(
            "After round {}, the monkeys are holding items with these worry levels:",
            self.round
        );
        for (i, monkey) in self.monkeys.iter().enumerate() {
            print!("Monkey {}: ", i);
            monkey.print_items();
            println!();
        }
    }

    pub fn print_monkeys(&self) {
        println!(
            "The Monkeys playing Monkey in the Middle at round {}:",
            self.round
        );
        for (i, monkey) in self.monkeys.iter().enumerate() {
            println!("Monkey {}:", i);
            monkey.print();
        }
        println!();
    }

    pub fn print_amount_of_items_inspected_per_monkey(&self) {
        println!(
            "The total number of times each Monkey inspected items at round {}:",
            self.round
        );
        for (i, monkey) in self.monkeys.iter().enumerate() {
            println!(
                "Monkey {} inspected items {} times.",
                i,
                monkey.amount_of_items_inspected()
            );
        }
    }

    pub fn calculate_monkey_business(&self) -> u64 {
        let mut monkey_activities: Vec<u32> = Vec::new();
        for monkey in &self.monkeys {
            monkey_activities.push(monkey.amount_of_items_inspected());
        }
        monkey_activities.sort();
        monkey_activities.reverse();
        let monkey_business =
            *monkey_activities.get(0).unwrap() as u64 * *monkey_activities.get(1).unwrap() as u64;
        monkey_business
    }

    ///Calculate the least common multiple of all of the Monkeys test_divisors
    pub fn calculate_least_common_multiple(&mut self) {
        self.least_common_multiple = 1;
        for monkey in &self.monkeys {
            self.least_common_multiple *= monkey.test_divisor() as u64;
        }
        println!("Least common multiple: {}", self.least_common_multiple);
    }
}
