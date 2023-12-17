use std::{fs, usize, vec};

fn main() {
    const PATH: &str =
        "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc16.txt";
    let contents = fs::read_to_string(PATH).expect("Failed to read file");
    let contents = contents.lines();

    let mut beams = Vec::new();
    for line in contents {
        beams.push(line.chars().collect::<Vec<char>>());
    }

    // Part 1
    println!("Part 1 - {}", part1(&beams, ((0, 0), (0, 1))));

    // Part 2
    let x_len = beams.len() as i32;
    let y_len = beams.len() as i32;

    // 1st and last row
    let mut mx = 0;
    for j in 0..y_len {
        let up_row = part1(&beams, ((0, j as i32), (1, 0)));
        let down_row = part1(&beams, ((y_len - 1, j as i32), (-1, 0)));

        let row_max = up_row.max(down_row);
        if row_max > mx { mx = row_max};
    }

    for i in 0..x_len {
        let up_col = part1(&beams, ((i as i32, 0), (0, 1)));
        let down_col = part1(&beams, ((i as i32, x_len - 1), (0, -1)));
        let row_max = up_col.max(down_col);
        if row_max > mx { mx = row_max};
    }
    println!("Part 2 - {}", mx);

}

fn move_light(
    matrix: &Vec<Vec<char>>,
    current: (i32, i32),
    direction: (i32, i32),
) -> Vec<((i32, i32), (i32, i32))> {
    /*
    1. get the current char
    2.
        a. if a beam, return get the new direction
        b. if not a beam, maintain the same direction
    3. return current_idx + direction. Also return new direction
    */
    let node = get_node(matrix, current);
    let new_directions = match node {
        Some(node) => {
            // if you encounter a beam, new direction
            if node == '/' || node == '\\' || node == '|' || node == '-' {
                get_new_direction(direction, node)
            }
            // otherwise continue going as is
            else {
                vec![direction]
            }
        }
        None => vec![],
    };
    // next position(s) + directions
    new_directions
        .iter()
        .map(|x| ((current.0 + x.0, current.1 + x.1), (x.0, x.1)))
        .collect()
}

fn get_new_direction(incoming: (i32, i32), beam: char) -> Vec<(i32, i32)> {
    let mut direction = Vec::new();
    // println!("Getting new direction for {:?}: {}", incoming, beam);
    if beam == '/' {
        let dirs = match incoming {
            (0, 1) => (-1, 0),
            (0, -1) => (1, 0),
            (1, 0) => (0, -1),
            (-1, 0) => (0, 1),
            _ => (-1, -1),
        };
        direction.push(dirs);
    } else if beam == '\\' {
        let dirs = match incoming {
            (0, 1) => (1, 0),
            (0, -1) => (-1, 0),
            (1, 0) => (0, 1),
            (-1, 0) => (0, -1),
            _ => (-1, -1),
        };
        direction.push(dirs);
    } else if beam == '|' {
        let dirs = match incoming {
            (0, 1) => vec![(-1, 0), (1, 0)],
            (0, -1) => vec![(-1, 0), (1, 0)],
            (1, 0) => vec![(1, 0)],
            (-1, 0) => vec![(-1, 0)],
            _ => vec![(-1, -1)],
        };
        direction = dirs.to_vec();
    } else if beam == '-' {
        let dirs = match incoming {
            (0, 1) => vec![(0, 1)],
            (0, -1) => vec![(0, -1)],
            (1, 0) => vec![(0, -1), (0, 1)],
            (-1, 0) => vec![(0, -1), (0, 1)],
            _ => vec![(-1, -1)],
        };
        direction = dirs.to_vec();
    }
    direction
}

fn get_node(matrix: &Vec<Vec<char>>, idx: (i32, i32)) -> Option<char> {
    let y_len = matrix[0].len() as i32;
    let x_len = matrix[0].len() as i32;

    if idx.0 >= 0 && idx.0 < x_len && idx.1 >= 0 && idx.1 < y_len {
        return Some(matrix[idx.0 as usize][idx.1 as usize]);
    }
    None
}

fn get_unique(paths: &Vec<((i32, i32), (i32, i32))>) -> i32 {
    let mut seen_vertices = Vec::new();
    for (node, _) in paths {
        if !seen_vertices.contains(node) {
            seen_vertices.push(*node);
        }
    }
    seen_vertices.len() as i32
}

fn part1(beams: &Vec<Vec<char>>, start: ((i32, i32), (i32, i32))) -> i32 {
    let mut seen = vec![];
    // keep track of traverals

    // dfs!
    let mut exploring = vec![start];
    while !exploring.is_empty() {
        // get the current node being explored
        let (current, direction) = exploring.pop().unwrap();
        // if current node is invalid
        if get_node(&beams, current).is_none() {
            continue;
        }

        // if already seen current and new positions, skip
        let new_positions = move_light(&beams, current, direction);
        // new_positions.iter().all(|(_, dir)| seen.contains(&dir)
        if seen.contains(&(current, direction)) && new_positions.iter().all(|x| seen.contains(&x)) {
            continue;
        } else if !seen.contains(&(current, direction)) {
            seen.push((current, direction))
        }
        // add positions ot exploring!
        exploring.extend(new_positions);
    }
    get_unique(&seen)
}
