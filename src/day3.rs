use std::error::Error;

pub fn day3() -> Result<(usize, usize), Box<dyn Error>> {
    let chars = "abcdefghijklmnopqrstuvwxyz";
    let chars = chars.to_string() + chars.to_uppercase().as_str();

    let get_priority = |char: char| -> usize {
        return chars.find(char).unwrap() + 1;
    };

    fn find_common(x: &str, y: &str) -> char {
        for char in x.chars() {
            if y.contains(char) {
                return char;
            }
        }
        return ' ';
    }

    fn find_common_3(x: &str, y: &str, z: &str) -> char {
        let mut chars = vec![];
        for char in x.chars() {
            if y.contains(char) {
                chars.push(char);
            }
        }
        for char in chars {
            if z.contains(char) {
                return char;
            }
        }
        return ' ';
    }

    let data = std::fs::read_to_string("inputs/day3.txt").unwrap();
    let data: Vec<&str> = data.split("\n").collect();
    let mut total = 0;
    for rucksack in data.iter() {
        let compartment_size = rucksack.len() / 2;
        let compartment1 = &rucksack[0..compartment_size];
        let compartment2 = &rucksack[compartment_size..];

        let common = find_common(compartment1, compartment2);
        let priority = get_priority(common);

        total += priority;
    }
    let part1_total = total;

    let group_size = 3;
    let mut total = 0;
    for i in (0..data.len()).step_by(group_size) {
        let items = &data[i..i + group_size];
        let common = find_common_3(items[0], items[1], items[2]);
        let priority = get_priority(common);
        total += priority;
    }
    let part2_total = total;
    
    return Ok((part1_total, part2_total));
}