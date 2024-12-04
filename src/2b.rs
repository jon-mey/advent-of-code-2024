use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input/2b.txt").expect("Could not open input file");
    let mut count = 0;

    for line in lines.flatten() {
        let numbers: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse::<i32>().expect(&format!("Error parsing number ({})", s)))
            .collect();

        // test every len - 1 sequence by ignoring 1 index
        for i in 0..numbers.len() {
            if is_valid_sequence(&numbers, i) {
                count += 1;
                break;
            }
        }
    }
    
    println!("safe count = {}", count);
}

fn is_valid_sequence(numbers: &[i32], ignore_index: usize) -> bool {
    let len = numbers.len();
    if len <= 1 {
        return false; 
    }

    let mut sign = 0;
    let mut first = true;

    for i in 1..len {
        if i == ignore_index {
            continue;
        }

        let mut previous = i - 1;
        if previous == ignore_index {
            if previous == 0 {
                continue;
            } else {
                previous -= 1;
            }
        }

        let diff = numbers[i] - numbers[previous];
        if diff == 0 || diff.abs() > 3 {
            return false;
        }

        if first {
            sign = diff.signum();
            first = false;
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