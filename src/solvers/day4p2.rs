use std::{collections::HashSet, io::BufRead, vec, cmp::min};
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

fn correct_guesses(card: &Card) -> u32 {
    card.actual.intersection(&card.guessed).count() as u32
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let cards: Vec<Card> = input.lines().map(|l| parse_card(l.unwrap())).collect();

        // This sort of recursive propagation is a bit of a pain to define declaratively
        // We go old school instead
        let mut card_copies = vec![1; cards.len()];
        for i in 0..card_copies.len() {
            let correct = correct_guesses(&cards[i]) as usize;
            let size = card_copies[i];
            for updates in i+1..min(i+correct+1, card_copies.len()) {
                card_copies[updates] += size;
            }
        }

        let total_cards = card_copies.iter().sum::<usize>();
        println!("Total cards: {}", total_cards);

    }
};