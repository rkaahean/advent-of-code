use regex::Regex;
use std::fs;

fn main() {
    const PATH: &str = "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc2.txt";

    let contents = fs::read_to_string(PATH).expect("Failed to read file");

    let contents = contents.lines();

    let red_regx = Regex::new(r"([0-9]+) red").unwrap();
    let green_regx = Regex::new(r"([0-9]+) green").unwrap();
    let blue_regx = Regex::new(r"([0-9]+) blue").unwrap();

    const RED: i32 = 12;
    const GREEN: i32 = 13;
    const BLUE: i32 = 14;

    let mut game_count = 0;
    for (game_num, line) in contents.clone().enumerate() {
        let mut is_too_big = false;

        for (color_regex, max_limit) in [(&red_regx, RED), (&green_regx, GREEN), (&blue_regx, BLUE)]
        {
            for (_, [ball]) in color_regex.captures_iter(line).map(|c| c.extract()) {
                let ball_count = ball.parse::<i32>().unwrap();
                if ball_count > max_limit {
                    is_too_big = true;
                    break;
                }
            }
        }

        if !is_too_big {
            game_count += game_num as i32 + 1;
        }
    }
    println!("Part 1: {}", game_count);

    let mut power_sum = 0;
    for line in contents {
        let mut power = 1;
        let mut ball_count: i32;
        let mut ball_min = 0;
        for (_, [ball]) in red_regx.captures_iter(line).map(|c| c.extract()) {
            ball_count = ball.parse::<i32>().unwrap();
            if ball_count > ball_min {
                ball_min = ball_count
            }
        }
        // println!("Red min {}", ball_min);
        power *= ball_min;

        ball_min = 0;
        for (_, [ball]) in green_regx.captures_iter(line).map(|c| c.extract()) {
            ball_count = ball.parse::<i32>().unwrap();
            if ball_count > ball_min {
                ball_min = ball_count
            }
        }
        // println!("Green min {}", ball_min);
        power *= ball_min;

        ball_min = 0;
        for (_, [ball]) in blue_regx.captures_iter(line).map(|c| c.extract()) {
            ball_count = ball.parse::<i32>().unwrap();
            if ball_count > ball_min {
                ball_min = ball_count
            }
        }
        // println!("Blue min {}", ball_min);

        power *= ball_min;
        power_sum += power;
    }
    println!("Power {}", power_sum);
}
