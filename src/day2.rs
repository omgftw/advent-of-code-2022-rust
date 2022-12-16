use std::collections::HashMap;
use std::fs;

struct RpsRound {
    their_move: String,
    other: String,
}

// map for a,b,c to rock,paper,scissors

fn parse(input: &str) -> Vec<RpsRound> {
    input
        .lines()
        .map(|line| {
            let mut moves = line.split_whitespace();
            RpsRound {
                their_move: moves.next().unwrap().to_string(),
                other: moves.next().unwrap().to_string(),
            }
        })
        .collect()
}

pub fn main() -> Result<(u32, u32), String> {
    let their_move_map: HashMap<&str, &str> = HashMap::from([
        ("A", "rock"),
        ("B", "paper"),
        ("C", "scissors"),
    ]);

    let other_move_map: HashMap<&str, &str> = HashMap::from([
        ("X", "rock"),
        ("Y", "paper"),
        ("Z", "scissors"),
    ]);

    let needed_result_map: HashMap<&str, &str> = HashMap::from([
        ("X", "lose"),
        ("Y", "draw"),
        ("Z", "win"),
    ]);

    let move_value_map = HashMap::from([
        ("rock", 1),
        ("paper", 2),
        ("scissors", 3),
    ]);

    let win_map = HashMap::from([
        ("rock", "scissors"),
        ("paper", "rock"),
        ("scissors", "paper"),
    ]);

    // read inputs/day2.txt
    // let input = fs::read_to_string("inputs/example1.txt").expect("Unable to read file");
    let input = fs::read_to_string("inputs/day2.txt").expect("Unable to read file");
    let rounds = parse(&input);
    let mut score1 = 0;
    let mut score2 = 0;
    for round in rounds {
        // index of their_move in this round
        let their_move = *their_move_map.get(&round.their_move.as_str()).unwrap();
        let other_move = *other_move_map.get(&round.other.as_str()).unwrap();
        let their_move_value = *move_value_map.get(their_move).unwrap();
        let other_move_value = *move_value_map.get(other_move).unwrap();
        let round_value = if their_move_value == other_move_value {
            3 // tie
        // } else if their_move_value == (other_move_value + 1) % 3 {
        } else if win_map.get(other_move).unwrap() == &their_move {
            6 // win
        } else {
            0 // lose
        };


        score1 += round_value + move_value_map.get(other_move).unwrap();
    }

    return Ok((score1, 0));
}