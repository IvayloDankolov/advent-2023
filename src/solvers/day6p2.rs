use std::io::BufRead;

use itertools::Itertools;

use super::Solver;

fn parse_prefixed_number_list(input: String) -> u64 {
    input
        .split_whitespace()
        .dropping(1) // Don't care about the header name in this case
        .join("")
        .parse().expect("Concatenated list must be a valid number")
}

fn satisfy_monotonic_predicate(predicate: impl Fn(u64) -> bool, range_start: u64, range_end: u64) -> u64 {
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

fn get_total_distance(time: u64, wind_up: u64) -> u64 {
    wind_up * (time - wind_up)
}

fn find_record_beaters(time: u64, record: u64) -> u64 {
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

        let time = parse_prefixed_number_list(line1);
        let record = parse_prefixed_number_list(line2);

        let record_beaters = find_record_beaters(time, record);

        println!("Total leeway: {}", record_beaters);
    }
};