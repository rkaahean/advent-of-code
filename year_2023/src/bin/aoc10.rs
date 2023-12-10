use std::{collections::HashMap, fs};

fn main() {
    const PATH: &str =
        "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc10.txt";
    let contents = fs::read_to_string(PATH).expect("Failed to read file");
    let contents = contents.lines();

    let mut matrix: Vec<Vec<char>> = Vec::new();
    let mut start: (usize, usize) = (0, 0);
    for (i, line) in contents.enumerate() {
        if line.contains("S") {
            start = (i, line.find("S").unwrap());
        }
        matrix.push(line.chars().collect());
    }
    part1(&matrix, start);
    part2(&matrix, start);
}

fn part1(matrix: &Vec<Vec<char>>, start: (usize, usize)) {
    let max_loop = get_loops(matrix, start);
    println!("Part 1 {}", max_loop.0.len() / 2);
}

fn part2(matrix: &Vec<Vec<char>>, start: (usize, usize)) {
    let max_loop = get_loops(matrix, start);
    // Pick's theorem
    let area = get_area(&max_loop.1);
    println!("Part 2: {}", area - (max_loop.1.len() / 2) as f64 + 1.0);
}

fn get_loops(matrix: &Vec<Vec<char>>, start: (usize, usize)) -> (String, Vec<(usize, usize)>) {
    let x_length = matrix.len() as i32;
    let y_length = matrix[0].len() as i32;

    let mut map: HashMap<(i32, i32), Vec<char>> = HashMap::new();
    map.insert((0, 1), ['-', 'J', '7', 'S'].to_vec());
    map.insert((-1, 0), ['|', 'F', '7', 'S'].to_vec());
    map.insert((0, -1), ['-', 'L', 'F', 'S'].to_vec());
    map.insert((1, 0), ['|', 'L', 'J', 'S'].to_vec());

    // loop_str contains the final loop
    let mut graph: Vec<(usize, usize, String, Vec<(usize, usize)>)> = Vec::new();
    graph.push((start.0, start.1, String::from("S"), [start].to_vec()));
    let mut loop_str: (String, Vec<(usize, usize)>) = ("S".to_string(), Vec::from([start]));
    let mut visited: Vec<(usize, usize)> = Vec::new();

    while !graph.is_empty() {
        let (idx_0, idx_1, path, turn) = graph.pop().unwrap();
        let idx = (idx_0, idx_1);
        let ch = matrix.get(idx.0).unwrap().get(idx.1).unwrap();
        // check if already seen this node
        if visited.contains(&idx) || (*ch == 'S' && path.len() > 3) {
            continue;
        }
        // mark as visited!
        visited.push(idx);
        // find and explore neighbours
        let mut neighbours: Vec<(usize, usize, String, Vec<(usize, usize)>)> = Vec::new();
        for (a, b) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let neigh_x = idx.0 as i32 + a;
            let neigh_y = idx.1 as i32 + b;
            let (neigh_x_idx, neigh_y_idx) = (neigh_x as usize, neigh_y as usize);

            // check if in bounds
            if neigh_x < 0 || neigh_x > x_length - 1 || neigh_y < 0 || neigh_y > y_length - 1 {
                continue;
            }
            // get neighbour, check if there's a path
            // check if not already visited, unless its start
            let neighbour = matrix.get(neigh_x_idx).unwrap().get(neigh_y_idx).unwrap();
            let possible = map.get(&(a, b)).unwrap();
            if possible.contains(neighbour)
                && (!visited.contains(&(neigh_x_idx, neigh_y_idx)) || *neighbour == 'S')
            {
                let mut new_path = path.clone().to_string();
                let mut new_turn = turn.clone();
                new_path.push(*neighbour);

                let neighbour_point = (neigh_x_idx, neigh_y_idx);
                if !new_turn.contains(&neighbour_point) {
                    new_turn.push(neighbour_point);
                }
                // keep track of longest closed cycles
                if new_path.starts_with("S")
                    && new_path.ends_with("S")
                    && new_path.len() > loop_str.0.len()
                {
                    loop_str = (new_path.clone(), new_turn.clone());
                }
                neighbours.push((neigh_x_idx, neigh_y_idx, new_path, new_turn));
            }
        }
        neighbours.reverse();
        graph.extend(neighbours);
    }
    loop_str
}

fn get_area(points: &Vec<(usize, usize)>) -> f64 {
    // using shoelace formula
    let mut sum1: f64 = 0.0;
    let mut sum2: f64 = 0.0;
    let n = points.len();

    for i in 0..n {
        let (x1, y1) = points[i];
        let (x2, y2) = points[(i + 1) % n];

        sum1 += (x1 * y2) as f64;
        sum2 += (x2 * y1) as f64;
    }
    (sum2 - sum1).abs() / 2.0
}
