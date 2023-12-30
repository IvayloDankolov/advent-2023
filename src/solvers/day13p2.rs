use std::{io::BufRead, cmp::min};

use itertools::Itertools;

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

fn get_horizontal_differences<'a>(grid: &'a Grid) -> impl Iterator<Item = usize> + 'a {
    (1..grid.height).map(move |refl_bottom| {
        let to_check = min(refl_bottom, grid.height - refl_bottom);
        (0..to_check).map(move |offset| {
            grid.iter_row(refl_bottom -1 - offset)
            .zip(grid.iter_row(refl_bottom + offset))
            .filter(|(a, b)| !a.eq(b))
            .count()
        }).sum()
    })
}

fn get_vertical_differences<'a>(grid: &'a Grid) -> impl Iterator<Item = usize> + 'a {
    (1..grid.width).map(move |refl_right| {
        let to_check = min(refl_right, grid.width - refl_right);
        (0..to_check).map(move |offset| {
            grid.iter_col(refl_right -1 - offset)
            .zip(grid.iter_col(refl_right + offset))
            .filter(|(a, b)| !a.eq(b))
            .count()
        }).sum()
    })
}

fn get_smudged_reflection_index(grid: &Grid) -> usize {
    if let Some(hor) = get_horizontal_differences(grid).find_position(|diff| *diff == 1) {
        return (hor.0 + 1) * 100;
    }
    if let Some(ver) = get_vertical_differences(grid).find_position(|diff| *diff == 1) {
        return ver.0 + 1;
    }
    return 0;
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let mut input = input.lines().map(|line| line.unwrap());

        let grids = parse_all_grids(&mut input);

        let total = grids.iter().map(|grid| get_smudged_reflection_index(grid)).sum::<usize>();

        println!("Total: {}", total);
    }
};