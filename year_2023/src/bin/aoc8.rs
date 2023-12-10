use num::integer::lcm;
use std::{collections::HashMap, fs};

fn main() {
    const PATH: &str =
        "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc8.txt";
    let contents = fs::read_to_string(PATH).expect("Failed to read file");
    let contents = contents.lines();

    let mut instr: &str = "";
    let mut starts: Vec<&str> = Vec::new();

    // creating a hashmap
    let mut directions: HashMap<&str, Vec<&str>> = HashMap::new();

    for (i, line) in contents.clone().enumerate() {
        if i == 0 {
            instr = line;
            continue;
        } else if i == 1 {
            continue;
        }
        let map = line.split("=").collect::<Vec<&str>>();

        // get hashmap key
        let key = map[0].trim();
        if key.ends_with("A") {
            starts.push(key);
        }

        // get routes
        let values = map[1]
            .trim()
            .trim_start_matches("(")
            .trim_end_matches(")")
            .split(",")
            .map(|x| x.trim())
            .collect::<Vec<&str>>();
        directions.insert(key, values);
    }

    part_1(instr, "AAA", &directions);

    let mut x: i64;
    let mut result: i64 = 0;
    for (i, start) in starts.iter().enumerate() {
        x = part_2(instr, start, &directions);
        if i == 0 {
            result = x;
        } else {
            result = lcm(result, x)
        }
    }
    println!("Part 2: {}", result);
}

fn distance_to_end(
    instr: &str,
    start: &str,
    directions: &HashMap<&str, Vec<&str>>,
    end: &str,
) -> i64 {
    let mut instuctions = instr.clone().to_string();
    let is_searching = true;

    let mut search_str = start;
    let mut sm = 0;
    while is_searching {
        sm += 1;
        if instuctions.len() == 0 {
            instuctions = instr.clone().to_string();
        }
        let direction = instuctions.remove(0);
        search_str = get_next_char(direction, search_str, directions);
        if search_str.ends_with(end) {
            break;
        }
    }
    sm
}

fn part_1(instr: &str, start: &str, directions: &HashMap<&str, Vec<&str>>) {
    let sm = distance_to_end(instr, start, directions, "ZZZ");
    println!("Part 1: {}", sm);
}

fn part_2(instr: &str, start: &str, directions: &HashMap<&str, Vec<&str>>) -> i64 {
    let sm = distance_to_end(instr, start, directions, "Z");
    sm
}

fn get_next_char<'a>(
    instr: char,
    search: &'a str,
    directions: &'a HashMap<&'a str, Vec<&'a str>>,
) -> &'a str {
    if instr == 'L' {
        directions.get(search).unwrap()[0]
    } else {
        directions.get(search).unwrap()[1]
    }
}
