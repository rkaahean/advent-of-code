use std::{fs, str::Lines};

fn main() {
    const PATH: &str =
        "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc18.txt";
    let contents = fs::read_to_string(PATH).expect("Failed to read file");
    let contents = contents.lines();

    part1(&contents);
    part2(&contents);
}

fn part1(contents: &Lines<'_>) {
    let mut points = Vec::new();
    let mut current: (i64, i64) = (0, 0);
    let mut perimeter = 0;
    for line in contents.clone() {
        let instr = line.split(" ").collect::<Vec<&str>>();
        let move_count = instr[1].parse::<i64>().unwrap();
        perimeter = perimeter + move_count;
        current = match instr[0] {
            "U" => (current.0, current.1 + move_count),
            "R" => (current.0 + move_count, current.1),
            "D" => (current.0, current.1 - move_count),
            "L" => (current.0 - move_count, current.1),
            _ => (0, 0),
        };
        let new_point = (current, instr[2]);
        points.push(new_point);
    }
    let inner = get_area(&points) - (perimeter / 2) as f64 + 1.0;
    println!("Part 1 - {}", inner + (perimeter as f64));
}

fn part2(contents: &Lines<'_>) {
    let mut points = Vec::new();
    let mut current: (i64, i64) = (0, 0);
    let mut perimeter = 0;
    for line in contents.clone() {
        let instr = line.split(" ").collect::<Vec<&str>>();
        let base_hex = instr[2]
            .trim_start_matches("(")
            .trim_end_matches(")")
            .strip_prefix("#")
            .unwrap();
        let move_count =
            i64::from_str_radix(base_hex.get(..base_hex.len() - 1).unwrap(), 16).unwrap();
        perimeter = perimeter + move_count;
        current = match base_hex.chars().last().unwrap() {
            '3' => (current.0, current.1 + move_count),
            '0' => (current.0 + move_count, current.1),
            '1' => (current.0, current.1 - move_count),
            '2' => (current.0 - move_count, current.1),
            _ => (0, 0),
        };
        let new_point = (current, instr[2]);
        points.push(new_point);
    }
    let inner = get_area(&points) - (perimeter / 2) as f64 + 1.0;
    println!("Part 2 - {}", inner + (perimeter as f64));
}

fn get_area(points: &Vec<((i64, i64), &str)>) -> f64 {
    // using shoelace formula
    let mut sum1: f64 = 0.0;
    let mut sum2: f64 = 0.0;
    let n = points.len();

    for i in 0..n {
        let ((x1, y1), _) = points[i];
        let ((x2, y2), _) = points[(i + 1) % n];

        sum1 += (x1 * y2) as f64;
        sum2 += (x2 * y1) as f64;
    }
    (sum2 - sum1).abs() / 2.0
}
