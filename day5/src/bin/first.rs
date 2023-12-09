use std::{fs::File, io::BufRead, io::BufReader};

fn get_next_id(seed: u64, map: &Vec<(u64, u64, u64)>) -> u64 {
    for (dest_range_start, source_range_start, range_len) in map {
        if seed >= *source_range_start && seed < *source_range_start + *range_len {
            return seed - source_range_start + dest_range_start;
        }
    }
    return seed;
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    // let file = File::open("test_input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let almanac: Vec<String> = Vec::from_iter(file_reader.lines().filter_map(|line| line.ok()));
    let seeds = almanac[0][7..]
        .split(" ")
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();
    let mut maps: Vec<Vec<(u64, u64, u64)>> = vec![];
    maps.push(vec![]);
    let mut i = 3;
    let mut line = &almanac[i];
    let mut map_num = 0;
    while i < almanac.len() {
        if line == "" {
            i += 1;
            line = &almanac[i];
            continue;
        }
        if line.ends_with("map:") {
            maps.push(vec![]);
            map_num += 1;
            i += 1;
            line = &almanac[i];
            continue;
        }
        let map_vec: Vec<u64> = line
            .split(" ")
            .map(|seed| seed.parse::<u64>().unwrap())
            .collect::<Vec<u64>>();
        let map = (map_vec[0], map_vec[1], map_vec[2]);
        maps[map_num].push(map);
        i += 1;
        if i >= almanac.len() {
            break;
        }
        line = &almanac[i];
    }
    let locations = seeds.iter()
        .map(|seed| {
            let mut next_id = get_next_id(*seed, &maps[0]);
            for i in 1..maps.len() {
                next_id = get_next_id(next_id, &maps[i]);
            }
            next_id
        })
        .collect::<Vec<u64>>();
    println!("{:?}", locations.iter().min().unwrap());
}
