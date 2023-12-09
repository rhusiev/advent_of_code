use std::{fs::File, io::BufRead, io::BufReader};
use itertools::Itertools;

static KINDS: [&str; 13] = [
    "A", "K", "Q", "J", "T", "N", "E", "S", "X", "F", "Z", "Y", "W",
];

static SMALL_KINDS: [&str; 13] = [
    "a", "k", "q", "j", "t", "n", "e", "s", "x", "f", "z", "y", "w",
];

fn get_kind_value(kind: char) -> u32 {
    match kind {
        'A' => 12,
        'K' => 11,
        'Q' => 10,
        'T' => 9,
        'N' => 8,
        'E' => 7,
        'S' => 6,
        'X' => 5,
        'F' => 4,
        'Z' => 3,
        'Y' => 2,
        'W' => 1,
        _ => 0,
    }
}

fn get_value(hand: &String) -> u32 {
    let mut value = 0;
    for kind in SMALL_KINDS.iter() {
        let count = hand.to_lowercase().matches(kind).count();
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

fn get_max_value(hand: String) -> u32 {
    let mut max_value = 0;
    let mut max_hand = hand.to_string();
    let j_count = hand.matches("J").count();
    if j_count == 0 {
        println!("{:?}", hand);
        return get_value(&hand);
    }
    // Generate combinations with replacement of [0..13] with length j_count
    for combination in (0..13).combinations_with_replacement(j_count) {
        let mut changed_hand = hand.to_string();
        for (_, kind) in combination.iter().enumerate() {
            changed_hand = changed_hand.replace("J", SMALL_KINDS[*kind]);
        }
        let value = get_value(&changed_hand);
        if value > max_value {
            max_value = value;
            max_hand = changed_hand;
        }
    }
    println!("{:?}", max_hand);
    max_value
}

// fn new_get_value(hand: &str) -> u32 {
//     let mut value = 0;
//     for (i, kind) in hand.chars().enumerate() {
//         value += get_kind_value(kind) * u32::pow(13, 5 - i as u32);
//     }
//     let changed_hand = hand.replace("J", KINDS.iter()
//         .map(|kind| (*kind, hand.matches(kind).count()))
//         .max_by(|a, b| a.1.cmp(&b.1))
//         .unwrap().0);
//     for kind in KINDS.iter() {
//         let count = changed_hand.matches(kind).count();
//         if count == 5 {
//             value += u32::pow(13, 6) * 6;
//             continue;
//         }
//         if count == 4 {
//             value += u32::pow(13, 6) * 5;
//             continue;
//         }
//         if count == 3 {
//             value += u32::pow(13, 6) * 3;
//             continue;
//         }
//         if count == 2 {
//             value += u32::pow(13, 6) * 1;
//             continue;
//         }
//     }
//     value
// }

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
                let hand = split_line[0]
                    .replace("9", "N")
                    .replace("8", "E")
                    .replace("7", "S")
                    .replace("6", "X")
                    .replace("5", "F")
                    .replace("4", "Z")
                    .replace("3", "Y")
                    .replace("2", "W");
                let bid: u32 = split_line[1].parse().unwrap();
                (get_max_value(hand), bid)
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
