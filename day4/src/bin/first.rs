use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    // let file = File::open("test_input.txt").expect("File not found");
    let file = File::open("input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let cards: Vec<(Vec<u32>, Vec<u32>)> = Vec::from_iter(
        file_reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| {
                line.split(": ").collect::<Vec<&str>>()[1]
                    .split(" | ")
                    .map(|numbers| numbers.split(" ")
                         .filter(|number| !number.is_empty())
                        .map(|number| number.parse::<u32>().unwrap())
                        .collect::<Vec<u32>>())
                    .collect::<Vec<Vec<u32>>>()
            })
            .map(|numbers| (numbers[0].clone(), numbers[1].clone())),
    );
    let values: Vec<u32> = cards
        .iter()
        .map(|card| {
            let mut value = 0;
            for number in card.1.iter() {
                if card.0.contains(number) {
                    if value == 0 {
                        value = 1;
                    } else {
                        value *= 2;
                    }
                }
            }
            return value;
        })
        .collect();
    println!("{:?}", values.iter().sum::<u32>());
}
