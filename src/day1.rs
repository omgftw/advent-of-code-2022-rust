use std::error::Error;

pub fn day1() -> Result<(usize, usize), Box<dyn Error>> {
    let input = std::fs::read_to_string("inputs/day1.txt").expect("Error reading day1.txt");
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let mut input = input
        .iter()
        .map(|x| {
            x.split("\n")
                .map(|x| x.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();
    input.sort_by(|a, b| b.cmp(a));

    // println!("Part 1: {}", input[0]);
    // println!("Part 2: {}", input.iter().take(3).sum::<usize>());
    // return first result and the sum of the top three results
    Ok((input[0], input.iter().take(3).sum::<usize>()))
}
