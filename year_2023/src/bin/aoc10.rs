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
    get_neighbours(&matrix, start);
}

fn get_neighbours(matrix: &Vec<Vec<char>>, start: (usize, usize)) {
    let x_length = matrix[0].len() as i32;
    let y_length = matrix.len() as i32;

    let mut map: HashMap<(i32, i32), Vec<char>> = HashMap::new();
    map.insert((0, 1), ['-', 'J', '7', 'S'].to_vec());
    map.insert((-1, 0), ['|', 'F', '7', 'S'].to_vec());
    map.insert((0, -1), ['-', 'L', 'F', 'S'].to_vec());
    map.insert((1, 0), ['|', 'L', 'J', 'S'].to_vec());

    // loop_str contains the final loop
    let mut graph: Vec<(usize, usize, String)> = Vec::new();
    graph.push((start.0, start.1, String::from("S")));
    let mut loop_str: String = String::from("");
    let mut visited: Vec<(usize, usize)> = Vec::new();

    while !graph.is_empty() {
        let (idx_0, idx_1, path) = graph.pop().unwrap();
        let idx = (idx_0, idx_1);

        let ch = matrix.get(idx.0).unwrap().get(idx.1).unwrap();
        // check if already seen this node
        if visited.contains(&idx) || (*ch == 'S' && path.len() > 3) {
            continue;
        }
        // mark as visited!
        visited.push(idx);

        // find and explore neighbours
        let mut neighbours: Vec<(usize, usize, String)> = Vec::new();
        for (a, b) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let neigh_x = idx.0 as i32 + a;
            let neigh_y = idx.1 as i32 + b;

            // check if in bounds
            if neigh_x < 0 || neigh_x > x_length - 1 || neigh_y < 0 || neigh_y > y_length - 1 {
                continue;
            }

            // check if the neighbour is a possible path
            let (neigh_x_idx, neigh_y_idx) = (neigh_x as usize, neigh_y as usize);
            let neighbour = matrix.get(neigh_x_idx).unwrap().get(neigh_y_idx).unwrap();
            let possible = map.get(&(a, b)).unwrap();
            if possible.contains(neighbour)
                && (!visited.contains(&(neigh_x_idx, neigh_y_idx)) || *neighbour == 'S')
            {
                let mut new_path = path.clone().to_string();
                new_path.push(*neighbour);

                if new_path.starts_with("S")
                    && new_path.ends_with("S")
                    && path.len() > loop_str.len()
                {
                    loop_str = new_path.clone();
                }
                neighbours.push((neigh_x_idx, neigh_y_idx, new_path));
            }
        }
        neighbours.reverse();
        graph.extend(neighbours);
    }
    println!("Part 1 {}", loop_str.len() / 2);
}
