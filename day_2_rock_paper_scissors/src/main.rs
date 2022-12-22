use core::panic;
use std::io::{BufRead, Write};

fn main() {
    //---Copy this to every puzzle program main---
    // File paths
    let relative_puzzle_path = "puzzle/";
    let input_file_path = format!("{}{}", relative_puzzle_path, "INPUT");
    let output_1_path = format!("{}{}", relative_puzzle_path, "OUTPUT_PART_ONE");
    let output_2_path = format!("{}{}", relative_puzzle_path, "OUTPUT_PART_TWO");

    //Open file in Rust
    let input_file = std::fs::File::open(input_file_path).unwrap();
    let mut output_1_file = std::fs::File::create(output_1_path).unwrap();
    let mut output_2_file = std::fs::File::create(output_2_path).unwrap();
    let mut reader = std::io::BufReader::new(input_file);
    let mut line = String::new();
    //---End---
    let mut total_score = 0;
    //let mut i = 0;
    while reader.read_line(&mut line).unwrap() > 0 {
        //line.pop(); //Remove trailing new-line character
        //Do stuff
        //print!("[{}]: {}", i, line);
        //i += 1;
        let opponent = HandSign::get_hand_sign(line.chars().nth(0).unwrap()).unwrap();
        let player = HandSign::get_hand_sign(line.chars().nth(2).unwrap()).unwrap();
        let score = play_round(player, opponent);
        total_score += score;

        line.clear(); //Clear line string
    }
    //Part 1
    writeln!(output_1_file, "{}", total_score).unwrap();
    //Part 2
    writeln!(output_2_file, "{}", "To do").unwrap();
}

#[derive(PartialEq)]
enum HandSign {
    Rock,
    Paper,
    Scissors,
}

impl HandSign {
    ///Get the HandSign associated with an character from our encrypted strategy guide for playing rock, paper, scissors
    fn get_hand_sign(encrypted_character: char) -> Result<HandSign, String> {
        static VALID_CHARACTERS: [char; 6] = ['A', 'B', 'C', 'X', 'Y', 'Z'];
        if VALID_CHARACTERS.contains(&encrypted_character) {
            match encrypted_character {
                'A' | 'X' => Ok(HandSign::Rock),
                'B' | 'Y' => Ok(HandSign::Paper),
                'C' | 'Z' => Ok(HandSign::Scissors),
                _ => panic!(
                    "This should never happen, something must be wrong with VALID_CHARACTERS."
                ),
            }
        } else {
            Err(format!("The encrypted_character ({}) is invalid. A valid encrypted_character is a member of {:?}.", encrypted_character, VALID_CHARACTERS))
        }
    }
}

enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    ///Returns the result of a game of rock, paper, scissors
    fn get_game_result(player: HandSign, opponent: HandSign) -> GameResult {
        if player == opponent {
            GameResult::Draw
        } else if player == HandSign::Rock && opponent == HandSign::Scissors
            || player == HandSign::Paper && opponent == HandSign::Rock
            || player == HandSign::Scissors && opponent == HandSign::Paper
        {
            GameResult::Win
        } else {
            GameResult::Lose
        }
    }
}

///Play a round of rock, paper, scissors, returns your score for the round
fn play_round(player: HandSign, opponent: HandSign) -> i32 {
    let hand_sign_score: i32 = match player {
        HandSign::Rock => 1,
        HandSign::Paper => 2,
        HandSign::Scissors => 3,
    };
    let game_result = GameResult::get_game_result(player, opponent);
    let game_result_score = match game_result {
        GameResult::Win => 6,
        GameResult::Draw => 3,
        GameResult::Lose => 0,
    };
    let round_score = hand_sign_score + game_result_score;
    round_score
}
