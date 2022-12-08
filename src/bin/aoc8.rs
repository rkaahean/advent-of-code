use std::fs;

fn main() {
    // trying to read the input data
    println!("Reading input file...");

    // specifying the library
    let file_path = "/Users/ranjanwork-personal/rustaoc/src/data/aoc8.txt";
    let contents = fs::read_to_string(file_path).expect("Something wrong in reading the file...");

    // parse input
    let trees = parse_input(contents);
    println!("{:?}", trees);

    // define max values
    let mut count: usize = 0;

    // PART 1
    // for (i, vec) in trees.iter().enumerate() {
    //     for (j, _) in vec.iter().enumerate() {
    //         let is_valid = is_visible(&trees, i as usize, j as usize)
    //             || is_visible(&transpose_trees(&trees), j as usize, i as usize);
    //         println!(
    //             "Checking for {}, {} = {}. Val = {}",
    //             i,
    //             j,
    //             is_valid,
    //             get_value(&trees, i as usize, j as usize)
    //         );
    //         if is_valid {
    //             count += 1
    //         }
    //     }
    // }
    // println!("{}", count);

    // PART 2
    let mut max_distance = 0;
    for (i, vec) in trees.iter().enumerate() {
        for (j, _) in vec.iter().enumerate() {
            let view_distance = part2(&trees, i as usize, j as usize)
                * part2(&transpose_trees(&trees), j as usize, i as usize);
            println!(
                "Checking for {}, {} = {}. Val = {}",
                i,
                j,
                view_distance,
                get_value(&trees, i as usize, j as usize)
            );
            if view_distance >= max_distance {
                max_distance = view_distance;
            }
        }
    }
    println!("{}", max_distance);
}

fn parse_input(contents: String) -> Vec<Vec<usize>> {
    // get the moves line by lines
    let mut trees: Vec<Vec<usize>> = Vec::new();

    for line in contents.lines() {
        let mut tree_row: Vec<usize> = Vec::new();
        for tree_val in line.chars() {
            let i_int: usize = tree_val.to_digit(10).unwrap().try_into().unwrap();
            tree_row.push(i_int);
        }
        trees.push(tree_row);
    }
    trees
}

fn transpose_trees(trees: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut transpose_tree: Vec<Vec<usize>> = Vec::new();

    for (i, vec) in trees.iter().enumerate() {
        let mut tree_row: Vec<usize> = Vec::new();
        for (j, _) in vec.iter().enumerate() {
            tree_row.push(trees[j][i]);
        }
        transpose_tree.push(tree_row);
    }
    transpose_tree
}

fn get_value(trees: &Vec<Vec<usize>>, i: usize, j: usize) -> usize {
    return trees[i][j];
}

fn is_visible(trees: &Vec<Vec<usize>>, i: usize, j: usize) -> bool {
    println!("{}, {}", i, j);
    // a function is visible or not
    let tree_val = get_value(trees, i, j);

    // if 1st row or column
    if i == 0 || j == 0 {
        return true;
    }
    // if last row or column
    else if i == trees.len() - 1 || j == trees[i].len() - 1 {
        return true;
    } else {
        // the get max value across all axes;
        let tree_row = &trees[i];

        // max value before it in the row
        let row_prev_max = tree_row[0..j].into_iter().max().unwrap();

        // max value after it in the row
        let row_after_max = tree_row[j + 1..].into_iter().max().unwrap();

        if tree_val > *row_prev_max || tree_val > *row_after_max {
            return true;
        }
    }
    return false;
}

fn part2(trees: &Vec<Vec<usize>>, i: usize, j: usize) -> usize {
    // a function is visible or not
    let tree_val = get_value(trees, i, j);
    println!("Tree Val: {}", tree_val);

    // if 1st row or column
    if j == 0 || j == trees[i].len() - 1 {
        // viewing distance in the 1st or last column = 0 as its on edge
        return 0;
    } else {
        // the get max value across all axes;
        let tree_row = &trees[i];

        // max value before it in the row
        let mut prev_vec = tree_row[0..j].to_vec();
        prev_vec.reverse();
        let row_prev_max = get_distance(tree_val, &prev_vec);

        // max value after it in the row
        let row_after_max = get_distance(tree_val, &tree_row[j + 1..].to_vec());

        return row_prev_max * row_after_max;
    }
}

fn get_distance(val: usize, trees: &Vec<usize>) -> usize {
    // tree count
    let mut count = 0;
    for i in trees {
        if val >= *i {
            count += 1;
            if val == *i {
                break;
            }
        } else {
            count += 1;
            break;
        }
    }
    // println!("{}", count);
    // println!("Tree Compare: {:?}", trees);
    count
}
