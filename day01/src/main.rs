use std::{env::args, fs::read_to_string, mem::swap};

fn main() {
    let file_path = args().nth(1).expect("not enough arguments");
    let contents = read_to_string(file_path).expect("error reading file");

    let part1 = contents
        .split("\n\n")
        .map(|input| {
            input
                .lines()
                .flat_map(|digit| digit.parse::<u32>())
                .sum::<u32>()
        })
        .max()
        .expect("no maximum");

    let part2 = contents
        .split("\n\n")
        .fold([0; 3], |mut acc, item| {
            let mut sum = item.lines().flat_map(|num| num.parse::<u32>()).sum::<u32>();
            for elem in acc.iter_mut() {
                if sum > *elem {
                    swap(&mut sum, elem);
                }
            }
            acc
        })
        .iter()
        .sum::<u32>();

    println!("Part 1:\t{part1}");
    println!("Part 2:\t{part2}");
}
