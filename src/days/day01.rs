use color_eyre::eyre::Result;
use std::fs::read_to_string;

/// There is a pack of Elves, each with a set of foods
/// each with a specific amount of calories. The calories
/// for each food item are stored in a newline in the file
/// "data/calorie_day1.dat". Each elf is separated by
/// an empty newline.

#[doc = r#"
Find the elf that carries the most 
calories and report the total number of
calories they carry.
"#]
pub fn max_calories() -> Result<u32> {
    let mut elves = Vec::<u32>::new();
    let filepath = "data/calorie_day1.dat";
    let mut invec: Vec<u32> = Vec::<u32>::new();
    for (i, numstr) in read_to_string(filepath)?.lines().enumerate() {
        if (!numstr.is_empty()) && (i != 0) {
            invec.push(numstr.parse::<u32>()?)
        } else {
            elves.push(invec.iter().sum());
            invec.clear();
        }
    }

    Ok(*elves.iter().max().unwrap_or(&0u32))
}

#[doc = r#"
Report the total number of calories the
top 3 calorie holding elves carry.
"#]
pub fn top3_calories() -> Result<u32> {
    let mut elves = Vec::<u32>::new();
    let filepath = "data/calorie_day1.dat";
    let mut invec: Vec<u32> = Vec::<u32>::new();
    for (i, numstr) in read_to_string(filepath)?.lines().enumerate() {
        if (!numstr.is_empty()) && (i != 0) {
            invec.push(numstr.parse::<u32>()?)
        } else {
            elves.push(invec.iter().sum());
            invec.clear();
        }
    }
    elves.sort();

    Ok(elves[(elves.len() - 3)..elves.len()].iter().sum())
}
