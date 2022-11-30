use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/inputs/day1.txt")?;
    let reader = BufReader::new(file);

    let mut calories = Vec::<u32>::new();
    calories.push(0);
    let mut elf_count = 1;

    reader.lines().for_each(|line| {
        let line_str = line.expect("Failed to get line");

        if line_str.is_empty() {
            elf_count += 1;
            calories.push(0);
            return;
        }

        calories[elf_count - 1] += line_str.parse::<u32>().expect("Failed to parse str to int");
    });

    calories.sort();
    println!("The elf carrying the most calories carries: {}", calories[elf_count - 1]);

    Ok(())
}
