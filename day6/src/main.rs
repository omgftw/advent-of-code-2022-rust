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

fn process_stream(input: &str, marker_length: usize) -> usize {
    let char_window = input.chars().collect::<Vec<char>>();
    for (index, chars) in char_window.windows(marker_length).enumerate() {
        if is_unique(chars) {
            return index + marker_length;
        }
    }
    0
}

pub fn main() {
    let input = include_str!("input.txt");
    let part1 = process_stream(input, 4);
    let part2 = process_stream(input, 14);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}