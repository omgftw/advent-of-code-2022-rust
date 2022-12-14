pub fn day1() {
    let input = std::fs::read_to_string("day1.txt").expect("Error reading day1.txt");
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let mut input = input
        .iter()
        .map(|x| {
            x.split("\n")
                .map(|x| x.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    input.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {}", input[0]);
    println!("Part 2: {}", input.iter().take(3).sum::<i32>());
}
