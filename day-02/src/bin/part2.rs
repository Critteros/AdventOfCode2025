use std::cmp;

fn is_invalid(input_str: &str) -> bool {
    let max_pattern_len = input_str.len() / 2;

    for pattern_len in 1..=max_pattern_len {
        let (pattern, mut current) = input_str.split_at(pattern_len);
        let mut is_pattern_reoccuring = !current.is_empty();

        while !current.is_empty() {
            let (chunk, rest) = current.split_at(cmp::min(pattern_len, current.len()));
            if chunk != pattern {
                is_pattern_reoccuring = false;
                break;
            }
            current = rest;
        }

        if is_pattern_reoccuring {
            return true;
        }
    }

    return false;
}

fn main() {
    const INPUT_DATA: &str = include_str!("../input.txt");
    let numbers: Vec<u64> = INPUT_DATA
        .split(',')
        .filter_map(|range_string| {
            let mut parts = range_string.split('-');
            let start: u64 = parts.next()?.parse().ok()?;
            let end: u64 = parts.next()?.parse().ok()?;
            let range: Vec<u64> = (start..=end).collect();
            Some(range)
        })
        .flatten()
        .collect();

    let invalid_numbers: Vec<u64> = numbers
        .iter()
        .filter(|number| is_invalid(&number.to_string()))
        .cloned()
        .collect();

    // println!("invalid numbers: {:?}", invalid_numbers);

    let sum: u64 = invalid_numbers.iter().sum();
    println!("sum={}", sum);
}
