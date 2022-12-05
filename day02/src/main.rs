use std::{env::args, fs::read_to_string};

use phf::{phf_map, Map};

static WINNING: Map<&'static str, &'static str> = phf_map! {
    "A" => "Y", // 89 - 65 = 24
    "B" => "Z", // 90 - 66 = 24
    "C" => "X", // 88 - 67 = 21
};

static LOSING: Map<&'static str, &'static str> = phf_map! {
    "A" => "Z", // 90 - 65 = 25
    "B" => "X", // 88 - 66 = 22
    "C" => "Y", // 89 - 67 = 22
};

static DRAWING: Map<&'static str, &'static str> = phf_map! {
    "A" => "X", // 88 - 65 = 23
    "B" => "Y", // 89 - 66 = 23
    "C" => "Z", // 90 - 67 = 23
};

static POINTS: Map<&'static str, u32> = phf_map! {
    "X" => 1,
    "Y" => 2,
    "Z" => 3,
};

fn main() {
    let file_path = args().nth(1).expect("should have at least one argument");
    let contents = read_to_string(file_path).expect("should read file");

    let part1 = contents
        .lines()
        .filter(|round| !round.is_empty())
        .map(|round| {
            let moves = round.split_whitespace().take(2).collect::<Vec<&str>>();
            let opponent = moves[0];
            let mine = moves[1];

            return if LOSING
                .get(opponent)
                .map(|step| *step == mine)
                .expect("should be valid move")
            {
                *POINTS.get(mine).expect("should have valid points")
            } else if DRAWING
                .get(opponent)
                .map(|step| *step == mine)
                .expect("should be valid move")
            {
                POINTS.get(mine).expect("should have valid points") + 3
            } else if WINNING
                .get(opponent)
                .map(|step| *step == mine)
                .expect("should be valid move")
            {
                POINTS.get(mine).expect("should have valid points") + 6
            } else {
                0
            };
        })
        .sum::<u32>();

    let part2 = contents
        .lines()
        .filter(|round| !round.is_empty())
        .map(|round| {
            let moves = round.split_whitespace().take(2).collect::<Vec<&str>>();
            let opponent = moves[0];
            let result = moves[1];

            match result {
                "X" => *POINTS
                    .get(LOSING.get(opponent).expect("should be valid move"))
                    .expect("should have valid points"),
                "Y" => {
                    POINTS
                        .get(DRAWING.get(opponent).expect("should be valid move"))
                        .expect("should have valid points")
                        + 3
                }
                "Z" => {
                    POINTS
                        .get(WINNING.get(opponent).expect("should be valid move"))
                        .expect("should have valid points")
                        + 6
                }
                _ => panic!("invalid outcome"),
            }
        })
        .sum::<u32>();

    println!("Part 1\t{part1}");
    println!("Part 2\t{part2}");
}
