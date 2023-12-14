use std::fs;

fn main() {
    const PATH: &str =
        "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc12.txt";
    let contents = fs::read_to_string(PATH).expect("Failed to read file");
    let contents = contents.lines();

    for line in contents {
        part1(line);
    }
}

fn part1(line: &str) -> i32 {
    let parts = line.split(" ").collect::<Vec<&str>>();

    let springs = parts[0];
    let counts = parts[1]
        .trim()
        .split(",")
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    println!("{} {:?}", springs, counts);
    0
}
