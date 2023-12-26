use std::{collections::HashMap, path::PathBuf, fs::File, io::BufReader};

mod day1p1; mod day1p2;
mod day2p1; mod day2p2;
mod day3p1; mod day3p2;
mod day4p1; mod day4p2;
mod day5p1; mod day5p2;
struct Solver {
    solve: fn(input: BufReader<File>) -> ()
}

pub fn solve_day(day: String, input: Option<PathBuf>) {
    let solvers: HashMap<String, Solver> = HashMap::from([
        (String::from("1p1"), day1p1::SOLVER), (String::from("1p2"), day1p2::SOLVER),
        (String::from("2p1"), day2p1::SOLVER), (String::from("2p2"), day2p2::SOLVER),
        (String::from("3p1"), day3p1::SOLVER), (String::from("3p2"), day3p2::SOLVER),
        (String::from("4p1"), day4p1::SOLVER), (String::from("4p2"), day4p2::SOLVER),
        (String::from("5p1"), day5p1::SOLVER), (String::from("5p2"), day5p2::SOLVER),
    ]);

    let current_solver = solvers.get(&day).unwrap_or_else(|| panic!("Not a valid day to solve: {day}"));

    let path = input.unwrap_or_else(|| {
        let work_dir = std::env::current_dir().expect("Solver should be called with a path or a working dir set");
        work_dir.join(format!("inputs/day{day}.txt"))
    });

    let file = File::open(path.clone()).unwrap_or_else(|_| panic!("Cannot open input file: {}", path.display()));
    
    let reader = BufReader::new(file);
    (current_solver.solve)(reader);
}
