static TASK_RE: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new("^(\\d+)-(\\d+)").expect("invalid regex"));

fn main() {
    let file_path = std::env::args_os().nth(1).expect("not enough arguments");
    let contents = std::fs::read_to_string(file_path).expect("can't read from file");

    let part1 = task_with_cond(&contents, ranges_contains);
    let part2 = task_with_cond(contents, range_within);

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
            let pairs = input
                .split(',')
                .take(2)
                .flat_map(|pair| TASK_RE.captures(pair))
                .collect::<Vec<_>>();
            let first_start = pairs[0].get(1).map(match_to_int).expect("invalid match");
            let first_end = pairs[0].get(2).map(match_to_int).expect("invalid match");
            let second_start = pairs[1].get(1).map(match_to_int).expect("invalid match");
            let second_end = pairs[1].get(2).map(match_to_int).expect("invalid match");

            cond((first_start, first_end), (second_start, second_end))
        })
        .filter(|value| *value)
        .count()
}

fn range_within(first: (u32, u32), second: (u32, u32)) -> bool {
    (first.0 >= second.0 && first.0 <= second.1)
        || (first.1 >= second.0 && first.1 <= second.1)
        || (second.0 >= first.0 && second.0 <= first.1)
        || (second.1 >= first.0 && second.1 <= first.1)
}

fn ranges_contains(first: (u32, u32), second: (u32, u32)) -> bool {
    (first.0 >= second.0 && first.0 <= second.1 && first.1 >= second.0 && first.1 <= second.1)
        || (second.0 >= first.0
            && second.0 <= first.1
            && second.1 >= first.0
            && second.1 <= first.1)
}

fn match_to_int(mat: regex::Match) -> u32 {
    mat.as_str().parse::<u32>().expect("parse error")
}
