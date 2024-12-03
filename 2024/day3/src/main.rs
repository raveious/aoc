use std::fs::File;
use std::io::Read;
use std::io::prelude::*;
use regex::Regex;

fn main() -> std::io::Result<()> {

    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut part1_sum: u32 = 0;

    file.read_to_string(&mut contents)?;

    for (_, [lhs, rhs]) in re.captures_iter(&contents).map(|c| c.extract()) {
        println!("lhs: {}  rhs: {}", lhs, rhs);

        part1_sum += lhs.parse::<u32>().unwrap() * rhs.parse::<u32>().unwrap();
    }

    println!("Part 1 Answer: {}", part1_sum);

    Ok(())
}
