use std::{
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
    iter::Cloned,
    ops::{Deref, DerefMut},
    vec::IntoIter,
};

fn parse_initial_state(lines: &mut Lines<BufReader<File>>) -> Vec<Vec<char>> {
    let mut initial_state: Vec<Vec<char>> = Vec::new();

    let mut inner_idx = 0;
    let mut str_idx = 0;

    lines
        .take_while(|line| {
            let line_str = line.as_ref().expect("Failed to get line");
            !line_str.contains("1")
        })
        .for_each(|line| {
            let line_str = line.expect("Failed to get line");
            let bytes = line_str.as_bytes();

            while str_idx < bytes.len() {
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

fn print_state(state: &Vec<Vec<char>>) {
    state.iter().for_each(|i| {
        i.iter().for_each(|k| {
            print!("[{}] ", k);
        });

        println!("");
    });
    println!("");
}

struct Movement {
    from: usize,
    to: usize,
    quantity: usize,
}

impl Movement {
    fn try_parse(input: &str) -> Result<Movement, String> {
        let first_parts: Vec<&str> = input.split(" from ").collect();

        let move_qty = first_parts
            .get(0)
            .ok_or_else(|| "Failed to get quantity to move")?
            .split(" ")
            .skip(1)
            .next()
            .ok_or_else(|| "Failed to get quantity after split")?
            .parse::<usize>()
            .map_err(|err| {err.to_string()})?;

        let move_pos: Vec<&str> = first_parts
            .get(1)
            .ok_or_else(|| "Failed to get pos")?
            .split(" to ")
            .collect();

        let move_from = move_pos
            .get(0)
            .ok_or_else(|| "Failed to get from")?
            .parse::<usize>()
            .map_err(|err| {err.to_string()})?
            - 1;
        let move_to = move_pos
            .get(1)
            .ok_or_else(|| "Failed to get to")?
            .parse::<usize>()
            .map_err(|err| {err.to_string()})?
            - 1;

        Ok(Movement {
            from: move_from,
            to: move_to,
            quantity: move_qty,
        })
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/inputs/day5.txt")?;
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    let mut state = parse_initial_state(lines.by_ref());

    print_state(&state);

    lines.next();
    lines.for_each(|line| {
        let line_str = line.expect("Failed to get line");
        let movement: Movement = Movement::try_parse(&line_str).expect("Failed to parse Movement");
        let mut move_qty_u = movement.quantity;

        while move_qty_u > 0 {
            move_qty_u -= 1;

            let element = state
                .get_mut(movement.from)
                .expect("Failed to get from")
                .remove(0);

            state
                .get_mut(movement.to)
                .expect("Failed to get to")
                .insert(0, element);
        }
    });

    print_state(&state);

    Ok(())
}
