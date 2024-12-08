use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input/5a.txt").expect("Could not open input file");

    // for each rule, store a list of numbers that can't have appeared before it
    // while reading an update, store numbers so far in a HashSet
    // for each next number in the update, check the rule list if any of those appear in the hashset

    let mut read_empty_line = false;
    let mut cant_appear_before: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut sum = 0;

    for line in lines.flatten() {
        if line.trim().len() == 0 {
            read_empty_line = true;
            continue;
        }

        if !read_empty_line {
            // read rules
            let mut split  = line.split('|');

            let num1 = split
                .next()
                .expect("Missing left number")
                .parse::<i32>()
                .expect("Invalid left number");
    
            let num2 = split
                .next()
                .expect("Missing right number")
                .parse::<i32>()
                .expect("Invalid right number");

            cant_appear_before
                .entry(num1)
                .or_insert_with(Vec::new)
                .push(num2);
        } else {
            // read updates
            let split: Vec<&str>  = line.split(',').collect();
            let middle_point_index = split.len() / 2; // assume len is always odd
            let mut appeared_so_far: HashSet<i32> = HashSet::new();
            let mut valid = true;
            let mut middle_number = 0;

            for (i, num_str) in split.into_iter().enumerate() {
                let num = num_str
                    .parse::<i32>()
                    .expect("Invalid number in split");

                if let Some(list) = cant_appear_before.get(&num) {
                    for item in list {
                        if let Some(_) = appeared_so_far.get(&item) {
                            valid = false;
                            break;
                        }
                    }
                }

                if !valid {
                    break;
                }

                appeared_so_far.insert(num);
                if i == middle_point_index {
                    middle_number = num;
                }
            }

            if valid {
                sum += middle_number;
            }
        }
    }

    println!("sum of middle numbers in valid updates = {}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}