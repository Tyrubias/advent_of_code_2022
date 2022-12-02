fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("Day 1 Part 1: {}", day_one_part_one(&args[1])?);
    println!("Day 1 Part 2: {}", day_one_part_two(&args[1])?);
    Ok(())
}

fn day_one_part_one(file_path: impl AsRef<str>) -> Result<i32, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(file_path.as_ref())?
        .split("\n\n")
        .map(|input| {
            input
                .split('\n')
                .flat_map(|digit| digit.parse::<i32>())
                .sum()
        })
        .max()
        .ok_or("No maximum")?)
}

fn day_one_part_two(file_path: impl AsRef<str>) -> Result<i32, Box<dyn std::error::Error>> {
    Ok(std::fs::read_to_string(file_path.as_ref())?
        .split("\n\n")
        .fold([0; 3], |mut acc, item| {
            let sum = item.split('\n').flat_map(|num| num.parse::<i32>()).sum();
            if sum > acc[0] {
                acc[2] = acc[1];
                acc[1] = acc[0];
                acc[0] = sum;
            } else if sum > acc[1] {
                acc[2] = acc[1];
                acc[1] = sum;
            } else if sum > acc[2] {
                acc[2] = sum;
            }
            acc
        })
        .iter()
        .sum())
}
