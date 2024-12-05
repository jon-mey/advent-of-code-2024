
use std::fs::File;
use std::i32;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let lines = read_lines("./input/4b.txt").expect("Could not open input file");
    let vec: Vec<Vec<char>> = lines
        .flatten()
        .map(|l| l.chars().collect())
        .collect();
    
    let mut count = 0;
    for (x, row) in vec.iter().enumerate() {
        for (y, &char) in row.iter().enumerate() {
            if char == 'A' && find_mas(x as i32, y as i32, &vec) {
                count += 1;
            }
        }
    }

    println!("X-MAS count = {}", count);
}

// find pattern starting at "A" at (x, y)
fn find_mas(x: i32, y: i32, source: &Vec<Vec<char>>) -> bool {
    // left to right horizontal
    let left = find_letter('M', x - 1, y - 1, source) && find_letter('S', x + 1, y + 1, source)
        || find_letter('S', x - 1, y - 1, source) && find_letter('M', x + 1, y + 1, source);

    // right to left horizontal
    let right = find_letter('M', x + 1, y - 1, source) && find_letter('S', x - 1, y + 1, source)
        || find_letter('S', x + 1, y - 1, source) && find_letter('M', x - 1, y + 1, source);
        
    return left && right;
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