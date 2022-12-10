use std::{collections::VecDeque, env::args_os, fs::read_to_string};

use color_eyre::{eyre::eyre, install, Result};
use itertools::Itertools;

fn main() -> Result<()> {
    install()?;

    let file_path = args_os()
        .nth(1)
        .ok_or_else(|| eyre!("should have one arg"))?;
    let stream = read_to_string(file_path)?;

    println!("Part 1: {}", find_unique_window_of_size(&stream, 4));
    println!("Part 2: {}", find_unique_window_of_size(stream, 14));

    Ok(())
}

fn find_unique_window_of_size(s: impl AsRef<str>, size: usize) -> usize {
    let mut window = VecDeque::with_capacity(size);

    for (i, c) in s.as_ref().chars().enumerate() {
        if window.iter().unique().count() == size {
            return i;
        }
        if window.len() >= size {
            window.pop_front();
        }
        window.push_back(c);
    }

    0
}
