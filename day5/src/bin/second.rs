use rangemap::RangeMap;
use std::ops::Range;
use std::{fs::File, io::BufRead, io::BufReader};

fn ranges_from_seeds(seeds: &Vec<i128>) -> Vec<Range<i128>> {
    let mut ranges: Vec<Range<i128>> = vec![];
    for i in 0..seeds.len() / 2 {
        let start = seeds[i * 2];
        let len = seeds[i * 2 + 1];
        ranges.push(start..start + len);
    }
    return ranges;
}

fn map_range(rangemap: &RangeMap<i128, i128>, input: Range<i128>) -> Vec<Range<i128>> {
    let mut output: Vec<Range<i128>> = vec![];
    // Map all subranges that are contained in the rangemap
    // All the subranges that are not contained in the rangemap map to themselves
    let mut ranges: Vec<Range<i128>> = vec![];
    ranges.push(input.clone());
    for (range, value) in rangemap.iter() {
        println!("Ranges: {:?}", ranges);
        let mut new_ranges: Vec<Range<i128>> = vec![];
        for subrange in ranges {
            if subrange.start >= range.start && subrange.end <= range.end {
                output.push((subrange.start + value)..(subrange.end + value));
                println!("Is fully contained in {:?}", range);
            } else if subrange.start >= range.end || subrange.end <= range.start {
                new_ranges.push(subrange);
                println!("Is not contained in {:?}", range);
            } else if subrange.start <= range.start && subrange.end >= range.end {
                output.push((range.start + value)..(range.end + value));
                new_ranges.push(subrange.start..range.start);
                new_ranges.push(range.end..subrange.end);
                println!("Is fully containing {:?}", range);
            } else if subrange.start < range.start && subrange.end <= range.end {
                output.push((range.start + value)..(subrange.end + value));
                new_ranges.push(subrange.start..range.start);
                println!("Is containing start of {:?}", range);
            } else if subrange.start >= range.start && subrange.end > range.end {
                output.push((subrange.start + value)..(range.end + value));
                new_ranges.push(range.end..subrange.end);
                println!("Is containing end of {:?}", range);
            }
        }
        ranges = new_ranges;
    }
    output.extend(ranges);
    println!("Output: {:?}", output);
    return output;
}

fn main() {
    let file = File::open("input.txt").expect("File not found");
    // let file = File::open("test_input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let almanac: Vec<String> = Vec::from_iter(file_reader.lines().filter_map(|line| line.ok()));
    let seeds = almanac[0][7..]
        .split(" ")
        .map(|seed| seed.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();
    let seed_ranges = ranges_from_seeds(&seeds);
    let mut rangemaps: Vec<RangeMap<i128, i128>> = vec![];
    rangemaps.push(RangeMap::new());
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
            rangemaps.push(RangeMap::new());
            map_num += 1;
            i += 1;
            line = &almanac[i];
            continue;
        }
        let map_vec: Vec<i128> = line
            .split(" ")
            .map(|seed| seed.parse::<i128>().unwrap())
            .collect::<Vec<i128>>();
        rangemaps[map_num].insert(map_vec[1]..map_vec[1] + map_vec[2], map_vec[0] - map_vec[1]);
        i += 1;
        if i >= almanac.len() {
            break;
        }
        line = &almanac[i];
    }
    println!("Rangemaps: {:?}", rangemaps);
    println!("Seed ranges: {:?}", seed_ranges);
    let mut output: Vec<Vec<Range<i128>>> = vec![];
    output.push(seed_ranges.clone());
    for i in 0..rangemaps.len() {
        let mut new_output: Vec<Range<i128>> = vec![];
        for range in &output[i] {
            new_output.append(&mut map_range(&rangemaps[i], range.clone()));
        }
        println!("New output: {:?}", new_output);
        output.push(new_output);
    }
    println!("Output: {:?}", output);
    let mut result = output[output.len() - 1].clone();
    result.sort_by(|a, b| a.start.cmp(&b.start));
    println!("Result: {:?}", result);
}
