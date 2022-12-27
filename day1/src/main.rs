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

    let part1 = input[0];
    let part2 = input.iter().take(3).sum::<usize>();
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    assert_eq!(part1, 67016);
    assert_eq!(part2, 200116);
}
