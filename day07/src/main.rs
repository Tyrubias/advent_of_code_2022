#![forbid(unsafe_code)]
#![forbid(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::{env::args_os, fs::read_to_string, path::MAIN_SEPARATOR};

use camino::Utf8PathBuf;
use color_eyre::{eyre::eyre, install, Result};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::u64,
    combinator::{all_consuming, map},
    error::Error,
    sequence::{preceded, separated_pair},
    Finish, IResult,
};

fn main() -> Result<()> {
    install()?;

    let file_path = args_os()
        .nth(1)
        .ok_or_else(|| eyre!("should have more args"))?;
    let contents = read_to_string(file_path)?;

    for line in contents.lines() {
        let (_, input_line) = all_consuming(parse_line)(line)
            .finish()
            .map_err(|err| Error::new(err.input.to_string(), err.code))?;
        println!("{input_line:?}");
    }

    Ok(())
}

#[derive(Debug)]
enum Line {
    Command(Command),
    Entry(Entry),
}

fn parse_line(input: &str) -> IResult<&str, Line> {
    alt((
        map(parse_entry, Line::Entry),
        map(parse_command, Line::Command),
    ))(input)
}

#[derive(Debug)]
enum Entry {
    Dir(Utf8PathBuf),
    File(usize, Utf8PathBuf),
}

fn parse_entry(input: &str) -> IResult<&str, Entry> {
    alt((parse_dir, parse_file))(input)
}

fn parse_file(input: &str) -> IResult<&str, Entry> {
    map(separated_pair(u64, tag(" "), parse_path), |(size, name)| {
        Entry::File(size as usize, name)
    })(input)
}

fn parse_dir(input: &str) -> IResult<&str, Entry> {
    preceded(tag("dir "), map(parse_path, Entry::Dir))(input)
}

#[derive(Debug)]
enum Command {
    Ls,
    Cd(Utf8PathBuf),
}

fn parse_command(input: &str) -> IResult<&str, Command> {
    preceded(tag("$ "), alt((parse_ls, parse_cd)))(input)
}

fn parse_cd(input: &str) -> IResult<&str, Command> {
    preceded(tag("cd "), map(parse_path, Command::Cd))(input)
}

fn parse_path(input: &str) -> IResult<&str, Utf8PathBuf> {
    map(
        take_while1(|c: char| c.is_alphanumeric() || c == '.' || c == MAIN_SEPARATOR),
        Into::into,
    )(input)
}

fn parse_ls(input: &str) -> IResult<&str, Command> {
    map(tag("ls"), |_| Command::Ls)(input)
}
