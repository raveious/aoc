use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()>{
    let mut left: Vec<u32> = Vec::with_capacity(1000);
    let mut right: Vec<u32> = Vec::with_capacity(1000);
    let mut sum: u32 = 0;

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        assert!(line.is_ok(), "Failed to read a line from the input file.");
        let line_string = line.unwrap();
        let left_string = &line_string[0..6].trim();
        let right_string = &line_string[8..].trim();

        left.push(left_string.parse::<u32>().unwrap());
        right.push(right_string.parse::<u32>().unwrap());
    }

    println!(r"Loaded {} left entries and {} right entries!", left.len(), right.len());

    left.sort();
    right.sort();

    assert_eq!(left.len(), right.len());

    for i in 0..left.len() {
        if left[i] < right[i] {
            sum += right[i] - left[i];
        } else {
            sum += left[i] - right[i];
        }
    }

    println!(r"Answer: {}", sum);

    Ok(())
}
