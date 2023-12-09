use std::{fs::File, io::BufReader, io::BufRead};

fn is_possible(game: String, configuration: (u32, u32, u32)) -> u32 {
    let (game_id, game) = game.split_at(game.find(": ").unwrap());
    for set in game[2..].split("; ") {
        // println!("Set: {:?}", set);
        for ball in set.split(", ") {
            // println!("Ball: {:?}", ball);
            let (number, color) = ball.split_at(ball.find(" ").unwrap());
            let int_number = number.parse::<u32>().unwrap();
            // println!("{}: {}", color, int_number);
            if color == " red" && int_number > configuration.0 {
                return 0;
            }
            if color == " green" && int_number > configuration.1 {
                return 0;
            }
            if color == " blue" && int_number > configuration.2 {
                return 0;
            }
            // println!("OK");
        }
    }
    return game_id[5..].parse::<u32>().unwrap();
}

fn main() {
    // let file = File::open("test_input.txt").expect("File not found");
    let file = File::open("input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let numbers: Vec<u32> = Vec::from_iter(
        file_reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| is_possible(line, (12, 13, 14))),
    );
    println!("{:?}", numbers.iter().sum::<u32>());
}
