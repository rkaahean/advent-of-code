use std::fs;
use std::collections::HashMap;

fn main() {
    // trying to read the input data
    println!("Reading input file...");

    // specifying the library
    let file_path = "/Users/ranjanwork-personal/rustaoc/src/data/aoc2.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something wrong in reading the file...");

    // get the moves line by line
    let moves = contents.lines();

    // lets collect the socres!
    let mut my_score = 0;
    for game_move in moves {
        let player_moves: Vec<&str> = game_move.split_whitespace().collect();

        // compute my score for every move!
        my_score += compute_score(
            player_moves[0].chars().nth(0).unwrap(),
            player_moves[1].chars().nth(0).unwrap()
        )

    }
    println!("{my_score}")

}

fn compute_score(opp_move: char, my_move: char) -> i32 {
    let mut score = 0;
    let am_i_winner = rock_paper_scissor(opp_move, my_move);

    // storing individual scores in a dictionary
    let mut move_scores: HashMap<char, i32> = HashMap::new();
    move_scores.insert('X', 1);
    move_scores.insert('Y', 2);
    move_scores.insert('Z', 3);

    // storing move definitions
    let mut move_definitions: HashMap<char, String> = HashMap::new();
    move_definitions.insert('X', "ROCK".to_string());
    move_definitions.insert('Y', "PAPER".to_string());
    move_definitions.insert('Z', "SCISSORS".to_string());
    move_definitions.insert('A', "ROCK".to_string());
    move_definitions.insert('B', "PAPER".to_string());
    move_definitions.insert('C', "SCISSORS".to_string());


    
    // in case both players play the same thing
    if move_definitions.get(&opp_move).unwrap() == move_definitions.get(&my_move).unwrap() {
        score += 3
    }

    // check if i am winner and compute score accordingly
    if am_i_winner {
        score += 6
    }

    // compute score for the indivual moves I make
    score += move_scores.get(&my_move).copied().unwrap();
    score


}

fn rock_paper_scissor(opp_move: char, my_move: char) -> bool{
    let mut is_win = false;
    if opp_move == 'A' && my_move == 'Y' {
        is_win = true
    }
    if opp_move == 'B' && my_move == 'Z' {
        is_win = true
    }
    if opp_move == 'C' && my_move == 'X' {
        is_win = true
    }
    is_win
}