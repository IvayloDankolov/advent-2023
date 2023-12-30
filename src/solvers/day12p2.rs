use std::{io::BufRead, str::FromStr, collections::HashMap, cmp::min};

use itertools::Itertools;

use super::Solver;

fn expand<T: Copy>(coll: Vec<T>, times: usize) -> Vec<T> {
    (0..times).map(|_| coll.clone()).flatten().collect()
}
fn expand_sep<T: Copy>(coll: Vec<T>, times: usize, separator: T) -> Vec<T> {
    Itertools::intersperse((0..times).map(|_| coll.clone()), vec![separator]).flatten().collect()
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum SpringState {
    Operational,
    Damaged,
    Unknown
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct SpringRow {
    states: Vec<SpringState>,
    damaged_sequences: Vec<usize>
}

#[derive(Debug, PartialEq, Eq)]
struct ParseSpringRowErr;

const REPEATS: usize = 5;
impl FromStr for SpringRow {
    type Err = ParseSpringRowErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (states_str, sequences_str) = s.split_whitespace().collect_tuple().ok_or(ParseSpringRowErr)?;

        let states: Vec<SpringState> = states_str.chars().map(|c| match c {
            '.' => SpringState::Operational,
            '#' => SpringState::Damaged,
            '?' => SpringState::Unknown,
            _ => panic!("Invalid spring state")
        }).collect();

        let damaged_sequences = sequences_str.split(',').map(|seq| seq.parse::<usize>().unwrap()).collect();

        Ok(SpringRow {
            states: expand_sep(states, REPEATS, SpringState::Unknown),
            damaged_sequences: expand(damaged_sequences, REPEATS)
        })
    }
}

fn can_start_with(states: &Vec<SpringState>, errors: usize) -> bool {
    if errors > states.len() {
        return false;
    }

    if errors < states.len() && states[errors] == SpringState::Damaged {
        return false;
    }

    states[0..errors].iter().all(|state| *state != SpringState::Operational)
}

fn valid_damaged_arrangements(row: &SpringRow) -> usize {
    let mut memo: HashMap<SpringRow, usize> = HashMap::new();

    fn valid(memo: &mut HashMap<SpringRow, usize>, row: &SpringRow) -> usize {
        if let Some(count) = memo.get(row) {
            return *count;
        }

        if row.damaged_sequences.len() == 0 {
            return if row.states.iter().all(|s| *s != SpringState::Damaged) {1} else {0};
        }
        if row.damaged_sequences[0] > row.states.len() {
            return 0;
        }

        let mut total = 0;

        if can_start_with(&row.states, row.damaged_sequences[0]) {
            total += valid(memo, &SpringRow {
                states: row.states[min(row.states.len(), row.damaged_sequences[0]+1)..].to_vec(),
                damaged_sequences: row.damaged_sequences[1..].to_vec()
            });
        }
        if row.states[0] != SpringState::Damaged {
            total += valid(memo, &SpringRow {
                states: row.states[1..].to_vec(),
                damaged_sequences: row.damaged_sequences.to_vec()
            });
        }

        memo.insert(row.clone(), total);
        total        
    }

    valid(&mut memo, row)
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let spring_rows = input.lines().map(|line| line.unwrap().parse::<SpringRow>().unwrap()).collect::<Vec<_>>();

        let total = spring_rows.iter().map(|r| valid_damaged_arrangements(r)).sum::<usize>();

        println!("{}", total);
    }
};