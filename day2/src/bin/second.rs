use std::{fs::File, io::BufReader, io::BufRead};

fn is_possible(game: String) -> u32 {
    let just_game = game[game.find(": ").unwrap() + 2..].to_string();
    let mut largest_red = 0;
    let mut largest_green = 0;
    let mut largest_blue = 0;
    for set in just_game.split("; ") {
        // println!("Set: {:?}", set);
        for ball in set.split(", ") {
            // println!("Ball: {:?}", ball);
            let (number, color) = ball.split_at(ball.find(" ").unwrap());
            let int_number = number.parse::<u32>().unwrap();
            // println!("{}: {}", color, int_number);
            if color == " red" && int_number > largest_red {
                largest_red = int_number;
            }
            if color == " green" && int_number > largest_green {
                largest_green = int_number;
            }
            if color == " blue" && int_number > largest_blue {
                largest_blue = int_number;
            }
            // println!("OK");
        }
    }
    return largest_red * largest_green * largest_blue;
}

fn main() {
    // let file = File::open("test_input.txt").expect("File not found");
    let file = File::open("input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let numbers: Vec<u32> = Vec::from_iter(
        file_reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| is_possible(line)),
    );
    println!("{:?}", numbers.iter().sum::<u32>());
}
