use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Outcome {
    Lose,
    Draw,
    Win,
}

#[derive(Debug)]
struct Round {
    your_move: Move,
    their_move: Move,
    outcome: Outcome,
    beats: HashMap<Move, Move>,
    score: i32,
}

impl Round {
    fn new(their_move: Move, your_move: Option<Move>, outcome: Option<Outcome>) -> Round {
        let mut beats = HashMap::new();
        beats.insert(Move::Rock, Move::Scissors);
        beats.insert(Move::Paper, Move::Rock);
        beats.insert(Move::Scissors, Move::Paper);

        let mut outcome_score = HashMap::new();
        outcome_score.insert(Outcome::Lose, 0);
        outcome_score.insert(Outcome::Draw, 3);
        outcome_score.insert(Outcome::Win, 6);

        let mut choice_score = HashMap::new();
        choice_score.insert(Move::Rock, 1);
        choice_score.insert(Move::Paper, 2);
        choice_score.insert(Move::Scissors, 3);


        let mut outcome_result: Outcome = outcome.unwrap_or(Outcome::Draw);
        let mut your_move_result: Move = your_move.unwrap_or(Move::Rock);

        if let Some(ym) = your_move {
            outcome_result = match ym {
                m if m == their_move => Outcome::Draw,
                m if their_move == *beats.get(&m).unwrap() => Outcome::Win,
                _ => Outcome::Lose,
            };
        } else if let Some(oc) = outcome {
            your_move_result = match oc {
                Outcome::Draw => their_move,
                // get key of beats by value
                Outcome::Win => *beats.iter().find(|&(_, &v)| v == their_move).unwrap().0,
                Outcome::Lose => *beats.get(&their_move).unwrap(),
            };
        }


        // let outcome = match your_move {
        //     m if m == their_move => Outcome::Draw,
        //     m if their_move == *beats.get(&m).unwrap() => Outcome::Win,
        //     _ => Outcome::Lose,
        // };

        let score = outcome_score.get(&outcome_result).unwrap() + choice_score.get(&your_move_result).unwrap();

        Round {
            your_move: your_move_result,
            their_move,
            outcome: outcome_result,
            beats,
            score,
        }
    }
}

impl TryFrom<char> for Move {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Move::Rock),
            'B' | 'Y' => Ok(Move::Paper),
            'C' | 'Z' => Ok(Move::Scissors),
            _ => Err("Invalid move"),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = &'static str;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Lose),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err("Invalid outcome"),
        }
    }
}

fn main() {
    // let input = include_str!("example.txt");
    let input = include_str!("input.txt");
    let rounds = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let your_move = parts.next().unwrap().chars().next().unwrap();
            let their_move = parts.next().unwrap().chars().next().unwrap();

            Round::new(
                your_move.try_into().unwrap(),
                Some(their_move.try_into().unwrap()),
                None,
            )
        })
        .collect::<Vec<_>>();

    let part1 = rounds.iter().map(|r| r.score).sum::<i32>();

    let rounds = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let your_move = parts.next().unwrap().chars().next().unwrap();
            let outcome = parts.next().unwrap().chars().next().unwrap();

            Round::new(
                your_move.try_into().unwrap(),
                None,
                Some(outcome.try_into().unwrap()),
            )
        })
        .collect::<Vec<_>>();

    let part2 = rounds.iter().map(|r| r.score).sum::<i32>();

    // dbg!(rounds);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
