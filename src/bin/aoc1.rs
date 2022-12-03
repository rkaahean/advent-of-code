use std::fs;

fn main() {
    // trying to read in the data
    println!("Reading input file...");

    // specifying the library
    let file_path = "/Users/ranjanwork-personal/rustaoc/src/data/aoc1.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something wrong in reading the file...");

    // get all the elf calories into a data-type
    let split_contents = contents.lines();

    // let's keep track of the maximum, and the sum of the elf, calories per elf
    let mut maximum = 0;
    let mut calorie_sum = 0;
    let mut all_calories: Vec<u32> = Vec::new();


    for calorie in split_contents {

        // in case new-line detected, end of elf
        if calorie == "" {
            
            // if maximum greater than current sum
            if calorie_sum > maximum {
                maximum = calorie_sum
            }
            // store and reset
            all_calories.push(calorie_sum);
            calorie_sum = 0;
        }

        // if not, get the calorie in question
        if calorie != "" {
            println!("{}", calorie);
            let calorie: u32 = calorie.trim().parse().expect("Not a number!");
            calorie_sum += calorie;
        }
    }

    all_calories.sort();
    let sum_calories: u32 =  all_calories.iter().rev().take(3).sum();
    println!("The maximum is...{maximum}");
    println!("Sum of the 3 largest values, {sum_calories}");

}