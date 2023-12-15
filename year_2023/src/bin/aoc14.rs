use std::fs;

fn main() {
    const PATH: &str =
        "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc14.txt";
    let contents = fs::read_to_string(PATH).expect("Failed to read file");
    let contents = contents.lines();

    // collect input
    let mut rocks: Vec<Vec<char>> = Vec::new();
    for line in contents {
        rocks.push(line.chars().collect());
    }

    println!("Part 1 - {}", get_north_load(&move_rocks(&rocks, 'N')));

    let mut seen: Vec<Vec<Vec<char>>> = Vec::new();
    let mut seen_idx = 0;
    seen.push(rocks.clone());
    for cycle_idx in 0..1000 {
        rocks = move_cycle(&rocks);
        let mut same = false;
        for (i, old) in seen.iter().enumerate() {
            if is_same(old.clone(), rocks.clone()) {
                seen_idx = i;
                same = true;
                break;
            }
        }
        // already seen it before
        if same {
            let num_iters = (cycle_idx - seen_idx) as i32 + 1;
            let current_cycle = cycle_idx as i32;
            let mult = ((1000000000.0  - current_cycle as f64)/ (num_iters as f64)).floor() as i32;
            let new_start = current_cycle + mult*num_iters;
            for _ in new_start..1000000000 - 1 {
                rocks = move_cycle(&rocks);
            }
            println!("Part 2 - {}", get_north_load(&rocks));
            break;
        }
        seen.push(rocks.clone());
    }
}

fn get_north_load(rocks: &Vec<Vec<char>>) -> i32 {
    transpose(rocks.to_vec())
        .iter()
        .fold(0, |acc, x| acc + get_row_scores(x))
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

fn get_row_scores(layer: &Vec<char>) -> i32 {
    let n = layer.len();
    layer
        .iter()
        .enumerate()
        .map(|(i, rock)| if *rock == 'O' { n - i } else { 0 })
        .sum::<usize>() as i32
}

fn move_cycle(rocks: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut new_cycled_rocks = rocks.clone();
    for direction in ['N', 'W', 'S', 'E'] {
        new_cycled_rocks = move_rocks(&new_cycled_rocks, direction);
    }
    new_cycled_rocks
}

fn move_rocks(rocks: &Vec<Vec<char>>, direction: char) -> Vec<Vec<char>> {
    /*
    if Left, just return
    if Right, compue reverse, and then insert the reverse
    if North, transpose, compute, then transpose again
    if South, transpose, compute reverse, reverse, then transpose again
     */
    let mut new_cycled: Vec<Vec<char>> = Vec::new();
    let mut input_rocks = rocks.clone();

    if direction == 'N' || direction == 'S' {
        input_rocks = transpose(input_rocks);
    }

    for rock_line in input_rocks {
        let mut desired_rock_line = rock_line.clone();
        if direction == 'E' || direction == 'S' {
            desired_rock_line.reverse();
        }
        let mut cycled_rock_line = cycle_rock_line(&desired_rock_line);
        if direction == 'E' || direction == 'S' {
            cycled_rock_line.reverse();
        }
        new_cycled.push(cycled_rock_line)
    }
    if direction == 'N' || direction == 'S' {
        transpose(new_cycled)
    } else {
        new_cycled
    }
}

fn cycle_rock_line(rock_line: &Vec<char>) -> Vec<char> {
    let mut row: Vec<char> = Vec::new();
    let mut num_empty = 0;
    for ch in rock_line {
        if *ch == 'O' {
            row.push('O');
        } else if *ch == '.' {
            num_empty += 1;
        } else {
            for _ in 0..num_empty {
                row.push('.');
            }
            num_empty = 0;
            row.push('#');
        }
    }
    for _ in 0..num_empty {
        row.push('.');
    }
    row
}

fn is_same(old: Vec<Vec<char>>, new: Vec<Vec<char>>) -> bool {
    old.iter()
        .zip(new.iter())
        .all(|(a, b)| a.into_iter().collect::<String>() == b.into_iter().collect::<String>())
}
