use std::{fs::File, io::BufRead, io::BufReader};

fn get_num_of_possible_races(time: u32, distance: u32) -> u32 {
    let x_upper = (time as f32 + ((time * time - 4 * distance) as f32).sqrt()) / 2.0;
    let x_lower = (time as f32 - ((time * time - 4 * distance) as f32).sqrt()) / 2.0;
    let x_upper_floor = x_upper.floor() as u32;
    let x_lower_ceil = x_lower.ceil() as u32;
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
    let times: Vec<u32> = Vec::from_iter(
        lines[0][8..]
            .split(" ")
            .filter_map(|line| line.parse::<u32>().ok()),
    );
    let distances: Vec<u32> = Vec::from_iter(
        lines[1][8..]
            .split(" ")
            .filter_map(|line| line.parse::<u32>().ok()),
    );
    let mut total: u32 = 1;
    for i in 0..times.len() {
        total *= get_num_of_possible_races(times[i], distances[i]);
    }
    println!("{}", total);
}
