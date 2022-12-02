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

    let mut line_intersection = HashSet::<u8>::new();
    let mut read_lines = 0;

    reader.lines().for_each(|line| {
        let line_str = line.expect("Failed to get line");
        let bytes = line_str.as_bytes();

        let mut line_bytes = HashSet::<u8>::new();

        for i in 0..bytes.len() {
            line_bytes.insert(bytes[i]);
        }

        if read_lines == 0 {
            line_intersection.clear();
            line_intersection = line_bytes;
            read_lines += 1;
            return;
        }

        let prev_intersection = line_intersection.clone();
        let intersect = prev_intersection.intersection(&line_bytes);
        line_intersection.clear();

        for inter_byte in intersect {
            line_intersection.insert(*inter_byte);
        }

        read_lines += 1;

        if read_lines == 3 {
            let intersect: Vec::<&u8> = line_intersection.iter().collect();
            let c = **intersect.get(0).expect("Failed to get intersection") as char;

            if c.is_ascii_lowercase() {
                priority_sum += ((c as u8) - ('a' as u8) + 1) as u32;
            } else {
                priority_sum += ((c as u8) - ('A' as u8) + 27) as u32;
            }

            read_lines = 0;
        }

    });


    println!("Sum of priorities of repeated characters: {}", priority_sum);
    Ok(())
}
