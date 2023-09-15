use color_eyre::eyre::Result;
use itertools::Itertools;
use std::collections::HashMap;
use std::fs::read_to_string;

fn find_duplicates(sets: Vec<&str>) -> char {
    let chars_of_other_sets: Vec<Vec<char>> = sets
        .iter()
        .skip(1)
        .map(|set| set.chars().collect::<Vec<_>>())
        .collect();
    // we iterate through the chars of the first set and check if all other sets contain it
    sets[0]
        .chars()
        .find(|c| {
            chars_of_other_sets
                .iter()
                .all(|other_set| other_set.contains(c))
        })
        .expect(format!("No duplicates in: {:?}", sets).as_str())
}

pub fn common_priority_sum() -> Result<u32> {
    let charhash: HashMap<char, u32> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, (i + 1) as u32))
        .collect();

    let mut prio_sum = 0u32;
    let filepath = "data/rucksack.dat";
    // let filepath = "data/rucktest.dat";

    for ruckline in read_to_string(filepath)?.lines() {
        let half = ruckline.len() / 2;
        let substr = find_duplicates(vec![&ruckline[..half], &ruckline[half..]]).to_string();

        prio_sum += substr
            .chars()
            .map(|c| *(charhash.get(&c).unwrap_or(&0u32)) as u32)
            .sum::<u32>();
    }

    Ok(prio_sum)
}

pub fn team_common_priority_sum() -> Result<u32> {
    let charhash: HashMap<char, u32> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, (i + 1) as u32))
        .collect();

    let mut badge_prio_sum = 0u32;
    let filepath = "data/rucksack.dat";
    // let filepath = "data/rucktest.dat";
    let binding = read_to_string(filepath)?
        .lines()
        .map(|s| s.to_owned())
        .collect_vec();
    let chunkvec = binding.chunks(3).collect_vec();

    for ruck3vec in chunkvec {
        badge_prio_sum += {
            let c = find_duplicates(fun_name(ruck3vec));
            *(charhash.get(&c).unwrap_or(&0u32))
        };
    }
    Ok(badge_prio_sum)
}

fn fun_name(ruck3vec: &[String]) -> Vec<&str> {
    ruck3vec.into_iter().map(|s| s.as_str()).collect_vec()
}
