use std::io::BufRead;
use regex::Regex;
use lazy_static::lazy_static;

use super::Solver;

lazy_static! {
    static ref LINE_REGEX: Regex = Regex::new(r"Game (\d+):(.*)").unwrap();
    static ref ROUND_PART_REGEX: Regex = Regex::new(r" (\d+) (red|green|blue)").unwrap();
}

struct Round {
    reds: i32,
    blues: i32,
    greens: i32
}
struct Game {
    _id: i32,
    rounds: Vec<Round>
}

fn parse_round(round_str: &str) -> Round {
    let mut round = Round {reds: 0, blues: 0, greens: 0};

    round_str.split(',').for_each(|part| {
        let part_info = ROUND_PART_REGEX.captures(part).unwrap_or_else(|| panic!("Round part must be valid: '{}'", part));
        let count = part_info.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let kind = part_info.get(2).unwrap().as_str();
        match kind {
            "red" => round.reds = count,
            "green" => round.greens = count,
            "blue" => round.blues = count,
            _ => panic!("Non-rgb part of round found")
        }
    });

    round
}
fn parse_game(str: &String) -> Game {
    let line_info = LINE_REGEX.captures(str).expect("Should be valid game line");
    // TODO: Like, this is all guaranteed by \d+, but all these unwrap calls are icky.
    // See if there's a parser library that's simple and low overhead
    let id = line_info.get(1).unwrap().as_str().parse::<i32>().unwrap();
    
    let round_strs = line_info.get(2).unwrap().as_str().split(';');
    return Game {
        _id: id,
        rounds: round_strs.map(parse_round).collect()
    }
}

fn min_power(game: Game) -> i32 {
    let min_red = game.rounds.iter().map(|r| r.reds).max().unwrap();
    let min_green = game.rounds.iter().map(|r| r.greens).max().unwrap();
    let min_blue = game.rounds.iter().map(|r| r.blues).max().unwrap();

    min_red * min_green * min_blue
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let games = input.lines().map(|line|
            parse_game(&line.unwrap())
        );

        let power_sum:i32 = games.map(min_power).sum();
        
        println!("{}", power_sum);
    }
};