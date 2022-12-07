use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
    ops::{Deref, DerefMut},
    vec::IntoIter,
};

fn parse_initial_state(lines: Lines<BufReader<File>>) -> Vec<Vec<char>> {
    let mut initial_state: Vec<Vec<char>> = Vec::new();

    let mut inner_idx = 0;
    let mut str_idx = 0;
    let mut finished = false;
    lines.for_each(|line| {
        if finished {
            return;
        }
        let line_str = line.expect("Failed to get line");
        if line_str.contains("1") {
            finished = true;
            return;
        }
        let bytes = line_str.as_bytes();

        while (str_idx < bytes.len()) {
            str_idx += 1;
            let c = bytes[str_idx] as char;

            if initial_state.get(inner_idx).is_none() {
                initial_state.push(Vec::new());
            }

            if c != ' ' {
                initial_state.get_mut(inner_idx).expect("Fail").push(c);
            }

            inner_idx += 1;
            str_idx += 3;
        }

        inner_idx = 0;
        str_idx = 0;
    });

    initial_state
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/inputs/day5.txt")?;
    let reader = BufReader::new(file);

    let lines = reader.lines();

    let mut initial_state = parse_initial_state(lines);

        initial_state.iter().for_each(|i| {
        i.iter().for_each(|k| {
            print!("{}", k);
        });

        println!("");
        });

    println!("");

    let file2 = File::open("examples/inputs/day5.txt")?;
    let reader2 = BufReader::new(file2);

    let lines2 = reader2.lines();

    let mut start = false;
    lines2.for_each(|line| {
        let line_str = line.expect("Failed to get line");

        if line_str.is_empty() {
            start = true;
            return;
        }

        if !start {
            return;
        }

        let first_parts: Vec<&str> = line_str.split(" from ").collect();

        let move_qty: Vec<&str> = first_parts
            .get(0)
            .expect("Fail to get first")
            .split(" ")
            .collect();

        let mut move_qty_u = move_qty
            .get(1)
            .expect("Failed to get qty")
            .parse::<usize>()
            .expect("Fail to parse qty");

        let move_pos: Vec<&str> = first_parts
            .get(1)
            .expect("Fail to get second")
            .split(" to ")
            .collect();

        let total_moving = move_qty_u;
        while move_qty_u > 0 {

            let move_from = move_pos
                .get(0)
                .expect("Failed to get from")
                .parse::<usize>()
                .expect("Fail to parse from")
                - 1;
            let move_to = move_pos
                .get(1)
                .expect("Failed to get to")
                .parse::<usize>()
                .expect("Fail to parse to")
                - 1;

            let element = initial_state
                .get_mut(move_from)
                .expect("Failed to get from")
                .remove(0);
                //.expect("Getting element");

            let dest_size = initial_state.get(move_to).expect("Get to ").len();
            initial_state
                .get_mut(move_to)
                .expect("Failed to get to")
                .insert(total_moving - move_qty_u, element);
            move_qty_u -= 1;
        }

            initial_state.iter().for_each(|i| {
        i.iter().for_each(|k| {
            print!("{}", k);
        });

        println!("");
    });
    });

            println!("");
    initial_state.iter().for_each(|i| {
        i.iter().for_each(|k| {
            print!("{}", k);
        });

        println!("");
    });

    Ok(())
}
