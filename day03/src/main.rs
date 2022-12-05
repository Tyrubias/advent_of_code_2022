use std::{collections::HashSet, env::args, fs::read_to_string};

use itertools::Itertools;

fn main() {
    let file_path = args().nth(1).expect("should have 1 arg");
    let contents = read_to_string(file_path).expect("should read from file");

    let part1 = contents
        .lines()
        .map(|bag| {
            let (first, second) = bag.split_at(bag.len() / 2);
            let (first, second) = (
                first
                    .bytes()
                    .map(|b| if b <= 90 { b - 38 } else { b - 96 })
                    .collect::<HashSet<u8>>(),
                second
                    .bytes()
                    .map(|b| if b <= 90 { b - 38 } else { b - 96 })
                    .collect::<HashSet<u8>>(),
            );
            first
                .intersection(&second)
                .fold(0u32, |acc, item| acc + (*item as u32))
        })
        .sum::<u32>();

    let part2 = contents
        .lines()
        .chunks(3)
        .into_iter()
        .map(|group| {
            let (first, second, third) = group
                .map(|line| {
                    line.bytes()
                        .map(|b| if b <= 90 { b - 38 } else { b - 96 })
                        .collect::<HashSet<u8>>()
                })
                .collect_tuple()
                .expect("should have 3-tuple");

            first
                .into_iter()
                .filter(|item| second.contains(item) && third.contains(item))
                .map(|item| item as u32)
                .sum::<u32>()
        })
        .sum::<u32>();

    println!("Part 1:\t{part1}");
    println!("Part 2:\t{part2}");
}
