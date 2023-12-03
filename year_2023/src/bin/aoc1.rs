use std::fs;

extern crate regex;

use regex::{Captures, Regex};

fn main() {
    const PATH: &str = "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc1.txt";

    let contents = fs::read_to_string(PATH).expect("Failed to read file");

    let contents = contents.lines();

    let mut sm = 0;
    for line in contents.clone() {
        let mut digit_1 = 0;
        let mut digit_2 = 0;
        for i in line.chars() {
            if i.is_numeric() {
                digit_1 = i.to_string().parse::<i32>().unwrap();
                // println!("Digit 1, {}", digit_1);
                break;
            }
        }
        for i in line.chars().rev() {
            if i.is_numeric() {
                digit_2 = i.to_string().parse::<i32>().unwrap();
                // println!("Digit 1, {}", digit_2);
                break;
            }
        }
        sm += 10 * digit_1 + digit_2;
    }
    println!("Part 1, {}", sm);

    sm = 0;
    for line in contents {
        let mut digit_1 = 0;
        let mut digit_2 = 0;

        let replaced_line = replace_numstring(line);
        // println!("Line {}", replaced_line);
        for i in replaced_line.chars() {
            if i.is_numeric() {
                digit_1 = i.to_string().parse::<i32>().unwrap();
                break;
            }
        }
        let replaced_line_rev =
            replace_numstring_reverse(line.chars().rev().collect::<String>().as_str());
        for i in replaced_line_rev.chars() {
            if i.is_numeric() {
                digit_2 = i.to_string().parse::<i32>().unwrap();
                // println!("Digit 1, {}", digit_2);
                break;
            }
        }

        sm += 10 * digit_1 + digit_2;
    }
    println!("Sum {}", sm);
}

fn replace_numstring(text: &str) -> String {
    let re = Regex::new("(one|two|three|four|five|six|seven|eight|nine)").unwrap();
    let result = re.replace_all(text, |cap: &Captures| {
        match &cap[0] {
            "one" => "1",
            "two" => "2",
            "three" => "3",
            "four" => "4",
            "five" => "5",
            "six" => "6",
            "seven" => "7",
            "eight" => "8",
            "nine" => "9",
            x => x,
        }
        .to_string()
    });
    // println!("{}", result.to_string());

    return result.to_string();
}

fn replace_numstring_reverse(text: &str) -> String {
    let re = Regex::new("(eno|owt|eerht|ruof|evif|xis|neves|thgie|enin)").unwrap();
    let result = re.replace_all(text, |cap: &Captures| {
        match &cap[0] {
            "eno" => "1",
            "owt" => "2",
            "eerht" => "3",
            "ruof" => "4",
            "evif" => "5",
            "xis" => "6",
            "neves" => "7",
            "thgie" => "8",
            "enin" => "9",
            x => x,
        }
        .to_string()
    });

    return result.to_string();
}
