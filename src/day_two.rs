static DAY_2_WINNING_MOVES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "A" => "Y", // 89 - 65 = 24
    "B" => "Z", // 90 - 66 = 24
    "C" => "X", // 88 - 67 = 21
};

static DAY_2_LOSING_MOVES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "A" => "Z", // 90 - 65 = 25
    "B" => "X", // 88 - 66 = 22
    "C" => "Y", // 89 - 67 = 22
};

static DAY_2_MATCHES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "A" => "X", // 88 - 65 = 23
    "B" => "Y", // 89 - 66 = 23
    "C" => "Z", // 90 - 67 = 23
};

static DAY_2_POINTS: phf::Map<&'static str, u32> = phf::phf_map! {
    "X" => 1,
    "Y" => 2,
    "Z" => 3,
};

pub fn day_two_part_one(file_path: impl AsRef<str>) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(file_path.as_ref())?
        .lines()
        .filter(|round| !round.is_empty())
        .flat_map(|round| -> Result<u32, Box<dyn std::error::Error>> {
            let moves: Vec<&str> = round.split_whitespace().take(2).collect();
            let opponent = moves[0];
            let mine = moves[1];
            let mut sum = 0;

            if DAY_2_MATCHES
                .get(opponent)
                .map(|mov| *mov == mine)
                .ok_or("Invalid move")?
            {
                sum += DAY_2_POINTS
                    .get(mine)
                    .map(|points| *points + 3)
                    .ok_or("Invalid points")?;
            } else if DAY_2_LOSING_MOVES
                .get(opponent)
                .map(|mov| *mov == mine)
                .ok_or("Invalid move")?
            {
                sum += *DAY_2_POINTS.get(mine).ok_or("Invalid points")?;
            } else if DAY_2_WINNING_MOVES
                .get(opponent)
                .map(|mov| *mov == mine)
                .ok_or("Invalid move")?
            {
                sum += DAY_2_POINTS
                    .get(mine)
                    .map(|points| *points + 6)
                    .ok_or("Invalid points")?;
            }

            Ok(sum)
        })
        .sum())
}

pub fn day_two_part_two(file_path: impl AsRef<str>) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(file_path.as_ref())?
        .lines()
        .filter(|line| !line.is_empty())
        .flat_map(|round| -> Result<u32, Box<dyn std::error::Error>> {
            let moves: Vec<&str> = round.split_whitespace().take(2).collect();
            let opponent = moves[0];
            let result = moves[1];
            let mut score = 0;

            if result == "X" {
                score += DAY_2_POINTS
                    .get(DAY_2_LOSING_MOVES.get(opponent).ok_or("invalid move")?)
                    .ok_or("invalid point")?;
            } else if result == "Y" {
                score += DAY_2_POINTS
                    .get(DAY_2_MATCHES.get(opponent).ok_or("invalid move")?)
                    .ok_or("invalid point")?
                    + 3;
            } else if result == "Z" {
                score += DAY_2_POINTS
                    .get(DAY_2_WINNING_MOVES.get(opponent).ok_or("invalid move")?)
                    .ok_or("invalid point")?
                    + 6;
            }

            Ok(score)
        })
        .sum())
}
