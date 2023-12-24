use std::path::PathBuf;

use clap::Parser;
use solvers::solve_day;

mod solvers;

#[derive(Parser)]
struct Cli {
    #[arg(short='d', long="day", default_value_t=String::from("2p1"))]
    day: String,
    input: Option<PathBuf>
}

fn main() {
    let args = Cli::parse();
    solve_day(args.day, args.input);
}
