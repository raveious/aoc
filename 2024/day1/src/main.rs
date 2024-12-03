use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> std::io::Result<()>{
    let mut left: Vec<u32> = Vec::with_capacity(1000);
    let mut right: Vec<u32> = Vec::with_capacity(1000);
    let mut right_count: Vec<u32> = vec![0; 99999];
    let mut part1_sum: u32 = 0;
    let mut part2_sum: u32 = 0;

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        assert!(line.is_ok(), "Failed to read a line from the input file.");
        let line_string = line.unwrap();
        let left_val = line_string[0..6].trim().parse::<u32>().unwrap();
        let right_val = line_string[8..].trim().parse::<u32>().unwrap();

        left.push(left_val);
        right.push(right_val);

        right_count[right_val as usize] += 1;
    }

    println!(r"Loaded {} left entries and {} right entries!", left.len(), right.len());

    left.sort();
    right.sort();

    assert_eq!(left.len(), right.len());

    for i in 0..left.len() {
        if left[i] < right[i] {
            part1_sum += right[i] - left[i];
        } else {
            part1_sum += left[i] - right[i];
        }

        part2_sum += left[i] * right_count[left[i] as usize];
    }

    println!(r"Answer 1: {}", part1_sum);
    println!(r"Answer 2: {}", part2_sum);

    Ok(())
}
