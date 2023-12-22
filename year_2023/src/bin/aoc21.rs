use std::{fs, ops::Index};

fn main() {
    const PATH: &str =
        "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc21.txt";
    let contents = fs::read_to_string(PATH).expect("Failed to read file");
    let contents = contents.lines();

    let mut garden = Vec::new();
    let mut start: (i32, i32) = (0, 0);
    for (i, line) in contents.enumerate() {
        garden.push(line.chars().collect::<Vec<char>>());
        if line.contains('S') {
            start = (i as i32, line.find('S').unwrap() as i32);
        }
    }

    // part 1
    part1(&garden, start);
}

fn part1(garden: &Vec<Vec<char>>, start: (i32, i32)) {
    println!("Starting at...{:?}", start);
    let mut nodes: Vec<(i32, i32)> = Vec::new();
    nodes.push(start);

    let mut count = 0;
    while count <= 131 {
        // println!("Count = {}, Length - {}", count, nodes.len());
        let mut new_neighbours = Vec::new();
        while let Some(current) = nodes.pop() {
            const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];
            for dir in DIRS {
                let (x, y) = (current.0 as i32 + dir.0, current.1 as i32 + dir.1);
                if is_valid_neighbour(garden, (x, y)) && !new_neighbours.contains(&(x, y)) {
                    // println!("Valid...{:?}", (x, y));
                    new_neighbours.push((x, y));
                }
            }
        }
        nodes = new_neighbours;
        count += 1;
    }
}

fn is_valid_neighbour(garden: &Vec<Vec<char>>, idx: (i32, i32)) -> bool {
    let x_len = garden.len() as i32;
    let y_len = garden[0].len() as i32;

    let (a, b) = idx;
    if 0 <= a && a < x_len && 0 <= b && b < y_len {
        let item = garden[a as usize][b as usize];
        if item != '#' {
            return true;
        }
    }
    false
}
