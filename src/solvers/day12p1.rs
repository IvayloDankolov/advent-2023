use std::{io::BufRead, str::FromStr};

use itertools::Itertools;

use super::Solver;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum SpringState {
    Operational,
    Damaged,
    Unknown
}

struct SpringRow {
    states: Vec<SpringState>,
    damaged_sequences: Vec<usize>
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSpringRowErr;

impl FromStr for SpringRow {
    type Err = ParseSpringRowErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (states_str, sequences_str) = s.split_whitespace().collect_tuple().ok_or(ParseSpringRowErr)?;

        let states = states_str.chars().map(|c| match c {
            '.' => SpringState::Operational,
            '#' => SpringState::Damaged,
            '?' => SpringState::Unknown,
            _ => panic!("Invalid spring state")
        }).collect();

        let damaged_sequences = sequences_str.split(',').map(|seq| seq.parse::<usize>().unwrap()).collect();

        Ok(SpringRow {
            states,
            damaged_sequences
        })
    }
}

fn is_valid_arrangement(states: &Vec<SpringState>, damaged_sequences: &Vec<usize>) -> bool {
    let dam_states: Vec<usize> = states.split(|state| *state != SpringState::Damaged)
        .map(|seq| seq.len())
        .filter(|l| *l > 0)
        .collect();
    dam_states.eq(damaged_sequences)
}

fn possible_damaged_arrangements(row: &SpringRow) -> usize {
    let unknown_indices: Vec<usize> = row.states.iter().enumerate().filter(|(_, state)| **state == SpringState::Unknown).map(|(i, _)| i).collect();

    let known_errors = row.states.iter().filter(|state| **state == SpringState::Damaged).count();
    let total_errors: usize = row.damaged_sequences.iter().sum();

    let errors_needed = total_errors - known_errors;

    let valid_sequences = unknown_indices.iter().combinations(errors_needed).filter(|seq| {
        let new_states = row.states
            .clone()
            .iter()
            .enumerate()
            .map(|(i, state)| {
                if *state == SpringState::Unknown {
                    if seq.contains(&&i) {
                        SpringState::Damaged
                    } else {
                        SpringState::Operational
                    }
                } else {
                    *state
                }
            }).collect();
        is_valid_arrangement(&new_states, &row.damaged_sequences)
    });

    valid_sequences.count()
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let spring_rows = input.lines().map(|line| line.unwrap().parse::<SpringRow>().unwrap());


        let possible_arrangements: usize = spring_rows.map(|r| possible_damaged_arrangements(&r)).sum();

        println!("{}", possible_arrangements);
    }
};