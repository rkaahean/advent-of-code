use std::{fs, collections::HashSet};

fn main() {
    // trying to read the input data
    println!("Reading input file...");

    // specifying the library
    let file_path = "/Users/ranjanwork-personal/rustaoc/src/data/aoc6.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something wrong in reading the file...");

    // get the moves line by line
    let mut count = 0;
    let mut char_vec: Vec<String> = Vec::new();

    for s in contents.chars() {
        // get the first 3 characters 
        let s_string = s.to_string();
        if count < 13 {
            char_vec.push(s_string.clone());
            count +=1;
            continue;
        }

        if check_vector_for_dupes(char_vec.clone(), &s.to_string()){
            println!("{:?}", count);
            println!("{:?}", s);
            break;
        } else {
            char_vec.push(s_string.clone());
            char_vec.remove(0);
            count+=1;
        }
    }
    println!("{:?}", char_vec);
}

fn check_vector_for_dupes(mut char_vec: Vec<String>, s: &String) -> bool {
    char_vec.push(s.to_string());
    let clones = char_vec.clone();
    let hs: HashSet<String> = HashSet::from_iter(char_vec);
    if clones.len() == hs.len() {
        return true;
    }   
    return false;
}