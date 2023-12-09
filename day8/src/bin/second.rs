use std::{collections::HashMap, fs::File, io::BufRead, io::BufReader};

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    return gcd(b, a % b);
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    // let file = File::open("test_input3.txt").expect("File not found");
    // let file = File::open("test_input2.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let file_lines = file_reader
        .lines()
        .map(|l| l.unwrap())
        .collect::<Vec<String>>();
    let instructions = &file_lines[0];
    let mut map: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
    for i in 2..file_lines.len() {
        let line = &file_lines[i];
        let mut split = line.split(" = ");
        let key = split.next().unwrap();
        let values_str = split.next().unwrap();
        let values = values_str[1..values_str.len() - 1]
            .split(", ")
            .collect::<Vec<&str>>();
        let mut value_map: HashMap<&str, &str> = HashMap::new();
        value_map.insert("L", values[0]);
        value_map.insert("R", values[1]);
        map.insert(key, value_map);
    }
    let vertexes: Vec<&str> = map
        .keys()
        .filter(|k| k.ends_with("A"))
        .map(|k| *k)
        .collect();
    let mut paths_counts: Vec<usize> = Vec::new();
    for vertex in vertexes {
        println!("Vertex {}", vertex);
        let mut current_vertex = vertex;
        let mut path_counts: Vec<usize> = Vec::new();
        let mut path: Vec<(&str, usize)> = Vec::new();
        let mut i = 0;
        let mut count = 0;
        loop {
            while !current_vertex.ends_with("Z") {
                let next_step = instructions.chars().nth(i % instructions.len()).unwrap();
                let next_vertex = map.get(current_vertex).unwrap().get(std::str::from_utf8(&[next_step as u8]).unwrap());
                if next_vertex.is_none() {
                    break;
                }
                current_vertex = next_vertex.unwrap();
                i += 1;
                count += 1;
            }
            if path.contains(&(current_vertex, i % instructions.len())) {
                println!("Path counts {:?}", path_counts);
                paths_counts.push(path_counts.iter().sum::<usize>());
                break;
            }
            path_counts.push(count);
            path.push((current_vertex, i % instructions.len()));
            count = 0;
            let next_step = instructions.chars().nth(i % instructions.len()).unwrap();
            let next_vertex = map.get(current_vertex).unwrap().get(std::str::from_utf8(&[next_step as u8]).unwrap());
            if next_vertex.is_none() {
                break;
            }
            current_vertex = next_vertex.unwrap();
            i += 1;
            count += 1;
        }
    }
    println!("Paths counts {:?}", paths_counts);
    // Find lowest common multiple
    let mut lcm = paths_counts[0];
    for i in 1..paths_counts.len() {
        lcm = lcm * paths_counts[i] / gcd(lcm, paths_counts[i]);
    }
    println!("Lowest common multiple {}", lcm);
}
