use serde_json;
use std::fs;
use std::error::Error;

fn correct_order(ai: &serde_json::Value, bi: &serde_json::Value) -> Option<bool> {
    let mut a = ai;
    let mut b = bi;
    if a.is_number() && b.is_number() {
        if a.as_f64().unwrap() < b.as_f64().unwrap() {
            return Some(true);
        }
        if a.as_f64().unwrap() > b.as_f64().unwrap() {
            return Some(false);
        }
        if a.as_f64().unwrap() == b.as_f64().unwrap() {
            return None;
        }
    }
    let temp_a = serde_json::json!([a]);
    let temp_b = serde_json::json!([b]);
    if !a.is_array() {
        a = &temp_a;
    } else if !b.is_array() {
        b = &temp_b;
    }

    for i in 0..std::cmp::max(a.as_array().unwrap().len(), b.as_array().unwrap().len()) {
        if b.as_array().unwrap().len() <= i {
            return Some(false);
        }
        if a.as_array().unwrap().len() <= i {
            return Some(true);
        }
        let result = correct_order(&a.as_array().unwrap()[i], &b.as_array().unwrap()[i]);
        if result != None {
            return result;
        }
    }

    return None;
}

pub fn main() -> Result<(usize, usize), Box<dyn Error>> {
    // let input = fs::read_to_string("./example.txt")?;
    let input = fs::read_to_string("data/day13-input.txt")?;
    let raw_pairs: Vec<&str> = input.split("\n\n").collect();
    let mut pairs: Vec<(serde_json::Value, serde_json::Value)> = Vec::new();
    for pair in raw_pairs {
        let mut split_pair = pair.split("\n");
        let left = serde_json::from_str(split_pair.next().unwrap())?;
        let right = serde_json::from_str(split_pair.next().unwrap())?;
        pairs.push((left, right));
    }

    let mut results: Vec<Option<bool>> = Vec::new();
    for (left, right) in pairs.iter() {
        let result = correct_order(&left, &right);
        results.push(result);
    }
    let mut correct_indices: Vec<usize> = Vec::new();
    for (i, result) in results.iter().enumerate() {
        if *result == Some(true) {
            correct_indices.push(i + 1);
        }
    }

    let mut all_pairs: Vec<serde_json::Value> = pairs
        .into_iter().flat_map(|(a, b)| vec![a, b]).collect();
    // Add divider packets
    all_pairs.extend_from_slice(&[serde_json::json!([[2]]), serde_json::json!([[6]])]);
    all_pairs.sort_by(|a, b| match correct_order(a, b) {
        Some(true) => std::cmp::Ordering::Less,
        Some(false) => std::cmp::Ordering::Greater,
        None => std::cmp::Ordering::Equal,
    });
    let divider1 = all_pairs.iter().position(|x| x == &serde_json::json!([[2]])).unwrap() + 1;
    let divider2 = all_pairs.iter().position(|x| x == &serde_json::json!([[6]])).unwrap() + 1;

    let part1 = correct_indices.iter().sum::<usize>();
    let part2 = divider1 * divider2;

    Ok((part1, part2))
}
