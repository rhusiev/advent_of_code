use std::{fs::File, io::BufRead, io::BufReader};

fn get_expanded_space(mut space: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let empty_horizontals: Vec<usize> = space
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            if line.iter().all(|&c| c == '.') {
                Some(i)
            } else {
                None
            }
        })
        .collect();
    let empty_verticals: Vec<usize> = (0..space[0].len())
        .filter(|&j| space.iter().all(|line| line[j] == '.'))
        .collect();
    for i in empty_horizontals.iter().rev() {
        space.insert(*i, space[*i].clone());
    }
    for j in empty_verticals.iter().rev() {
        for i in 0..space.len() {
            let elem = space[i][*j];
            space[i].insert(*j, elem);
        }
    }
    space
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    // let file = File::open("test_input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let mut space: Vec<Vec<char>> = Vec::from_iter(
        file_reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| line.chars().collect()),
    );
    space = get_expanded_space(space);
    let mut galaxies = vec![];
    for i in 0..space.len() {
        for j in 0..space[0].len() {
            if space[i][j] == '#' {
                galaxies.push((i, j));
            }
        }
    }
    let mut result = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            result += (galaxies[i].0 as i64 - galaxies[j].0 as i64).abs()
                + (galaxies[i].1 as i64 - galaxies[j].1 as i64 as i64).abs();
        }
    }
    println!("{}", result);
}
