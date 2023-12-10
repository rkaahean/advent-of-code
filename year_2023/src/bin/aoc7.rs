use std::cmp::max;
use std::collections::HashMap;
use std::{cmp::Ordering, fs};

fn main() {
    const PATH: &str =
        "/Users/rkaahean/projects/learnings/advent_of_code/year_2023/src/data/aoc7.txt";
    let contents = fs::read_to_string(PATH).expect("Failed to read file");
    let contents = contents.lines();

    let mut cards: Vec<(String, i32)> = Vec::new();

    // parse data
    for line in contents {
        let hand = line.split(" ").collect::<Vec<&str>>();
        let (card, bid) = (hand[0].trim(), hand[1].trim().parse::<i32>().unwrap());

        cards.push((card.to_string(), bid));
    }

    cards.sort_by(|a, b| compare_card(a, b));
    // println!("Cards {:?}",cards);

    let mut sm = 0;
    for (i, hand) in cards.iter().enumerate() {
        let (_, bid) = hand;
        sm += (i as i32 + 1) * *bid
    }
    println!("Part 1 {}", sm);
}

fn compare_card(hand_a: &(String, i32), hand_b: &(String, i32)) -> Ordering {
    let mut constant_map = HashMap::new();
    constant_map.insert("A", 14);
    constant_map.insert("K", 13);
    constant_map.insert("Q", 12);
    constant_map.insert("J", 11);
    constant_map.insert("T", 10);
    constant_map.insert("9", 9);
    constant_map.insert("8", 8);
    constant_map.insert("7", 7);
    constant_map.insert("6", 6);
    constant_map.insert("5", 5);
    constant_map.insert("4", 4);
    constant_map.insert("3", 3);
    constant_map.insert("2", 2);

    let (card_a, bid_a) = hand_a;
    let (card_b, bid_b) = hand_b;

    let mut char_count_a: HashMap<char, i32> = HashMap::new();

    for c in card_a.chars() {
        char_count_a
            .entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let mut char_count_b: HashMap<char, i32> = HashMap::new();

    for c in card_b.chars() {
        char_count_b
            .entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    let card_a_max_count = char_count_a.values().fold(0, |acc, x| max(acc, *x));
    let card_b_max_count = char_count_b.values().fold(0, |acc, x| max(acc, *x));

    let mut num_max_count_a = 0;
    for val in char_count_a.values() {
        if *val == card_a_max_count {
            num_max_count_a += 1;
        }
    }

    let mut num_max_count_b = 0;
    for val in char_count_b.values() {
        if *val == card_b_max_count {
            num_max_count_b += 1;
        }
    }

    // println!("{}, {}, {}, {}", card_a, card_b, num_max_count_a, num_max_count_b);
    if card_a_max_count > card_b_max_count {
        // println!("{} greater than {}", card_a, card_b);
        return Ordering::Greater;
    } else if card_a_max_count < card_b_max_count {
        // println!("{} greater than {}", card_b, card_a);
        return Ordering::Less;
    } else {
        if num_max_count_a > num_max_count_b {
            // println!("{} greater than {}", card_a, card_b);
            return Ordering::Greater;
        } else if num_max_count_a < num_max_count_b {
            // println!("{} greater than {}", card_b, card_a);
            return Ordering::Less;
        }
        // check full house and three of a kind
        else if card_a_max_count == 3 {
            if char_count_a.values().filter(|x| **x == 2).count()
                > char_count_b.values().filter(|x| **x == 2).count()
            {
                println!("Full house > 3 of a kind");
                return Ordering::Greater;
            } else if char_count_a.values().filter(|x| **x == 2).count()
                < char_count_b.values().filter(|x| **x == 2).count()
            {
                println!("Full house > 3 of a kind");
                return Ordering::Less;
            }
        }

        let mut a_iter = card_a.chars();
        let mut b_iter = card_b.chars();

        let is_searching = true;
        while is_searching {
            let char_a_1: char = a_iter.next().unwrap();
            let char_b_1 = b_iter.next().unwrap();

            let rank_a = constant_map.get(char_a_1.to_string().as_str()).unwrap();
            let rank_b = constant_map.get(char_b_1.to_string().as_str()).unwrap();

            // println!("{:?}, {:?}", char_a_1, char_b_1);
            if rank_a > rank_b {
                // println!("{} greater", char_a_1);
                return Ordering::Greater;
            } else if rank_a < rank_b {
                // println!("{} greater", char_b_1);
                return Ordering::Less;
            } else {
                continue;
            }
        }
        return Ordering::Greater;
    }
}
