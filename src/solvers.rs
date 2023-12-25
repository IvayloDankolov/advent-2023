use std::{collections::HashMap, path::PathBuf, fs::File, io::BufReader};

mod day1p1;
mod day1p2;
mod day2p1;
mod day2p2;

struct Solver {
    solve: fn(input: BufReader<File>) -> ()
}

pub fn solve_day(day: String, input: Option<PathBuf>) {
    let solvers: HashMap<String, Solver> = HashMap::from([
        (String::from("1p1"), day1p1::SOLVER),
        (String::from("1p2"), day1p2::SOLVER),
        (String::from("2p1"), day2p1::SOLVER),
        (String::from("2p2"), day2p2::SOLVER)
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
