use color_eyre::eyre::Result;
use itertools::iproduct;
use std::cmp::max;
use std::collections::HashMap;
use std::fs::read_to_string;

fn lcs(str1: &str, str2: &str) -> Option<String> {
    let mut matrix = vec![vec![0i32; str1.len() + 1]; str2.len() + 1];
    // let mut substr = Vec::<char>::new();

    for (i, j) in iproduct!(1..str1.len() + 1, 1..str2.len() + 1) {
        if str1.chars().nth(i - 1)? == str2.chars().nth(j - 1)? {
            matrix[i][j] = matrix[i - 1][j - 1] + 1;
            // substr.push(str1.chars().nth(i)?);
        } else {
            matrix[i][j] = max(matrix[i - 1][j], matrix[i][j - 1]);
        }
    }

    let mut lensubstr = matrix[str1.len()][str2.len()];
    // lensubstr -= 1;
    let mut substr: Vec<Option<char>> = vec![None; lensubstr as usize];
    // let mut substr: Vec<Option<char>> = vec![None; lensubstr as usize];

    let (mut a, mut b) = (str1.len(), str2.len());

    while a > 0 && b > 0 {
        if str1.chars().nth(a - 1)? == str2.chars().nth(b - 1)? {
            substr.push(str1.chars().nth(a - 1));
            a -= 1;
            b -= 1;
            lensubstr -= 1;
        } else if matrix[a - 1][b] > matrix[a][b - 1] {
            a -= 1;
        } else {
            b -= 1;
        }
    }

    Some(
        substr
            .iter()
            .map(|c| c.to_owned().unwrap())
            .collect::<String>(),
    )
}

pub fn common_priority_sum() -> Result<u32> {
    let charhash: HashMap<char, usize> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect();

    let mut prio_sum = 0u32;
    let filepath = "data/rucktest.dat"; //"data/rucksack.dat";

    read_to_string(filepath)?.lines().for_each(|ruckline| {
        let half = ruckline.len() / 2;
        let substr = lcs(&ruckline[..half], &ruckline[half..]).unwrap_or(String::from(""));

        substr
            .chars()
            .inspect(|x| println!("{}", x))
            .for_each(|c| prio_sum += *(charhash.get(&c).unwrap_or(&0usize)) as u32);
        // .inspect(|x| println!("{}", x))
    });
    Ok(prio_sum)
}
