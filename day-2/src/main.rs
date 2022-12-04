use std::fs;
use std::collections::HashMap;

const WIN_SCORE: i32 = 6;
const DRAW_SCORE: i32 = 3;
const LOSE_SCORE: i32 = 0;

fn main() {
    let wins: HashMap<&str, &str> = HashMap::from([
        ("rock", "scissors"),
        ("paper", "rock"),
        ("scissors", "paper"),
    ]);

    let codes: HashMap<&str, &str> = HashMap::from([
        ("A", "rock"),
        ("B", "paper"),
        ("C", "scissors"),
        ("X", "rock"),
        ("Y", "paper"),
        ("Z", "scissors"),
    ]);

    fn game_result(pairs: &Vec<(&str, &str)>, wins: HashMap<&str, &str>) -> i32 {
        let play_scores: HashMap<&str, i32> = HashMap::from([
            ("rock", 1),
            ("paper", 2),
            ("scissors", 3),
        ]);

        let mut total_score = 0;
        for (a, b) in pairs {
            total_score += play_scores[b];

            if wins.get(b).unwrap() == a {
                total_score += WIN_SCORE;
            } else if a == b {
                total_score += DRAW_SCORE;
            } else {
                total_score += LOSE_SCORE;
            }
        }

        total_score
    }

    let input = fs::read_to_string("../2.input").unwrap();
    let pairs = input.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let a = parts.next().unwrap();
        let b = parts.next().unwrap();
        (a, b)
    });

    let direct_response_pairs = pairs.clone().map(|(a, b)| {
        (codes[a], codes[b])
    }).collect::<Vec<(&str, &str)>>();

    println!("{}", game_result(&direct_response_pairs, wins.clone()));

    let losses = wins.iter().map(|(a, b)| (*b, *a)).collect::<HashMap<&str, &str>>();

    let target_result_pairs = pairs.map(|(a, b)| {
        let other_player = codes[a]; // rock, paper, scissors
        let target_result = b; // lose, draw, win (as X, Y, Z)

        let play = match target_result {
            "X" => wins[other_player],
            "Y" => other_player,
            "Z" => losses[other_player],
            _ => panic!("Invalid target result: {}", target_result),
        };

        (other_player, play)
    }).collect::<Vec<(&str, &str)>>();

    println!("{}", game_result(&target_result_pairs, wins));
}
