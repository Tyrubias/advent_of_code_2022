use nom::{
    branch::alt,
    bytes::{complete::take, streaming::tag},
    character::complete::{anychar, char},
    combinator::{map, opt},
    sequence::{delimited, preceded},
    IResult,
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
    delimited(char('['), |i| map(anychar, Crate)(i), char(')'))(i)
}

fn parse_no_crate(i: &str) -> IResult<&str, ()> {
    map(tag("   "), drop)(i)
}

fn main() {
    println!("Hello, world!");
}
