use std::{
    collections::HashMap,
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() -> Result<(), Box<dyn Error>> {
    let scoring_pairs = HashMap::from([
        ("A", HashMap::from([("X", 3), ("Y", 6), ("Z", 0)])),
        ("B", HashMap::from([("X", 0), ("Y", 3), ("Z", 6)])),
        ("C", HashMap::from([("X", 6), ("Y", 0), ("Z", 3)])),
    ]);

    let scoring_per_type = HashMap::from([("X", 1), ("Y", 2), ("Z", 3)]);

    let my_play_translator = HashMap::from([
        ("X", HashMap::from([("A", "Z"), ("B", "X"), ("C", "Y")])),
        ("Y", HashMap::from([("A", "X"), ("B", "Y"), ("C", "Z")])),
        ("Z", HashMap::from([("A", "Y"), ("B", "Z"), ("C", "X")])),
    ]);

    let file = File::open("examples/inputs/day2.txt")?;
    let reader = BufReader::new(file);

    let mut score = 0;
    reader.lines().for_each(|line| {
        let line_str = line.expect("Failed to read line");
        let play = line_str.split(" ").collect::<Vec<&str>>();

        let elf_play = play[0];
        let my_play = my_play_translator
            .get(play[1])
            .expect("Failed to get translated play")
            .get(elf_play)
            .expect("Failed to get translated play for elf play");

        let score_from_pairs = scoring_pairs
            .get(elf_play)
            .expect("Couldn´t find elf play")
            .get(my_play)
            .expect("Couldn´t find my play");

        let score_from_type = scoring_per_type
            .get(my_play)
            .expect("Couldn´t find my play in per type");

        score += score_from_pairs;
        score += score_from_type;
    });

    println!("My score: {}", score);
    Ok(())
}
