use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
    iter::Cloned,
    ops::{Deref, DerefMut},
    vec::IntoIter,
};



fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/inputs/day7.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    //let mut state = parse_initial_state(lines.by_ref());

    //print_state(&state);

    lines.next();
    lines.for_each(|line| {

    });

    //print_state(&state);

    Ok(())
}
