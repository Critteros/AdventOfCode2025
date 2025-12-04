fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let mut map: Vec<Vec<char>> = INPUT
        .split('\n')
        .map(|line| line.chars().collect())
        .collect();

    let mut count = 0;

    loop {
        pad_map(&mut map);
        let (new_count, out_map) = count_valid_positions(&map);
        map = out_map;
        count += new_count;
        if new_count == 0 {
            break;
        }
    }

    print_map(&map);
    println!("count={count}");
}

fn pad_map(map: &mut Vec<Vec<char>>) {
    let line_length = map[0].len();

    // Add '.' to the left and right side of every line
    for line in map.iter_mut() {
        line.push('.');
        line.insert(0, '.');
    }

    // Add two lines
    map.insert(0, vec!['.'; line_length + 2]);
    map.push(vec!['.'; line_length + 2]);
}

fn print_map(map: &Vec<Vec<char>>) {
    for line in map.iter() {
        for char in line.iter() {
            print!("{char}");
        }
        print!("\n");
    }
}

fn count_neighbouts(map: &Vec<Vec<char>>, symbol: char, x: usize, y: usize) -> usize {
    let tuples = vec![
        // Top
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        // Middle
        (x - 1, y),
        (x + 1, y),
        // Bottom
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ];
    let mut count = 0;

    for (x, y) in tuples.into_iter() {
        if map[y][x] == symbol {
            count += 1;
        }
    }

    return count;
}

fn count_valid_positions(map: &Vec<Vec<char>>) -> (usize, Vec<Vec<char>>) {
    // Fewer than 4
    const THRESHOLD: usize = 4;
    let mut count = 0;

    // The map is padded with 2 elements per row/line
    let y_line_count = map.len() - 2;
    let x_line_count = map[0].len() - 2;

    let mut out_vec: Vec<Vec<char>> = vec![vec!['i'; x_line_count]; y_line_count];
    // Valid indexes are from 1 to x_line_count
    for y in 1..=y_line_count {
        for x in 1..=x_line_count {
            let symbol = map[y][x];
            out_vec[y - 1][x - 1] = symbol;
            if symbol == '@' && count_neighbouts(map, '@', x, y) < THRESHOLD {
                count += 1;
                out_vec[y - 1][x - 1] = 'x';
            }
        }
    }

    return (count, out_vec);
}
