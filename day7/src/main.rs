use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Item {
    path: String,
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
            path: path.clone(),
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
            path: "/".to_string(),
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
            // else if parts[1] == "ls" {
            //     println!();
            // }
        }

        else if parts[0] == "dir" {
            let name = parts[1];
            let path = cur_path.to_string() + name;
            if !items.contains_key(&path) {
                let item = Item {
                    path: path.clone(),
                    parent_path: cur_path.to_string(),
                    is_dir: true,
                    size: 0,
                };
                items.insert(path.clone(), item);
            }
        }

        else {
            let size = parts[0].parse::<u64>().unwrap();
            let name = parts[1];
            let path = cur_path.to_string() + name;
            // if !items.contains_key(&path) {
            let item = Item {
                path: path.clone(),
                parent_path: cur_path.to_string(),
                is_dir: false,
                size,
            };
            items.insert(path.clone(), item);
            // }
        }
    }
    items
}

fn main() {
    let input = include_str!("example.txt");
    let mut items = parse_terminal(input);
    for item in items.clone().values_mut() {
        if !item.is_dir {
            let parent = items.get_mut(&item.parent_path).unwrap();
            parent.size += item.size;
        }
    }

    println!("{:#?}", items);
}
