use std::{fs::File, io::BufRead, io::BufReader};

fn get_expanded(space: &Vec<Vec<char>>) -> (Vec<usize>, Vec<usize>) {
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
    return (empty_horizontals, empty_verticals);
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    // let file = File::open("test_input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let space: Vec<Vec<char>> = Vec::from_iter(
        file_reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| line.chars().collect()),
    );
    let (empty_horizontals, empty_verticals) = get_expanded(&space);
    let mut galaxies = vec![];
    let empty_verticals_less: Vec<usize> = (0..space[0].len())
        .map(|j| empty_verticals.iter().filter(|&&x| x < j).count())
        .collect();
    for i in 0..space.len() {
        let empty_horizontals_less = empty_horizontals.iter().filter(|&&x| x < i).count();
        for j in 0..space[0].len() {
            if space[i][j] == '#' {
                let expanded_i = i + empty_horizontals_less * 999999;
                let expanded_j = j + empty_verticals_less[j] * 999999;
                galaxies.push((expanded_i, expanded_j));
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
