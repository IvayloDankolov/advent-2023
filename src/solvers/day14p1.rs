use std::io::BufRead;

use crate::helpers::vec2d::Vec2d;

use super::Solver;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum DishTile {
    Empty,
    Rolling,
    Static
}

type Dish = Vec2d<DishTile>;

fn parse_dish<It: Iterator<Item=String>>(input: &mut It) -> Option<Dish> {
    Dish::from_strings(
        input,
        |c| match c {
            '.' => DishTile::Empty,
            '#' => DishTile::Static,
            'O' => DishTile::Rolling,
            _ => panic!("Invalid dish tile")
        }
    )
}

fn roll(dish: &mut Dish, row: usize, col: usize, delta_rows: i64, delta_cols: i64) {
    let mut curr_row = row as i64;
    let mut curr_col = col as i64;
    loop {
        let next_row: i64 = curr_row + delta_rows;
        let next_col:i64 = curr_col + delta_cols;
        if !dish.is_in_bounds(next_row, next_col) {
            break;
        }
        if let Some(kind) = dish.try_get(next_row, next_col) {
            if !kind.eq(&DishTile::Empty) {
                break;
            }
        }
        curr_row = next_row;
        curr_col = next_col;
    }
    if curr_row != row as i64 || curr_col != col as i64 {
        dish.set(curr_row as usize, curr_col as usize, DishTile::Rolling);
        dish.set(row as usize, col as usize, DishTile::Empty);
    }
}

fn tilt(dish: &mut Dish, delta_rows: i64, delta_cols: i64) {
    let start_left = delta_cols < 0;
    let start_top = delta_rows < 0;

    let rows = {
        let mut r = (0..dish.height).collect::<Vec<_>>();
        if !start_top {
            r.reverse();
        }
        r
    };

    let cols = {
        let mut c = (0..dish.width).collect::<Vec<_>>();
        if !start_left {
            c.reverse();
        }
        c
    };

    for row in rows {
        for col in cols.clone() {
            if dish.get(row, col).eq(&DishTile::Rolling) {
                roll(dish, row, col, delta_rows, delta_cols);
            }   
        }
    }
}

fn print_dish(dish: &Dish) {
    for row in 0..dish.height {
        for col in 0..dish.width {
            match dish.get(row, col) {
                DishTile::Empty => print!("."),
                DishTile::Rolling => print!("O"),
                DishTile::Static => print!("#")
            }
        }
        println!();
    }
}

fn calculate_load(dish: &Dish) -> usize {
    dish
        .enumerate()
        .filter(|(_, _, tile)| **tile == DishTile::Rolling)
        .map(|(row, _,_)| dish.height - row)
        .sum()
}


pub const SOLVER: Solver = Solver {
    solve: |input| {
        let mut dish = parse_dish(&mut input.lines().map(|line| line.unwrap())).unwrap();

        println!("Initial dish:");
        print_dish(&dish);

        tilt(&mut dish, -1, 0);

        println!("");

        println!("Tilted dish:");
        print_dish(&dish);

        let load = calculate_load(&dish);
        println!("Load: {}", load);
    }
};