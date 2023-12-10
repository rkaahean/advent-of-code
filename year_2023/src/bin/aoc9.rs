use std::fs;

fn main() {
    const PATH: &str =
        "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc9.txt";
    let contents = fs::read_to_string(PATH).expect("Failed to read file");
    let contents = contents.lines();

    let p1 = contents
        .clone()
        .into_iter()
        .fold(0, |acc, x| acc + part1(x));
    let p2 = contents.into_iter().fold(0, |acc, x| acc + part2(x));

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

fn part1(line: &str) -> i32 {
    let history: Vec<i32> = line
        .trim()
        .split(" ")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    let mut histories: Vec<Vec<i32>> = Vec::new();
    histories.push(history.clone());

    let mut new_history: Vec<i32> = history.clone();

    while new_history.iter().filter(|x| **x != 0).count() > 0 {
        new_history = new_history
            .iter()
            .zip(new_history.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();
        histories.push(new_history.clone());
    }
    let mut downs: Vec<i32> = Vec::new();

    let mut prev_hist = 0;
    for h in histories.iter().rev() {
        let mut rev_iter = h.iter().rev();
        let left = rev_iter.next().unwrap();
        prev_hist = left + prev_hist;
        downs.push(prev_hist);
    }
    downs.pop().unwrap()
}

fn part2(line: &str) -> i32 {
    let history: Vec<i32> = line
        .trim()
        .split(" ")
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();
    let mut histories: Vec<Vec<i32>> = Vec::new();
    histories.push(history.clone());

    let mut new_history: Vec<i32> = history.clone();

    while new_history.iter().filter(|x| **x != 0).count() > 0 {
        new_history = new_history
            .iter()
            .zip(new_history.iter().skip(1))
            .map(|(a, b)| b - a)
            .collect();
        histories.push(new_history.clone());
    }
    let mut downs: Vec<i32> = Vec::new();

    let mut prev_hist = 0;
    for h in histories.iter().rev() {
        let mut rev_iter = h.iter();
        let left = rev_iter.next().unwrap();
        prev_hist = left - prev_hist;
        downs.push(prev_hist);
    }
    downs.pop().unwrap()
}
