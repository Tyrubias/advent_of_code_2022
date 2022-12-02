static DAY_2_WINNING_MOVES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "A" => "Y",
    "B" => "Z",
    "C" => "X",
};

static DAY_2_LOSING_MOVES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "A" => "Z",
    "B" => "X",
    "C" => "Y",
};

static DAY_2_MATCHES: phf::Map<&'static str, &'static str> = phf::phf_map! {
    "A" => "X",
    "B" => "Y",
    "C" => "Z",
};

static DAY_2_POINTS: phf::Map<&'static str, u32> = phf::phf_map! {
    "X" => 1,
    "Y" => 2,
    "Z" => 3,
};

pub fn day_two_part_one(file_path: impl AsRef<str>) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(file_path.as_ref())?
        .split('\n')
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
