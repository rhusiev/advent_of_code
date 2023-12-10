use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

fn main() {
    let lefts = ["L", "F", "-"];
    let rights = ["J", "7", "-"];
    let ups = ["F", "7", "|"];
    let downs = ["J", "L", "|"];

    let file = File::open("input.txt").expect("File not found");
    // let file = File::open("test_input_second1.txt").expect("File not found");
    // let file = File::open("test_input_second2.txt").expect("File not found");
    // let file = File::open("test_input_second3.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let mut field: Vec<String> = Vec::from_iter(file_reader.lines().filter_map(|line| line.ok()));
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
    let mut s_directions = vec![];
    if left.0 >= 0
        && left.1 >= 0
        && lefts.contains(
            &field[left.0 as usize]
                .chars()
                .nth(left.1 as usize)
                .unwrap()
                .to_string()
                .as_str(),
        )
    {
        connected.get_mut(&s).unwrap().push(left);
        connected.insert(left, vec![s]);
        not_visited.push(left);
        s_directions.push("left");
    }
    if right.0 >= 0
        && right.1 >= 0
        && rights.contains(
            &field[right.0 as usize]
                .chars()
                .nth(right.1 as usize)
                .unwrap()
                .to_string()
                .as_str(),
        )
    {
        connected.get_mut(&s).unwrap().push(right);
        connected.insert(right, vec![s]);
        not_visited.push(right);
        s_directions.push("right");
    }
    if up.0 >= 0
        && up.1 >= 0
        && ups.contains(
            &field[up.0 as usize]
                .chars()
                .nth(up.1 as usize)
                .unwrap()
                .to_string()
                .as_str(),
        )
    {
        connected.get_mut(&s).unwrap().push(up);
        connected.insert(up, vec![s]);
        not_visited.push(up);
        s_directions.push("up");
    }
    if down.0 >= 0
        && down.1 >= 0
        && downs.contains(
            &field[down.0 as usize]
                .chars()
                .nth(down.1 as usize)
                .unwrap()
                .to_string()
                .as_str(),
        )
    {
        connected.get_mut(&s).unwrap().push(down);
        connected.insert(down, vec![s]);
        not_visited.push(down);
        s_directions.push("down");
    }
    let s_figure = match s_directions.as_slice() {
        ["left", "right"] | ["right", "left"] => {
            "-".to_string()
        },
        ["up", "down"] | ["down", "up"] => {
            "|".to_string()
        },
        ["left", "up"] | ["up", "left"] => {
            "J".to_string()
        },
        ["left", "down"] | ["down", "left"] => {
            "7".to_string()
        },
        ["right", "up"] | ["up", "right"] => {
            "L".to_string()
        },
        ["right", "down"] | ["down", "right"] => {
            "F".to_string()
        },
        _ => panic!("Unknown figure: {:?}", s_directions),
    };
    field[s.0 as usize] = field[s.0 as usize].replace("S", s_figure.as_str());
    visited.push(s);
    // println!("Connected: {:?}", connected);
    // println!("Visited: {:?}", visited);
    // println!("Not visited: {:?}", not_visited);
    while not_visited.len() > 0 {
        let current = not_visited.pop().unwrap();
        let mut to_check = vec![];
        let element = field[current.0 as usize]
            .chars()
            .nth(current.1 as usize)
            .unwrap()
            .to_string();
        match element.as_str() {
            "L" => {
                to_check.push((current.0, current.1 + 1));
                to_check.push((current.0 - 1, current.1));
            }
            "J" => {
                to_check.push((current.0, current.1 - 1));
                to_check.push((current.0 - 1, current.1));
            }
            "F" => {
                to_check.push((current.0, current.1 + 1));
                to_check.push((current.0 + 1, current.1));
            }
            "7" => {
                to_check.push((current.0, current.1 - 1));
                to_check.push((current.0 + 1, current.1));
            }
            "-" => {
                to_check.push((current.0, current.1 - 1));
                to_check.push((current.0, current.1 + 1));
            }
            "|" => {
                to_check.push((current.0 - 1, current.1));
                to_check.push((current.0 + 1, current.1));
            }
            _ => panic!("Unknown element: {}", element),
        }
        to_check = to_check
            .into_iter()
            .filter(|x| !visited.contains(x))
            .collect();
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
    // println!("Visited len / 2: {:?}", visited.len() / 2);
    let mut horizontal_counts: Vec<Vec<i64>> = vec![];
    for i in 0..field.len() {
        horizontal_counts.push(vec![]);
        for j in 0..field[i].len() {
            let element = field[i].chars().nth(j).unwrap().to_string();
            if visited.contains(&(i as i64, j as i64)) {
                if ups.contains(&element.as_str()) || downs.contains(&element.as_str()) {
                    if j == 0 {
                        horizontal_counts[i].push(1);
                        continue;
                    }
                    let mut next = horizontal_counts[i].last().unwrap_or(&0) + 1;
                    let element = field[i].chars().nth(j).unwrap().to_string();
                    let mut prev_j = j - 1;
                    let mut prev_element = field[i].chars().nth(prev_j).unwrap().to_string();
                    while prev_element == "-" {
                        prev_element = field[i].chars().nth(prev_j).unwrap().to_string();
                        if prev_j == 0 {
                            break;
                        }
                        prev_j -= 1;
                    }
                    if element == "7" && prev_element == "L" ||
                        element == "J" && prev_element == "F" {
                        next += 1;
                    }
                    horizontal_counts[i].push(next);
                } else {
                    let next = horizontal_counts[i].last().unwrap_or(&0) + 0;
                    horizontal_counts[i].push(next.clone());
                }
            } else {
                let next = horizontal_counts[i].last().unwrap_or(&0) + 0;
                horizontal_counts[i].push(next.clone());
            }
        }
    }
    let mut result = 0;
    for i in 0..field.len() {
        // println!("Hor: {:?}", horizontal_counts[i]);
        // println!("Ver: {:?}", vertical_counts[i]);
        for j in 0..field[i].len() {
            let horizontal_count = horizontal_counts[i][j];
            if horizontal_count % 2 == 1 && !visited.contains(&(i as i64, j as i64)) {
                result += 1;
            }
        }
    }
    println!("Result: {}", result);
}
