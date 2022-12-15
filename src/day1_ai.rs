use std::fs;

pub fn day1_ai() {
    let contents = fs::read_to_string("inputs/day1.txt")
        .expect("Something went wrong reading the file");

    let mut highest = 0;
    let mut totals: Vec<i64> = vec![];

    for (index, x) in contents.split("\n\n").enumerate() {
        let items: Vec<i64> = x.split("\n").map(|x| x.parse().unwrap()).collect();
        let total = items.iter().sum();
        if total > highest {
            highest = total;
        }
        totals.push(total);
    }

    println!("Part 1: {}", highest);
    totals.sort();
    let top_three = totals[totals.len() - 3..totals.len()].iter().sum::<i64>();
    println!("Part 2: {}", top_three);
    println!("AI");
}