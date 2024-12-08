use std::collections::HashSet;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use array2d::Array2D;

enum Direction {
    North,
    South,
    East,
    West,
}

// (x, y)
impl Direction {
    fn movement(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }

    fn next(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            Direction::North => "North",
            Direction::South => "South",
            Direction::East => "East",
            Direction::West => "West",
        };

        f.write_str(description)
    }
}

fn main() {
    let lines = read_lines("./input/6a.txt").expect("Could not open input file");
    let mut grid: Vec<Vec<char>> = Vec::new();
    let mut position: (i32, i32) = (0, 0); // (x, y)

    for (y, line) in lines.flatten().enumerate() {
        let chars = line.chars();
        let mut vec: Vec<char> = vec![];

        for (x, char) in chars.enumerate() {
            if char == '^' {
                position = (x as i32, y as i32);
            }

            vec.push(char);
        }

        grid.push(vec);
    }

    let arr = Array2D::from_rows(&grid).unwrap();
    let y_max = arr.column_len() as i32;
    let x_max = arr.row_len() as i32;

    let mut visited: HashSet<u32> = HashSet::new();
    visited.insert(szudzik_pair(position.0 as u32, position.1 as u32));

    let mut direction = Direction::North;
    let mut movement = direction.movement();

    while position_within_bounds(position, x_max, y_max) {
        visited.insert(szudzik_pair(position.0 as u32, position.1 as u32));

        let mut candidate_next = (position.0 + movement.0, position.1 + movement.1);
        if !position_within_bounds(candidate_next, x_max, y_max) {
            break;
        }

        let mut char = arr[(candidate_next.1 as usize, candidate_next.0 as usize)];
        while char == '#' {
            direction = direction.next();
            movement = direction.movement();
            candidate_next = (position.0 + movement.0, position.1 + movement.1);
            char = arr[(candidate_next.1 as usize, candidate_next.0 as usize)];
        }

        position = candidate_next;
    }

    let distinct_positions = visited.len();
    println!("number of distinct visited positions = {}", distinct_positions);
}

fn position_within_bounds(position: (i32, i32), x_max: i32, y_max: i32) -> bool {
    position.0 >= 0 && position.0 < x_max && position.1 >= 0 && position.1 < y_max
}

// good as long as the inputs are < ushort-ish
fn szudzik_pair(x: u32, y: u32) -> u32 {
    if x >= y {
        x * x + x + y
    } else {
        y * y + x
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}