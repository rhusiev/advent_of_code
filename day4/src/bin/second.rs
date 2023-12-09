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
                    value += 1;
                }
            }
            return value;
        })
        .collect();

    let mut counts: Vec<u32> = vec![1; values.len()];
    for i in 0..counts.len() {
        for j in 0..values[i] {
            let index: usize = i as usize + j as usize + 1;
            if index < counts.len() {
                counts[index] += counts[i];
            }
        }
    }
    println!("{:?}", counts.iter().sum::<u32>());
}
