#[derive(Clone, Copy, Debug)]
struct Point {
    x: usize,
    y: usize,
    elevation: isize,
}

#[derive(Debug)]
struct BoardPoint {
    pos: Point,
    visited: bool,
}

#[derive(Clone, Copy, Debug)]
struct Path {
    point: Point,
    length: usize,
}

fn find_viable_neighbors(board: &mut Vec<Vec<BoardPoint>>, point: &Point) -> Vec<Point> {
    let mut neighbors = Vec::new();
    // Check around point to see if any are viable
    if point.x > 0 {
        let left = &board[point.y][point.x - 1];
        if !left.visited && (left.pos.elevation == point.elevation + 1 || left.pos.elevation <= point.elevation) {
            neighbors.push(left.pos);
        }
    }
    if point.x < board[0].len() - 1 {
        let right = &board[point.y][point.x + 1];
        if !right.visited && (right.pos.elevation == point.elevation + 1 || right.pos.elevation <= point.elevation) {
            neighbors.push(right.pos);
        }
    }
    if point.y > 0 {
        let up = &board[point.y - 1][point.x];
        if !up.visited && (up.pos.elevation == point.elevation + 1 || up.pos.elevation <= point.elevation) {
            neighbors.push(up.pos);
        }
    }
    if point.y < board.len() - 1 {
        let down = &board[point.y + 1][point.x];
        if !down.visited && (down.pos.elevation == point.elevation + 1 || down.pos.elevation <= point.elevation) {
            neighbors.push(down.pos);
        }
    }

    for neighbor in neighbors.iter() {
        board[neighbor.y][neighbor.x].visited = true;
    }

    neighbors
}

fn process_queue(queue: &mut Vec<Path>, board: &mut Vec<Vec<BoardPoint>>, target: &Point) -> Option<Path> {
    while !queue.is_empty() {
        let path = queue[0];
        queue.remove(0);
        if path.point.elevation == target.elevation {
            println!("Found end point: {:?}", path.point);
            return Some(path);
        }
        let neighbors = find_viable_neighbors(board, &path.point);
        for neighbor in neighbors.iter() {
            queue.push(Path {
                point: *neighbor,
                length: path.length + 1,
            });
        }
    }
    None
}

fn visualize_grid(board: &Vec<Vec<BoardPoint>>, elevation_order: &str, final_path: &Path) {
    for y in 0..board.len() {
        for x in 0..board[0].len() {
            let point = &board[y][x];
            if point.pos.x == final_path.point.x && point.pos.y == final_path.point.y {
                print!("[=]");
            } else if point.visited {
                print!("[{}]", elevation_order.chars().nth(point.pos.elevation as usize).unwrap());
            } else {
                print!(" {} ", elevation_order.chars().nth(point.pos.elevation as usize).unwrap());
            }
        }
        println!();
    }
}

fn main() {
    let elevation_order = "SabcdefghijklmnopqrstuvwxyzE";
    // let input = include_str!("example.txt");
    let input = include_str!("input.txt");
    // get a grid of items from input as usize based on index in elevation order
    let grid = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| elevation_order.find(c).unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Vec<usize>>>();

    // map grid into BoardPoint
    let mut board = grid
        .iter()
        .enumerate()
        .map(|(row, row_items)| {
            row_items
                .iter()
                .enumerate()
                .map(|(col, item)| BoardPoint {
                    pos: Point {
                        x: col,
                        y: row,
                        elevation: *item as isize,
                    },
                    visited: false,
                })
                .collect::<Vec<BoardPoint>>()
        })
        .collect::<Vec<Vec<BoardPoint>>>();

    let start = board
        .iter()
        .flat_map(|row| row.iter())
        .find(|point| point.pos.elevation == 0)
        .unwrap()
        .pos;

    // find end point
    let end = board
        .iter()
        .flat_map(|row| row.iter())
        .find(|point| point.pos.elevation == (elevation_order.len() - 1) as isize)
        .unwrap()
        .pos;

    // let max_elevation = board
    //     .iter()
    //     .flat_map(|row| row.iter())
    //     .map(|point| point.pos.elevation)
    //     .max()
    //     .unwrap();

    // let mut queue: Vec<Point> = vec![start];
    let mut queue: Vec<Path> = vec![Path {
        point: start,
        length: 0,
    }];
    board[start.y][start.x].visited = true;

    // let mut final_path: Path = Path {
    //     point: start,
    //     length: 0,
    // };

    let final_path = process_queue(&mut queue, &mut board, &end).unwrap();

    // while !queue.is_empty() {
    //     let path = queue[0];
    //     final_path = path;
    //     queue.remove(0);
    //     if path.point.elevation == max_elevation {
    //         println!("Found end point: {:?}", path.point);
    //         // part1 = path.length;
    //         final_path = path;
    //         break;
    //     }
    //     let neighbors = find_viable_neighbors(&mut board, &path.point);
    //     for neighbor in neighbors.iter() {
    //         queue.push(Path {
    //             point: *neighbor,
    //             length: path.length + 1,
    //         });
    //     }
    // }
    
    println!("Part 1: {}", final_path.length);
}
