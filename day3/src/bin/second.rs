use std::{fs::File, io::BufRead, io::BufReader};

fn in_occupied(index: (usize, usize), occupied_indexes: &Vec<(usize, usize, usize)>) -> bool {
    occupied_indexes.iter().any(|occupied_index| {
        index.0 == occupied_index.0 && index.1 >= occupied_index.1 && index.1 <= occupied_index.2
    })
}

fn push_possible_digit(
    symbol_index: (usize, usize),
    numbers: &mut Vec<String>,
    line: &String,
    occupied_indexes: &mut Vec<(usize, usize, usize)>,
) {
    // println!("Trying to push {:?} from {:?}", symbol_index, line);
    // println!("Occupied indexes: {:?}", occupied_indexes);
    if line.chars().nth(symbol_index.1).unwrap().is_numeric() {
        if !in_occupied(symbol_index, occupied_indexes) {
            let (number, start, end) = get_number_from_index(symbol_index, line);
            // println!("Pushing {} from {} to {}", number, start, end);
            numbers.push(number);
            occupied_indexes.push((symbol_index.0, start, end));
        } else {
            // println!("Already occupied");
        }
    } else {
        // println!("Not numeric");
    }
    // println!();
}

fn get_number_from_index(index: (usize, usize), line: &String) -> (String, usize, usize) {
    let mut start = index.1;
    let mut end = index.1;
    while start > 0 && line.chars().nth(start - 1).unwrap().is_numeric() {
        start -= 1;
    }
    while end < line.len() - 1 && line.chars().nth(end + 1).unwrap().is_numeric() {
        end += 1;
    }
    let number = line[start..=end].to_string();
    (number, start, end)
}

fn get_adjacent_numbers(
    index: (usize, usize),
    engine: &Vec<String>,
) -> Vec<String> {
    let (width, height) = (engine[0].len(), engine.len());
    let mut adjacent_numbers: Vec<String> = Vec::new();
    let mut occupied_indexes: Vec<(usize, usize, usize)> = Vec::new();
    match index {
        (0, 0) => {
            push_possible_digit((0, 1), &mut adjacent_numbers, &engine[0], &mut occupied_indexes);
            push_possible_digit((1, 0), &mut adjacent_numbers, &engine[1], &mut occupied_indexes);
            push_possible_digit((1, 1), &mut adjacent_numbers, &engine[1], &mut occupied_indexes);
        }
        (y, x) if y == height - 1 && x == width - 1 => {
            push_possible_digit(
                (y - 1, x - 1),
                &mut adjacent_numbers,
                &engine[y - 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y - 1, x),
                &mut adjacent_numbers,
                &engine[y - 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y, x - 1),
                &mut adjacent_numbers,
                &engine[y],
                &mut occupied_indexes,
            );
        }
        (y, x) if y == height - 1 && x == 0 => {
            push_possible_digit(
                (y - 1, x),
                &mut adjacent_numbers,
                &engine[y - 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y - 1, x + 1),
                &mut adjacent_numbers,
                &engine[y - 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y, x + 1),
                &mut adjacent_numbers,
                &engine[y],
                &mut occupied_indexes,
            );
        }
        (y, x) if y == 0 && x == width - 1 => {
            push_possible_digit(
                (y, x - 1),
                &mut adjacent_numbers,
                &engine[y],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y + 1, x - 1),
                &mut adjacent_numbers,
                &engine[y + 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y + 1, x),
                &mut adjacent_numbers,
                &engine[y + 1],
                &mut occupied_indexes,
            );
        }
        (y, x) if y == 0 => {
            push_possible_digit(
                (y, x - 1),
                &mut adjacent_numbers,
                &engine[y],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y, x + 1),
                &mut adjacent_numbers,
                &engine[y],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y + 1, x - 1),
                &mut adjacent_numbers,
                &engine[y + 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y + 1, x),
                &mut adjacent_numbers,
                &engine[y + 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y + 1, x + 1),
                &mut adjacent_numbers,
                &engine[y + 1],
                &mut occupied_indexes,
            );
        }
        (y, x) if y == height - 1 => {
            push_possible_digit(
                (y - 1, x - 1),
                &mut adjacent_numbers,
                &engine[y - 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y - 1, x),
                &mut adjacent_numbers,
                &engine[y - 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y - 1, x + 1),
                &mut adjacent_numbers,
                &engine[y - 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y, x - 1),
                &mut adjacent_numbers,
                &engine[y],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y, x + 1),
                &mut adjacent_numbers,
                &engine[y],
                &mut occupied_indexes,
            );
        }
        (y, x) if x == 0 => {
            push_possible_digit(
                (y - 1, x),
                &mut adjacent_numbers,
                &engine[y - 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y - 1, x + 1),
                &mut adjacent_numbers,
                &engine[y - 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y, x + 1),
                &mut adjacent_numbers,
                &engine[y],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y + 1, x),
                &mut adjacent_numbers,
                &engine[y + 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y + 1, x + 1),
                &mut adjacent_numbers,
                &engine[y + 1],
                &mut occupied_indexes,
            );
        }
        (y, x) if x == width - 1 => {
            push_possible_digit(
                (y - 1, x - 1),
                &mut adjacent_numbers,
                &engine[y - 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y - 1, x),
                &mut adjacent_numbers,
                &engine[y - 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y, x - 1),
                &mut adjacent_numbers,
                &engine[y],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y + 1, x - 1),
                &mut adjacent_numbers,
                &engine[y + 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y + 1, x),
                &mut adjacent_numbers,
                &engine[y + 1],
                &mut occupied_indexes,
            );
        }
        (y, x) => {
            push_possible_digit(
                (y - 1, x - 1),
                &mut adjacent_numbers,
                &engine[y - 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y - 1, x),
                &mut adjacent_numbers,
                &engine[y - 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y - 1, x + 1),
                &mut adjacent_numbers,
                &engine[y - 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y, x - 1),
                &mut adjacent_numbers,
                &engine[y],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y, x + 1),
                &mut adjacent_numbers,
                &engine[y],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y + 1, x - 1),
                &mut adjacent_numbers,
                &engine[y + 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y + 1, x),
                &mut adjacent_numbers,
                &engine[y + 1],
                &mut occupied_indexes,
            );
            push_possible_digit(
                (y + 1, x + 1),
                &mut adjacent_numbers,
                &engine[y + 1],
                &mut occupied_indexes,
            );
        }
    }
    if adjacent_numbers.len() == 2 {
        return adjacent_numbers;
    }
    Vec::new()
}

fn main() {
    // let file = File::open("test_input.txt").expect("File not found");
    let file = File::open("input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let engine: Vec<String> = Vec::from_iter(file_reader.lines().filter_map(|line| line.ok()));
    let symbol_indexes: Vec<(usize, usize)> = engine
        .iter()
        .enumerate()
        .map(|(i, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, chr)| *chr == '*')
                .map(|(j, _)| (i, j))
                .collect::<Vec<(usize, usize)>>()
        })
        .flatten()
        .collect();
    let numbers: Vec<u64> = symbol_indexes
        .iter()
        .map(|index| get_adjacent_numbers(*index, &engine))
        .filter(|numbers| numbers.len() == 2)
        .map(|gear| gear.iter().map(|number| number.parse::<u64>().unwrap()).product::<u64>())
        .collect();
    println!("{:?}", numbers.iter().sum::<u64>());
    // println!("{:?}", numbers);
}
