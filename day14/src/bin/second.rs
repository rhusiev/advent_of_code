use std::{fs::File, io::BufRead, io::BufReader};

fn hash_field(field: Vec<String>) -> u64 {
    let mut result = 0;
    for (i, line) in field.iter().enumerate() {
        for (j, c) in line.chars().enumerate() {
            if c == 'O' {
                result += (i * line.len() + j) as u64;
            }
        }
    }
    result
}

fn get_new_field_up(field: Vec<String>) -> Vec<String> {
    let mut new_field: Vec<String> = Vec::new();
    new_field.push(field[0].clone());
    for line in field.iter().skip(1) {
        let mut new_line = String::new();
        for c in line.chars() {
            if c == '#' {
                new_line.push('#');
            } else {
                new_line.push('.');
            }
        }
        new_field.push(new_line);
    }
    let mut where_to_land: Vec<Vec<u32>> = Vec::new();
    where_to_land.push(vec![0; field[0].len()]);
    for (i, line) in field.iter().enumerate().skip(1) {
        let mut line_to_land: Vec<u32> = Vec::new();
        for j in 0..line.len() {
            let prev = new_field[i - 1].chars().nth(j).unwrap();
            if prev == '#' || prev == 'O' {
                line_to_land.push(i as u32);
            } else {
                line_to_land.push(where_to_land[i - 1][j]);
            }
            let cur = line.chars().nth(j).unwrap();
            if cur == 'O' {
                new_field[line_to_land[j] as usize].replace_range(j..j + 1, "O");
                line_to_land[j] += 1;
            }
        }
        where_to_land.push(line_to_land);
    }
    new_field
}

fn get_new_field_down(field: Vec<String>) -> Vec<String> {
    let mut new_field: Vec<String> = Vec::new();
    new_field.push(field[field.len() - 1].clone());
    for line in field.iter().rev().skip(1) {
        let mut new_line = String::new();
        for c in line.chars() {
            if c == '#' {
                new_line.push('#');
            } else {
                new_line.push('.');
            }
        }
        new_field.insert(0, new_line);
    }
    let mut where_to_land: Vec<Vec<u32>> = Vec::new();
    where_to_land.push(vec![field.len() as u32 - 1; field[0].len()]);
    for (i, line) in field.iter().enumerate().rev().skip(1) {
        let mut line_to_land: Vec<u32> = Vec::new();
        for j in 0..line.len() {
            let prev = new_field[i + 1].chars().nth(j).unwrap();
            if prev == '#' || prev == 'O' {
                line_to_land.push(i as u32);
            } else {
                line_to_land.push(where_to_land[0][j]);
            }
            let cur = line.chars().nth(j).unwrap();
            if cur == 'O' {
                new_field[line_to_land[j] as usize].replace_range(j..j + 1, "O");
                if line_to_land[j] > 0 {
                    line_to_land[j] -= 1;
                }
            }
        }
        where_to_land.insert(0, line_to_land);
    }
    new_field
}

fn get_new_field_left(field: Vec<String>) -> Vec<String> {
    let mut new_field: Vec<String> = Vec::new();
    for line in field.iter() {
        let mut new_line = line.chars().nth(0).unwrap().to_string() +
            line[1..line.len()].chars().map(|c| if c == '#' { '#' } else { '.' }).collect::<String>().as_str();
        let mut col_to_land: Vec<u32> = Vec::new();
        col_to_land.push(0);
        for j in 1..line.len() {
            let prev = new_line.chars().nth(j - 1).unwrap();
            if prev == '#' || prev == 'O' {
                col_to_land.push(j as u32);
            } else {
                col_to_land.push(col_to_land[j - 1]);
            }
            let cur = line.chars().nth(j).unwrap();
            if cur == 'O' {
                new_line.replace_range(col_to_land[j] as usize..col_to_land[j] as usize + 1, "O");
                col_to_land[j] += 1;
            }
        }
        new_field.push(new_line);
    }
    new_field
}

fn get_new_field_right(field: Vec<String>) -> Vec<String> {
    let mut new_field: Vec<String> = Vec::new();
    for line in field.iter() {
        let mut new_line = line[0..line.len() - 1].chars().map(|c| if c == '#' { '#' } else { '.' }).collect::<String>().as_str().to_owned() +
            line.chars().nth(line.len() - 1).unwrap().to_string().as_str();
        let mut col_to_land: Vec<u32> = Vec::new();
        col_to_land.push(line.len() as u32 - 1);
        for j in (0..line.len() - 1).rev() {
            let prev = new_line.chars().nth(j + 1).unwrap();
            if prev == '#' || prev == 'O' {
                col_to_land.insert(0, j as u32);
            } else {
                col_to_land.insert(0, col_to_land[0]);
            }
            let cur = line.chars().nth(j).unwrap();
            if cur == 'O' {
                new_line.replace_range(col_to_land[0] as usize..col_to_land[0] as usize + 1, "O");
                if col_to_land[0] > 0 {
                    col_to_land[0] -= 1;
                }
            }
        }
        new_field.push(new_line);
    }
    new_field
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    // let file = File::open("test_input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    // Example field:
    // O....#....
    // O.OO#....#
    // .....##...
    // OO.#O....O
    // .O.....O#.
    // O.#..O.#.#
    // ..O..#O..O
    // .......O..
    // #....###..
    // #OO..#....

    // # - can't move
    // O - move up as far as possible
    let field: Vec<String> = Vec::from_iter(
        file_reader
            .lines()
            .filter_map(|line| line.ok())
    );

    let mut new_field = field.clone();
    let mut prev_fields: Vec<u64> = Vec::new();
    for i in 0..1000000000 {
        new_field = get_new_field_up(new_field);
        new_field = get_new_field_left(new_field);
        new_field = get_new_field_down(new_field);
        new_field = get_new_field_right(new_field);
        let hash = hash_field(new_field.clone());
        if prev_fields.contains(&hash) {
            let index = prev_fields.iter().position(|&x| x == hash).unwrap();
            let cycle_len = i - index;
            let index = (1000000000 - i - 1) % cycle_len;
            for _ in 0..index {
                new_field = get_new_field_up(new_field);
                new_field = get_new_field_left(new_field);
                new_field = get_new_field_down(new_field);
                new_field = get_new_field_right(new_field);
            }
            break;
        }
        prev_fields.push(hash);
    }

    for line in new_field.iter() {
        println!("{}", line);
    }

    let mut result = 0;
    let lines = new_field.len();
    for (i, line) in new_field.iter().enumerate() {
        result += line.chars().filter(|c| *c == 'O').count() * (lines - i);
    }
    println!("{}", result);
}
