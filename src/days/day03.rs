use color_eyre::eyre::Result;
use std::fs::read_to_string;


pub fn common_priority_sum() -> Result<u32> {
    
    let mut prio_sum = 0u32;
    let filepath = "data/rucksack.dat";
    
    for ruckline in read_to_string(filepath)?.lines() {
        if ruckline.len() % 2 != 0 {
            println!("{ruckline}");
        }
    Ok(prio_sum)
    }
}
