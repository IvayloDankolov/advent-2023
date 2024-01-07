use std::{io::{BufRead, Error}, str::FromStr};

use linked_hash_map::LinkedHashMap;

use super::Solver;
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Remove,
    Place(usize)
}
impl FromStr for Operation {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.chars().next().unwrap() {
            '-' => Ok(Operation::Remove),
            '=' => if let Ok(lens) = s[1..].parse() {
                Ok(Operation::Place(lens))
            } else {
                Err(self::Error::other("Invalid lens number"))
            },
            _ => Err(self::Error::other("Invalid operation"))
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Instruction {
    label: String,
    operation: Operation
}

lazy_static! {
    static ref INSTRUCTION_REGEX: Regex = Regex::new(r"^(?P<label>[a-z]+)(?P<operation>[-=]\d*)$").unwrap();
}
impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = INSTRUCTION_REGEX.captures(s);
        match captures {
            None => Err(self::Error::other("Invalid instruction")),
            Some(captures) => {
                let label = captures.name("label").unwrap().as_str().to_string();
                let operation = captures.name("operation").unwrap().as_str().parse();

                match operation {
                    Err(e) => Err(e),
                    Ok(operation) =>
                        Ok(Instruction {
                            label,
                            operation
                        })
                }
            }
        }
    }
}

const BOXES:usize = 256;

fn ascii_hash(input: &str) -> usize {
    input.bytes().fold(0, |acc, b| {
        (acc + b as usize) * 17 % BOXES
    })
}

fn parse_instructions(input: &String) -> Vec<Instruction> {
    input.split(",").map(|s| s.parse().unwrap()).collect()
}

fn compute_lenses(instructions: &Vec<Instruction>) -> Vec<LinkedHashMap<&String, usize>> {
    let mut boxes = vec![LinkedHashMap::new(); BOXES];

    for instruction in instructions {
        let curr_box = &mut boxes[ascii_hash(&instruction.label)];
        match instruction.operation {
            Operation::Remove => {
                curr_box.remove(&instruction.label);
            },
            Operation::Place(lens) => {
                if let Some(entry) = curr_box.get_mut(&instruction.label) {
                    *entry = lens;
                } else {
                    curr_box.insert(&instruction.label, lens);
                }
            }
        }
    }

    boxes
}

fn focusing_power(boxes: &Vec<LinkedHashMap<&String, usize>>) -> usize {
    boxes.iter().enumerate().map(|(i, curr_box)| {
        let box_factor = i + 1;
        let lens_powers = curr_box.iter().enumerate().map(|(i, (_, lens))| {
            (i + 1) * lens
        }).sum::<usize>();
        box_factor * lens_powers
    })
    .sum()
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let line = input.lines().next().expect("Expecting a line of input here").unwrap();
        
        let instructions = parse_instructions(&line);

        let boxes = compute_lenses(&instructions);
        
        let power = focusing_power(&boxes);

        println!("Power: {}", power);
    }
};