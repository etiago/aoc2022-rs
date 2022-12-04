use std::{
    collections::HashMap,
    collections::HashSet,
    error::Error,
    fs::File,
    io::{BufRead, BufReader}, iter::FromIterator,
};

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/inputs/day3.txt")?;
    let reader = BufReader::new(file);

    let mut priority_sum: u32 = 0;

    let mut read_lines = 0;
    let mut three_lines: [Vec<u8>; 3] = [Vec::new(), Vec::new(), Vec::new()];

    reader.lines().for_each(|line| {
        let line_str = line.expect("Failed to get line");
        let bytes = line_str.as_bytes().into_iter();

        three_lines[read_lines].clear();
        three_lines[read_lines].extend(bytes);
        read_lines += 1;

        if read_lines == 3 {
            let first_line: HashSet<u8> =
                HashSet::from_iter(three_lines[0].iter().map(|b| *b));
            let second_line: HashSet<u8> =
                HashSet::from_iter(three_lines[1].iter().map(|b| *b));
            let third_line: HashSet<u8> =
                HashSet::from_iter(three_lines[2].iter().map(|b| *b));

            let first_second: HashSet<u8> =
                first_line.intersection(&second_line).map(|b| *b).collect();
            let common = first_second
                .intersection(&third_line)
                .into_iter()
                .next()
                .expect("Failed to get common element");

            let c = *common as char;

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
