use std::fs;

fn main() {
    const PATH: &str =
        "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc3.txt";

    let contents = fs::read_to_string(PATH).expect("Failed to read file");

    let contents = contents.lines();
    let matrix: Vec<Vec<char>> = contents.map(|x| x.chars().collect::<Vec<char>>()).collect();

    const DIRS: [(i32, i32); 8] = [
        (0, 1),
        (1, 0),
        (-1, 0),
        (0, -1),
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ];

    let col_len = matrix[0].len() as i32;
    let row_len = matrix.len() as i32;

    let mut nums: Vec<((usize, usize), (usize, usize))> = Vec::new();

    let mut sm = 0;
    for i in 0..row_len {
        for j in 0..col_len {
            let val = matrix
                .get(i as usize)
                .unwrap()
                .get(j as usize)
                .unwrap()
                .to_string();

            if val != "*" {
                continue;
            }

            let mut num_neighbours: Vec<i32> = Vec::new();
            for (x, y) in DIRS {
                let neigh_x = i as i32 + x;
                let neigh_y = j as i32 + y;
                if (neigh_x >= 0 && neigh_x < col_len) && (neigh_y >= 0 && neigh_y < row_len) {
                    let neigh_value = matrix
                        .get(neigh_x as usize)
                        .unwrap()
                        .get(neigh_y as usize)
                        .unwrap()
                        .to_string();

                    // if neighbour not a number
                    if neigh_value.parse::<i32>().is_err() {
                        continue;
                    }
                    let (num, found_start, found_end) =
                        get_num_for_idx(neigh_x as usize, neigh_y as usize, &matrix);

                    if nums.contains(&(found_start, found_end)) {
                        continue;
                    }
                    num_neighbours.push(num);
                    nums.push((found_start, found_end));
                }
            }
            if num_neighbours.len() == 2 {
                sm += num_neighbours.iter().fold(1, |acc, x| acc * x);
            }
        }
    }

    println!("Sum {}", sm);
}

fn get_num_for_idx(
    i: usize,
    j: usize,
    matrix: &Vec<Vec<char>>,
) -> (i32, (usize, usize), (usize, usize)) {
    let mut nums: Vec<char> = Vec::new();
    let row = matrix.get(i).unwrap();

    let mut start_idx = (i, j);
    let mut end_idx = (i, j);
    for idx in (0..j + 1).rev() {
        let val = row.get(idx).unwrap();
        if val.is_numeric() {
            nums.push(*val)
        } else {
            start_idx = (i, idx + 1);
            break;
        }
        start_idx = (i, 0);
    }
    nums.reverse();
    for idx in j + 1..matrix[0].len() {
        let val = row.get(idx).unwrap();
        if val.is_numeric() {
            nums.push(*val)
        } else {
            end_idx = (i, idx - 1);
            break;
        }
        end_idx = (i, matrix[0].len() - 1)
    }

    let num_string: String = nums.into_iter().collect();
    (num_string.parse::<i32>().unwrap(), start_idx, end_idx)
}
