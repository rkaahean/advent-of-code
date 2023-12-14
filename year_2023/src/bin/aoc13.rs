use std::fs;

fn main() {
    const PATH: &str =
        "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc13.txt";
    let contents = fs::read_to_string(PATH).expect("Failed to read file");
    let contents = contents.lines();

    let mut patterns: Vec<Vec<char>> = Vec::new();
    let mut pt_1 = 0;
    let mut pt_2 = 0;
    for line in contents {
        if line.is_empty() {
            pt_1 += solve(&patterns, 0);
            pt_2 += solve(&patterns, 1);
            patterns = Vec::new();
        } else {
            patterns.push(line.chars().collect())
        }
    }
    println!("Part 1: {}", pt_1);
    println!("Part 2: {}", pt_2);
}

fn solve(patterns: &Vec<Vec<char>>, n: i32) -> i32 {
    let transpose = transpose(patterns.to_vec());
    let num_cols_left = count_near_reflection(patterns, n);
    let num_cols_up = count_near_reflection(&transpose, n);

    if num_cols_left != 0 {
        return 100 * num_cols_left;
    } else {
        return num_cols_up;
    }
}

fn transpose(pattern: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let num_rows = pattern.len();
    let num_cols = pattern[0].len();
    let mut transposed: Vec<Vec<char>> = vec![Vec::with_capacity(num_rows); num_cols];
    for (i, chars) in pattern.clone().iter().enumerate() {
        for (j, _) in chars.iter().enumerate() {
            transposed[j].push(pattern[i][j]);
        }
    }
    transposed
}

fn count_near_reflection(patterns: &Vec<Vec<char>>, n: i32) -> i32 {
    let mut num_cols_left = 0;
    for i in 1..patterns.len() {
        let mut is_match = true;
        let mut num_differences = 0;
        for j in 0..i.min(patterns.len() - i) {
            let left = patterns.get(i - j - 1).unwrap();
            let right = patterns.get(i + j).unwrap();
            let left_str = left.iter().collect::<String>();
            let right_str = right.iter().collect::<String>();

            num_differences += get_num_string_diff(left_str, right_str);
            if num_differences <= n {
                continue;
            } else {
                is_match = false;
                break;
            }
        }
        if is_match && num_differences == n {
            num_cols_left = i as i32;
            break;
        }
    }
    num_cols_left
}

fn get_num_string_diff(a: String, b: String) -> i32 {
    let mut diff_count = 0;
    for (char1, char2) in a.chars().zip(b.chars()) {
        if char1 != char2 {
            diff_count += 1;
        }
    }
    diff_count
}
