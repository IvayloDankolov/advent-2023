use std::{io::{BufRead, BufReader}, fs::File};

use crate::helpers::{vec2d::Vec2d, direction::{Directions, direction_vector}};

use super::Solver;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Tile {
    Empty,
    Mirror45AntiClockwise,
    Mirror45Clockwise,
    SplitterHorizontal,
    SplitterVertical,
}

type MirrorGrid = Vec2d<Tile>;

type LightGrid = Vec2d<Directions>;


fn parse_mirror_grid(input: BufReader<File>) -> MirrorGrid {
    Vec2d::from_strings(input.lines().map(|line| line.unwrap()), |c| match c {
        '.' => Tile::Empty,
        '\\' => Tile::Mirror45AntiClockwise,
        '/' => Tile::Mirror45Clockwise,
        '-' => Tile::SplitterHorizontal,
        '|' => Tile::SplitterVertical,
        _ => panic!("Invalid tile")
    }).expect("Invalid mirror grid")
}

fn light_directions(grid: &MirrorGrid, beam: (usize, usize, Directions)) -> Directions {
    let entry = grid.get(beam.0, beam.1);
    let in_dir = beam.2;
    
    match entry {
        Tile::Empty => in_dir,
        Tile::Mirror45AntiClockwise => {
            if in_dir.contains(Directions::N) {
                Directions::W
            } else if in_dir.contains(Directions::E) {
                Directions::S
            } else if in_dir.contains(Directions::S) {
                Directions::E
            } else if in_dir.contains(Directions::W) {
                Directions::N
            } else {
                panic!("Invalid direction")
            }
        },
        Tile::Mirror45Clockwise => {
            if in_dir.contains(Directions::N) {
                Directions::E
            } else if in_dir.contains(Directions::E) {
                Directions::N
            } else if in_dir.contains(Directions::S) {
                Directions::W
            } else if in_dir.contains(Directions::W) {
                Directions::S
            } else {
                panic!("Invalid direction")
            }
        },
        Tile::SplitterHorizontal => {
            if in_dir.contains(Directions::N) || in_dir.contains(Directions::S) {
                Directions::E | Directions::W
            } else {
                in_dir
            }
        },
        Tile::SplitterVertical => {
            if in_dir.contains(Directions::E) || in_dir.contains(Directions::W) {
                Directions::N | Directions::S
            } else {
                in_dir
            }
        }
    }
}

fn track_beam<'a>(grid: &'a MirrorGrid, beam: (usize, usize, Directions)) -> impl Iterator<Item = (usize, usize, Directions)> + 'a {
    let (row, col, _) = beam;
    light_directions(grid, beam)
        .iter()
        .filter_map(move |d| {
            let (new_width, new_height) = grid.offset_position(row, col, direction_vector(d))?;
            Some((new_width, new_height, d))
        })
}

fn compute_light_grid(grid: &MirrorGrid, initial_beam: (usize, usize, Directions)) -> LightGrid {
    let mut stack = vec![initial_beam];
    
    let mut light_grid = LightGrid::new(grid.width, grid.height, Directions::empty());

    while let Some((x, y, dir)) = stack.pop() {
        let grid_entry = light_grid.get_mut(x, y);
        if grid_entry.contains(dir) {
            continue;
        }
        grid_entry.insert(dir);

        for beam in track_beam(grid, (x, y, dir)) {
            stack.push(beam);
        }
    }

    light_grid
}

fn lit_cells(light_grid: &LightGrid) -> usize {
    light_grid.iter().filter(|d| !d.is_empty()).count()
}

fn maximal_light_grid(grid: &MirrorGrid) -> (LightGrid, usize) {
    let initial_beams = (
        (0..grid.width).map(|col| (0, col, Directions::S))
    ).chain(
        (0..grid.height).map(|row| (row, 0, Directions::E))
    ).chain(
        (0..grid.height).map(|row| (row, grid.width-1, Directions::W))
    ).chain(
        (0..grid.width).map(|col| (grid.height-1, col, Directions::N))
    );

    initial_beams.map(|beam| {
        let light_grid = compute_light_grid(grid, beam);
        let lit = lit_cells(&light_grid);
        (light_grid, lit)
    }).max_by_key(|(_, lit)| *lit).unwrap()
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
       let grid = parse_mirror_grid(input);

       let (_, lit) = maximal_light_grid(&grid);

       println!("Lit cells: {}", lit);
    }
};