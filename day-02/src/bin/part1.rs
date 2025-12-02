fn is_invalid(input_str: &str) -> bool {
    // If the string if odd characters length then it is not build from two repeating patterns
    if input_str.len() % 2 == 1 {
        return false;
    }

    let midpoint = input_str.len() / 2;
    let (sub1, sub2) = input_str.split_at(midpoint);
    return sub1 == sub2;
}

fn main() {
    const INPUT_DATA: &str = include_str!("../example_input.txt");
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
