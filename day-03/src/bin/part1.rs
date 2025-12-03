fn main() {
    const DATA_STR: &str = include_str!("../input_data.txt");
    let rows: Vec<Vec<u8>> = DATA_STR
        .split('\n')
        .map(|line| {
            return Vec::from_iter(line.chars().map(|char| char as u8 - b'0'));
        })
        .collect();

    let sum: u32 = rows
        .iter()
        .map(|row| {
            let value = max_two_ordered_values(row);
            return value;
        })
        .sum();

    println!("sum={}", sum);
}

fn score(a: u32, b: u32) -> u32 {
    return a * 10 + b;
}

fn max_two_ordered_values(values: &[u8]) -> u32 {
    let mut max_value = 0;
    for i in 0..(values.len() - 1) {
        for j in (i + 1)..values.len() {
            let a = values[i];
            let b = values[j];

            let values_score = score(a as u32, b as u32);
            if values_score > max_value {
                max_value = values_score as u32;
            }
        }
    }
    return max_value;
}
