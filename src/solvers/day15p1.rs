use std::io::BufRead;

use itertools::Itertools;

use super::Solver;

fn ascii_hash(input: &str) -> usize {
    input.bytes().fold(0, |acc, b| {
        (acc + b as usize) * 17 % 256
    })
}

fn parse_instructions<'a>(input: &'a String) -> Vec<&'a str> {
    input.split(",").collect()
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let line = input.lines().next().expect("Expecting a line of input here").unwrap();
        
        let instructions = parse_instructions(&line);

        let hashes = instructions.iter().map(|s| ascii_hash(s)).collect_vec();

        let total: usize = hashes.iter().sum();

        println!("Total: {}", total);
    }
};