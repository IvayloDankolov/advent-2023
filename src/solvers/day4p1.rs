use std::{collections::HashSet, io::BufRead};
use lazy_static::lazy_static;
use regex::Regex;
use super::Solver;

lazy_static! {
    static ref CARD_REGEX: Regex = Regex::new(r"Card\s+(?P<id>\d+): (?P<actual>.*) \| (?P<guessed>.*)").unwrap();
}

struct Card {
    _id: i32,
    actual: HashSet<i32>,
    guessed: HashSet<i32>
}

fn parse_numbers(str: &str) -> HashSet<i32> {
    str.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect()
}
fn parse_card(line: String) -> Card {
    let captures = CARD_REGEX.captures(&line).unwrap();
    let id = captures.name("id").unwrap().as_str().parse::<i32>().unwrap();
    let actual = parse_numbers(captures.name("actual").unwrap().as_str());
    let guessed = parse_numbers(captures.name("guessed").unwrap().as_str());
    Card {
        _id: id,
        actual,
        guessed
    }
}

fn correct_guesses(card: Card) -> u32 {
    card.actual.intersection(&card.guessed).count() as u32
}

fn score(card: Card) -> i32 {
    let correct = correct_guesses(card);
    return if correct == 0 {0} else {(2 as i32).pow(correct-1)}
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let cards = input.lines().map(|l| parse_card(l.unwrap()));

        let total_score:i32 = cards.map(score).sum();
        println!("Total score: {}", total_score);
    }
};