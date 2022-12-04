use std::fs;

fn main() {

    // trying to read the input data
    println!("Reading input file...");

    // specifying the library
    let file_path = "/Users/ranjanwork-personal/rustaoc/src/data/aoc4.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something wrong in reading the file...");

    // get the pairs line by line
    let elf_pairs = contents.lines();

    // num overlaps
    let mut part1 = 0;
    let mut part2 = 0;

    for pair in elf_pairs {
        let elf_pair: Vec<&str> = pair.split(",").collect();
        // for every pair, get the numbers by splitting
        // flat_map will flatten all the numbers into a list
        // instead of having a list inside a list.
        // i.e [1, 2, 3, 4] instead of [[1, 2], [3, 4]]
        let values: Vec<&str>= elf_pair.iter().flat_map(|x| x.split("-").collect::<Vec<&str>>()).collect();
    
        // check if overlapping pairs
        let pair1: (u32, u32) = (values[0].parse().expect(""), values[1].parse().expect(""));
        let pair2: (u32, u32) = (values[2].parse().expect(""), values[3].parse().expect(""));

        part1 += is_overlapping(pair1, pair2) as u32;
        part2 += is_overlapping_part_two(pair1, pair2) as u32;
    }

    println!("{part1}");
    println!("{part2}");

}

fn is_overlapping(pair1: (u32, u32), pair2: (u32, u32)) -> bool{
    if pair1.0 >= pair2.0 && pair1.1 <= pair2.1 {
        // println!("Pair 1, Pair 2: {:?}, {:?}", pair1, pair2);
        return true;
    }
    else if pair2.0 >= pair1.0 && pair2.1 <= pair1.1 {
        // println!("*Pair 1, Pair 2: {:?}, {:?}", pair1, pair2);
        return true;
    }
    else {
        return false;
    }
}

fn is_overlapping_part_two(pair1: (u32, u32), pair2: (u32, u32)) -> bool{
    if pair2.0 > pair1.1 || pair1.0 > pair2.1 {
        return false;
    }
    else {
        println!("Pair 1, Pair 2: {:?}, {:?}", pair1, pair2);
        return true;
    }
}