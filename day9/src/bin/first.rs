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
        let length_0 = derivatives[0].len();
        let length = derivatives.len();
        derivatives[length - 1].push(0);
        for i in (0..length - 1).rev() {
            let length = derivatives[i].len();
            let next = derivatives[i][length - 1] + derivatives[i + 1][length - 1];
            derivatives[i].push(next);
        }
        result += derivatives[0][length_0 - 1] + derivatives[1][length_0 - 1];
    }
    println!("{}", result);
}
