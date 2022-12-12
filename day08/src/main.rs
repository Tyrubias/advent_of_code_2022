use std::{env::args_os, fs::read_to_string};

use color_eyre::{eyre::eyre, install, Result};
use itertools::{
    iproduct,
    FoldWhile::{Continue, Done},
    Itertools,
};
use ndarray::Array;

fn main() -> Result<()> {
    install()?;

    let file_path = args_os()
        .nth(1)
        .ok_or_else(|| eyre!("program should have argument"))?;
    let contents = read_to_string(file_path)?;

    let num_rows = contents.lines().count();
    let num_columns = contents
        .lines()
        .next()
        .map(|line| line.chars().count())
        .ok_or_else(|| eyre!("input should have at least 1 row"))?;

    let mut grid = Array::zeros((num_rows, num_columns));

    for (x, line) in contents.lines().enumerate() {
        for (y, tree) in line.chars().map(|c| c.to_digit(10)).enumerate() {
            grid[[x, y]] = tree.ok_or_else(|| eyre!("char should be numeric"))?;
        }
    }

    let moves = [(0, 1), (0, -1), (-1, 0), (1, 0)];

    let part1 = iproduct!(0..num_rows, 0..num_columns)
        .filter(|&(x, y)| {
            let our_height = grid[[x, y]];

            moves.iter().any(|&(dx, dy)| {
                (1..)
                    .into_iter()
                    .map_while(|step| {
                        grid.get((
                            x.checked_add_signed(dx * step)?,
                            y.checked_add_signed(dy * step)?,
                        ))
                    })
                    .all(|&their_height| our_height > their_height)
            })
        })
        .count();

    println!("{part1}");

    let part2: u32 = iproduct!(0..num_rows, 0..num_columns)
        .map(|(x, y)| {
            let our_height = grid[[x, y]];

            moves
                .iter()
                .map(|&(dx, dy)| {
                    (1..)
                        .into_iter()
                        .map_while(|step| {
                            grid.get((
                                x.checked_add_signed(dx * step)?,
                                y.checked_add_signed(dy * step)?,
                            ))
                        })
                        .fold_while(0u32, |total, &their_height| {
                            let next = total + 1;
                            if their_height >= our_height {
                                Done(next)
                            } else {
                                Continue(next)
                            }
                        })
                        .into_inner()
                })
                .product()
        })
        .max()
        .ok_or_else(|| eyre!("answer should have max"))?;

    println!("{part2}");

    Ok(())
}
