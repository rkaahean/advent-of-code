use std::{collections::HashSet, fs };

fn main() {

    // trying to read the input data
    println!("Reading input file...");

    // specifying the library
    let file_path = "/Users/ranjanwork-personal/rustaoc/src/data/aoc3.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something wrong in reading the file...");

    // read lines
    let rucksacks = contents.lines();

    // sum up priorities
    let mut total_priorities = 0;
    let mut elfs: Vec<&str> = Vec::new();
    let mut badge_priorities = 0;

    for rucksack in rucksacks {

        // (PART 1) get prioirties for common characters
        total_priorities+=get_common_character_priorities(rucksack);

        // (PART 2)
        elfs.push(rucksack);
        if elfs.len() == 3 {
            for s1 in elfs[0].chars() {
                if elfs[1].contains(s1) && elfs[2].contains(s1) {
                    badge_priorities += get_priorities(s1);
                    break;
                }
            }
            elfs.clear();
        }
    }

    println!("{total_priorities}");
    println!("{badge_priorities}");


}

fn get_common_character_priorities (rucksack: &str) -> u32 {
    // convert to String type
    let rucksack = rucksack.to_string();

    // split into 2 halves
    let (first_half, second_half) =  rucksack.split_at(rucksack.len()/2);

    // unique set of characters
    let mut set_left: HashSet<char> = HashSet::new();
    let mut set_right: HashSet<char> = HashSet::new();

    
    // insert characters
    for chr in first_half.chars() {
        set_left.insert(chr);
    }
    
    // insert characters
    for chr in second_half.chars() {
        set_right.insert(chr);
    } 
    
    // characters in both left and right half
    let common_characters = set_left.intersection(&set_right);

    // get priorities for all the common characters
    let priorities: u32 = common_characters.map(|x| get_priorities(*x)).sum();
    priorities
}

fn get_priorities(item: char) -> u32{

    if (item as u32) < 97 {
        return (item as u32) - 38;
    } else {
        return (item as u32) - 96;
    }

}