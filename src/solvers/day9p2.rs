use std::io::BufRead;

use itertools::Itertools;
use num::Zero;

use super::Solver;

fn parse_sequence(input: String) -> Vec<i64> {
    input.split_whitespace().map(|s| s.parse::<i64>().expect("Expected numbers only")).collect()
}

fn predict_next_number(sequence: &Vec<i64>) -> i64 {
    let mut accumulated = *sequence.first().expect("Expected at least one number");
    let mut next_sequence = sequence.clone();
    let mut sign = 1;

    while !next_sequence.iter().all(|n| n.is_zero()) {
        next_sequence = next_sequence.iter().tuple_windows().map(|(a, b)| b - a).collect();
        accumulated -= sign * next_sequence.first().expect("Expected at least one number");
        sign *= -1;
    }

    accumulated
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let sequences = input.lines().map(|line| parse_sequence(line.unwrap()));
        let next_numbers = sequences.map(|seq| predict_next_number(&seq));

        let total = next_numbers.sum::<i64>();
        println!("Total: {}", total);
    }
};