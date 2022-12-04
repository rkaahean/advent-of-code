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

fn compute_score(opp_move: char, outcome: char) -> i32 {
    let mut score = 0;

    // storing individual scores in a dictionary
    let mut move_scores: HashMap<char, i32> = HashMap::new();
    move_scores.insert('A', 1);
    move_scores.insert('B', 2);
    move_scores.insert('C', 3);

    // stores scores for wins, draws and losses
    let mut games_scores: HashMap<char, i32> = HashMap::new();
    games_scores.insert('X', 0);
    games_scores.insert('Y', 3);
    games_scores.insert('Z', 6);   
    
    // get the score for the game
    score += games_scores.get(&outcome).copied().unwrap();

    // get the score for the move I should be playing
    let my_move = compute_my_move(opp_move, outcome);
    score += move_scores.get(&my_move).copied().unwrap();
    println!("{}", score);
    score


}

fn compute_my_move(opp_move: char, desired_outcome: char) -> char {
    let mut my_move: char = 'A';

    // storing game logic
    let mut game_logic: HashMap<char, char> = HashMap::new();

    // {key} wins against {value}
    game_logic.insert('C', 'B');
    game_logic.insert('B', 'A');
    game_logic.insert('A', 'C');
 
    if desired_outcome == 'X' {
        // find the move which loses to opp_move
        // find the opp move in the key. value = your move
        my_move = game_logic.get(&opp_move).copied().unwrap();
    } 
    else if desired_outcome == 'Z' {
        // find the move which wins against opp_move
        // find the opp move in the value. key = your move
        for mv in game_logic.keys() {
            if game_logic.get(mv).copied().unwrap() == opp_move {
                my_move = opp_move
            }
        }
    }
    else {
        my_move = opp_move;
    }
    my_move
}