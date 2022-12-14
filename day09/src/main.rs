use std::{
    env::args_os,
    fs::read_to_string,
    ops::{Add, AddAssign, Sub, SubAssign},
    vec,
};

use color_eyre::{eyre::eyre, install, Result};
use itertools::Itertools;
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

    let mut head = Coordinate::default();
    let mut tail = Coordinate::default();

    let part1 = motions
        .iter()
        .flat_map(|&motion| {
            let mut tails = vec![];
            for _ in 0..motion.steps {
                head += motion.direction.into();
                tail += diff_to_delta((head - tail).into()).into();
                tails.push(tail);
            }
            tails
        })
        .unique()
        .count();

    println!("Part 1: {part1}");

    let mut knots = vec![Coordinate::default(); 10];

    let part2 = motions
        .iter()
        .flat_map(|&motion| {
            let mut tails = vec![];

            for _ in 0..motion.steps {
                knots[0] += motion.direction.into();

                for i in 1..knots.len() {
                    let current = knots[i];
                    let previous = knots[i - 1];

                    knots[i] += diff_to_delta((previous - current).into()).into();

                    if i == knots.len() - 1 {
                        tails.push(knots[i]);
                    }
                }
            }

            tails
        })
        .unique()
        .count();

    println!("Part 2: {part2}");

    Ok(())
}

fn diff_to_delta((x, y): (i32, i32)) -> (i32, i32) {
    match (x, y) {
        // overlapping
        (0, 0) => (0, 0),
        // touching up/left/down/right
        (0, 1) | (1, 0) | (0, -1) | (-1, 0) => (0, 0),
        // touching diagonally
        (1, 1) | (1, -1) | (-1, 1) | (-1, -1) => (0, 0),
        // need to move up/left/down/right
        (0, 2) => (0, 1),
        (0, -2) => (0, -1),
        (2, 0) => (1, 0),
        (-2, 0) => (-1, 0),
        // need to move to the right diagonally
        (2, 1) => (1, 1),
        (2, -1) => (1, -1),
        // need to move to the left diagonally
        (-2, 1) => (-1, 1),
        (-2, -1) => (-1, -1),
        // need to move up/down diagonally
        (1, 2) => (1, 1),
        (-1, 2) => (-1, 1),
        (1, -2) => (1, -1),
        (-1, -2) => (-1, -1),
        // 🆕 need to move diagonally
        (-2, -2) => (-1, -1),
        (-2, 2) => (-1, 1),
        (2, -2) => (1, -1),
        (2, 2) => (1, 1),
        _ => unreachable!(),
    }
}

#[derive(Debug, Clone, Copy, Default, Hash, PartialEq, Eq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Coordinate {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl From<Direction> for Coordinate {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Right => Coordinate { x: 1, y: 0 },
            Direction::Up => Coordinate { x: 0, y: 1 },
            Direction::Left => Coordinate { x: -1, y: 0 },
            Direction::Down => Coordinate { x: 0, y: -1 },
        }
    }
}

impl From<(i32, i32)> for Coordinate {
    fn from((x, y): (i32, i32)) -> Self {
        Coordinate { x, y }
    }
}

impl From<Coordinate> for (i32, i32) {
    fn from(value: Coordinate) -> Self {
        (value.x, value.y)
    }
}

#[derive(Debug, Clone, Copy)]
struct Motion {
    direction: Direction,
    steps: i32,
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
