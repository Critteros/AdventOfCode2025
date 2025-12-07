fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let lines: Vec<&str> = INPUT.lines().collect();
    let parsed: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();
    let borrowed: Vec<&[char]> = parsed[..parsed.len() - 1]
        .iter()
        .map(|row| row.as_slice())
        .collect();
    let numbers: Vec<Vec<u64>> = parse_numbers(&borrowed);

    let operands = parse_operands_line(&parsed[parsed.len() - 1]);
    println!("{numbers:?}");
    println!("{operands:?}");
    let outputs = apply_operands(&numbers, &operands);
    println!("{outputs:?}");
    let sum: u64 = outputs.iter().sum();
    println!("sum = {sum:?}");
}

fn parse_numbers(lines: &[&[char]]) -> Vec<Vec<u64>> {
    let mut output_lines: Vec<Vec<u64>> = Vec::new();
    let line_length = lines[0].len();
    let mut current_line: Vec<u64> = Vec::new();

    for index in (0..line_length).rev() {
        let selection: String = lines.iter().map(|entry| entry[index]).collect();
        let number = selection.trim().parse::<u64>();
        if let Ok(valid_number) = number {
            current_line.push(valid_number);
        } else {
            output_lines.push(current_line);
            current_line = Vec::new();
        }
    }
    output_lines.push(current_line);

    return output_lines;
}

fn parse_operands_line(line: &Vec<char>) -> Vec<char> {
    line.clone().into_iter().filter(|el| *el != ' ').collect()
}

fn apply_operands(numbers: &Vec<Vec<u64>>, operands: &Vec<char>) -> Vec<u64> {
    let mut outputs = vec![0 as u64; operands.len()];

    for index in 0..operands.len() {
        let inputs = &numbers[operands.len() - 1 - index];
        let current_operand = operands[index];
        let result = inputs
            .iter()
            .cloned()
            .reduce(|acc, number| {
                if current_operand == '*' {
                    return acc * number;
                } else {
                    return acc + number;
                }
            })
            .unwrap();

        outputs[index] = result;
    }

    return outputs;
}
