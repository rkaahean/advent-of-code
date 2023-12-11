use std::fs;

fn main() {
    const PATH: &str =
        "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc11.txt";
    let contents = fs::read_to_string(PATH).expect("Failed to read file");
    let contents = contents.lines();

    // create a list of galaxies
    let mut galaxies: Vec<(usize, usize)> = Vec::new();
    let mut num_rows = 0;
    let mut num_cols = 0;
    for (row_idx, line) in contents.enumerate() {
        num_cols = line.len();
        num_rows = row_idx + 1;
        let galaxy_idxs = line
            .chars()
            .enumerate()
            .filter(|(_, ch)| *ch == '#')
            .map(|(i, _)| (row_idx, i))
            .collect::<Vec<(usize, usize)>>();
        galaxies.extend(galaxy_idxs);
    }

    let x_idxs = galaxies.iter().map(|(x, _)| *x).collect::<Vec<usize>>();
    let y_idxs = galaxies.iter().map(|(_, y)| *y).collect::<Vec<usize>>();

    let missing_rows = (0..num_rows)
        .filter(|x| !x_idxs.contains(x))
        .collect::<Vec<usize>>();
    let missing_cols = (0..num_cols)
        .filter(|y| !y_idxs.contains(y))
        .collect::<Vec<usize>>();

    let mut p1 = 0;
    let mut p2 = 0;
    for i in 0..galaxies.len() {
        for j in i + 1..galaxies.len() {
            let (x0, y0) = galaxies.get(i).unwrap();
            let (x1, y1) = galaxies.get(j).unwrap();
            p1 += get_distance(
                (*x0 as i32, *y0 as i32),
                (*x1 as i32, *y1 as i32),
                &missing_rows,
                &missing_cols,
                2,
            );
            p2 += get_distance(
                (*x0 as i32, *y0 as i32),
                (*x1 as i32, *y1 as i32),
                &missing_rows,
                &missing_cols,
                1000000,
            );
        }
    }
    println!("Part 1 {}", p1);
    println!("Part 2 {}", p2);
}

fn get_distance(
    a: (i32, i32),
    b: (i32, i32),
    missing_rows: &Vec<usize>,
    missing_cols: &Vec<usize>,
    gap_length: i64,
) -> i64 {
    let (x0, y0) = a;
    let (x1, y1) = b;

    // traiditional
    let x_dist = (x1 - x0).abs() as i64;
    let y_dist = (y1 - y0).abs() as i64;

    let (x_start, x_end) = (x0.min(x1), x0.max(x1));
    let (y_start, y_end) = (y0.min(y1), y0.max(y1));

    // now to find how many double distances to add
    let num_double_x = missing_rows
        .iter()
        .filter(|x| **x as i32 - x_start > 0 && **x as i32 - x_end < 0)
        .count();
    let num_double_y = missing_cols
        .iter()
        .filter(|y| **y as i32 - y_start > 0 && **y as i32 - y_end < 0)
        .count();

    x_dist
        + y_dist
        + (gap_length - 1) * num_double_x as i64
        + (gap_length - 1) * num_double_y as i64
}
