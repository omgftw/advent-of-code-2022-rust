fn main() {
    // let input = include_str!("example.txt");
    let input = include_str!("input.txt");

    let priority = ('a'..='z').chain('A'..='Z').collect::<Vec<_>>();

    let mut priorities: Vec<usize> = vec![];

    for line in input.lines() {
        let first = &line[..(line.len() / 2)];
        let second = &line[(line.len() / 2)..];
        for c in first.chars() {
            if second.contains(c) {
                let p = priority.iter().position(|&x| x == c).unwrap() + 1;
                priorities.push(p);
                break;
            }
        }
    }

    let part1 = priorities.iter().sum::<usize>();

    let groups = input.lines().collect::<Vec<_>>();
    let groups = groups.chunks(3).collect::<Vec<_>>();

    let mut priorities: Vec<usize> = vec![];
    for group in groups {
        // find the common item in the group
        let mut common = group[0].chars().collect::<Vec<_>>();
        for elf in group[1..].iter() {
            common = common
                .iter()
                .filter(|&x| elf.contains(*x))
                .copied()
                .collect::<Vec<_>>();
        }
        priorities.push(priority.iter().position(|&c| c == common[0]).unwrap() + 1);
    }

    let part2 = priorities.iter().sum::<usize>();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    assert_eq!(part1, 8139);
    assert_eq!(part2, 2668);
}
