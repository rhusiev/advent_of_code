use std::{fs::File, io::BufRead, io::BufReader};

fn derivative(sequence: &Vec<i64>) -> Vec<i64> {
    let mut result: Vec<i64> = Vec::new();
    for i in 0..sequence.len() - 1 {
        result.push(sequence[i + 1] - sequence[i]);
    }
    result
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    // let file = File::open("test_input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let sequences: Vec<Vec<i64>> = Vec::from_iter(
        file_reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| line.split(" ").map(|s| s.parse::<i64>().unwrap()).collect()),
    );
    println!("{:?}", sequences);
    let mut result = 0;
    for sequence in sequences {
        let mut derivatives: Vec<Vec<i64>> = Vec::new();
        let mut current_sequence = sequence;
        derivatives.push(current_sequence.clone());
        while !Iterator::all(&mut current_sequence.iter(), |&x| x == 0) {
            current_sequence = derivative(&current_sequence);
            derivatives.push(current_sequence.clone());
        }
        let length = derivatives.len();
        derivatives[length - 1].insert(0, 0);
        for i in (0..length - 1).rev() {
            let next = derivatives[i][0] - derivatives[i + 1][0];
            derivatives[i].insert(0, next);
        }
        println!("{:?}", derivatives);
        result += derivatives[0][0];
    }
    println!("{}", result);
}
