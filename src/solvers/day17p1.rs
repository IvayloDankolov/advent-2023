use std::{io::BufRead, collections::HashMap};

use crate::helpers::{vec2d::Vec2d, offset::Offset};

use super::Solver;
use itertools::Itertools;
use lazy_static::lazy_static;

use priority_queue::PriorityQueue;

type CityGrid = Vec2d<i64>;

type DirectionalDistances = HashMap<((usize, usize), Offset), i64>;

const MAX_LINEAR_DISTANCE: i64 = 3;

lazy_static! {
    static ref ALL_OFFSETS: Vec<Offset> = (1..MAX_LINEAR_DISTANCE+1)
        .flat_map(|d| vec![
            Offset::new(d, 0),
            Offset::new(-d, 0),
            Offset::new(0, d),
            Offset::new(0, -d)
        ]).collect();
}

fn allowed_directions<'a>(grid: &'a CityGrid, pos: (usize, usize), incoming_offset: Offset) -> impl Iterator<Item = ((usize, usize), Offset)> + 'a {
    let (row, col) = pos;
    ALL_OFFSETS.iter()
    .filter_map(move |offset| {
        if offset.is_same_direction(incoming_offset * -1) {
            return None;
        }
        let total = (incoming_offset + *offset).abs();
        if total.rows > MAX_LINEAR_DISTANCE || total.cols > MAX_LINEAR_DISTANCE {
            return None;
        }
        let pos = grid.offset_position(row, col, *offset)?;
        Some((pos, *offset))
    })
}

fn directional_move_cost(grid: &CityGrid, from: ((usize, usize), Offset), to: ((usize, usize), Offset)) -> i64 {
    // The cost doesn't actually depend on the offset of how we got there.
    // The offsets are purely an elegibility check.
    let cost = grid.iter_between(from.0, to.0).skip(1).sum();

    cost
}

fn compute_distance_map(grid: &CityGrid, pos: (usize, usize)) -> DirectionalDistances {
    let mut distances: DirectionalDistances = HashMap::new();

    // NB: Our highest priority is the lowest distance.
    let mut queue = PriorityQueue::new();
    queue.push((pos, Offset::new(0, 0)), 0);
    
    while let Some((item, prio)) = queue.pop() {
        let dist = -prio;
        let (pos, offset) = item;

        distances.insert((pos, offset), dist);

        // println!("Processing {:?} with distance {}", item, dist);

        let next_items = allowed_directions(grid, pos, offset);

        for (next_pos, next_offset) in next_items {
            
            let final_offset = if next_offset.is_same_direction(offset) {offset + next_offset} else {next_offset};

            if distances.contains_key(&(next_pos, final_offset)) {
                continue;
            }

            let next_dist = dist + directional_move_cost(grid, (pos, offset), (next_pos, final_offset));

            // println!("    Next item: {:?} {:?} with weight: {}", next_pos, final_offset, next_dist);

            if let Some(existing) = queue.get(&(next_pos, final_offset)) {
                if *existing.1 < next_dist {
                    continue;
                }
            }

            queue.push((next_pos, final_offset), -next_dist); // Mind the negative sign!
        }
    }
    
    distances

} 

fn distance_to_precomputed(distances: &DirectionalDistances, target: (usize, usize)) -> i64 {
    let distances = ALL_OFFSETS.iter()
        .filter_map(|offset| distances.get(&(target, *offset)))
        .collect_vec();
    
    **distances.iter().min()
        .unwrap()
}

fn parse_grid(input: impl BufRead) -> CityGrid {
    Vec2d::from_strings(
        input.lines().map(|line| line.unwrap()), 
        |c| c.to_digit(10).expect("Invalid block number") as i64
    )
    .expect("Invalid city grid")
}

// fn print_grid(grid: &CityGrid) {
//     for row in grid.iter_rows() {
//         for block in row {
//             print!("{}", block);
//         }
//         println!();
//     }
// }

pub const SOLVER: Solver = Solver {
    solve: |input| {

        let grid = parse_grid(input);

        // print_grid(&grid);

        // let distances = compute_distance_map(&grid, (0, 0));

        // (0..grid.height).for_each(|row| {
        //     (0..grid.width).for_each(|col| {
        //         let distance = distance_to_precomputed(&distances, (row, col));
        //         print!("{:3} ", distance);
        //     });
        //     println!();
        // });

        // println!("ALL OFFSETS: {:?}", ALL_OFFSETS.iter().collect_vec());
        // println!("Debug hashmap: {:?}", distances);

        let distance = distance_to_precomputed(&compute_distance_map(&grid, (0, 0)), (grid.height-1, grid.width-1));

        println!("Distance: {}", distance);
    }
};