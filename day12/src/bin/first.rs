use std::{fs::File, io::BufRead, io::BufReader};

fn main() {
    let file = File::open("input.txt").expect("File not found");
    // let file = File::open("test_input.txt").expect("File not found");
    let file_reader = BufReader::new(file);
    let lines: Vec<(String, Vec<u32>)> = Vec::from_iter(
        file_reader
            .lines()
            .filter_map(|line| line.ok())
            .map(|line| {
                let split = line.split(" ").collect::<Vec<&str>>();
                let springs = split.first().unwrap().to_string() + ".";
                let groups = split
                    .last()
                    .unwrap()
                    .to_string()
                    .split(",")
                    .map(|s| s.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                (springs.to_string(), groups)
            }),
    );
    // let mut possible_placements: Vec<Vec<Vec<u32>>> = Vec::new();
    // let line = ("???#????#???.", [2, 4]);
    let mut result = 0;
    for line in lines.iter() {
        // println!("line: {:?}", line);
        let mut possible_placements_for_spring: Vec<Vec<u32>> = Vec::new();
        let mut first = true;
        for group_len in line.1.iter() {
            // println!("group_len: {}", group_len);
            let mut non_dots: u32 = line.0[0..*group_len as usize]
                .chars()
                .filter(|c| c != &'.')
                .count() as u32;
            // println!("non_dots 0: {}", non_dots);
            let mut possible_springs_for_group: Vec<u32> = Vec::new();
            let mut prev_char = line.0.chars().nth(0).unwrap();
            if non_dots == *group_len && line.0.chars().nth(*group_len as usize).unwrap() != '#' {
                possible_springs_for_group.push(0);
            }
            // println!("line.0.len() - *group_len as usize: {}", line.0.len() - *group_len as usize);
            for i in 1 as usize..line.0.len() - *group_len as usize {
                // println!("i: {}", i);
                let next_char = line.0.chars().nth(i + *group_len as usize - 1).unwrap();
                if next_char != '.' {
                    // println!("non_dots += 1");
                    non_dots += 1;
                }
                if prev_char != '.' {
                    // println!("non_dots -= 1, because prev_char at i - 1: {}", prev_char);
                    non_dots -= 1;
                }
                if non_dots == *group_len
                    && line.0.chars().nth(i + *group_len as usize).unwrap() != '#'
                    && line.0.chars().nth(i - 1).unwrap() != '#'
                {
                    // println!("pushing: {}", i);
                    if first {
                        // println!("first: {}", i);
                        prev_char = line.0.chars().nth(i).unwrap();
                        // If there is a # before the beginning of the spring, we can't start there
                        if line.0[0..i].chars().filter(|c| c == &'#').count() > 0 {
                            // println!("there is a # before the beginning of the spring, we can't start there");
                            continue;
                        }
                        // println!("{}", line.0[0..i].to_string());
                        possible_springs_for_group.push(i as u32);
                        continue;
                    }
                    possible_springs_for_group.push(i as u32);
                }
                // println!("non_dots: {}", non_dots);
                // println!();
                prev_char = line.0.chars().nth(i).unwrap();
            }
            possible_placements_for_spring.push(possible_springs_for_group);
            // println!("possible_placements_for_spring: {:?}", possible_placements_for_spring);
            first = false;
        }
        // possible_placements.push(possible_placements_for_spring);

        // println!(
        //     "possible_placements_for_spring: {:?}",
        //     possible_placements_for_spring
        // );
        let mut count_placements: Vec<Vec<u32>> = Vec::new();
        let mut prev_len = line.1.first().unwrap();
        count_placements.push(vec![1; possible_placements_for_spring[0].len()]);
        for i in 1..possible_placements_for_spring.len() {
            // println!("i: {}", i);
            // println!("possible_placements_for_spring[i]: {:?}", possible_placements_for_spring[i]);
            let this_len = possible_placements_for_spring[i].len();
            // println!("this_len: {}", this_len);
            let mut count_placements_for_group: Vec<u32> = Vec::new();
            let next_len = line.1.get(i).unwrap();
            for j in 0..this_len {
                // println!("i: {}, j: {}", i, j);
                let value = possible_placements_for_spring[i][j];
                if value < *prev_len {
                    count_placements_for_group.push(0);
                    continue;
                }
                let max_prev_value = value - *prev_len;
                // println!(
                //     "value, *prev_len, max_prev_value: {}, {}, {}",
                //     value, *prev_len, max_prev_value
                // );
                let count = possible_placements_for_spring[i - 1]
                    .iter()
                    .enumerate()
                    .map(|(k, &x)| (count_placements[i - 1][k], x))
                    .filter(|(_, x)| {
                        *x < max_prev_value
                            && (*x == 0 || line.0.chars().nth(*x as usize - 1).unwrap() != '#')
                            // && there are no # between x + prev_len and value
                            && {
                                if line.0[*x as usize + *prev_len as usize..value as usize]
                                .chars()
                                .filter(|c| c == &'#')
                                .count()
                                == 0
                                {
                                    true
                                } else {
                                    // println!("line.0[*x as usize..value as usize]: {}", line.0[*x as usize..value as usize].to_string());
                                    false
                                }
                            }
                            && {
                                if i == possible_placements_for_spring.len() - 1 {
                                    line.0[(value + *next_len) as usize..line.0.len()]
                                        .chars()
                                        .filter(|c| c == &'#')
                                        .count()
                                        == 0
                                } else {
                                    true
                                }
                            }
                    })
                    .map(|(x, k)| {
                        // println!("x k {} {}", x, k);
                        x
                    })
                    .sum::<u32>();
                // println!("count: {}", count);
                count_placements_for_group.push(count as u32);
            }
            count_placements.push(count_placements_for_group);
            prev_len = next_len;
        }
        // println!("count_placements: {:?}", count_placements);
        let count = count_placements.last().unwrap().iter().sum::<u32>();
        result += count;
        // println!("count: {}", count);
        // Get std input
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input).unwrap();
    }
    // println!("{:?}", possible_placements_for_spring);
    println!("result: {}", result);
}
