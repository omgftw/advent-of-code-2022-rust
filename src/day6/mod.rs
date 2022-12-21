use std::fs;

fn is_unique(chars: &[char]) -> bool {
    let mut unique = true;
    for (i, c) in chars.iter().enumerate() {
        for (j, d) in chars.iter().enumerate() {
            if i != j && c == d {
                unique = false;
                break;
            }
        }
    }
    unique
}

fn process_stream(input: String, marker_length: usize) -> usize {
    let char_window = input.chars().collect::<Vec<char>>();
    for (index, chars) in char_window.windows(marker_length).enumerate() {
        if is_unique(chars) {
            return index + marker_length;
        }
    }
    0
}

pub fn main() -> Result<(usize, usize), String> {
    let input = fs::read_to_string("data/day6-input.txt").expect("Unable to read file");
    let part1 = process_stream(input.clone(), 4);
    let part2 = process_stream(input, 14);
    return Ok((part1, part2));
}