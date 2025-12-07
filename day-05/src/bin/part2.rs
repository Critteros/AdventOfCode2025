fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let input_lines: Vec<&str> = INPUT.lines().collect();
    let seperator_index = input_lines.iter().position(|el| el == &"").unwrap();
    let ranges_lines = &input_lines[0..seperator_index];

    let ranges: Vec<(u64, u64)> = ranges_lines
        .iter()
        .map(|range_str| {
            let mut split = range_str.split('-');
            let start = split.next().unwrap().parse::<u64>().unwrap();
            let end = split.next().unwrap().parse::<u64>().unwrap();
            return (start, end);
        })
        .collect();

    println!("ranges to check -> {ranges_lines:?}");

    let merged_ranges = merge_ranges(&ranges);
    println!("merged ranges -> {merged_ranges:?}");

    let count = count_items_in_ranges(&merged_ranges);
    println!("items in ranges {count}");
}

fn count_items_in_ranges(ranges: &Vec<(u64, u64)>) -> u64 {
    ranges.iter().fold(0, |acc, (range_start, range_end)| {
        acc + range_end - range_start + 1
    })
}

fn merge_ranges(ranges: &Vec<(u64, u64)>) -> Vec<(u64, u64)> {
    let mut sorted_ranges = ranges.clone();
    sorted_ranges.sort_by_key(|el| el.0);

    let mut merged_ranges: Vec<(u64, u64)> = Vec::new();
    let mut current = sorted_ranges[0].to_owned();

    for (next_start, next_end) in &sorted_ranges[1..] {
        let (_, current_end) = current.clone();
        if *next_start <= current_end + 1 {
            current.1 = std::cmp::max(current_end, *next_end)
        } else {
            merged_ranges.push(current);
            current = (*next_start, *next_end);
        }
    }

    merged_ranges.push(current);

    return merged_ranges;
}
