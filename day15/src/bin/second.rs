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
        .map(|line| {
            line.split(',')
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .unwrap()
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    let ascii_codes: Vec<usize> = line
        .iter()
        .map(|s| {
            let mut code = 0;
            let chars;
            if s.ends_with('-') {
                chars = s.chars().take(s.len() - 1).collect::<Vec<char>>();
            } else {
                chars = s.chars().take(s.len() - 2).collect::<Vec<char>>();
            }
            for c in chars {
                code = ((code + c as usize) * 17) % 256;
            }
            code
        })
        .collect::<Vec<usize>>();
    // println!("{:?}", line);
    // println!("{:?}", ascii_codes);
    let mut boxes: Vec<Vec<(String, usize)>> =
        vec![Vec::new(); *ascii_codes.iter().max().unwrap() + 1];
    for (i, code) in ascii_codes.iter().enumerate() {
        if line[i].ends_with('-') {
            boxes[*code].retain(|pair| pair.0 != line[i][0..line[i].len() - 1]);
        } else {
            let (lense, focal) = line[i].split_at(line[i].len() - 2);
            let found = boxes[*code]
                .iter()
                .position(|pair| pair.0 == lense.to_string());
            if found.is_none() {
                boxes[*code].push((lense.to_string(), focal[1..].parse::<usize>().unwrap()));
                continue;
            } else {
                boxes[*code][found.unwrap()].1 = focal[1..].parse::<usize>().unwrap();
            }
        }
    }
    println!("{:?}", boxes.iter().enumerate().map(|(i, lenses)| lenses.iter().enumerate()
        .map(|(j, (_, focal))| (i + 1) * (j + 1) * focal).sum::<usize>()).sum::<usize>());
}
