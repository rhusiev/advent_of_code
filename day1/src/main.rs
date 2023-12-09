use std::{fs::File, io::BufRead, io::BufReader};

fn get_first_last_digits(line: String) -> Option<u32> {
    let line_copy = line.clone()
        .replace("one", "one1one")
        .replace("two", "two2two")
        .replace("three", "three3three")
        .replace("four", "four4four")
        .replace("five", "five5five")
        .replace("six", "six6six")
        .replace("seven", "seven7seven")
        .replace("eight", "eight8eight")
        .replace("nine", "nine9nine");
    let mut first: u8 = 255;
    let mut last: u8 = 255;
    for chr in line_copy.chars() {
        if chr.is_numeric() {
            if first == 255 {
                first = chr as u8 - 48;
            }
            last = chr as u8 - 48;
        }
    }
    if first != 255 && last != 255 {
        return Some((first * 10 + last) as u32);
    }
    return None;
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let numbers: Vec<u32> = Vec::from_iter(
        file_reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| get_first_last_digits(line).unwrap()),
    );
    println!("{:?}", numbers.iter().sum::<u32>());
}
