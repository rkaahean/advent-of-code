use std::{
    collections::{HashMap, HashSet},
    fs,
    hash::Hash,
};

fn main() {
    // trying to read in the data
    println!("Reading input file...");

    // specifying the library
    let file_path = "/Users/ranjanwork-personal/rustaoc/src/data/aoc9.txt";
    let contents = fs::read_to_string(file_path).expect("Something wrong in reading the file...");

    // points visited
    let mut visited_location: Vec<(i32, i32)> = Vec::from([(0, 0)]);
    let mut vis_has: HashSet<(i32, i32)> = HashSet::from([(0, 0)]);

    // starting point
    let mut point: (i32, i32) = (0, 0);

    for rope_move in contents.lines() {
        // get move direction and count
        let split_string = rope_move.split(" ").collect::<Vec<&str>>();
        let move_direction = split_string[0];
        let move_count: u32 = split_string[1].parse().unwrap();

        println!("{}, {}", move_direction, move_count);
        for _ in 0..move_count {
            handle_move(move_direction, &mut visited_location, &mut point);
        }
    }
    println!("{:?}, {:?}", visited_location, point);

    for i in visited_location {
        vis_has.insert(i);
    }
    println!("{:?}", vis_has.len());
}

fn handle_move(
    move_direction: &str,
    visited_location: &mut Vec<(i32, i32)>,
    head_location: &mut (i32, i32),
) {
    // get latest tail location
    let (a, b) = visited_location.pop().unwrap();
    println!("{:?}, H: {:?}", (a, b), head_location);
    let move_map: HashMap<&str, i32> = HashMap::from([("L", -1), ("R", 1), ("U", 1), ("D", -1)]);

    visited_location.push((a, b));
    // get tail loacation
    let (x, y) = head_location;

    // move head locations accordingly
    if move_direction == "L" || move_direction == "R" {
        // get latest tail
        *x = *x + move_map.get(move_direction).unwrap();
    } else {
        *y = *y + move_map.get(move_direction).unwrap();
    }

    // decide where to move tail
    let (mut new_tail_x, mut new_tail_y) = (a, b);

    if *y == b {
        // head and tail are in the same row
        if i32::abs((*x - a) as i32) <= 1 {
            // if AFTER moving head, it is still
            // 1 unit away, do nothing
            visited_location.push((new_tail_x, new_tail_y));
            return;
        }
        // else, move tail row wise
        new_tail_x = move_tail(*x, a);
    } else if *x == a {
        // head and tail are in the same column
        if i32::abs((*y - b) as i32) <= 1 {
            // if AFTER moving head, it is still
            // 1 unit away, do nothing
            visited_location.push((new_tail_x, new_tail_y));
            return;
        }
        // else, move tail column wise
        new_tail_y = move_tail(*y, b);
    } else if i32::abs((*y - b) as i32) == 1 && i32::abs((*x - a) as i32) == 1 {
        // if diagonally opposite
    } else {
        // in a diagonal but far away
        if move_direction == "U" {
            // move towards head
            new_tail_y = b + 1;
            new_tail_x = move_tail(*x, a);
        }
        if move_direction == "D" {
            new_tail_y = b - 1;
            new_tail_x = move_tail(*x, a);
        }
        if move_direction == "L" {
            new_tail_x = a - 1;
            new_tail_y = move_tail(*y, b);
        }
        if move_direction == "R" {
            new_tail_x = a + 1;
            new_tail_y = move_tail(*y, b);
        }
    }
    visited_location.push((new_tail_x, new_tail_y));
}

fn move_tail(head: i32, tail: i32) -> i32 {
    if head > tail {
        return tail + 1;
    } else {
        return tail - 1;
    }
}
