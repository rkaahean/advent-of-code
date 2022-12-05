use std::{fs, str::{self, Lines}, collections::{HashMap}};

fn main() {

    // trying to read the input data
    println!("Reading input file...");

    // specifying the library
    let file_path = "/Users/ranjanwork-personal/rustaoc/src/data/aoc5.txt";
    let contents = fs::read_to_string(file_path)
        .expect("Something wrong in reading the file...");

    // get the pairs line by line
    let stacks = contents.lines();

    // get stack data
    let mut elf_stacks = construct_stack_data(stacks);

    // reverse all stacks!
    elf_stacks = reverse_all(elf_stacks);
    // println!("{:?}", elf_stacks["3"]);

    let mut line_count = 0;
    for move_text in contents.lines() {
        line_count+=1;
        if line_count > 10 {
            println!("{move_text}");
            elf_stacks = parse_move(move_text, elf_stacks);
            
        }
        // println!("{:?}", elf_stacks["3"]);
    }
    println!("{:?}", elf_stacks);
    for index in 0..10 {
        let vec = elf_stacks.get(&index.to_string()).unwrap().to_vec();
        println!("{:?}", vec[vec.len() -1]);
    }
}

fn parse_move<'a>(move_text: &'a str, mut elf_stacks: HashMap<String, Vec<&'a str>>) -> HashMap<String, Vec<&'a str>>{
    let moves = move_text.split_whitespace().collect::<Vec<&str>>();

    // get move info numbers
    let move_to: u32 = moves[5].parse().expect("not a number!");
    let move_from: u32 = moves[3].parse().expect("not a number!");
    let move_amount: u32 = moves[1].parse().expect("not a number!");

    // get vector to move from
    let mut stack_from = elf_stacks.get(&(move_from - 1).to_string()).unwrap().to_vec();
    // get vector to move to
    let mut stack_to = elf_stacks.get(&(move_to - 1).to_string()).unwrap().to_vec();

    // PART 1
    // for _ in 0..move_amount {
    //     // pop value FROM
    //     let stack_val = stack_from.pop().unwrap();
    //     // push vale TO
    //     stack_to.push(stack_val);
    // }

    // PART 2
    let mut vec: Vec<&str> = Vec::new();
    for _ in 0..move_amount {
        // pop value FROM
        let stack_val = stack_from.pop().unwrap();
        // push vale TO
        // stack_to.push(stack_val);
        vec.push(stack_val);
    }
    vec.reverse();
    stack_to.append(&mut vec);
    // update
    elf_stacks.insert((move_from - 1).to_string(), stack_from);
    elf_stacks.insert((move_to - 1).to_string(), stack_to);

    // println!("{:?}", elf_stacks);
    elf_stacks

}

fn reverse_all(mut elf_stacks: HashMap<String, Vec<&str>>) -> HashMap<String, Vec<&str>>{
    for index in 0..9 {
        let mut stack_vec = elf_stacks.get(&index.to_string()).unwrap().to_vec();
        stack_vec.reverse();
        elf_stacks.insert(index.to_string(), stack_vec);
    }
    elf_stacks
}

fn construct_stack_data(stacks: Lines) -> HashMap<String, Vec<&str>> {
    // define a Hashmap of vectors
    // each vector represent a stack (1-9)
    let mut elf_stacks: HashMap<String, Vec<&str>> = HashMap::new(); 
    let mut line_count = 0;
    for stack in stacks {
        line_count +=1;
        // break if empty
        if line_count == 9 {
            break;
        }

        // split by space
        let s = stack.split(" ").collect::<Vec<&str>>();

        // construct a row level elf stack
        let mut to_read: usize = 0;
        let mut stack_vec: Vec<&str> = Vec::new();
        for (index, i) in s.iter().enumerate() {
            if i.is_empty() && index == to_read {
                to_read += 4;
                stack_vec.push(i);
            } else if index == to_read {
                to_read +=1;
                stack_vec.push(i);
            }
        }

        for (index, &crt) in stack_vec.iter().enumerate() {

            // remove whitespaces
            let crate_value = crt.trim();

            // ignore empty string values
            if crate_value.is_empty() {
                continue;
            }
            
            let mut es: Vec<&str> = Vec::new();
            // if vector already initialized
            if !elf_stacks.get(&index.to_string()).is_none() {
                 es = elf_stacks.get(&index.to_string()).unwrap().to_vec();
            }
            // add element to vector
            es.push(crate_value);
            // set vector to hasmap
            elf_stacks.insert(index.to_string(), es);
        }
    }
    elf_stacks
}