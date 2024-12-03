use std::fs::File;
use std::io::{prelude::*, BufReader};

#[derive(Debug, PartialEq, Eq)]
enum State {
    Unknown,
    Increasing,
    Decreasing
}

fn check_safety(prev_val: u8, next_val: u8, prev_state: State) -> (bool, State) {
    if prev_val == next_val {
        return (false, prev_state);
    }

    let val = prev_val.abs_diff(next_val);
    let mut next_state = State::Unknown;

    if next_val > prev_val {
        next_state = State::Increasing;
    } else {
        next_state = State::Decreasing;
    }

    if prev_state != next_state && prev_state != State::Unknown {
        return (false, next_state);
    }

    return (val >= 1 && val <= 3, next_state);
}

fn main() -> std::io::Result<()>{
    let mut part1_sum: u32 = 0;
    // let mut part2_sum: u32 = 0;

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        assert!(line.is_ok(), "Failed to read a line from the input file.");
        
        let mut line_values: Vec<u8> = Vec::with_capacity(10);
        line.unwrap().split_whitespace().for_each(|v| {
            line_values.push(v.parse::<u8>().unwrap());
        });

        let mut state = State::Unknown;
        let mut next_state = State::Unknown;
        let mut safe = false;
        for i in 1..line_values.len() {
            (safe, next_state) = check_safety(
                line_values[i - 1],
                line_values[i],
                state
            );

            state = next_state;

            if !safe {
                break;
            }
        }

        if safe {
            part1_sum += 1;
        }
    }

    println!(r"Answer 1: {}", part1_sum);
    // println!(r"Answer 2: {}", part2_sum);

    Ok(())
}
