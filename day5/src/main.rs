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

fn main() {
    // let input = include_str!("example.txt");
    let input = include_str!("input.txt");
    let mut parts = input.split("\n\n");
    let crates = parts.next().unwrap().lines().collect::<Vec<_>>();
    let commands = parts.next().unwrap().lines().collect::<Vec<_>>();

    let (last, crates) = crates.split_last().unwrap();
    let max_stack = last.split_whitespace().last().unwrap().parse::<usize>().unwrap();

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

    let mut part2_stacks = stacks.clone();

    // let r = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    let commands = parse_commands(commands);

    for command in commands.iter() {
        // let r = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        // let caps = r.captures(command).unwrap();
        // let amount = caps[1].parse::<usize>().unwrap();
        // let from = caps[2].parse::<usize>().unwrap();
        // let to = caps[3].parse::<usize>().unwrap();
        // let r = command.split_whitespace().collect::<Vec<_>>();
        // let amount = r[1].parse::<usize>().unwrap();
        // let from = r[3].parse::<usize>().unwrap();
        // let to = r[5].parse::<usize>().unwrap();


        for _ in 0..command.amount {
            let item = stacks[command.from-1].pop().unwrap();
            stacks[command.to-1].push(item);
        }
    }

    for command in commands.iter() {
        // let r = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        // let caps = r.captures(command).unwrap();
        // let amount = caps[1].parse::<usize>().unwrap();
        // let from = caps[2].parse::<usize>().unwrap();
        // let to = caps[3].parse::<usize>().unwrap();
        // let r = command.split_whitespace().collect::<Vec<_>>();
        // let amount = r[1].parse::<usize>().unwrap();
        // let from = r[3].parse::<usize>().unwrap();
        // let to = r[5].parse::<usize>().unwrap();

        // rewrite this
        let mut items = part2_stacks[command.from-1].iter().rev().take(command.amount).cloned().collect::<Vec<_>>();
        let from_len = part2_stacks[command.from-1].len();
        part2_stacks[command.from-1].truncate( from_len - command.amount);
        items.reverse();
        for item in items.iter() {
            part2_stacks[command.to-1].push(*item);
        }
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
}
