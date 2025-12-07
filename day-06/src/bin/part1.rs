fn main() {
    const INPUT: &str = include_str!("../input.txt");
    let lines: Vec<&str> = INPUT.lines().collect();
    let parsed: Vec<Vec<&str>> = lines.iter().map(|line| line.split(" ").collect()).collect();
    let numbers: Vec<Vec<u64>> = parsed[..parsed.len() - 1]
        .iter()
        .map(parse_numbers_line)
        .collect();
    let operands = parse_operands_line(&parsed[parsed.len() - 1]);
    println!("{numbers:?}");
    println!("{operands:?}");
    let outputs = apply_operands(&numbers, &operands);
    println!("{outputs:?}");
    let sum: u64 = outputs.iter().sum();
    println!("sum = {sum:?}");
}

fn parse_numbers_line(line: &Vec<&str>) -> Vec<u64> {
    line.iter()
        .filter(|el| *el != &"")
        .map(|str_number| str_number.parse::<u64>().unwrap())
        .collect()
}

fn parse_operands_line(line: &Vec<&str>) -> Vec<char> {
    line.iter()
        .filter(|el| *el != &"")
        .map(|el| el.chars().next().unwrap())
        .collect()
}

fn apply_operands(numbers: &Vec<Vec<u64>>, operands: &Vec<char>) -> Vec<u64> {
    let input_length = operands.len();
    let mut outputs = vec![0 as u64; input_length];

    for index in 0..input_length {
        let inputs: Vec<u64> = numbers.iter().map(|line| line[index]).collect();
        let current_operand = operands[index];
        let result = inputs
            .into_iter()
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
