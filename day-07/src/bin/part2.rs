fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let mut map: Vec<Vec<String>> = INPUT
        .lines()
        .map(|line| line.chars().map(|symbol| symbol.to_string()).collect())
        .collect();

    let start_index = map[0].iter().position(|el| *el == "S");
    if let Some(index) = start_index {
        map[0][index] = "1".to_string();
    }

    print_map(&map);

    for i in 1..map.len() {
        println!("");
        advance_map(&mut map, i);
        print_map(&map);
        println!("");
    }

    let sum: u64 = map[map.len() - 1]
        .iter()
        .map(|number| number.parse::<u64>().unwrap_or_default())
        .sum();
    println!("sum -> {sum}");
}

fn advance_map(map: &mut [Vec<String>], line_to_advance: usize) {
    let previous_row = map[line_to_advance - 1].clone();
    let current_row = &mut map[line_to_advance];
    for (index, symbol) in previous_row.iter().enumerate() {
        if let Ok(number) = symbol.parse::<u64>() {
            if current_row[index] == "^" {
                if index > 0 {
                    let prev_value = current_row[index - 1].parse::<u64>().unwrap_or_default();
                    current_row[index - 1] = (prev_value + number).to_string();
                }
                if index + 1 < current_row.len() {
                    let prev_value = current_row[index + 1].parse::<u64>().unwrap_or_default();
                    current_row[index + 1] = (prev_value + number).to_string()
                }
            } else {
                let prev_value = current_row[index].parse::<u64>().unwrap_or_default();
                current_row[index] = (prev_value + number).to_string();
            }
        }
    }
}

fn print_map(map: &[Vec<String>]) {
    for line in map {
        for symbol in line {
            print!("{symbol:>2}");
        }
        println!("");
    }
}
