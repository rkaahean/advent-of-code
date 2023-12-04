use std::{cmp::max, fs};

fn main() {
    const PATH: &str =
        "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc4.txt";
    let contents = fs::read_to_string(PATH).expect("Failed to read file");
    let contents = contents.lines();

    // initialize vector for part 2
    let mut copy_count: Vec<i32> = Vec::new();
    let mut sm = 0;
    for line in contents.clone() {
        sm += points_for_matches(part_1(line));
        copy_count.push(0);
    }
    println!("Part 1 - {}", sm);

    // start calculating from index 1, so add another zero to the end
    copy_count.push(0);
    for (i, line) in contents.enumerate() {
        let card_num = i + 1;
        copy_count[card_num] += 1;
        let count = part_1(line);
        for j in card_num + 1..max(card_num + count + 1, count + 1) {
            copy_count[j] += copy_count[card_num];
        }
    }
    println!("Part 2 - {}", copy_count.iter().sum::<i32>());
}

fn part_1(text: &str) -> usize {
    let cards = text.split("|").collect::<Vec<&str>>();

    let card_2 = cards[1]
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let card_1 = cards[0].split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    card_2.iter().filter(|x| card_1.contains(x)).count()
}

fn points_for_matches(n: usize) -> i32 {
    if n == 0 {
        return 0;
    }
    2_i32.pow(n as u32 - 1)
}
