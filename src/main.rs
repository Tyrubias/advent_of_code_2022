fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("{}", day_one_part_one(&args[1])?);
    Ok(())
}

fn day_one_part_one(file_path: impl AsRef<str>) -> Result<i32, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(file_path.as_ref())?
        .split("\n\n")
        .map(|input| {
            input
                .split("\n")
                .flat_map(|digit| digit.parse::<i32>())
                .sum()
        })
        .max()
        .ok_or_else(|| "No maximum")?)
}

fn day_one_part_two(file_path: impl AsRef<str>) -> Result<i32, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(file_path.as_ref())?;
}
