use crate::structs::{monkey::Monkey, monkey_in_the_middle::MonkeyInTheMiddle};
use core::panic;
use std::{
    env, fs,
    io::{BufRead, Write},
};
use structs::monkey_in_the_middle;

mod structs;

// File paths
static INPUT_PATH: &str = "puzzle/INPUT";
static ANSWER_PART_ONE_PATH: &str = "puzzle/ANSWER_PART_ONE";
static ANSWER_PART_TWO_PATH: &str = "puzzle/ANSWER_PART_TWO";

fn main() -> std::io::Result<()> {
    env::set_var("RUST_BACKTRACE", "1");

    //Create our main Advent of Code puzzle data structure from the INPUT file
    let mut monkey_in_the_middle_part_one: MonkeyInTheMiddle = parse(INPUT_PATH).unwrap();
    let mut monkey_in_the_middle_part_two: MonkeyInTheMiddle =
        monkey_in_the_middle_part_one.clone();

    //Solve part one of the Advent of Code puzzle
    let monkey_business_part_one: u64 = solve_part_one(&mut monkey_in_the_middle_part_one);
    fs::write(
        ANSWER_PART_ONE_PATH,
        format!("{}", monkey_business_part_one),
    )?;

    //Solve part two of the Advent of Code puzzle
    let monkey_business_part_two: u64 = solve_part_two(&mut monkey_in_the_middle_part_two);
    fs::write(
        ANSWER_PART_TWO_PATH,
        format!("{}", monkey_business_part_two),
    )?;

    Ok(())
}

///Parse the INPUT file at the relative input_file_path into our main data structure, MonkeyInTheMiddle
fn parse(input_file_path: &str) -> Result<MonkeyInTheMiddle, std::io::Error> {
    //Temporary variables to construct a Monkey
    let mut monkey_in_the_middle = MonkeyInTheMiddle::new();
    let mut monkey_in_construction = Monkey::new();
    let mut monkey_counter = 0;

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
                let worry_level: u64 = word.parse().unwrap();
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
}

///Solve part one of the Advent of Code 2022 puzzle, returns the puzzle answer
fn solve_part_one(monkey_in_the_middle: &mut MonkeyInTheMiddle) -> u64 {
    let amount_of_rounds = 20;
    println!("Part 1:");
    monkey_in_the_middle.print_monkeys();
    for _ in 0..amount_of_rounds {
        monkey_in_the_middle.play_round_part_1();
        monkey_in_the_middle.print_round_result();
        println!();
    }
    monkey_in_the_middle.print_amount_of_items_inspected_per_monkey();
    let monkey_business = monkey_in_the_middle.calculate_monkey_business();
    println!(
        "The level of monkey business in this situation is: {}",
        monkey_business
    );
    println!();
    monkey_business
}

///Solve part two of the Advent of Code 2022 puzzle, returns the puzzle answer
fn solve_part_two(monkey_in_the_middle: &mut MonkeyInTheMiddle) -> u64 {
    let amount_of_rounds = 10000;
    println!("Part 2:");
    //We need to find a common divisor for all of the test_divisor values, we can attain this value by
    //multiplying all of the test divisors together. https://sites.millersville.edu/bikenaga/number-theory/divisibility/divisibility.pdf
    //If i understand it correctly, we can use the least common multiple to divide the balooning worry levels: https://en.wikipedia.org/wiki/Least_common_multiple
    monkey_in_the_middle.calculate_least_common_multiple();

    monkey_in_the_middle.print_monkeys();
    for _ in 0..amount_of_rounds {
        monkey_in_the_middle.play_round_part_2();
        monkey_in_the_middle.print_round_result();
        println!();
    }
    monkey_in_the_middle.print_amount_of_items_inspected_per_monkey();
    let monkey_business = monkey_in_the_middle.calculate_monkey_business();
    println!(
        "The level of monkey business in this situation is: {}",
        monkey_business
    );
    monkey_business
}
