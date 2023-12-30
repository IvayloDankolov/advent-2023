use std::{collections::HashMap, path::PathBuf, fs::File, io::BufReader};

mod  day1p1; mod  day1p2;
mod  day2p1; mod  day2p2;
mod  day3p1; mod  day3p2;
mod  day4p1; mod  day4p2;
mod  day5p1; mod  day5p2;
mod  day6p1; mod  day6p2;
mod  day7p1; mod  day7p2;
mod  day8p1; mod  day8p2;
mod  day9p1; mod  day9p2;
mod day10p1; mod day10p2;
mod day11p1; mod day11p2;
mod day12p1; mod day12p2;

struct Solver {
    solve: fn(input: BufReader<File>) -> ()
}

pub fn solve_day(day: String, input: Option<PathBuf>) {
    let solvers: HashMap<String, Solver> = HashMap::from([
        (String::from( "1p1"),  day1p1::SOLVER), (String::from( "1p2"),  day1p2::SOLVER),
        (String::from( "2p1"),  day2p1::SOLVER), (String::from( "2p2"),  day2p2::SOLVER),
        (String::from( "3p1"),  day3p1::SOLVER), (String::from( "3p2"),  day3p2::SOLVER),
        (String::from( "4p1"),  day4p1::SOLVER), (String::from( "4p2"),  day4p2::SOLVER),
        (String::from( "5p1"),  day5p1::SOLVER), (String::from( "5p2"),  day5p2::SOLVER),
        (String::from( "6p1"),  day6p1::SOLVER), (String::from( "6p2"),  day6p2::SOLVER),
        (String::from( "7p1"),  day7p1::SOLVER), (String::from( "7p2"),  day7p2::SOLVER),
        (String::from( "8p1"),  day8p1::SOLVER), (String::from( "8p2"),  day8p2::SOLVER),
        (String::from( "9p1"),  day9p1::SOLVER), (String::from( "9p2"),  day9p2::SOLVER),
        (String::from("10p1"), day10p1::SOLVER), (String::from("10p2"), day10p2::SOLVER),
        (String::from("11p1"), day11p1::SOLVER), (String::from("11p2"), day11p2::SOLVER),
        (String::from("12p1"), day12p1::SOLVER), (String::from("12p2"), day12p2::SOLVER),
    ]);

    let current_solver = solvers.get(&day).unwrap_or_else(|| panic!("Not a valid day to solve: {day}"));

    let path = input.unwrap_or_else(|| {
        let work_dir = std::env::current_dir().expect("Solver should be called with a path or a working dir set");
        work_dir.join(format!("inputs/day{day}.txt"))
    });

    let file = File::open(path.clone()).unwrap_or_else(|_| panic!("Cannot open input file: {}", path.display()));
    
    let reader = BufReader::new(file);

    use std::time::Instant;
    let now = Instant::now();

    {
        (current_solver.solve)(reader);
    }

    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
    
}
