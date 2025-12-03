use std::cmp::max;

fn main() {
    const DATA_STR: &str = include_str!("../input_data.txt");
    let rows: Vec<Vec<u8>> = DATA_STR
        .split('\n')
        .map(|line| {
            return Vec::from_iter(line.chars().map(|char| char as u8 - b'0'));
        })
        .collect();

    let sum: u64 = rows
        .iter()
        .map(|row| {
            let value = max_value_of_digits(row, 12);
            println!("row sum = {:?}", value);
            return value;
        })
        .sum();

    println!("sum={}", sum);
}

fn max_value_of_digits(values: &[u8], num_of_digits: usize) -> u64 {
    let mut picks: Vec<u8> = Vec::with_capacity(num_of_digits);
    let mut working_index = 0;

    for battery_index in 0..num_of_digits {
        let last_valid_index = values.len() - 1;

        // Battery index starts at 0 for the 1 battery so -1 to get remaining battery count
        let remaining_battery_count = num_of_digits - battery_index - 1;
        let maximum_searchable_index =
            max(working_index, last_valid_index - remaining_battery_count);

        let slice_to_search = &values[working_index..=maximum_searchable_index];

        let mut max_value = 0;
        let mut max_local_index = 0;

        for (index, value) in slice_to_search.iter().enumerate() {
            if *value > max_value {
                max_value = *value;
                max_local_index = index;
            }
        }

        picks.push(max_value);
        // +1 as we do not want to re-use the same battery
        working_index = max_local_index + working_index + 1;
    }

    picks.reverse();

    return picks
        .into_iter()
        .enumerate()
        .fold(0 as u64, |acc, (index, value)| {
            return acc + (value as u64) * (10 as u64).pow(index as u32);
        });
}
