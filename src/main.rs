fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    println!("Day 1 Part 1: {}", day_one_part_one(&args[1])?);
    println!("Day 1 Part 2: {}", day_one_part_two(&args[1])?);
    println!("Day 2 Part 1: {}", day_two_part_one(&args[2])?);
    Ok(())
}

fn day_two_part_one(file_path: impl AsRef<str>) -> Result<i32, Box<dyn std::error::Error>> {
    Ok(0)
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
            let mut sum = item
                .split('\n')
                .flat_map(|num| num.parse::<i32>())
                .sum::<i32>();
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
