use std::{fs::File, io::BufRead, io::BufReader};

fn find_same_neighbouring_rows(pattern: &Vec<&String>) -> Vec<usize> {
    let mut same_neighbouring_rows = Vec::new();
    let mut last_row = "";
    for (i, row) in pattern.iter().enumerate() {
        if row == &last_row {
            same_neighbouring_rows.push(i);
        }
        last_row = row;
    }
    same_neighbouring_rows
}

fn find_same_neighbouring_columns(pattern: &Vec<&String>) -> Vec<usize> {
    let mut same_neighbouring_columns = Vec::new();
    let mut last_column: String = "".to_string();
    for j in 0..pattern[0].len() {
        let col = pattern.iter().map(|row| row.chars().nth(j).unwrap()).collect::<String>();
        if col == last_column {
            same_neighbouring_columns.push(j);
        }
        last_column = col;
    }
    same_neighbouring_columns
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    // let file = File::open("test_input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let lines = Vec::from_iter(
        file_reader
            .lines()
            .filter_map(|line| line.ok())
    );
    let mut patterns = Vec::new();
    patterns.push(Vec::new());
    for line in lines.iter() {
        if line == "" {
            patterns.push(Vec::new());
            continue;
        }
        patterns.last_mut().unwrap().push(line);
    }

    let mut result = 0;
    for pattern in patterns.iter() {
        let same_neighbouring_rows = find_same_neighbouring_rows(pattern);
        let same_neighbouring_columns = find_same_neighbouring_columns(pattern);
        let mut mirror_rows = Vec::new();
        for row in same_neighbouring_rows.iter() {
            if *row == 1 || *row == pattern.len() - 1 {
                mirror_rows.push(*row);
                continue;
            }
            let mut i = *row - 2;
            let mut j = *row + 1;
            let mut found = false;
            loop {
                if pattern[i] != pattern[j] {
                    break;
                }
                if i == 0 || j == pattern.len() - 1 {
                    found = true;
                    break;
                }
                i -= 1;
                j += 1;
            }
            if found {
                mirror_rows.push(*row);
            }
        }
        let mut mirror_columns = Vec::new();
        for column in same_neighbouring_columns.iter() {
            if *column == 1 || *column == pattern[0].len() - 1 {
                mirror_columns.push(*column);
                continue;
            }
            let mut i = *column - 2;
            let mut j = *column + 1;
            let mut found = false;
            loop {
                let mut col_i = "".to_string();
                let mut col_j = "".to_string();
                for row in pattern.iter() {
                    col_i.push(row.chars().nth(i).unwrap());
                    col_j.push(row.chars().nth(j).unwrap());
                }
                if col_i != col_j {
                    break;
                }
                if i == 0 || j == pattern[0].len() - 1 {
                    found = true;
                    break;
                }
                i -= 1;
                j += 1;
            }
            if found {
                mirror_columns.push(*column);
            }
        }
        if mirror_rows.len() > 0 {
            result += 100 * mirror_rows.first().unwrap();
        }
        if mirror_columns.len() > 0 {
            result += mirror_columns.first().unwrap();
        }
    }
    println!("{:?}", result);
}
