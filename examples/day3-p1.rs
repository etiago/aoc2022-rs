use std::{
    collections::HashMap,
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/inputs/day3.txt")?;
    let reader = BufReader::new(file);

    let mut priority_sum: u32 = 0;

    reader.lines().for_each(|line| {
        let line_str = line.expect("Failed to get line");
        let bytes = line_str.as_bytes();

        let mut first_half = HashSet::<u8>::new();
        let mut second_half = HashSet::<u8>::new();

        for i in 0..bytes.len()/2 {
            first_half.insert(bytes[i]);
        }

        for i in bytes.len()/2..bytes.len() {
            second_half.insert(bytes[i]);
        }

        let intersect: Vec::<&u8> = first_half.intersection(&second_half).collect();

        let c = **intersect.get(0).expect("Failed to get intersection") as char;

        if c.is_ascii_lowercase() {
            priority_sum += ((c as u8) - ('a' as u8) + 1) as u32;
        } else {
            priority_sum += ((c as u8) - ('A' as u8) + 27) as u32;
        }
    });


    println!("Sum of priorities of repeated characters: {}", priority_sum);
    Ok(())
}
