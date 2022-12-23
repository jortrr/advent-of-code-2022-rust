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
    let mut total_score_part_1 = 0;
    let mut total_score_part_2 = 0;
    //let mut i = 0;
    while reader.read_line(&mut line).unwrap() > 0 {
        //line.pop(); //Remove trailing new-line character
        //Do stuff
        //print!("[{}]: {}", i, line);
        //i += 1;
        let first_char = line.chars().nth(0).unwrap();
        let second_char = line.chars().nth(2).unwrap();
        //Part one
        let opponent = HandSign::get_hand_sign(first_char).unwrap();
        let player_part_1 = HandSign::get_hand_sign(second_char).unwrap();
        let score_part_1 = play_round(player_part_1, &opponent);
        total_score_part_1 += score_part_1;
        //Part two
        let game_result = GameResult::get_game_result_from_char(second_char).unwrap();
        let player_part_2 = HandSign::get_hand_sign_from_game_result(&opponent, game_result);
        let score_part_2 = play_round(player_part_2, &opponent);
        total_score_part_2 += score_part_2;

        line.clear(); //Clear line string
    }
    //Part 1
    writeln!(output_1_file, "{}", total_score_part_1).unwrap();
    //Part 2
    writeln!(output_2_file, "{}", total_score_part_2).unwrap();
}

#[derive(PartialEq, Clone, Copy)]
enum HandSign {
    Rock,
    Paper,
    Scissors,
}

impl HandSign {
    ///Get the HandSign associated with a character from our encrypted strategy guide for playing rock, paper, scissors
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

    fn get_hand_sign_from_game_result(opponent: &HandSign, game_result: GameResult) -> HandSign {
        match game_result {
            GameResult::Win => HandSign::loses_against(*opponent),
            GameResult::Draw => HandSign::draws_against(*opponent),
            GameResult::Lose => HandSign::wins_against(*opponent),
        }
    }

    ///Returns the HandSign that hand_sign wins against
    fn wins_against(hand_sign: HandSign) -> HandSign {
        match hand_sign {
            HandSign::Rock => HandSign::Scissors,
            HandSign::Paper => HandSign::Rock,
            HandSign::Scissors => HandSign::Paper,
        }
    }

    ///Returns the HandSign that hand_sign loses against
    fn loses_against(hand_sign: HandSign) -> HandSign {
        match hand_sign {
            HandSign::Rock => HandSign::Paper,
            HandSign::Paper => HandSign::Scissors,
            HandSign::Scissors => HandSign::Rock,
        }
    }

    ///Returns the HandSign that hand_sign draws against
    fn draws_against(hand_sign: HandSign) -> HandSign {
        return hand_sign;
    }
}

#[derive(PartialEq)]
enum GameResult {
    Win,
    Lose,
    Draw,
}

impl GameResult {
    ///Returns the result of a game of rock, paper, scissors
    fn get_game_result(player: HandSign, opponent: &HandSign) -> GameResult {
        if HandSign::wins_against(player) == *opponent {
            GameResult::Win
        } else if HandSign::loses_against(player) == *opponent {
            GameResult::Lose
        } else {
            GameResult::Draw
        }
    }

    ///Get the GameResult associated with a character from our encrypted strategy guide for playing rock, paper, scissors (part two)
    fn get_game_result_from_char(encrypted_character: char) -> Result<GameResult, String> {
        static VALID_CHARACTERS: [char; 3] = ['X', 'Y', 'Z'];
        if VALID_CHARACTERS.contains(&encrypted_character) {
            match encrypted_character {
                'X' => Ok(GameResult::Lose),
                'Y' => Ok(GameResult::Draw),
                'Z' => Ok(GameResult::Win),
                _ => panic!(
                    "This should never happen, something must be wrong with VALID_CHARACTERS."
                ),
            }
        } else {
            Err(format!("The encrypted_character ({}) is invalid. A valid encrypted_character is a member of {:?}.", encrypted_character, VALID_CHARACTERS))
        }
    }
}

///Play a round of rock, paper, scissors, returns your score for the round
fn play_round(player: HandSign, opponent: &HandSign) -> i32 {
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
