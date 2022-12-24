pub fn main() {
    let input = include_str!("input.txt");
    let input = input.split("\n\n").collect::<Vec<&str>>();
    let mut input = input
        .iter()
        .map(|x| {
            x.split('\n')
                .map(|x| x.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<usize>>();
    input.sort_by(|a, b| b.cmp(a));

    println!("Part 1: {}", input[0]);
    println!("Part 2: {}", input.iter().take(3).sum::<usize>());
}
