use std::collections::HashMap;
use std::fs;

const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;
const LOSE_SCORE: i32 = 0;

fn main() {
    let wins = HashMap::from([
        ("rock", "scissors"),
        ("paper", "rock"),
        ("scissors", "paper"),
    ]);

    let codes = HashMap::from([
        ("A", "rock"),
        ("B", "paper"),
        ("C", "scissors"),
        ("X", "rock"),
        ("Y", "paper"),
        ("Z", "scissors"),
    ]);

    fn game_result(pairs: Vec<(&str, &str)>, wins: HashMap<&str, &str>) -> i32 {
        let play_scores = HashMap::from([("rock", 1), ("paper", 2), ("scissors", 3)]);

        let mut total_score = 0;
        for (opponent_move, own_move) in pairs {
            total_score += play_scores[own_move];

            total_score += if wins[own_move] == opponent_move {
                WIN_SCORE
            } else if own_move == opponent_move {
                DRAW_SCORE
            } else {
                LOSE_SCORE
            };
        }

        total_score
    }

    let input = fs::read_to_string("../2.input").unwrap();
    let pairs: Vec<_> = input
        .lines()
        .map(
            |line| match line.split_whitespace().collect::<Vec<_>>()[..] {
                [opponent_move, own_move] => (opponent_move, own_move),
                _ => panic!("Invalid input"),
            },
        )
        .collect();

    let direct_response_pairs: Vec<_> = pairs.iter().map(|(a, b)| (codes[a], codes[b])).collect();

    println!("{}", game_result(direct_response_pairs, wins.clone()));

    let losses: HashMap<_, _> = wins.iter().map(|(a, b)| (*b, *a)).collect();

    let target_result_pairs: Vec<_> = pairs
        .iter()
        .map(|(a, b)| {
            let opponent_move = codes[a]; // rock, paper, scissors
            let target_result = *b; // lose, draw, win (as X, Y, Z)

            let own_move = match target_result {
                "X" => wins[opponent_move],
                "Y" => opponent_move,
                "Z" => losses[opponent_move],
                _ => panic!("Invalid target result: {}", target_result),
            };

            (opponent_move, own_move)
        })
        .collect();

    println!("{}", game_result(target_result_pairs, wins));
}
