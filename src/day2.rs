use std::collections::HashMap;
use std::fs;

struct RpsRound {
    their_move: String,
    other: String,
}

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
    let win_value_map = HashMap::from([
        ("win", 6),
        ("draw", 3),
        ("lose", 0),
    ]);

    let win_map = HashMap::from([
        ("rock", "scissors"),
        ("paper", "rock"),
        ("scissors", "paper"),
    ]);

    // let input = fs::read_to_string("inputs/example2.txt").expect("Unable to read file");
    let input = fs::read_to_string("inputs/day2.txt").expect("Unable to read file");
    let rounds = parse(&input);
    let mut score1 = 0;
    let mut score2 = 0;
    for round in rounds {
        let their_move = *their_move_map.get(&round.their_move.as_str()).unwrap();
        let other_move = *other_move_map.get(&round.other.as_str()).unwrap();
        let their_move_value = *move_value_map.get(their_move).unwrap();
        let other_move_value = *move_value_map.get(other_move).unwrap();
        let round_value = if their_move_value == other_move_value {
            win_value_map.get("draw").unwrap()
        } else if win_map.get(other_move).unwrap() == &their_move {
            win_value_map.get("win").unwrap()
        } else {
            win_value_map.get("lose").unwrap()
        };

        let needed_result = *needed_result_map.get(&round.other.as_str()).unwrap();
        let needed_move = if needed_result == "win" {
            win_map.iter().find(|(_, v)| **v == their_move).unwrap().0
        } else if needed_result == "draw" {
            their_move
        } else {
            win_map.get(their_move).unwrap()
        };

        score1 += round_value + move_value_map.get(other_move).unwrap();
        score2 += win_value_map.get(needed_result).unwrap() + move_value_map.get(needed_move).unwrap();
    }

    return Ok((score1, score2));
}