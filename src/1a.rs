use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input/1a.txt").expect("Could not open input file");
    let mut left_numbers: Vec<i32> = vec![];
    let mut right_numbers: Vec<i32> = vec![];

    for line in lines.flatten() {
        let mut split  = line.split_whitespace();

        let num1 = split
            .next()
            .expect("Missing left number")
            .parse::<i32>()
            .expect("Invalid left number");
        left_numbers.push(num1);

        let num2 = split
            .next()
            .expect("Missing right number")
            .parse::<i32>()
            .expect("Invalid right number");
        right_numbers.push(num2);
    }

    left_numbers.sort_unstable();
    right_numbers.sort_unstable();
    assert_eq!(left_numbers.len(), right_numbers.len(), "Mismatched lengths");

    let mut sum = 0;
    for i in 0..left_numbers.len() {
        let num1 = left_numbers[i];
        let num2 = right_numbers[i];

        sum += (num1 - num2).abs();
    }

    println!("total distance = {}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}