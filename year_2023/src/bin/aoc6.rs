use std::fs;

fn main() {
    const PATH: &str =
        "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc6.txt";
    let contents = fs::read_to_string(PATH).expect("Failed to read file");
    let contents = contents.lines();

    let mut times: Vec<f64> = Vec::new();
    let mut distances: Vec<f64> = Vec::new();

    for (i, line) in contents.enumerate() {
        if i == 0 {
            times = parse(line)
        }
        if i == 1 {
            distances = parse(line)
        }
    }
    let num_races = times.len();

    let mut part_1 = 1;
    for i in 0..num_races {
        let t = times[i];
        let d = distances[i];

        let comp = (t * t - 4 as f64 * d) as f64;

        let mut root1 = (t - comp.sqrt()) / 2 as f64;
        let mut root2 = (t + comp.sqrt()) / 2 as f64;

        if root1.fract() == 0.0 {
            root1 += 1 as f64;
        }

        if root2.fract() == 0.0 {
            root2 -= 1 as f64;
        }
        let nums = root1.ceil() as i32..root2.floor() as i32;
        println!("Power {}", nums.len() + 1);
        part_1 *= nums.len() + 1
    }
    println!("Power {}", part_1)
}

fn parse(line: &str) -> Vec<f64> {
    line.split(":").collect::<Vec<&str>>()[1]
        .trim()
        .split(" ")
        .filter(|x| !x.is_empty())
        .map(|x| x.trim().parse::<f64>().unwrap())
        .collect()
}
