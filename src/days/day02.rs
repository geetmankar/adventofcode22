use color_eyre::eyre::Result;
use std::fs::read_to_string;

#[derive(Clone)]
#[repr(u32)]
enum Play {
    Invalid = 0,
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

#[derive(Clone)]
#[repr(u32)]
enum PlayerState {
    Win = 6,
    Draw = 3,
    Loss = 0,
}

fn get_char_play(c: char) -> Play {
    match c {
        'A' | 'X' => Play::Rock,
        'B' | 'Y' => Play::Paper,
        'C' | 'Z' => Play::Scissors,
        _ => Play::Invalid,
    }
}

// ! PART ONE
fn tuple_fight(playtup: (Play, Play)) -> (PlayerState, PlayerState) {
    match playtup {
        (Play::Rock, Play::Paper) => (PlayerState::Loss, PlayerState::Win),
        (Play::Paper, Play::Rock) => (PlayerState::Win, PlayerState::Loss),

        (Play::Paper, Play::Scissors) => (PlayerState::Loss, PlayerState::Win),
        (Play::Scissors, Play::Paper) => (PlayerState::Win, PlayerState::Loss),

        (Play::Scissors, Play::Rock) => (PlayerState::Loss, PlayerState::Win),
        (Play::Rock, Play::Scissors) => (PlayerState::Win, PlayerState::Loss),

        _ => (PlayerState::Draw, PlayerState::Draw),
    }
}

fn line_score(line: &str) -> (u32, u32) {
    let ctup: Vec<char> = line.chars().collect();
    let playtup = (get_char_play(ctup[0]), get_char_play(ctup[2]));
    let resulttup = tuple_fight(playtup.clone());

    (
        playtup.0 as u32 + resulttup.0 as u32,
        playtup.1 as u32 + resulttup.1 as u32,
    )
}
// ! PART ONE ENDS
//-----------------
// ! PART TWO

fn get_winning_play(enemyplay: Play) -> Play {
    match enemyplay {
        Play::Rock => Play::Paper,
        Play::Paper => Play::Scissors,
        Play::Scissors => Play::Rock,
        Play::Invalid => Play::Invalid,
    }
}

fn get_losing_play(enemyplay: Play) -> Play {
    match enemyplay {
        Play::Paper => Play::Rock,
        Play::Scissors => Play::Paper,
        Play::Rock => Play::Scissors,
        Play::Invalid => Play::Invalid,
    }
}

fn get_fixed_play(enemyplay: Play, c: char) -> (Play, Play) {
    let result = match c {
        'X' => PlayerState::Loss,
        'Y' => PlayerState::Draw,
        _ => PlayerState::Win,
    };

    let yourplay = match result {
        PlayerState::Draw => enemyplay.clone(),
        PlayerState::Win => get_winning_play(enemyplay.clone()),
        PlayerState::Loss => get_losing_play(enemyplay.clone()),
    };

    (enemyplay, yourplay)
}

fn line_score_fixed(line: &str) -> (u32, u32) {
    let ctup: Vec<char> = line.chars().collect();
    let enemyplay = get_char_play(ctup[0]);
    let playtup = get_fixed_play(enemyplay.clone(), ctup[2]);

    let resulttup = tuple_fight(playtup.clone());

    (
        playtup.0 as u32 + resulttup.0 as u32,
        playtup.1 as u32 + resulttup.1 as u32,
    )
}
// ! PART TWO ENDS
// -------------------------------------------

pub fn given_strat_score_guess() -> Result<u32> {
    let mut scores = (0u32, 0u32);
    let filepath = "data/rockpaperscissors.dat";

    for playstr in read_to_string(filepath)?.lines() {
        let (sc1, sc2) = line_score(playstr);
        scores.0 += sc1; // enemy score, on the left side
        scores.1 += sc2; // our score, on the right side
    }

    Ok(scores.1)
}

pub fn given_strat_score_true() -> Result<u32> {
    let mut scores = (0u32, 0u32);
    let filepath = "data/rockpaperscissors.dat";

    for playstr in read_to_string(filepath)?.lines() {
        let (sc1, sc2) = line_score_fixed(playstr);
        scores.0 += sc1; // enemy score, on the left side
        scores.1 += sc2; // our score, on the right side
    }

    Ok(scores.1)
}
