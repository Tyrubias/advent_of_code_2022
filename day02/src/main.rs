static WINNING: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "A" => "Y", // 89 - 65 = 24
    "B" => "Z", // 90 - 66 = 24
    "C" => "X", // 88 - 67 = 21
};

static LOSING: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "A" => "Z", // 90 - 65 = 25
    "B" => "X", // 88 - 66 = 22
    "C" => "Y", // 89 - 67 = 22
};

static DRAWING: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "A" => "X", // 88 - 65 = 23
    "B" => "Y", // 89 - 66 = 23
    "C" => "Z", // 90 - 67 = 23
};

static POINTS: phf::Map<&'static str, u32> = phf::phf_map! {
    "X" => 1,
    "Y" => 2,
    "Z" => 3,
};

fn main() {
    let file_path = std::env::args().nth(1).expect("not enough arguments");
    let contents = std::fs::read_to_string(file_path).expect("error reading file");

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
                .expect("invalid move")
            {
                *POINTS.get(mine).expect("invalid points")
            } else if DRAWING
                .get(opponent)
                .map(|step| *step == mine)
                .expect("invalid move")
            {
                POINTS.get(mine).expect("invalid points") + 3
            } else if WINNING
                .get(opponent)
                .map(|step| *step == mine)
                .expect("invalid move")
            {
                POINTS.get(mine).expect("invalid points") + 6
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
                    .get(LOSING.get(opponent).expect("invalid move"))
                    .expect("invalid point"),
                "Y" => {
                    POINTS
                        .get(DRAWING.get(opponent).expect("invalid move"))
                        .expect("invalid point")
                        + 3
                }
                "Z" => {
                    POINTS
                        .get(WINNING.get(opponent).expect("invalid move"))
                        .expect("invalid point")
                        + 6
                }
                _ => panic!("invalid outcome"),
            }
        })
        .sum::<u32>();

    println!("Part 1\t{part1}");
    println!("Part 2\t{part2}");
}
