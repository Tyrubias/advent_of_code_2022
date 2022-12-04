use std::collections::HashSet;

fn main() {
    let file_path = std::env::args().nth(1).expect("not enough arguments");
    let contents = std::fs::read_to_string(file_path).expect("can't read from file");

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

    println!("Part 1:\t{part1}");
}
