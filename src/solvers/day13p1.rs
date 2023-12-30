use std::{io::BufRead, cmp::min};

use crate::helpers::vec2d::Vec2d;

use super::Solver;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TileType {
    Ash,
    Rock
}

type Grid = Vec2d<TileType>;


fn parse_grid<It: Iterator<Item=String>>(input: &mut It) -> Option<Grid> {
    Grid::from_strings(
        input.take_while(|line| !line.is_empty()),
        |c| match c {
            '#' => TileType::Rock,
            _ => TileType::Ash
        }
    )
}

fn parse_all_grids<It: Iterator<Item=String>>(input: &mut It) -> Vec<Grid> {
    let mut grids = Vec::new();
    while let Some(grid) = parse_grid(input) {
        grids.push(grid);
    }
    grids
}

fn get_vertical_reflection_index(grid: &Grid) -> Option<usize> {
    (1..grid.width).find(|refl_right| {
        let to_check = min(*refl_right, grid.width - *refl_right);
        (0..to_check).all(|offset| {
            grid.iter_col(refl_right -1 - offset).eq(grid.iter_col(refl_right + offset))
        })
    })
}

fn get_horizontal_reflection_index(grid: &Grid) -> Option<usize> {
    (1..grid.height).find(|refl_bottom| {
        let to_check = min(*refl_bottom, grid.height - *refl_bottom);
        (0..to_check).all(|offset| {
            grid.iter_row(refl_bottom -1 - offset).eq(grid.iter_row(refl_bottom + offset))
        })
    })
}

fn get_reflection_index(grid: &Grid) -> usize {
    get_vertical_reflection_index(grid).unwrap_or_else(|| get_horizontal_reflection_index(grid).unwrap_or(0) * 100)
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let mut input = input.lines().map(|line| line.unwrap());

        let grids = parse_all_grids(&mut input);

        let total = grids.iter().map(|grid| get_reflection_index(grid)).sum::<usize>();

        println!("Total: {}", total);
    }
};