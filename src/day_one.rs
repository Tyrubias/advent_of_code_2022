pub fn day_one_part_one(file_path: impl AsRef<str>) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(file_path.as_ref())?
        .split("\n\n")
        .map(|input| {
            input
                .split('\n')
                .flat_map(|digit| digit.parse::<u32>())
                .sum()
        })
        .max()
        .ok_or("No maximum")?)
}

pub fn day_one_part_two(file_path: impl AsRef<str>) -> Result<u32, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(file_path.as_ref())?
        .split("\n\n")
        .fold([0; 3], |mut acc, item| {
            let mut sum = item
                .split('\n')
                .flat_map(|num| num.parse::<u32>())
                .sum::<u32>();
            for elem in acc.iter_mut() {
                if sum > *elem {
                    std::mem::swap(&mut sum, elem)
                }
            }
            acc
        })
        .iter()
        .sum())
}
