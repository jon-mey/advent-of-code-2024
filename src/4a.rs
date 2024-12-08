use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input/4a.txt").expect("Could not open input file");
    let vec: Vec<Vec<char>> = lines
        .flatten()
        .map(|l| l.chars().collect())
        .collect();
    
    let mut count = 0;
    for (x, row) in vec.iter().enumerate() {
        for (y, &char) in row.iter().enumerate() {
            if char == 'X' {
                count += find_mas(x as i32, y as i32, &vec);
            }
        }
    }

    println!("XMAS count = {}", count);
}

// find "MAS" starting at x, y
fn find_mas(x: i32, y: i32, source: &Vec<Vec<char>>) -> i32 {
    let mut count = 0;

    // horizontal forwards
    if find_letter('M', x + 1, y, source)
        && find_letter('A', x + 2, y, source)
        && find_letter('S', x + 3, y, source) {
        count += 1;
    }
    
    // horizontal backwards
    if find_letter('M', x - 1, y, source)
        && find_letter('A', x - 2, y, source)
        && find_letter('S', x - 3, y, source) {
        count += 1;
    }

    // vertical downwards
    if find_letter('M', x, y + 1, source)
        && find_letter('A', x, y + 2, source)
        && find_letter('S', x, y + 3, source) {
        count += 1;
    }

    // vertical upwards
    if find_letter('M', x, y - 1, source)
        && find_letter('A', x, y - 2, source)
        && find_letter('S', x, y - 3, source) {
        count += 1;
    }

    // diagonal down right
    if find_letter('M', x + 1, y + 1, source)
        && find_letter('A', x + 2, y + 2, source)
        && find_letter('S', x + 3, y + 3, source) {
        count += 1;
    }

    // diagonal down left
    if find_letter('M', x - 1, y + 1, source)
        && find_letter('A', x - 2, y + 2, source)
        && find_letter('S', x - 3, y + 3, source) {
        count += 1;
    }

    // diagonal up right
    if find_letter('M', x + 1, y - 1, source)
        && find_letter('A', x + 2, y - 2, source)
        && find_letter('S', x + 3, y - 3, source) {
        count += 1;
    }

    // diagonal up left
    if find_letter('M', x - 1, y - 1, source)
        && find_letter('A', x - 2, y - 2, source)
        && find_letter('S', x - 3, y - 3, source) {
        count += 1;
    }

    return count;
}

fn find_letter(letter: char, x: i32, y: i32, source: &Vec<Vec<char>>) -> bool {
    if x < 0 || y < 0 || x as usize >= source.len() {
        return false;
    }

    let row = &source[x as usize];
    if y as usize >= row.len() {
        return false;
    }

    return row[y as usize] == letter;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}