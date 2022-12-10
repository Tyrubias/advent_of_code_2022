#![forbid(unsafe_code)]
#![forbid(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::{
    env::args_os,
    fmt::{self, Debug, Formatter},
    fs::read_to_string,
};

use color_eyre::{eyre::eyre, install, Result};
use itertools::Itertools;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char},
    combinator::{all_consuming, map, opt},
    sequence::{delimited, preceded},
    Finish, IResult,
};

fn main() -> Result<()> {
    install()?;

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

    let cargo_lines = transpose(cargo_lines);

    for cargo_line in &cargo_lines {
        println!("{cargo_line:?}");
    }

    Ok(())
}

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

fn transpose<T>(v: Vec<Vec<Option<T>>>) -> Vec<Vec<T>> {
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
