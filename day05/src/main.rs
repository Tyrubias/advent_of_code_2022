use std::{env::args_os, fs::read_to_string};

use color_eyre::eyre::eyre;
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char},
    combinator::{all_consuming, map, opt},
    sequence::{delimited, preceded},
    Finish, IResult,
};

#[derive(Debug)]
struct Crate(char);

fn parse_ship_line(i: &str) -> IResult<&str, Vec<Option<Crate>>> {
    let mut ship_line = Vec::new();
    let (mut remainder, maybe_crate) = parse_maybe_crate(i)?;

    ship_line.push(maybe_crate);

    loop {
        let (maybe_remainder, maybe_maybe_crate) =
            opt(preceded(char(' '), parse_maybe_crate))(remainder)?;
        match maybe_maybe_crate {
            Some(maybe_crate) => ship_line.push(maybe_crate),
            None => break,
        }
        remainder = maybe_remainder;
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

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let file_path = args_os().nth(1).ok_or_else(|| eyre!("expect a CLI arg"))?;
    let contents = read_to_string(file_path)?;
    let (ship, _) = contents
        .split("\n\n")
        .take(2)
        .collect_tuple()
        .ok_or_else(|| eyre!("should have 2-tuple"))?;

    let mut cargo_lines = Vec::new();

    for line in ship.lines() {
        if let Ok((_, cargo_line)) = all_consuming(parse_ship_line)(line).finish() {
            cargo_lines.push(cargo_line)
        }
    }

    for cargo_line in &cargo_lines {
        println!("{cargo_line:?}");
    }

    Ok(())
}
