use std::{io::BufRead, collections::HashMap, cmp::Ordering, iter};
use itertools::Itertools;
use lazy_static::lazy_static;
use super::Solver;

lazy_static! {
    static ref CARD_VAL: HashMap<char, u32> = HashMap::from([
        ('J', 1),
        ('2', 2),
        ('3', 3),
        ('4', 4),
        ('5', 5),
        ('6', 6),
        ('7', 7),
        ('8', 8),
        ('9', 9),
        ('T', 10),
        ('Q', 12),
        ('K', 13),
        ('A', 14)
    ]);
}

fn hand_values<'a>(hand: &'a String) -> impl Iterator<Item=u32> + 'a {
    hand.chars().map(|c| *CARD_VAL.get(&c).unwrap())
}

fn hand_group_sizes<'a>(hand: &'a String) -> impl Iterator<Item=u32> + 'a {
    let mut groups = HashMap::new();
    for c in hand.chars() {
        let count = groups.entry(c).or_insert(0);
        *count += 1;
    }

    let without_jokers = groups.iter().filter(|(k, _)| **k != 'J');
    let jokers = *groups.get(&'J').unwrap_or(&0);
    // Apply jokers count to the highest group

    let without_jokers_ranked = without_jokers.map(|(_, v)| *v).sorted_by(|a, b| b.cmp(a));
    
    let largest = without_jokers_ranked.clone().next().unwrap_or(0);

    return iter::once(largest + jokers).chain(without_jokers_ranked.dropping(1))
}

fn lexicographic_order<It1: Iterator<Item = u32>, It2: Iterator<Item=u32>>(vec1: It1, vec2: It2) -> Ordering {
    for (v1, v2) in vec1.zip(vec2) {
        if v1 > v2 {
            return Ordering::Greater;
        } else if v1 < v2 {
            return Ordering::Less;
        }
    }
    return Ordering::Equal;
}

fn compare_hands(hand1: &String, hand2: &String) -> Ordering {
    let hand1_groups = hand_group_sizes(hand1);
    let hand2_groups = hand_group_sizes(hand2);
    
    let groups_order = lexicographic_order(hand1_groups, hand2_groups);

    if groups_order != Ordering::Equal {
        return groups_order;
    }

    let hand1_values = hand_values(hand1);
    let hand2_values = hand_values(hand2);

    return lexicographic_order(hand1_values, hand2_values);
}


struct Entry {
    hand: String,
    bet: u32
}

fn parse_entry(line: &String) -> Entry {
    let mut parts = line.split_whitespace();
    let hand = parts.next().unwrap().to_string();
    let bet = parts.next()
        .expect("Each line needs to have a hand component and bet component")
        .parse::<u32>()
        .expect("Bet component must be a valid number");
    Entry {
        hand,
        bet
    }
}

fn bet_payouts<It: Iterator<Item = Entry>>(entries: It) -> u32 {
    let ranked = entries.sorted_by(|a, b| compare_hands(&a.hand, &b.hand));
    
    ranked.enumerate().map(|(rank, entry)| entry.bet * (rank as u32 + 1)).sum()
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let entries = input.lines().map(|line| parse_entry(&line.unwrap()));

        let payouts = bet_payouts(entries);

        println!("Total payouts: {}", payouts);
    }
};