use std::{fs::File, io::BufRead, io::BufReader, collections::HashMap};

fn main() {
    let file = File::open("input.txt").expect("File not found");
    // let file = File::open("test_input1.txt").expect("File not found");
    // let file = File::open("test_input2.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let file_lines = file_reader.lines().map(|l| l.unwrap()).collect::<Vec<String>>();
    let instructions = &file_lines[0];
    let mut map: HashMap<&str, HashMap<&str, &str>> = HashMap::new();
    for i in 2..file_lines.len() {
        let line = &file_lines[i];
        let mut split = line.split(" = ");
        let key = split.next().unwrap();
        let values_str = split.next().unwrap();
        let values = values_str[1..values_str.len()-1].split(", ").collect::<Vec<&str>>();
        let mut value_map: HashMap<&str, &str> = HashMap::new();
        value_map.insert("L", values[0]);
        value_map.insert("R", values[1]);
        map.insert(key, value_map);
    }
    let mut vertex = "AAA";
    let mut count = 0;
    while vertex != "ZZZ" {
        vertex = map[vertex][instructions.chars().nth(count % instructions.len()).unwrap().to_string().as_str()];
        count += 1;
    }
    println!("{}", count);
}
