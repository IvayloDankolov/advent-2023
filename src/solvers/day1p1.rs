use std::io::BufRead;

use super::Solver;


pub const SOLVER: Solver = Solver {
    solve: |input| {
        let control_nums = input.lines().map(|line| {
            let bytes = line.unwrap().into_bytes();
            let first = bytes.iter().find(|c| c.is_ascii_digit());
            let last = bytes.iter().rev().find(|c| c.is_ascii_digit());
            if first.is_none() {
                println!("Weird line with no numbers");
                return 0;
            }
            
            let control_number= 
                (*first.unwrap() as char).to_digit(10).unwrap() * 10 + (*last.unwrap() as char).to_digit(10).unwrap();
            return control_number;
        });

        let control_sum: u32 = control_nums.sum();
        println!("Control sum: {control_sum}");
    }
};