use std::{
    collections::{HashSet, VecDeque},
    error::Error,
    fs::File,
    io::{BufRead, BufReader, Lines},
    iter::Cloned,
    ops::{Deref, DerefMut},
    vec::IntoIter,
};

use std::arch::asm;
use std::ptr;

fn main() -> Result<(), Box<dyn Error>> {
    let file = File::open("examples/inputs/day6.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut line_s = lines.next().expect("Failed to get line").expect("Failed");
    let line = line_s.as_bytes();

    let mut s = HashSet::<u8>::new();

    let mut window: [u8; 4] = [0; 4];
    let mut window_ptr = window.as_ptr() as *const u32;

    let mut bytes_ptr = line.as_ptr();

    let mut idx = 0;

    while idx < line.len() {
        unsafe {
            let bytes_ptr_i = *(bytes_ptr as *const u32) as u32;
            asm!(
                "mov [{0}], {1}",
                in(reg) window_ptr,
                in(reg) bytes_ptr_i
            );

            s.clear();
            s.extend(window.iter());

            if s.len() == 4 {
                println!("Found index: {}", idx + 4);
                return Ok(());
            }
            idx += 1;
            // Still not happy that I can´t just advance the pointer :-(
            bytes_ptr = &line[idx];//bytes_ptr.wrapping_offset(1);
        }
    }

    Ok(())
}
