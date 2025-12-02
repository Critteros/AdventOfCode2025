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
        let next_position_value = self.curr_position as i64 + value;

        let next_value = ((next_position_value % Self::DIGIT_COUNT as i64)
            + Self::DIGIT_COUNT as i64)
            % Self::DIGIT_COUNT as i64;

        self.curr_position = next_value as u32;

        return self.curr_position;
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

        let after_rotation;

        if direction == 'L' {
            after_rotation = dial.move_left(rotataion_value);
        } else {
            after_rotation = dial.move_right(rotataion_value);
        }

        if after_rotation == 0 {
            zero_count += 1;
        }
    }

    println!("zero_count = {}", zero_count);

    Ok(())
}
