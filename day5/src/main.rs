use lazy_static::lazy_static;
use regex::Regex;

struct Command {
    amount: usize,
    from: usize,
    to: usize,
}

lazy_static! {
    static ref COMMAND_REGEX: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
}

fn parse_commands(input: Vec<&str>) -> Vec<Command> {
    let mut commands: Vec<Command> = vec![];
    for command in input.into_iter() {
        let caps = COMMAND_REGEX.captures(command).unwrap();
        let amount = caps[1].parse::<usize>().unwrap();
        let from = caps[2].parse::<usize>().unwrap();
        let to = caps[3].parse::<usize>().unwrap();
        commands.push(Command { amount, from, to });
    }
    commands
}

fn parse_input(input: &str) -> (Vec<Vec<char>>, Vec<Command>) {
    let mut parts = input.split("\n\n");
    let crates = parts.next().unwrap().lines().collect::<Vec<_>>();
    let commands = parts.next().unwrap().lines().collect::<Vec<_>>();

    let (last, crates) = crates.split_last().unwrap();
    let max_stack = last
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<usize>()
        .unwrap();

    let mut stacks: Vec<Vec<char>> = vec![vec![]; max_stack];

    for row in crates.iter() {
        for (i, chunk) in row.chars().collect::<Vec<_>>().chunks(4).enumerate() {
            if chunk[1] == ' ' {
                continue;
            }
            stacks[i].push(chunk[1]);
        }
    }

    for stack in stacks.iter_mut() {
        stack.reverse();
    }

    let commands = parse_commands(commands);

    (stacks, commands)
}

fn main() {
    // let input = include_str!("example.txt");
    let input = include_str!("input.txt");
    let (mut stacks, commands) = parse_input(input);
    let mut part2_stacks = stacks.clone();

    for command in commands.iter() {
        for _ in 0..command.amount {
            let item = stacks[command.from - 1].pop().unwrap();
            stacks[command.to - 1].push(item);
        }
    }

    for command in commands.iter() {
        let part2_stacks_len = part2_stacks[command.from - 1].len();
        let items = part2_stacks[command.from - 1].split_off(part2_stacks_len - command.amount);
        part2_stacks[command.to - 1].extend(items);
    }

    let mut part1 = "".to_string();
    for stack in stacks.iter() {
        part1 += stack.last().unwrap().to_string().as_str();
    }

    let mut part2 = "".to_string();
    for stack in part2_stacks.iter() {
        part2 += stack.last().unwrap().to_string().as_str();
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
    assert_eq!(part1, "BWNCQRMDB");
    assert_eq!(part2, "NHWZCBNBF");
}
