use std::{fs, str::Lines};

fn main() {
    const PATH: &str =
        "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc5.txt";
    let contents = fs::read_to_string(PATH).expect("Failed to read file");
    let contents = contents.lines();

    part_1(&contents);
    part_2(&contents);
}

fn part_1(contents: &Lines) {
    let mut seeds: Vec<i64> = Vec::new();
    let mut mappings: Vec<Vec<i64>> = Vec::new();

    for (i, line) in contents.clone().enumerate() {
        // get the seeds
        if i == 0 {
            seeds = line.split(":").collect::<Vec<&str>>()[1]
                .trim()
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        }
        // end of a mapping
        else if (line == "" && i > 1) || contents.clone().last() == Some(line) {
            seeds = get_mapping(&mappings, &seeds);
            continue;
        }
        // time to reset
        else if line.contains("map") {
            mappings = Vec::new();
        } else {
            let instructions = line
                .split(" ")
                .filter_map(|x| x.trim().parse::<i64>().ok())
                .collect::<Vec<i64>>();
            mappings.push(instructions);
        }
    }
    println!("Part 1: {:?}", seeds.iter().min().unwrap());
}

fn part_2(contents: &Lines) {
    let mut seeds: Vec<i64> = Vec::new();
    let mut mappings: Vec<Vec<i64>> = Vec::new();

    for (i, line) in contents.clone().enumerate() {
        // get the seeds
        if i == 0 {
            let seeds_range = line.split(":").collect::<Vec<&str>>()[1]
                .trim()
                .split(" ")
                .filter(|x| !x.is_empty())
                .map(|x| x.trim().parse::<i64>().unwrap())
                .collect::<Vec<i64>>();

            for i in (0..seeds_range.len() - 1).step_by(2) {
                let (start, size) = (seeds_range[i], seeds_range[i + 1]);
                // print!("{} - {}", start, size);

                seeds.extend(start..start + size);
            }
            // print!("Starting seeds {:?}", seeds);
        }
        // end of a mapping
        else if (line == "" && i > 1) || contents.clone().last() == Some(line) {
            seeds = get_mapping(&mappings, &seeds);
            continue;
        }
        // time to reset
        else if line.contains("map") {
            mappings = Vec::new();
        } else {
            let instructions = line
                .split(" ")
                .filter_map(|x| x.trim().parse::<i64>().ok())
                .collect::<Vec<i64>>();
            mappings.push(instructions);
        }
    }
    println!("Part 2: {:?}", seeds.iter().min().unwrap());
}

fn get_mapping(mappings: &Vec<Vec<i64>>, current: &Vec<i64>) -> Vec<i64> {
    // println!("Parsing mapping...{:?}", current);
    let mut new_items: Vec<i64> = Vec::new();
    let mut prev_start = 0;
    let mut prev_end = 0;

    for i in 0..current.len() {
        let mut item = current[i];
        // check if in the same range as previous element
        if (item > prev_start && item < prev_end) && (i > 0 && current[i - 1] + 1 == item) {
            let last = new_items.last().unwrap();
            new_items.push(last + 1);
            continue;
        }
        for farm_map in mappings {
            let (dest, source, size) = (farm_map[0], farm_map[1], farm_map[2]);
            if item >= source && item <= source + size - 1 {
                (prev_start, prev_end) = (source, source + size - 1);
                item = dest + (item - source);
                break;
            }
        }
        new_items.push(item);
    }
    new_items
}
