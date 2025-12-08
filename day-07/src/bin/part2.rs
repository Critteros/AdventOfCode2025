use std::char;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let map: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();

    println!("Initial map");
    print_map(&map);
    println!("\n\n");

    let count = advance_map(map.clone(), 1);

    println!("Count => {count}")
}

fn advance_map(mut map: Vec<Vec<char>>, line_to_advance: usize) -> usize {
    if line_to_advance == map.len() {
        // print_map(&map);
        // println!("");
        // print!("+");
        return 1;
    }

    let previous_row = map[line_to_advance - 1].clone();
    let mut current_row_copy = map[line_to_advance].clone();
    for (index, &char) in previous_row.iter().enumerate() {
        if char == '|' || char == 'S' {
            if current_row_copy[index] != '^' {
                current_row_copy[index] = '|';
                map[line_to_advance] = current_row_copy;
                return advance_map(map, line_to_advance + 1);
            }

            let mut count = 0;

            if index > 0 {
                let mut row_clone = current_row_copy.clone();
                row_clone[index - 1] = '|';
                let mut map = map.clone();
                map[line_to_advance] = row_clone;
                count += advance_map(map, line_to_advance + 1);
            };

            if index + 1 < current_row_copy.len() {
                let mut row_clone = current_row_copy.clone();
                row_clone[index + 1] = '|';
                let mut map = map.clone();
                map[line_to_advance] = row_clone;
                count += advance_map(map, line_to_advance + 1);
            }
            return count;
        }
    }

    return 0;
}

fn print_map(map: &[Vec<char>]) {
    for line in map {
        for symbol in line {
            print!("{symbol}");
        }
        println!("");
    }
}
