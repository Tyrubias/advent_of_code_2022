#![forbid(unsafe_code)]
#![forbid(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::{
    env::args_os,
    fmt::{self, Debug, Formatter},
    fs::read_to_string,
    slice::Iter,
};

use color_eyre::{eyre::eyre, install, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, digit1},
    combinator::{all_consuming, map, map_res, opt},
    sequence::{delimited, preceded, tuple},
    Finish, IResult,
};

fn main() -> Result<()> {
    install()?;

    let file_path = args_os().nth(1).ok_or_else(|| eyre!("expect a CLI arg"))?;
    let contents = read_to_string(file_path)?;
    let (ship, instructions) = contents
        .split("\n\n")
        .take(2)
        .collect_tuple()
        .ok_or_else(|| eyre!("should have 2-tuple"))?;

    let cargo_lines = ship
        .lines()
        .flat_map(|line| {
            all_consuming(parse_ship_line)(line)
                .finish()
                .map(|(_, c)| c)
        })
        .collect_vec();

    let mut cargo_lines_1 = Stacks(transpose_reverse(cargo_lines));
    let mut cargo_lines_2 = cargo_lines_1.clone();

    for line in instructions.lines() {
        if let Ok((_, r#move)) = all_consuming(parse_move)(line).finish() {
            cargo_lines_1.apply_part_1(r#move)?;
            cargo_lines_2.apply_part_2(r#move)?;
        }
    }

    println!(
        "Part 1: {}",
        cargo_lines_1
            .0
            .iter()
            .flat_map(|stack| stack.last())
            .map(|c| c.0)
            .join("")
    );

    println!(
        "Part 2: {}",
        cargo_lines_2
            .0
            .iter()
            .flat_map(|stack| stack.last())
            .map(|c| c.0)
            .join("")
    );

    Ok(())
}

#[derive(Clone)]
struct Stacks(Vec<Vec<Crate>>);

impl Stacks {
    fn apply_part_1(&mut self, r#move: Move) -> Result<()> {
        let from = &mut self.0[r#move.from];
        let mut moved = from.split_off(from.len().saturating_sub(r#move.count));
        moved.reverse();
        self.0[r#move.to].append(&mut moved);
        Ok(())
    }

    fn apply_part_2(&mut self, r#move: Move) -> Result<()> {
        let from = &mut self.0[r#move.from];
        let mut moved = from.split_off(from.len().saturating_sub(r#move.count));
        self.0[r#move.to].append(&mut moved);
        Ok(())
    }

    fn iter(&self) -> Iter<Vec<Crate>> {
        self.0.iter()
    }
}

impl<'a> IntoIterator for &'a Stacks {
    type Item = &'a Vec<Crate>;

    type IntoIter = Iter<'a, Vec<Crate>>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl Debug for Stacks {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for (i, item) in self.iter().enumerate() {
            writeln!(f, "Stack {i}: {item:?}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
struct Move {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_move(i: &str) -> IResult<&str, Move> {
    let move_parser = preceded(
        tag("move "),
        tuple((
            map_res(digit1, |s: &str| s.parse::<usize>()),
            preceded(
                tag(" from "),
                map_res(digit1, |s: &str| s.parse::<usize>().map(|n| n - 1)),
            ),
            preceded(
                tag(" to "),
                map_res(digit1, |s: &str| s.parse::<usize>().map(|n| n - 1)),
            ),
        )),
    );

    map(move_parser, |(count, from, to)| Move { from, to, count })(i)
}

#[derive(Clone, Copy)]
struct Crate(char);

impl Debug for Crate {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn parse_ship_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let mut ship_line = Vec::new();
    let (mut remainder, mut maybe_maybe_crate) = opt(parse_maybe_crate)(i)?;

    while let Some(maybe_crate) = maybe_maybe_crate {
        ship_line.push(maybe_crate);
        (remainder, maybe_maybe_crate) = opt(preceded(char(' '), parse_maybe_crate))(remainder)?;
    }

    Ok((remainder, ship_line))
}

fn parse_maybe_crate(i: &str) -> IResult<&str, Option<Crate>> {
    alt((map(parse_crate, Some), map(parse_no_crate, |_| None)))(i)
}

fn parse_crate(i: &str) -> IResult<&str, Crate> {
    delimited(char('['), map(anychar, Crate), char(']'))(i)
}

fn parse_no_crate(i: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(i)
}

fn transpose_reverse<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
    if v.is_empty() {
        return Vec::new();
    }

    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .rev()
                .flat_map(|n| n.next())
                .flatten()
                .collect_vec()
        })
        .collect_vec()
}
