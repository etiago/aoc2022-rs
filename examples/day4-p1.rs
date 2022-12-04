use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader},
    ops::Deref,
};

struct Segment {
    start: u32,
    end: u32,
}

impl Segment {
    pub fn contains(&self, other: &Segment) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

impl From<&str> for Segment {
    fn from(s: &str) -> Self {
        let parts: Vec<&str> = s.split("-").collect();
        let start = parts
            .get(0)
            .expect("Failed to get start for segment")
            .parse::<u32>()
            .expect("Failed to parse start of segment");
        let end = parts
            .get(1)
            .expect("Failed to get end for segment")
            .parse::<u32>()
            .expect("Failed to parse end of segment");
        Segment { start, end }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/inputs/day4.txt")?;
    let reader = BufReader::new(file);

    let mut fully_overlapping: u32 = 0;

    reader.lines().for_each(|line| {
        let line_str = line.expect("Failed to get line");

        let segments: Vec<&str> = line_str.split(",").collect();

        let first_half: Segment = segments
            .get(0)
            .expect("Failed to get first segment")
            .deref()
            .into();
        let second_half: Segment = segments
            .get(1)
            .expect("Failed to get second segment")
            .deref()
            .into();

        if first_half.contains(&second_half) || second_half.contains(&first_half) {
            fully_overlapping += 1;
        }
    });

    println!("Overlaps: {}", fully_overlapping);
    Ok(())
}
