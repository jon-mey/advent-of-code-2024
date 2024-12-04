use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input/1b.txt").expect("Could not open input file");
    let mut left_numbers: Vec<u32> = vec![];
    let mut right_numbers: Vec<u32> = vec![];

    for line in lines.flatten() {
        let mut split  = line.split_whitespace();

        let num1 = split
            .next()
            .expect("Missing left number")
            .parse::<u32>()
            .expect("Invalid left number");
        left_numbers.push(num1);

        let num2 = split
            .next()
            .expect("Missing right number")
            .parse::<u32>()
            .expect("Invalid right number");
        right_numbers.push(num2);
    }

    let mut occurences: HashMap<u32, u32> = HashMap::new();
    for i in 0..right_numbers.len() {
        let num = right_numbers[i];
        occurences.entry(num)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    let mut sum: u32 = 0;
    for i in 0..left_numbers.len() {
        let num = left_numbers[i];

        if let Some(occurence_count) = occurences.get(&num) {
            sum += num * occurence_count;
        }
    }

    println!("similarity score = {}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}