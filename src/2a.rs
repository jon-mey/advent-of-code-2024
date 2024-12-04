use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input/2.txt").expect("Could not open input file");
    let mut count = 0;

    for line in lines.flatten() {
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse::<i32>().expect(&format!("Error parsing number ({})", s)))
            .collect();

        if is_valid_sequence(&numbers) {
            count += 1;
        }
    }
    
    println!("safe count = {}", count);
}

fn is_valid_sequence(numbers: &[i32]) -> bool {
    let len = numbers.len();
    if len <= 1 {
        return false; 
    }

    let mut sign = 0;

    for i in 1..len {
        let diff = numbers[i] - numbers[i - 1];
        if diff == 0 || diff.abs() > 3 {
            return false;
        }

        if i == 1 {
            sign = diff.signum();
        } else if diff.signum() != sign {
            return false;
        }
    }

    return true
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}