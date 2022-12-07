use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
    iter::Cloned,
    ops::{Deref, DerefMut},
    vec::IntoIter,
};

fn print_bytes_as_chars(s: &HashSet::<u8>) {
    println!("s");
    s.iter().for_each(|b| { print!("{} ", *b as char)});
    println!("");
}

fn print_bytes_as_chars_vec(s: &VecDeque::<u8>) {
    println!("v");
    s.iter().for_each(|b| { print!("{} ", *b as char)});
    println!("");
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/inputs/day6.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let line = lines.next().expect("Failed to get line").expect("Failed");

    let bytes = line.as_bytes();
    let mut sliding_window_vec = VecDeque::<u8>::new();
    let mut current_idx = 0;
    let mut last_byte = bytes[0];
    let mut finished = false;
    let mut initialized = false;
    bytes.iter().for_each(|b| {
        if finished {
            return;
        }


        if current_idx <= 13 {
            sliding_window_vec.push_back(*b);
            current_idx += 1;
            return;
        }


        //sliding_window_set.remove(&last_byte);
        //sliding_window_set.insert(*b);
        last_byte = sliding_window_vec.pop_front().expect("Err");
        sliding_window_vec.push_back(*b);

        let mut s = HashSet::<u8>::new();
        s.extend(&sliding_window_vec);
        if s.len() == 14 {
            finished = true;
            return;
        }

        current_idx += 1;

        //print_bytes_as_chars(&sliding_window_set);


    });

    println!("Current idx: {}", current_idx);
    Ok(())
}
