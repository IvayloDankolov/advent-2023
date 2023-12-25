use std::{io::{BufRead, BufReader}, fs::File};

use super::Solver;


#[derive(Debug)]
struct Number {
    value: u32,
    row: i32,
    start_col: i32,
    end_col: i32
}

struct Symbol {
    _value: char,
    row: i32,
    col: i32
}

struct Schematic {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>
}

fn parse_schematic(input: BufReader<File>) -> Schematic {
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();

    let mut row = 0;
    for line in input.lines() {
        let mut col = 0;
        let mut current_number = None;
        let mut current_number_start = None;
        for c in line.unwrap_or_default().chars() {
            if c.is_ascii_digit() {
                if current_number.is_none() {
                    current_number = Some(0);
                    current_number_start = Some(col);
                }
                current_number = Some(current_number.unwrap() * 10 + c.to_digit(10).unwrap());
            } else {
                if current_number.is_some() {
                    numbers.push(Number {
                        value: current_number.unwrap(),
                        row,
                        start_col: current_number_start.unwrap(),
                        end_col: col-1
                    });
                    current_number = None;
                    current_number_start = None;
                }

                if c != '.' && !c.is_ascii_whitespace() {
                    symbols.push(Symbol {
                        _value: c,
                        row,
                        col
                    });    
                }
            }
            col += 1;
        }
        if current_number.is_some() {
            numbers.push(Number {
                value: current_number.unwrap(),
                row,
                start_col: current_number_start.unwrap(),
                end_col: col-1
            });
        }
        row += 1;
    }

    Schematic {
        numbers,
        symbols
    }
}

fn is_adjacent(num: &Number, symbol: &Symbol) -> bool {
    if (num.row - symbol.row).abs() > 1 {
        return false;
    }
    if num.start_col - symbol.col > 1 {
        return false;
    }
    if symbol.col - num.end_col > 1 {
        return false;
    }

    return true;
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let schematic = parse_schematic(input);

        let gear_ratios = 
            schematic.symbols.iter()
            .filter(|symbol| symbol._value == '*') // Only '*' symbols are possible gears
            .map(|s| schematic.numbers.iter().filter(|n| is_adjacent(n, s))) // Collect adjacent numbers
            .filter(|adj| adj.clone().count() == 2) // Gears only have exactly 2 adjacent numbers
            .map(|nums| nums.fold(1, |a, b| a * b.value));
        
        let sum_ratios: u32 = gear_ratios.sum();
        println!("{}", sum_ratios);
    }
};