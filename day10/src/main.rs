use std::{env::args_os, fs::read_to_string};

use color_eyre::{eyre::eyre, install, Result};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1},
    combinator::{all_consuming, map, map_res, opt, recognize, value},
    error::Error,
    sequence::{preceded, tuple},
    Finish, IResult,
};

fn main() -> Result<()> {
    install()?;

    let file_path = args_os()
        .nth(1)
        .ok_or_else(|| eyre!("program should have one argument"))?;
    let contents = read_to_string(file_path)?;

    let instructions = contents
        .lines()
        .map(|line| {
            all_consuming(parse_instruction)(line)
                .finish()
                .map(|(_, instruction)| instruction)
                .map_err(|error| Error::new(error.input.to_string(), error.code))
        })
        .collect::<Result<Vec<_>, _>>()?;

    for instruction in instructions {
        println!("{instruction:?}");
    }

    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    alt((parse_noop, parse_addx))(input)
}

fn parse_addx(input: &str) -> IResult<&str, Instruction> {
    map(preceded(tag("addx "), parse_integer), Instruction::Addx)(input)
}

fn parse_noop(input: &str) -> IResult<&str, Instruction> {
    value(Instruction::Noop, tag("noop"))(input)
}

fn parse_integer(input: &str) -> IResult<&str, i32> {
    map_res(recognize(tuple((opt(char('-')), digit1))), str::parse)(input)
}
