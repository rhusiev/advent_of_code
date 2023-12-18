use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let file = File::open("input.txt").expect("File not found");
    // let file = File::open("test_input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let line: Vec<String> = file_reader
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>()
        .first()
        .map(|line| line.split(',')
            .map(|s| s.to_string())
            .collect::<Vec<String>>())
        .unwrap()
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let ascii_codes: Vec<usize> = line.iter()
        .map(|s| {
            let mut code = 0;
            for c in s.chars() {
                code = ((code + c as usize) * 17) % 256;
            }
            code
        })
        .collect::<Vec<usize>>();
    // println!("{:?}", line);
    // println!("{:?}", ascii_codes);
    println!("{}", ascii_codes.iter().sum::<usize>());
}
