use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader},
};

#[derive(Debug, Clone, Copy)]
struct Dial {
    curr_position: u32,
}

impl Dial {
    const DIGIT_COUNT: u32 = 100;

    pub fn new(initial_value: u32) -> Self {
        Dial {
            curr_position: initial_value,
        }
    }

    fn advance_counter(&mut self, value: i64) -> u32 {
        let mut seen_zeros: u32 = 0;
        let prev_value = self.curr_position;

        let next_position_value = self.curr_position as i64 + value;

        if prev_value != 0 && next_position_value <= 0 {
            seen_zeros += 1;
        }

        seen_zeros += (next_position_value.abs() as u32) / Self::DIGIT_COUNT;

        let next_value = ((next_position_value % Self::DIGIT_COUNT as i64)
            + Self::DIGIT_COUNT as i64)
            % Self::DIGIT_COUNT as i64;

        self.curr_position = next_value as u32;

        return seen_zeros;
    }

    pub fn move_right(&mut self, value: u32) -> u32 {
        self.advance_counter(value as i64)
    }

    pub fn move_left(&mut self, value: u32) -> u32 {
        self.advance_counter(-(value as i64))
    }
}

fn main() -> io::Result<()> {
    let mut zero_count = 0;
    let mut dial = Dial::new(50);
    let args: Vec<String> = env::args().collect();

    let filename = match args.get(1) {
        Some(x) => x,
        None => {
            eprintln!("Filename not provided!");
            std::process::exit(1);
        }
    };

    let file = File::open(filename).map_err(|e| {
        eprintln!("Filed to open file {}: {}", filename, e);
        e
    })?;

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line?;
        let direction = line_contents.chars().nth(0).unwrap();
        let slice = &line_contents[1..];
        let rotataion_value: u32 = slice.parse().unwrap();

        if direction == 'L' {
            zero_count += dial.move_left(rotataion_value);
        } else {
            zero_count += dial.move_right(rotataion_value);
        }
    }

    println!("zero_count = {}", zero_count);

    Ok(())
}
