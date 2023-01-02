use core::panic;
use std::io::{BufRead, Write};

fn main() {
    //---Copy this to every puzzle program main---
    // File paths
    let relative_puzzle_path = "puzzle/";
    let input_file_path = format!("{}{}", relative_puzzle_path, "EXAMPLE_INPUT");
    let output_1_path = format!("{}{}", relative_puzzle_path, "OUTPUT_PART_ONE");
    let output_2_path = format!("{}{}", relative_puzzle_path, "OUTPUT_PART_TWO");

    //Open file in Rust
    let input_file = std::fs::File::open(input_file_path).unwrap();
    let mut output_1_file = std::fs::File::create(output_1_path).unwrap();
    let mut output_2_file = std::fs::File::create(output_2_path).unwrap();
    let mut reader = std::io::BufReader::new(input_file);
    let mut line = String::new();
    //---End---

    //Temporary variables to construct a Monkey
    let mut monkey_in_the_middle = MonkeyInTheMiddle::new();
    let mut monkey_in_construction = Monkey::new();
    let mut monkey_counter = 0;
    let amount_of_rounds = 20;
    while reader.read_line(&mut line).unwrap() > 0 {
        //Remove trailing new-line character
        print!("line:\t{}", line);
        line = line.trim().to_string();
        //Do stuff
        //Part 1
        if line.is_empty() {
            //Monkey found, add Monkey to MonkeyInTheMiddle
            monkey_in_the_middle.monkeys.push(monkey_in_construction);
            monkey_in_construction = Monkey::new();
            monkey_counter += 1;
            continue;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        let first_word = *words.get(0).unwrap();

        if first_word == "Monkey" {
            //Skip Monkey line
            line.clear();
            continue;
        }
        let last_word = *words.get(words.len() - 1).unwrap();
        let second_to_last_word = *words.get(words.len() - 2).unwrap();
        if first_word == "Starting" {
            //Starting items line
            for word in &words[2..] {
                let mut word = word.to_string();
                if word.chars().last().unwrap() == ',' {
                    word.pop();
                }
                let worry_level: u32 = word.parse().unwrap();
                monkey_in_construction
                    .items
                    .push(Item::new(worry_level, monkey_counter))
            }
        } else if first_word == "Operation:" {
            //Operation line
            monkey_in_construction.operation = Operation::new(second_to_last_word, last_word);
        } else if first_word == "Test:" {
            //Test line
            monkey_in_construction.test_divisor = last_word.to_string().parse().unwrap();
        } else if first_word == "If" {
            let second_word = *words.get(1).unwrap();
            if second_word == "true:" {
                //Throw to monkey if test is true
                monkey_in_construction.monkey_if_test_is_true =
                    last_word.to_string().parse().unwrap();
            } else if second_word == "false:" {
                //Throw to monkey if test is false
                monkey_in_construction.monkey_if_test_is_false =
                    last_word.to_string().parse().unwrap();
            } else {
                panic!(
                    "Invalid input on line: {}; expected second word ({}) to be 'true:' or 'false:'.",
                    line,
                    second_word
                );
            }
        } else {
            panic!(
                "Invalid input on line: {}; first word ({}) is not in {{'Monkey', 'Starting', 'Operation', 'Test', 'If'}}.",
                line,
                first_word
            );
        }

        line.clear(); //Clear line string
    }
    //Monkey found, add Monkey to MonkeyInTheMiddle
    monkey_in_the_middle.monkeys.push(monkey_in_construction);
    println!();
    //Part 1
    monkey_in_the_middle.print_monkeys();
    for _ in 0..amount_of_rounds {
        monkey_in_the_middle.play_round();
        monkey_in_the_middle.print_round_result();
        println!();
    }
    monkey_in_the_middle.print_amount_of_items_inspected_per_monkey();
    let monkey_business = monkey_in_the_middle.calculate_monkey_business();
    println!(
        "The level of monkey business in this situation is: {}",
        monkey_business
    );
    //TODO: Play the game
    writeln!(output_1_file, "{}", monkey_business).unwrap();
    //Part 2
    writeln!(output_2_file, "{}", "To do").unwrap();
}

struct MonkeyInTheMiddle {
    monkeys: Vec<Monkey>,
    round: u32,
}

impl MonkeyInTheMiddle {
    fn new() -> MonkeyInTheMiddle {
        MonkeyInTheMiddle {
            monkeys: Vec::new(),
            round: 0,
        }
    }

    ///Play a round of Monkey in the Middle
    fn play_round(&mut self) {
        for i in 0..self.monkeys.len() {
            //Inspect and test each item
            {
                let monkey = self.monkeys.get_mut(i).unwrap();
                for j in 0..monkey.items.len() {
                    let item = &mut monkey.items[j];
                    item.inspect(&monkey.operation);
                    monkey.amount_of_items_inspected += 1;
                    item.get_bored_with_item();
                    let item = &monkey.items[j];
                    let recipient_monkey_index = monkey.test(item) as usize;
                    let item = &mut monkey.items[j];
                    item.monkey_index = recipient_monkey_index;
                }
            }
            //Transfer all the items to the next Monkey
            for j in 0..self.monkeys[i].items.len() {
                let item = self.monkeys[i].items.pop().unwrap();
                self.monkeys[item.monkey_index].items.push(item);
            }
        }
        self.round += 1;
    }

    fn print_round_result(&self) {
        println!(
            "After round {}, the monkeys are holding items with these worry levels:",
            self.round
        );
        for i in 0..self.monkeys.len() {
            print!("Monkey {}: ", i);
            self.monkeys[i].print_items();
            println!();
        }
    }

    fn print_monkeys(&self) {
        println!(
            "The Monkeys playing Monkey in the Middle at round {}:",
            self.round
        );
        for (i, monkey) in self.monkeys.iter().enumerate() {
            println!("Monkey {}:", i);
            print!("\tItems: ");
            monkey.print_items();
            println!();
            print!("\tOperation: new = old -> ");
            println!("{:?}", monkey.operation);
            println!("\tTest: divisible by {}", monkey.test_divisor);
            println!(
                "\t\tIf true: throw to monkey {}",
                monkey.monkey_if_test_is_true
            );
            println!(
                "\t\tIf false: throw to monkey {}",
                monkey.monkey_if_test_is_false
            );
        }
        println!();
    }

    fn print_amount_of_items_inspected_per_monkey(&self) {
        println!(
            "The total number of times each Monkey inspected items at round {}:",
            self.round
        );
        for (i, monkey) in self.monkeys.iter().enumerate() {
            println!(
                "Monkey {} inspected items {} times.",
                i, monkey.amount_of_items_inspected
            );
        }
    }

    fn calculate_monkey_business(&self) -> u32 {
        let mut monkey_activities: Vec<u32> = Vec::new();
        for monkey in &self.monkeys {
            monkey_activities.push(monkey.amount_of_items_inspected);
        }
        monkey_activities.sort();
        monkey_activities.reverse();
        let monkey_business = monkey_activities.get(0).unwrap() * monkey_activities.get(1).unwrap();
        monkey_business
    }
}

#[derive(Debug)]
enum Factor {
    Old,
    Number(u32),
}

#[derive(Debug)]
enum Operation {
    Add(u32),
    Multiply(Factor),
}

impl Operation {
    fn new(operation: &str, operand: &str) -> Operation {
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

struct Monkey {
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
    fn test(&self, item: &Item) -> u32 {
        let test_holds: bool = item.worry_level % self.test_divisor == 0;
        match test_holds {
            true => self.monkey_if_test_is_true,
            false => self.monkey_if_test_is_false,
        }
    }

    fn new() -> Monkey {
        Monkey {
            items: Vec::new(),
            operation: Operation::Add(0),
            test_divisor: 1,
            monkey_if_test_is_true: 0,
            monkey_if_test_is_false: 0,
            amount_of_items_inspected: 0,
        }
    }

    fn print_items(&self) {
        let amount_of_items = self.items.len();
        for j in 0..amount_of_items {
            print!("{}", self.items[j].worry_level);
            if j < amount_of_items - 1 {
                print!(", ");
            }
        }
    }
}

struct Item {
    worry_level: u32,
    monkey_index: usize,
}

impl Item {
    ///Inspect the Item, applies a Monkeys Operation to the Item
    fn inspect(&mut self, operation: &Operation) {
        match operation {
            Operation::Add(x) => self.worry_level += x,
            Operation::Multiply(factor) => match factor {
                Factor::Old => self.worry_level *= self.worry_level,
                Factor::Number(x) => self.worry_level *= x,
            },
        }
    }

    ///The Monkey gets bored with the Item after inspecting it. worry_level is divided by 3 because the Item
    /// wasn't damaged.
    fn get_bored_with_item(&mut self) {
        self.worry_level = (self.worry_level as f64 / 3.0).floor() as u32;
    }

    fn new(worry_level: u32, monkey_index: usize) -> Item {
        Item {
            worry_level,
            monkey_index,
        }
    }
}
