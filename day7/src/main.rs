use std::collections::HashMap;

#[derive(Debug)]
struct Item {
    path: String,
    parent_path: String,
    is_dir: bool,
    size: u64,
}

fn main() {
    let input = include_str!("example.txt");
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
                let name = parts[2];
                if name == ".." {
                    let item = &items[&cur_path];
                    cur_path = item.parent_path.clone();
                    continue;
                }
                if name == "/" {
                    cur_path = "/".to_string();
                    continue;
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
                cur_path = path;
            } else if parts[1] == "ls" {
                println!();
            }
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

    println!("{:#?}", items);
}
