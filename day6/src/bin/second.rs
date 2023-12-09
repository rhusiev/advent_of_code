use std::{fs::File, io::BufRead, io::BufReader};

fn get_num_of_possible_races(time: u128, distance: u128) -> u128 {
    let x_upper = (time as f64 + ((time * time - 4 * distance) as f64).sqrt()) / 2.0;
    let x_lower = (time as f64 - ((time * time - 4 * distance) as f64).sqrt()) / 2.0;
    let x_upper_floor = x_upper.floor() as u128;
    let x_lower_ceil = x_lower.ceil() as u128;
    let mut res = x_upper_floor - x_lower_ceil + 1;
    if x_upper.fract() == 0.0 {
        res -= 1;
    }
    if x_lower.fract() == 0.0 {
        res -= 1;
    }
    return res;
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    // let file = File::open("test_input.txt").expect("File not found");
    let lines = BufReader::new(file)
        .lines()
        .filter_map(|line| line.ok())
        .collect::<Vec<String>>();
    let time: u128 = lines[0][8..]
        .replace(" ", "")
        .parse::<u128>()
        .unwrap();
    let distance: u128 = lines[1][9..]
        .replace(" ", "")
        .parse::<u128>()
        .unwrap();
    println!("{} {}", time, distance);
    let total = get_num_of_possible_races(time, distance);
    println!("{}", total);
}
