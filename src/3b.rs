use std::fs;

fn main() {
    let input = fs::read_to_string("./input/3b.txt").expect("Could not open input file");
    let chars: Vec<char> = input.chars().collect();
    let len = chars.len();
    
    let mul_pattern = ['m', 'u', 'l'];
    let do_pattern = ['d', 'o', '(', ')'];
    let dont_pattern = ['d', 'o', 'n', '\'', 't', '(', ')'];

    let mut mul_index = 0;
    let mut do_dont_index = 0;

    let mut i = 0;
    let mut sum: u32 = 0;
    let mut enabled = true;

    while i < len {
        let char = chars[i];

        if enabled {
            // don't() case
            if char == dont_pattern[do_dont_index] {
                do_dont_index += 1;

                if do_dont_index == dont_pattern.len() {
                    // 'don't()' has been read
                    enabled = false;
                    do_dont_index = 0;
                    i += 1;
                    continue;
                }
            } else {
                do_dont_index = 0;
            }

            // mul(x, y) case
            if char == mul_pattern[mul_index] {
                mul_index += 1;

                if mul_index == mul_pattern.len() {
                    // 'mul' has been read, look for '(x, y)'
                    let (new_index, mult_result) = read_factors(i + 1, &chars);
                    i = new_index;

                    if let Some(value) = mult_result {
                        sum += value;
                    }

                    mul_index = 0;
                    continue; // already set index with i = new_index
                }
            }
        } else {
            // do() case
            if char == do_pattern[do_dont_index] {
                do_dont_index += 1;

                if do_dont_index == do_pattern.len() {
                    // 'do()' has been read
                    enabled = true;
                    do_dont_index = 0;
                    i += 1;
                    continue;
                }
            } else {
                do_dont_index = 0;
            }
        }

        i += 1;
    }

    println!("sum of all multiplications = {}", sum);
}

// reads factors on the format "(x, y)" where x and y are arbitrary length radix 10 numbers
fn read_factors(start: usize, chars: &[char]) -> (usize, Option<u32>) {
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
fn read_number(start: usize, chars: &[char]) -> (usize, Option<u32>) {
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