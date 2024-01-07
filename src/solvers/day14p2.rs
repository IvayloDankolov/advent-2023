use std::{io::BufRead, collections::HashMap};

use crate::helpers::vec2d::Vec2d;

use super::Solver;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
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

fn calculate_load(dish: &Dish) -> usize {
    dish
        .enumerate()
        .filter(|(_, _, tile)| **tile == DishTile::Rolling)
        .map(|(row, _,_)| dish.height - row)
        .sum()
}


fn spin_cycle(dish: &mut Dish) {
    tilt(dish, -1, 0);
    tilt(dish, 0, -1);
    tilt(dish, 1, 0);
    tilt(dish, 0, 1);
}

fn spin_iteration(dish: &Dish, iterations: usize) -> Dish {
    let mut memo = HashMap::new();

    let mut next = dish.clone();
    for it in 0..iterations {
        let curr = next.clone();
        let previously_seen_at = memo.get(&curr);
        if let Some(prev) = previously_seen_at {
            let cycle_length = it - prev;
            let remaining_iterations = (iterations - it - 1) % cycle_length;
            
            for _ in 0..remaining_iterations {
                spin_cycle(&mut next);
            }
            return next;
        }
        
        memo.insert(curr, it);
        
        spin_cycle(&mut next);
    }
    next
}

const CYCLES: usize = 1000000000;

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let dish = parse_dish(&mut input.lines().map(|line| line.unwrap())).unwrap();

        let spun = spin_iteration(&dish, CYCLES);
        
        let load = calculate_load(&spun);
        println!("Load: {}", load);
    }
};