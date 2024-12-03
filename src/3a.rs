use std::fs;

fn main() {
    let input = fs::read_to_string("./input/3a.txt").expect("Could not open input file");
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();

    let mut i = 0;
    let mut ti = 0;
    let mut sum: u32 = 0;

    while i < len {
        let char = chars[i];

        if ti == 0 && char == 'm' || ti == 1 && char == 'u' || ti == 2 && char == 'l' {
            ti += 1;
            i += 1;
        } else if ti == 3 {
            let (new_index, mult_result) = read_factors(i, &chars);
            i = new_index;

            if mult_result.is_some() {
                sum += mult_result.unwrap();
            }

            ti = 0;
        } else {
            ti = 0;
            i += 1;
        }
    }

    println!("sum of all multiplications = {}", sum);
}

// reads factors on the format "(x, y)" where x and y are arbitrary length radix 10 numbers
fn read_factors(start: usize, chars: &Vec<char>) -> (usize, Option<u32>) {
    let mut index = start;
    if chars[index] != '(' {
        return (index + 1, None);
    }

    // read first number
    let (new_index, first) = read_number(index + 1, chars);
    index = new_index;

    if first.is_none() {
        return (index, None);
    }

    // next char must be ','
    if index >= chars.len() || chars[index] != ',' {
        return (index + 1, None)
    }

    // read second number
    let (new_index, second) = read_number(index + 1, chars);
    index = new_index;

    if second.is_none() {
        return (index, None);
    }

    // next char must be ')'
    if index >= chars.len() || chars[index] != ')' {
        return (index, None)
    }

    return (index + 1, Some(first.unwrap() * second.unwrap()));
}

// returns new index (after the number) and optionally the number
fn read_number(start: usize, chars: &Vec<char>) -> (usize, Option<u32>) {
    let len = chars.len();
    let mut index = start;
    let mut result: Vec<u32> = vec![];

    while index < len {
        let char = chars[index];

        if let Some(n) = char.to_digit(10) {
            result.push(n);
        } else {
            break;
        }

        index += 1;
    }

    if result.len() > 0 {
        // convert Vec<u32> to single number
        let n = result
            .iter()
            .fold(0, |acc, &digit| acc * 10 + digit);

        return (index, Some(n));
    }

    return (index, None);
}