use std::{env::args_os, fs::read_to_string};

use itertools::Itertools;
use once_cell::sync::Lazy;
use regex::{Match, Regex};

static TASK_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new("^(\\d+)-(\\d+)").expect("should be valid regex"));

fn main() {
    let file_path = args_os().nth(1).expect("should have 1 arg");
    let contents = read_to_string(file_path).expect("should read from file");

    let part1 = task_with_cond(&contents, ranges_contains);
    let part2 = task_with_cond(contents, ranges_overlap);

    println!("Part 1:\t{part1}");
    println!("Part 2:\t{part2}");
}

fn task_with_cond(
    contents: impl AsRef<str>,
    cond: impl Fn((u32, u32), (u32, u32)) -> bool,
) -> usize {
    contents
        .as_ref()
        .lines()
        .map(|input| {
            let (first, second) = input
                .split(',')
                .take(2)
                .flat_map(|pair| {
                    TASK_RE.captures(pair).and_then(|capture| {
                        capture
                            .iter()
                            .skip(1)
                            .flat_map(|item| item.map(match_to_int))
                            .collect_tuple::<(_, _)>()
                    })
                })
                .collect_tuple::<(_, _)>()
                .expect("should be a 2-tuple");

            cond(first, second) as usize
        })
        .sum()
}

fn ranges_overlap<T: PartialOrd>(first: (T, T), second: (T, T)) -> bool {
    (first.0 >= second.0 && first.0 <= second.1)
        || (first.1 >= second.0 && first.1 <= second.1)
        || (second.0 >= first.0 && second.0 <= first.1)
        || (second.1 >= first.0 && second.1 <= first.1)
}

fn ranges_contains<T: PartialOrd>(first: (T, T), second: (T, T)) -> bool {
    (first.0 >= second.0 && first.0 <= second.1 && first.1 >= second.0 && first.1 <= second.1)
        || (second.0 >= first.0
            && second.0 <= first.1
            && second.1 >= first.0
            && second.1 <= first.1)
}

fn match_to_int(mat: Match) -> u32 {
    mat.as_str().parse::<u32>().expect("parse error")
}
