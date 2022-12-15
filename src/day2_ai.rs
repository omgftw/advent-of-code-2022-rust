use std::collections::HashMap;
use std::fs;

pub fn main() -> Result<(usize, usize), Box<dyn std::error::Error>> {
    let mut value: HashMap<&str, usize> = HashMap::new();
    value.insert("rock", 1);
    value.insert("paper", 2);
    value.insert("scissors", 3);
    let mut outcomes: HashMap<&str, usize> = HashMap::new();
    outcomes.insert("win", 6);
    outcomes.insert("draw", 3);
    outcomes.insert("lose", 0);
    let mut choices: HashMap<&str, &str> = HashMap::new();
    choices.insert("A", "rock");
    choices.insert("B", "paper");
    choices.insert("C", "scissors");
    choices.insert("X", "rock");
    choices.insert("Y", "paper");
    choices.insert("Z", "scissors");
    let mut day2_choices: HashMap<&str, &str> = HashMap::new();
    day2_choices.insert("X", "lose");
    day2_choices.insert("Y", "draw");
    day2_choices.insert("Z", "win");
    let mut beats: HashMap<&str, &str> = HashMap::new();
    beats.insert("rock", "scissors");
    beats.insert("paper", "rock");
    beats.insert("scissors", "paper");

    let play_round = |you: &str, them: &str| -> usize {
        if you == them {
            return outcomes["draw"];
        } else if them == beats[you] {
            return outcomes["win"];
        } else {
            return outcomes["lose"];
        }
    };

    let raw_data = fs::read_to_string("inputs/day2.txt").unwrap();
    let mut data: Vec<HashMap<&str, &str>> = Vec::new();
    for one_round in raw_data.split("\n") {
        let mut tmp: HashMap<&str, &str> = HashMap::new();
        let mut tmp_data = one_round.split(" ");
        tmp.insert("them", tmp_data.next().unwrap());
        tmp.insert("you", tmp_data.next().unwrap());
        data.push(tmp);
    }

    let mut total = 0;
    for one_round in data.iter() {
        let your_choice = choices[one_round["you"]];
        let their_choice = choices[one_round["them"]];
        let choice_value = value[your_choice];
        let round_value = play_round(your_choice, their_choice);
        total += choice_value + round_value;
    }
    // println!("Part 1: {}", total);
    let part1_total = total;

    let mut total = 0;
    for one_round in data.iter() {
        let their_choice = choices[one_round["them"]];
        let round_result = day2_choices[one_round["you"]];
        let your_choice: &str;
        if round_result == "draw" {
            your_choice = their_choice;
        } else if round_result == "lose" {
            your_choice = beats[their_choice];
        } else {
            your_choice = *beats.iter().find(|&(_, &v)| v == their_choice).unwrap().0;
        }
        let choice_value = value[your_choice];
        let round_value = play_round(your_choice, their_choice);
        total += choice_value + round_value;
    }
    // println!("Part 2: {}", total);
    let part2_total = total;

    return Ok((part1_total, part2_total));
}