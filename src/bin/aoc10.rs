use std::{collections::HashMap, fs, hash::Hash};

fn main() {
    // trying to read in the data
    println!("Reading input file...");

    // specifying the library
    let file_path = "/Users/ranjanwork-personal/rustaoc/src/data/aoc10.txt";
    let contents = fs::read_to_string(file_path).expect("Something wrong in reading the file...");

    // register value
    let mut X = 1;
    let mut cycle: i32 = 1;
    let mut values: HashMap<i32, i32> = HashMap::new();
    values.insert(0, 1);
    // values.insert(1, 1);
    // compute cycle values
    for operation in contents.lines() {
        let command: Vec<&str> = operation.split(" ").collect();

        if command[0] == "addx" {
            // get out the value
            X += command[1].parse::<i32>().unwrap();
            cycle += 2;
        } else {
            cycle += 1;
        }
        values.insert(cycle, X);
    }
    // get cycle values: PART 1
    // let mut val = 0;
    // for mut i in [20, 60, 100, 140, 180, 220] {
    //     val += i * get_cycle_value(&mut i, &mut values);
    //     println!("{}: {}", i, val);
    // }

    // PART 2
    // println!("{:?}", values);
    for i in 1..240 {
        let mut l = i.clone();
        print!("{}--{}  ", i, get_cycle_value(&mut l, &mut values));
    }
    println!();
    let mut sprite_pos = 1;
    for cycle_iterate in 1..241 {
        // print!("{}", sprite_pos);

        let mut disp_position = cycle_iterate % 40;
        let is_display = should_display(&mut disp_position, &mut values, &mut sprite_pos);
        if is_display {
            print!("#")
        } else {
            print!(".")
        }
        // move sprite
        let mut next_pos = cycle_iterate + 1;
        sprite_pos = get_cycle_value(&mut next_pos, &mut values);

        if cycle_iterate % 40 == 0 {
            println!();
            sprite_pos = 1;
        }
    }
}

fn get_cycle_value(cycle: &mut i32, values: &mut HashMap<i32, i32>) -> i32 {
    if values.contains_key(cycle) {
        return values.get(&cycle).copied().unwrap();
    } else if values.contains_key(&(*cycle - 1)) {
        return values.get(&(*cycle - 1)).copied().unwrap();
    } else {
        return values.get(&(*cycle - 2)).copied().unwrap();
    }
}

fn should_display(cycle: &mut i32, values: &mut HashMap<i32, i32>, sprite_pos: &mut i32) -> bool {
    // let register_value = get_cycle_value(cycle, values);

    // if *cycle <= *sprite_pos && *sprite_pos <= *cycle + 2 {
    if *sprite_pos <= *cycle && *cycle <= *sprite_pos + 2 {
        return true;
    } else {
        false
    }
}
