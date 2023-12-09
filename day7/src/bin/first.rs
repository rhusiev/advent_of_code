use std::{fs::File, io::BufRead, io::BufReader};

static KINDS: [&str; 13] = [
    "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
];

fn get_kind_value(kind: char) -> u32 {
    match kind {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'J' => 9,
        'T' => 8,
        '9' => 7,
        '8' => 6,
        '7' => 5,
        '6' => 4,
        '5' => 3,
        '4' => 2,
        '3' => 1,
        '2' => 0,
        _ => panic!("Invalid kind"),
    }
}

fn get_value(hand: &str) -> u32 {
    let mut value = 0;
    for kind in KINDS.iter() {
        let count = hand.matches(kind).count();
        if count == 5 {
            value += u32::pow(13, 6) * 6;
            continue;
        }
        if count == 4 {
            value += u32::pow(13, 6) * 5;
            continue;
        }
        if count == 3 {
            value += u32::pow(13, 6) * 3;
            continue;
        }
        if count == 2 {
            value += u32::pow(13, 6) * 1;
            continue;
        }
    }
    for (i, kind) in hand.chars().enumerate() {
        value += get_kind_value(kind) * u32::pow(13, 5 - i as u32);
    }
    value
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    // let file = File::open("test_input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let mut hands: Vec<(u32, u32)> = Vec::from_iter(
        file_reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| {
                let split_line = line.split(" ").collect::<Vec<&str>>();
                let hand = split_line[0];
                let bid: u32 = split_line[1].parse().unwrap();
                (get_value(hand), bid)
            }),
    );
    hands.sort_by(|a, b| a.0.cmp(&b.0));
    let mut winnings = 0;
    for (i, hand) in hands.iter().enumerate() {
        let bid = hand.1;
        winnings += (i + 1) as u32 * bid;
    }
    println!("{:?}", winnings);
}
