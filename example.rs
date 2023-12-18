use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    // let file = File::open("input.txt").expect("File not found");
    let file = File::open("test_input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let numbers: Vec<String> = Vec::from_iter(
        file_reader
            .lines()
            .filter_map(|line| line.ok())
    );
    println!("{:?}", numbers);
}
