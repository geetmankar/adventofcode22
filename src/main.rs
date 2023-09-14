pub mod days;
use color_eyre::eyre::Result;

use days::{
    day01::{max_calories, top3_calories},
    day02::{given_strat_score_guess, given_strat_score_true},
    day03::common_priority_sum,
};

fn main() -> Result<()> {
    println!("Day 1:");
    println!("Maximum Calories is {}", max_calories()?);
    println!("Total of top 3 calories is {}\n", top3_calories()?);

    println!("Day 2:");
    println!("My Guess-strat Score is {}", given_strat_score_guess()?);
    println!("My True-strat Score is {}", given_strat_score_true()?);

    println!("Day 3:");
    println!(
        "Sum of prioirities of common items: {}",
        common_priority_sum()?
    );
    // println!("{}", 2);
    Ok(())
}
