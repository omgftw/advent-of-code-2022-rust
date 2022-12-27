use std::collections::HashMap;

fn handle_job(job: &str, monkey_map: &HashMap<&str, &str>) -> i64 {
    return match job.parse::<i64>() {
        Ok(job) => {
            job
        },
        Err(_) => {
            let mut parts = job.split_whitespace();
            let monkey1 = parts.next().unwrap();
            let operator = parts.next().unwrap();
            let monkey2 = parts.next().unwrap();
            let monkey1 = monkey_map.get(monkey1).unwrap();
            let monkey2 = monkey_map.get(monkey2).unwrap();
            let monkey1 = handle_job(monkey1, monkey_map);
            let monkey2 = handle_job(monkey2, monkey_map);
            match operator {
                "+" => monkey1 + monkey2,
                "-" => monkey1 - monkey2,
                "*" => monkey1 * monkey2,
                "/" => monkey1 / monkey2,
                _ => panic!("Unknown operator"),
            }
        },
    };
}

fn main() {
    // let input = include_str!("example.txt");
    let input = include_str!("input.txt");
    let mut monkey_map: HashMap<&str, &str> = HashMap::new();
    for monkey in input.lines() {
        let mp = monkey.split(':').collect::<Vec<&str>>();
        let name = mp[0];
        let job = mp[1].trim();
        monkey_map.insert(name, job);
    }

    let result = handle_job(monkey_map["root"], &monkey_map);

    println!("Part1: {}", result);

    assert_eq!(result, 364367103397416);
}
