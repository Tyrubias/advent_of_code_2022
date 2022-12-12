use std::{borrow::Borrow, env::args_os, fs::read_to_string, ops::Add};

use color_eyre::{eyre::eyre, install, Result};
use itertools::{
    iproduct,
    FoldWhile::{Continue, Done},
    Itertools,
};
use ndarray::{Array, Dim};

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

    let directions = [(0, 1), (0, -1), (-1, 0), (1, 0)];

    let part1 = iproduct!(0..num_rows, 0..num_columns)
        .filter(|&(x, y)| {
            directions
                .iter()
                .any(|&dir| is_visible_in_direction(&grid, (x, y), dir))
        })
        .count();

    println!("{part1}");

    let part2: usize = iproduct!(0..num_rows, 0..num_columns)
        .map(|(x, y)| {
            directions
                .iter()
                .map(|&dir| num_visible_in_direction(&grid, (x, y), dir))
                .product()
        })
        .max()
        .ok_or_else(|| eyre!("answer should have max"))?;

    println!("{part2}");

    Ok(())
}

fn num_visible_in_direction<T>(
    grid: impl Borrow<Array<T, Dim<[usize; 2]>>>,
    (x, y): (usize, usize),
    dir: (isize, isize),
) -> usize
where
    T: Add + Copy + PartialOrd,
{
    let grid = grid.borrow();
    let our_height = grid[[x, y]];
    neighbors_in_direction(grid, (x, y), dir)
        .into_iter()
        .fold_while(0, |total, their_height| {
            let next = total + 1;
            if their_height >= our_height {
                Done(next)
            } else {
                Continue(next)
            }
        })
        .into_inner()
}

fn is_visible_in_direction<T>(
    grid: impl Borrow<Array<T, Dim<[usize; 2]>>>,
    (x, y): (usize, usize),
    dir: (isize, isize),
) -> bool
where
    T: Copy + PartialOrd,
{
    let grid = grid.borrow();
    let our_height = grid[[x, y]];
    neighbors_in_direction(grid, (x, y), dir)
        .into_iter()
        .all(|their_height| our_height > their_height)
}

fn neighbors_in_direction<T>(
    grid: impl Borrow<Array<T, Dim<[usize; 2]>>>,
    (x, y): (usize, usize),
    (dx, dy): (isize, isize),
) -> Vec<T>
where
    T: Copy,
{
    let grid = grid.borrow();
    (1..)
        .into_iter()
        .map_while(|step| {
            grid.get((
                x.checked_add_signed(dx * step)?,
                y.checked_add_signed(dy * step)?,
            ))
        })
        .copied()
        .collect()
}
