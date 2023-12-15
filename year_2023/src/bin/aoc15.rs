use std::{collections::HashMap, fs};

fn main() {
    const PATH: &str =
        "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc15.txt";
    let contents = fs::read_to_string(PATH).expect("Failed to read file");

    // Part 1
    println!(
        "Part 1- {}",
        contents.split(",").map(|x| hash_algorithm(x)).sum::<i32>()
    );

    // Part 2
    let mut boxes: HashMap<i32, Vec<(&str, i32)>> = HashMap::new();
    let instructions = contents.split(",").collect::<Vec<&str>>();
    // initialize
    for i in 0..256 {
        boxes.insert(i, Vec::new());
    }

    for instr in instructions {
        let (label, box_idx, focal) = hash_algorithm_part_2(instr);
        let focal_box = boxes.get(&box_idx).unwrap();
        let mut new_box = focal_box.clone();
        // = operation, to insert
        let idx = new_box.iter().position(|(lbl, _)| *lbl == label);
        if focal > 0 {
            // element already exists
            if !idx.is_none() {
                new_box[idx.unwrap()] = (label, focal);
            } else {
                new_box.push((label, focal));
            }
        }
        // remove operation
        else if !idx.is_none() {
            new_box.remove(idx.unwrap());
        }
        boxes.insert(box_idx, new_box);
    }

    let mut sm = 0;
    for i in 0..256 {
        let bx = boxes.get(&i).unwrap();
        if !bx.is_empty() {
            for (j, (_, focal)) in bx.iter().enumerate() {
                sm += (i + 1) as i32 * (j + 1) as i32 * focal;
            }
        }
    }
    println!("Part 2 - {}", sm);
}

fn hash_algorithm(hash: &str) -> i32 {
    let mut current = 0;
    for ch in hash.as_bytes() {
        current += *ch as i32;
        current *= 17;
        current = current % 256;
    }
    current
}

fn hash_algorithm_part_2(hash: &str) -> (&str, i32, i32) {
    let n = hash.len();
    let mut focal: i32 = -1;
    let label;
    if hash.contains("-") {
        label = &hash[0..n - 1];
    } else {
        label = &hash[0..n - 2];
        focal = hash.chars().last().unwrap().to_digit(10).unwrap() as i32;
    }
    (label, hash_algorithm(label), focal)
}
