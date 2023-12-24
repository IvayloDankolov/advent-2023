use std::io::BufRead;
use super::Solver;

struct Pattern {
    text: &'static str,
    val: u32
}

const PATTERNS: &[Pattern; 18] = &[
    Pattern {text: "one",   val: 1},
    Pattern {text: "two",   val: 2},
    Pattern {text: "three", val: 3},
    Pattern {text: "four",  val: 4},
    Pattern {text: "five",  val: 5},
    Pattern {text: "six",   val: 6},
    Pattern {text: "seven", val: 7},
    Pattern {text: "eight", val: 8},
    Pattern {text: "nine",  val: 9},

    Pattern {text: "1", val: 1},
    Pattern {text: "2", val: 2},
    Pattern {text: "3", val: 3},
    Pattern {text: "4", val: 4},
    Pattern {text: "5", val: 5},
    Pattern {text: "6", val: 6},
    Pattern {text: "7", val: 7},
    Pattern {text: "8", val: 8},
    Pattern {text: "9", val: 9},
];

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let control_nums = input.lines().map(|line| {
            let l = line.unwrap();

            /*
            Now, I do realise there are way more efficient ways to run this.
            If this input file was a billion lines long, we could compile an Aho-Corasick search tree.
            I even included a crate for it before I realised I just don't want to waste my time.

            It would have to be two trees as well, one with all the strings reversed and iterating from the back of the line.
            Which is just way too much overkill, Rust is plenty fast already.
            */

            let first = PATTERNS.iter()
                .map(|p| (l.find(p.text), p))
                .filter(|p| p.0.is_some())
                .min_by(|a, b|a.0.cmp(&b.0));
            let last = PATTERNS.iter()
                .map(|p| (l.rfind(p.text), p))
                .max_by(|a, b| a.0.cmp(&b.0));
            
            if first.is_none() {
                println!("Weird line with no numbers");
                return 0;
            }

            let control_number= first.unwrap().1.val * 10 + last.unwrap().1.val;
            return control_number;
        });

        let control_sum: u32 = control_nums.sum();
        println!("Control sum: {control_sum}");
    }
};