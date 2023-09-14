use color_eyre::eyre::Result;
use itertools::iproduct;
use std::collections::HashMap;
use std::fs::read_to_string;

fn lcs(str1: &str, str2: &str) -> Option<String> {
    let mut matrix = vec![vec![0i32; str1.len() + 1]; str2.len() + 1];
    let mut maxlen = 0usize;
    let mut last_idx = str1.len();

    for (i, j) in iproduct!(1..str1.len() + 1, 1..str2.len() + 1) {
        if str1.chars().nth(i - 1)? == str2.chars().nth(j - 1)? {
            matrix[i][j] = matrix[i - 1][j - 1] + 1;

            if matrix[i][j] as usize > maxlen {
                maxlen = matrix[i][j] as usize;
                last_idx = i;
            }
        }
    }

    Some((str1[(last_idx - maxlen)..last_idx]).to_string())
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
        if ruckline.len() % 2 != 0 {
            println!("{}", ruckline)
        }
        let half = ruckline.len() / 2;
        let substr = lcs(&ruckline[..half], &ruckline[half..]).unwrap_or("".to_string());

        prio_sum += substr
            .chars()
            .map(|c| *(charhash.get(&c).unwrap_or(&0u32)) as u32)
            .sum::<u32>();
    }

    Ok(prio_sum)
}
