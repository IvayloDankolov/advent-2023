use std::{io::BufRead, iter::zip};

use itertools::Itertools;

use super::Solver;

fn parse_prefixed_number_list(input: String) -> Vec<u32> {
    input
        .split_whitespace()
        .dropping(1) // Don't care about the header name in this case
        .map(|s| s.parse::<u32>().expect("List must contain valid numbers"))
        .collect()
}

fn satisfy_monotonic_predicate(predicate: impl Fn(u32) -> bool, range_start: u32, range_end: u32) -> u32 {
    let mut left = range_start;
    let mut right = range_end;
    while left < right {
        let mid = left + (right - left) / 2;
        if predicate(mid) {
            right = mid;
        } else {
            left = mid+1;
        }
    }
    return left;
}

fn get_total_distance(time: u32, wind_up: u32) -> u32 {
    wind_up * (time - wind_up)
}

fn find_record_beaters(time: u32, record: u32) -> u32 {
    let mid = time / 2;
    let first_viable = satisfy_monotonic_predicate(|guess| get_total_distance(time, guess) > record, 0, mid);
    
    // If even we only have one middle number that maximises the score, otherwise we have two
    // Besides potentially the midpoint, the other solutions are symmetric,
    2 * (mid - first_viable) + 1 + (time % 2)
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let (line1, line2) = input
            .lines()
            .map(|line| line.expect("Input should have 2 lines"))
            .collect_tuple()
            .expect("Input should have exactly 2 lines");

        let times = parse_prefixed_number_list(line1);
        let records = parse_prefixed_number_list(line2);

        let record_beaters = zip(times, records).map(|(time, record)| find_record_beaters(time, record));

        let total_leeway = record_beaters.reduce(|a, b| a * b).unwrap();

        println!("Total leeway: {}", total_leeway);
    }
};