use std::{fs::File, io::BufRead, io::BufReader};

fn get_directions(direction: usize, tile: char) -> Vec<usize> {
    let mut directions: Vec<usize> = vec![];
    match tile {
        '.' => {
            directions.push(direction);
        }
        '|' => {
            if direction == 1 || direction == 3 {
                directions.push(direction);
            } else {
                directions.push(1);
                directions.push(3);
            }
        }
        '-' => {
            if direction == 2 || direction == 4 {
                directions.push(direction);
            } else {
                directions.push(2);
                directions.push(4);
            }
        }
        '/' => {
            if direction == 1 {
                directions.push(2);
            } else if direction == 2 {
                directions.push(1);
            } else if direction == 3 {
                directions.push(4);
            } else if direction == 4 {
                directions.push(3);
            }
        }
        '\\' => {
            if direction == 1 {
                directions.push(4);
            } else if direction == 2 {
                directions.push(3);
            } else if direction == 3 {
                directions.push(2);
            } else if direction == 4 {
                directions.push(1);
            }
        }
        _ => {}
    }
    directions
}

fn print_directions(tiles_directions: &Vec<Vec<Vec<usize>>>) {
    for i in 0..tiles_directions.len() {
        for j in 0..tiles_directions[0].len() {
            for k in 0..tiles_directions[i][j].len() {
                print!("{} ", tiles_directions[i][j][k]);
            }
            print!("| ");
        }
        println!();
    }
    println!();
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    // let file = File::open("test_input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let field: Vec<String> = Vec::from_iter(file_reader.lines().filter_map(|line| line.ok()));
    // 1 - up, 2 - right, 3 - down, 4 - left
    let mut tiles_directions: Vec<Vec<Vec<usize>>> =
        vec![vec![vec![]; field[0].len()]; field.len()];
    let mut tiles_examined_directions: Vec<Vec<Vec<usize>>> =
        vec![vec![vec![]; field[0].len()]; field.len()];
    let directions = get_directions(2, field[0].chars().nth(0).unwrap());
    for i in 0..directions.len() {
        tiles_directions[0][0].push(directions[i]);
        tiles_examined_directions[0][0].push(directions[i]);
    }

    // print_directions(&tiles_directions);

    while tiles_directions
        .iter()
        .any(|x| x.iter().any(|y| y.len() > 0))
    {
        for i in 0..field.len() {
            for j in 0..field[0].len() {
                let directions = tiles_directions[i][j].clone();
                if directions.contains(&1) {
                    if i > 0 {
                        for k in get_directions(1, field[i - 1].chars().nth(j).unwrap()) {
                            if !tiles_examined_directions[i - 1][j].contains(&k) {
                                tiles_directions[i - 1][j].push(k);
                                tiles_examined_directions[i - 1][j].push(k);
                            }
                        }
                    }
                }
                if directions.contains(&2) {
                    if j < field[0].len() - 1 {
                        for k in get_directions(2, field[i].chars().nth(j + 1).unwrap()) {
                            if !tiles_examined_directions[i][j + 1].contains(&k) {
                                tiles_directions[i][j + 1].push(k);
                                tiles_examined_directions[i][j + 1].push(k);
                            }
                        }
                    }
                }
                if directions.contains(&3) {
                    if i < field.len() - 1 {
                        for k in get_directions(3, field[i + 1].chars().nth(j).unwrap()) {
                            if !tiles_examined_directions[i + 1][j].contains(&k) {
                                tiles_directions[i + 1][j].push(k);
                                tiles_examined_directions[i + 1][j].push(k);
                            }
                        }
                    }
                }
                if directions.contains(&4) {
                    if j > 0 {
                        for k in get_directions(4, field[i].chars().nth(j - 1).unwrap()) {
                            if !tiles_examined_directions[i][j - 1].contains(&k) {
                                tiles_directions[i][j - 1].push(k);
                                tiles_examined_directions[i][j - 1].push(k);
                            }
                        }
                    }
                }
                tiles_directions[i][j] = vec![];
            }
        }
        // print_directions(&tiles_directions);
    }
    // Sum all that have at least one direction in tiles_examined_directions
    println!(
        "{}",
        tiles_examined_directions
            .iter()
            .map(|x| x.iter().filter(|y| y.len() > 0).count())
            .sum::<usize>()
    );
}
