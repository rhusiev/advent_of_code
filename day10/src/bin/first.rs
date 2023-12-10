use std::{fs::File, io::BufRead, io::BufReader, collections::HashMap};

fn main() {
    let lefts = ["L", "F", "-"];
    let rights = ["J", "7", "-"];
    let ups = ["F", "7", "|"];
    let downs = ["J", "L", "|"];

    let file = File::open("input.txt").expect("File not found");
    // let file = File::open("test_input1.txt").expect("File not found");
    // let file = File::open("test_input2.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let field: Vec<String> = Vec::from_iter(
        file_reader
            .lines()
            .filter_map(|line| line.ok())
    );
    // println!("{:?}", field);
    // Find S:
    let mut s: (i64, i64) = (-1, -1);
    for (i, line) in field.iter().enumerate() {
        if let Some(j) = line.find('S') {
            s = (i as i64, j as i64);
            break;
        }
    }
    if s == (-1, -1) {
        panic!("No S found");
    }
    // println!("S: {:?}", s);
    let mut connected: HashMap<(i64, i64), Vec<(i64, i64)>> = HashMap::new();
    let mut visited: Vec<(i64, i64)> = vec![];
    let mut not_visited: Vec<(i64, i64)> = vec![];
    connected.insert(s, vec![]);
    let left = (s.0, s.1 - 1);
    let right = (s.0, s.1 + 1);
    let up = (s.0 - 1, s.1);
    let down = (s.0 + 1, s.1);
    if left.0 >= 0 && left.1 >= 0 && lefts.contains(&field[left.0 as usize].chars().nth(left.1 as usize).unwrap().to_string().as_str()) {
        connected.get_mut(&s).unwrap().push(left);
        connected.insert(left, vec![s]);
        not_visited.push(left);
    }
    if right.0 >= 0 && right.1 >= 0 && rights.contains(&field[right.0 as usize].chars().nth(right.1 as usize).unwrap().to_string().as_str()) {
        connected.get_mut(&s).unwrap().push(right);
        connected.insert(right, vec![s]);
        not_visited.push(right);
    }
    if up.0 >= 0 && up.1 >= 0 && ups.contains(&field[up.0 as usize].chars().nth(up.1 as usize).unwrap().to_string().as_str()) {
        connected.get_mut(&s).unwrap().push(up);
        connected.insert(up, vec![s]);
        not_visited.push(up);
    }
    if down.0 >= 0 && down.1 >= 0 && downs.contains(&field[down.0 as usize].chars().nth(down.1 as usize).unwrap().to_string().as_str()) {
        connected.get_mut(&s).unwrap().push(down);
        connected.insert(down, vec![s]);
        not_visited.push(down);
    }
    visited.push(s);
    // println!("Connected: {:?}", connected);
    // println!("Visited: {:?}", visited);
    // println!("Not visited: {:?}", not_visited);
    while not_visited.len() > 0 {
        let current = not_visited.pop().unwrap();
        let mut to_check = vec![];
        let element = field[current.0 as usize].chars().nth(current.1 as usize).unwrap().to_string();
        match element.as_str() {
            "L" => {
                to_check.push((current.0, current.1 + 1));
                to_check.push((current.0 - 1, current.1));
            },
            "J" => {
                to_check.push((current.0, current.1 - 1));
                to_check.push((current.0 - 1, current.1));
            },
            "F" => {
                to_check.push((current.0, current.1 + 1));
                to_check.push((current.0 + 1, current.1));
            },
            "7" => {
                to_check.push((current.0, current.1 - 1));
                to_check.push((current.0 + 1, current.1));
            },
            "-" => {
                to_check.push((current.0, current.1 - 1));
                to_check.push((current.0, current.1 + 1));
            },
            "|" => {
                to_check.push((current.0 - 1, current.1));
                to_check.push((current.0 + 1, current.1));
            },
            _ => panic!("Unknown element: {}", element),
        }
        to_check = to_check.into_iter().filter(|x| !visited.contains(x)).collect();
        if !visited.contains(&current) {
            visited.push(current);
        }
        not_visited.extend(to_check.clone());
        for check in to_check {
            if check.0 < 0 || check.1 < 0 {
                continue;
            }
            if !connected.contains_key(&check) {
                connected.insert(check, vec![current]);
            } else {
                connected.get_mut(&check).unwrap().push(current);
            }
            if !connected.contains_key(&current) {
                connected.insert(current, vec![check]);
            } else {
                connected.get_mut(&current).unwrap().push(check);
            }
        }
    }
    // println!("Connected: {:?}", connected);
    // println!("Visited: {:?}", visited);
    println!("Visited len / 2: {:?}", visited.len() / 2);
}
