use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    // [Elf [Me]]
    // [R [R, P, S], P [R, P, S], S [R, P, S]]
    let scoring_array: [[i32; 3]; 3] = [[3, 6, 0], [0, 3, 6], [6, 0, 3]];

    let file = File::open("examples/inputs/day2.txt")?;
    let reader = BufReader::new(file);

    let final_score = reader.lines().fold(0, |score, line| {
        let line_str = line.expect("Failed to read line");
        let chars = line_str.as_bytes();

        // For the Elf play, normalize ASCII bytes such that A == 0
        // to use as an index into the array.
        // For my play, normalize ASCII bytes such that Z == 0.
        let elf_play = (chars[0] - ('A' as u8)) as usize;
        let my_play = (chars[2] - ('X' as u8)) as usize;

        let score_from_pairs = scoring_array[elf_play][my_play];
        let score_from_type = my_play as i32 + 1;

        score + score_from_type + score_from_pairs
    });

    println!("My score: {}", final_score);
    Ok(())
}
