use std::{env::args_os, fs::read_to_string};

use color_eyre::{eyre::eyre, install, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{all_consuming, map, map_res, value},
    error::Error,
    sequence::separated_pair,
    Finish, IResult,
};

fn main() -> Result<()> {
    install()?;

    let file_path = args_os()
        .nth(1)
        .ok_or_else(|| eyre!("program should have arg"))?;
    let contents = read_to_string(file_path)?;

    let motions = contents
        .lines()
        .map(|line| {
            all_consuming(parse_motion)(line)
                .finish()
                .map(|(_, motion)| motion)
                .map_err(|error| Error::new(error.input.to_string(), error.code))
        })
        .collect::<Result<Vec<_>, _>>()?;

    dbg!(motions);

    Ok(())
}

#[derive(Debug)]
struct Motion {
    direction: Direction,
    steps: usize,
}

fn parse_motion(input: &str) -> IResult<&str, Motion> {
    map(
        separated_pair(parse_direction, char(' '), map_res(digit1, str::parse)),
        |(direction, steps)| Motion { direction, steps },
    )(input)
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Up,
    Left,
    Down,
}

fn parse_direction(input: &str) -> IResult<&str, Direction> {
    alt((
        value(Direction::Right, tag("R")),
        value(Direction::Up, tag("U")),
        value(Direction::Left, tag("L")),
        value(Direction::Down, tag("D")),
    ))(input)
}
