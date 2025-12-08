use std::char;

fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let mut map: Vec<Vec<char>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let mut split_count = 0;

    print_map(&map);

    for i in 1..map.len() {
        println!("");
        split_count += advance_map(&mut map, i);
        print_map(&map);
        println!("");
    }

    println!("Split count => {split_count}")
}

fn advance_map(map: &mut [Vec<char>], line_to_advance: usize) -> usize {
    let mut split_count: usize = 0;
    let previous_row = map[line_to_advance - 1].clone();
    let current_row = &mut map[line_to_advance];
    for (index, &char) in previous_row.iter().enumerate() {
        match char {
            '|' | 'S' => {
                if current_row[index] == '^' {
                    split_count += 1;
                    if index > 0 {
                        current_row[index - 1] = '|';
                    };
                    if index + 1 < current_row.len() {
                        current_row[index + 1] = '|';
                    }
                } else {
                    current_row[index] = '|';
                }
            }
            _ => {}
        }
    }
    return split_count;
}

fn print_map(map: &[Vec<char>]) {
    for line in map {
        for symbol in line {
            print!("{symbol}");
        }
        println!("");
    }
}
