// // import regex
// use regex::Regex;
//
// struct Monkey {
//     id: u32,
//     starting_items: Vec<u32>,
//     op_operator: String,
//     op_value: String,
//     test_operator: String,
//     test_value: u32,
//     true_monkey_id: u32,
//     false_monkey_id: u32,
//     inspection_count: i32
// }
//
// impl Clone for Monkey {
//     fn clone(&self) -> Self {
//         Monkey {
//             id: self.id,
//             starting_items: self.starting_items.clone(),
//             op_operator: self.op_operator.clone(),
//             op_value: self.op_value.clone(),
//             test_operator: self.test_operator.clone(),
//             test_value: self.test_value,
//             true_monkey_id: self.true_monkey_id,
//             false_monkey_id: self.false_monkey_id,
//             inspection_count: self.inspection_count
//         }
//     }
// }
//
// fn gcd(a: u32, b: u32) -> u32 {
//     if b == 0 {
//         a
//     } else {
//         gcd(b, a % b)
//     }
// }
//
// fn lcm(a: u32, b: u32) -> u32 {
//     a * b / gcd(a, b)
// }
//
// fn parse_monkeys(data: String) -> Vec<Monkey> {
//     let mut monkeys: Vec<Monkey> = vec![];
//     let unprocessed_monkeys: Vec<&str> = data.split("\n\n").collect();
//     for monkey in unprocessed_monkeys {
//         let lines: Vec<&str> = monkey.split("\n").collect();
//
//         let monkey_id_regex = Regex::new(r"Monkey (\d+):").unwrap();
//         let monkey_id = monkey_id_regex.captures(lines[0]).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();
//
//         let starting_items_regex = Regex::new(r"Starting items: (.+)").unwrap();
//         let starting_items = starting_items_regex.captures(lines[1]).unwrap().get(1).unwrap().as_str().split(", ").map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
//
//         let op_regex = Regex::new(r"Operation: new = old (.) (.+)").unwrap();
//         let captures = op_regex.captures(lines[2]).unwrap();
//         let op_operator = captures.get(1).unwrap().as_str().to_string();
//         let op_value = captures.get(2).unwrap().as_str().to_string();
//
//         let test_regex = Regex::new(r"Test: (\w+) by (\d)").unwrap();
//         let test_operator = test_regex.captures(lines[3]).unwrap().get(1).unwrap().as_str().to_string();
//         let test_value = test_regex.captures(lines[3]).unwrap().get(2).unwrap().as_str().parse::<u32>().unwrap();
//
//         let test_result_regex = Regex::new(r"If (\w+): throw to monkey (\d+)").unwrap();
//         let true_monkey_id = test_result_regex.captures(lines[4]).unwrap().get(2).unwrap().as_str().parse::<u32>().unwrap();
//         let false_monkey_id = test_result_regex.captures(lines[5]).unwrap().get(2).unwrap().as_str().parse::<u32>().unwrap();
//
//         // let true_monkey_id_regex = Regex::new(r"Monkey #(\d+)").unwrap();
//         // let true_monkey_id = true_monkey_id_regex.captures(lines[4]).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();
//         // let false_monkey_id_regex = Regex::new(r"Monkey #(\d+)").unwrap();
//         // let false_monkey_id = false_monkey_id_regex.captures(lines[5]).unwrap().get(1).unwrap().as_str().parse::<u32>().unwrap();
//
//
//
//
//         // let starting_items: Vec<u32> = lines[1].split(": ").collect::<Vec<&str>>()[1].split(", ").map(|s| s.parse::<u32>().unwrap()).collect();
//         // let op_operator: String = lines[2].split(" ").collect::<Vec<&str>>()[2].to_string();
//         // let op_value: String = lines[2].split(" ").collect::<Vec<&str>>()[3].replace(".", "");
//         // let test_operator: String = lines[3].split(" ").collect::<Vec<&str>>()[1].to_string();
//         // let test_value: String = lines[3].split(" ").collect::<Vec<&str>>()[3].replace(".", "");
//         // let true_monkey_id = lines[4].split(" ").collect::<Vec<&str>>()[3].replace(".", "");
//         // let false_monkey_id = lines[5].split(" ").collect::<Vec<&str>>()[3].replace(".", "");
//         let monkey = Monkey {
//             // id: monkey_id.parse::<u32>().unwrap(),
//             id: monkey_id,
//             starting_items,
//             // starting_items,
//             op_operator,
//             // op_value: if op_value.parse::<u32>().is_ok() { op_value.parse::<u32>().unwrap() } else { op_value.parse::<String>().unwrap() },
//             op_value,
//             test_operator,
//             test_value,
//             true_monkey_id,
//             false_monkey_id,
//             // test_value: test_value.parse::<u32>().unwrap(),
//             // true_monkey_id: true_monkey_id.parse::<u32>().unwrap(),
//             // false_monkey_id: false_monkey_id.parse::<u32>().unwrap(),
//             inspection_count: 0,
//         };
//         monkeys.push(monkey);
//     }
//     monkeys
// }
//
// fn simulate_rounds(
//     monkeys: &mut [Monkey],
//     rounds: u32,
//     reduce_worry_each_round: bool,
// ) {
//     // Get LCM of all test values to keep worry levels low
//     // let modulo = lcm(&monkeys.iter().map(|monkey| monkey.test_value).collect());
//     let modulo = monkeys.iter().map(|monkey| monkey.test_value).fold(1, |acc, x| lcm(acc, x));
//
//     for rnd in 0..rounds {
//         print!("\rSimulating: {}", rnd);
//         // for i in length of monkeys
//         for i in 0..monkeys.len() {
//         // for monkey in monkeys.clone() {
//             let mut new_starting_items = monkeys[i].starting_items.clone();
//             for x in 0..monkeys[i].starting_items.len() {
//             // for (index, value) in monkeys[i].starting_items.iter().enumerate() {
//                 let value = monkeys[i].starting_items[x];
//                 monkeys[i].inspection_count += 1;
//
//                 let op_value = if monkeys[i].op_value == "old".to_string() {
//                     value.clone()
//                 } else {
//                     // monkey.op_value.clone()
//                     monkeys[i].op_value.parse::<u32>().unwrap()
//                 };
//
//                 // map op_operator to an operator
//                 let operator = match monkeys[i].op_operator.as_str() {
//                     "+" => |x, y| x + y,
//                     "*" => |x, y| x * y,
//                     _ => panic!("Unsupported operator"),
//                 };
//
//                 // map test_operator to an operator
//                 let test_operator = match monkeys[i].test_operator.as_str() {
//                     "divisible" => |x, y| x % y == 0,
//                     _ => panic!("Unsupported operator"),
//                 };
//
//                 // apply the operation
//                 // let result = operator(value.clone(), op_value);
//                 let result = operator(value, op_value);
//
//                 let result = if reduce_worry_each_round {
//                     // value is divided by 3 and floor rounded
//                     result / 3
//                 } else {
//                     result % modulo
//                 };
//
//                 // Update value in new_starting_items at index with result
//                 new_starting_items[x] = result.clone();
//
//                 // apply the test
//                 let next_monkey_id = if test_operator(result, monkeys[i].test_value) {
//                     monkeys[i].true_monkey_id
//                 } else {
//                     monkeys[i].false_monkey_id
//                 };
//                 // monkeys[next_monkey_id as usize].starting_items.push(result);
//                 monkeys[next_monkey_id as usize].starting_items.push(result);
//
//             }
//             monkeys[i].starting_items = vec![];
//         }
//     }
//     println!("");
// }
//
// pub(crate) fn main() {
//     let raw_data = std::fs::read_to_string("inputs/day11.txt").unwrap();
//     // let mut monkey_data = parse_monkeys(&raw_data);
//     let mut monkey_data = parse_monkeys(raw_data.clone());
//     simulate_rounds(&mut monkey_data, 20, true);
//     // get top two monkeys by inspection count
//     let mut top_monkeys = monkey_data.iter().take(2).collect::<Vec<&Monkey>>();
//     // let mut top_monkeys = monkey_data
//     //     .iter()
//     //     .sorted_by(|x, y| Ord::cmp(&y.inspection_count, &x.inspection_count))
//     //     .take(2)
//     //     .collect::<Vec<_>>();
//     // multiply their inspection counts
//     let result1 = top_monkeys[0].inspection_count * top_monkeys[1].inspection_count;
//
//     // // let mut monkey_data = parse_monkeys(&raw_data);
//     // let mut monkey_data = parse_monkeys(raw_data);
//     // simulate_rounds(&mut monkey_data, 10000, false);
//     // // get top two monkeys by inspection count
//     // let mut top_monkeys = monkey_data.iter().take(2).collect::<Vec<&Monkey>>();
//     // // let mut top_monkeys = monkey_data
//     // //     .iter()
//     // //     .sorted_by(|x, y| Ord::cmp(&y.inspection_count, &x.inspection_count))
//     // //     .take(2)
//     // //     .collect::<Vec<_>>();
//     // // multiply their inspection counts
//     // let result2 = top_monkeys[0].inspection_count * top_monkeys[1].inspection_count;
//
//     println!("Part 1: {}", result1);
//     // println!("Part 2: {}", result2);
// }