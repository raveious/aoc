use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() -> std::io::Result<()> {

    let mut file = File::open("input.txt")?;
    let mut contents = String::new();
    let multiply_regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let word_regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\))").unwrap();
    let mut enabled: bool = true;
    let mut sum: u32 = 0;

    file.read_to_string(&mut contents)?;

    for (_, [word]) in word_regex.captures_iter(&contents).map(|c| c.extract()) {
        println!("word: {}", word);

        match word{
            "do()" => enabled = true,
            "don't()" => enabled = false,
            &_ => (),
        }

        if enabled && &word[0..3] == "mul"{
            let caps = multiply_regex.captures(word).unwrap();
            println!("lhs: {}  rhs: {}", &caps[1], &caps[2]);
            sum += caps[1].parse::<u32>().unwrap() * caps[2].parse::<u32>().unwrap();
        }
    }

    println!("Answer: {}", sum);

    Ok(())
}
