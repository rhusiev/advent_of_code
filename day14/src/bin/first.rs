use std::{fs::File, io::BufRead, io::BufReader};

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

    let new_field = get_new_field_up(field);

    // println!("{:?}", where_to_land);
    // for line in new_field {
    //     println!("{}", line);
    // }

    let mut result = 0;
    let lines = new_field.len();
    for (i, line) in new_field.iter().enumerate() {
        result += line.chars().filter(|c| *c == 'O').count() * (lines - i);
    }
    println!("{}", result);
}
