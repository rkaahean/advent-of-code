use std::{collections::HashMap, fs};

fn main() {
    // trying to read the input data
    println!("Reading input file...");

    // specifying the library
    let file_path = "/Users/ranjanwork-personal/rustaoc/src/data/aoc7.txt";
    let contents = fs::read_to_string(file_path).expect("Something wrong in reading the file...");

    // get the moves line by line
    // let current_directory: String = "".to_string();
    let mut directories: Vec<String> = Vec::new();

    //create and initialize hashmap
    let mut directory_size_map: HashMap<String, i64> = HashMap::new();
    directory_size_map.insert("/".to_string(), 0);

    for line in contents.lines() {
        parse_line(line.to_string(), &mut directories, &mut directory_size_map);
    }

    let mut sum = 0;
    // PART 1
    // for (_, sz) in directory_size_map {
    //     if sz <= 100000 {
    //         sum += sz
    //     }
    // }
    let avail_space = 70000000 - directory_size_map.get("/").copied().unwrap();

    let mut min_size = 70000000;
    // PART 2
    for (_, sz) in directory_size_map {
        if sz < min_size && avail_space + sz > 30000000 {
            min_size = sz
        }
    }
    println!("{min_size}")
}

fn parse_line<'a>(
    line: String,
    directory: &'a mut Vec<String>,
    directory_size_map: &'a mut HashMap<String, i64>,
) {
    if is_command(&line) {
        // println!("{:?}", line);

        // parse out the command type
        let command_type: Vec<&str> = line.split(" ").collect();

        if command_type[1] == "cd" {
            // this is the directory of the command'
            // it could mean going back to the parent
            if command_type[2] == ".." {
                directory.pop();
                return;
            }
            directory.push(command_type[2].to_string());
        }

        if command_type[1] == "ls" {
            // then we are going to be reading the next few lines
            //  until we see the command sign again
        }
    } else {
        // this means that the command is "ls"
        // so we must be reading in the data
        let list_output: Vec<&str> = line.split(" ").collect();

        if list_output[0] == "dir" {
            // it is a directory initialize its sizing
            let mut new_dir_name: String = "/".to_string() + &directory[1..].join(".");
            if new_dir_name == "/" {
                new_dir_name = new_dir_name + &list_output[1].to_string()
            } else {
                new_dir_name = new_dir_name + &".".to_string() + &list_output[1].to_string()
            }

            directory_size_map.insert(new_dir_name, 0);
        } else {
            let dir_name = "/".to_string() + &directory[1..].join(".");
            println!("CURR DIR{:?}", dir_name);
            let dir_size = directory_size_map.get(&dir_name).copied().unwrap();
            // println!("OUTPUT, {:?}", list_output);
            directory_size_map.insert(
                dir_name,
                dir_size + list_output[0].parse::<i64>().expect(""),
            );

            // insert for parent directories too
            for i in 1..directory.len() {
                let slice = &directory[..i];
                let dir_name = "/".to_string() + &slice[1..].join(".");

                let dir_size = directory_size_map.get(&dir_name).copied().unwrap();
                // println!("OUTPUT, {:?}", list_output);
                directory_size_map.insert(
                    dir_name,
                    dir_size + list_output[0].parse::<i64>().expect(""),
                );
            }
        }
    }

    // otherwise, just return the old directory
    // println!("{:?}", directory);
    // println!("{:?}", directory_size_map);
}

fn is_command(line: &String) -> bool {
    // get first character to see if its a command
    let command = line.chars().nth(0).unwrap();
    return command == '$';
}
