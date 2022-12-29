use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Item {
    parent_path: String,
    is_dir: bool,
    size: u64,
}

fn handle_cd(name: &str, mut cur_path: String, items: &mut HashMap<String, Item>) -> String {
    if name == ".." {
        let item = &items[&cur_path];
        cur_path = item.parent_path.clone();
        return cur_path;
    }
    if name == "/" {
        cur_path = "/".to_string();
        return cur_path;
    }
    let path = cur_path.to_string() + name + "/";
    if !items.contains_key(&path) {
        let item = Item {
            parent_path: cur_path.to_string(),
            is_dir: true,
            size: 0,
        };
        items.insert(path.clone(), item);
    }
    path
}

fn parse_terminal(input: &str) -> HashMap<String, Item> {
    let mut items: HashMap<String, Item> = HashMap::new();
    items.insert(
        "/".to_string(),
        Item {
            parent_path: "/".to_string(),
            is_dir: true,
            size: 0,
        },
    );
    let mut cur_path = "/".to_string();
    for line in input.lines() {
        let parts = line.split_whitespace().collect::<Vec<_>>();
        if parts[0] == "$" {
            if parts[1] == "cd" {
                cur_path = handle_cd(parts[2], cur_path, &mut items);
            }
        } else if parts[0] == "dir" {
            let name = parts[1];
            let path = cur_path.to_string() + name;
            if !items.contains_key(&path) {
                let item = Item {
                    parent_path: cur_path.to_string(),
                    is_dir: true,
                    size: 0,
                };
                items.insert(path.clone(), item);
            }
        } else {
            let size = parts[0].parse::<u64>().unwrap();
            let name = parts[1];
            let path = cur_path.to_string() + name;
            let item = Item {
                parent_path: cur_path.to_string(),
                is_dir: false,
                size,
            };
            items.insert(path.clone(), item);
        }
    }
    items
}

fn propagate_size(item: &Item, items: &mut HashMap<String, Item>) {
    let mut queue = vec![item.parent_path.clone()];
    while !queue.is_empty() {
        let path = queue.pop().unwrap();
        let parent = items.get_mut(&path).unwrap();
        if parent.is_dir {
            parent.size += item.size;
            if path == "/" {
                break;
            }
            queue.push(parent.parent_path.clone());
        }
    }
}

fn main() {
    // let input = include_str!("example.txt");
    let input = include_str!("input.txt");
    let mut items = parse_terminal(input);
    for item in items.clone().values_mut() {
        propagate_size(item, &mut items);
    }

    let items_under_100k = items
        .values()
        .filter(|item| item.is_dir && item.size < 100_000)
        .collect::<Vec<_>>();

    // sum size of all files under 100k
    let sum = items_under_100k.iter().map(|item| item.size).sum::<u64>();

    let fs_size = 70_000_000;
    let space_needed = 30_000_000;
    let space_used = items["/"].size;
    let space_left = fs_size - space_used;
    let space_needed = space_needed - space_left;

    // find smallest dir that is larger than space_needed
    let smallest_delete_dir = items
        .values()
        .filter(|item| item.is_dir && item.size > space_needed)
        .min_by_key(|item| item.size)
        .unwrap();

    println!("Part 1: {}", sum);
    println!("Part 2: {}", smallest_delete_dir.size);

    assert_eq!(sum, 2_061_777);
    assert_eq!(smallest_delete_dir.size, 4_473_403);
}
