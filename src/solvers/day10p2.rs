use core::panic;
use std::{io::{BufRead, BufReader}, fs::File, ops::{Sub, Add}, fmt::Display, collections::HashSet};

use bitflags::bitflags;
use num::Integer;

use super::Solver;


bitflags! {
    #[derive(PartialEq, Eq, Clone, Copy, Debug)]
    struct Directions: u32 {
        const N = 0b0001;
        const E = 0b0010;
        const S = 0b0100;
        const W = 0b1000;
    }
}

fn opposite_direction(direction: Directions) -> Directions {
    match direction {
        Directions::N => Directions::S,
        Directions::E => Directions::W,
        Directions::S => Directions::N,
        Directions::W => Directions::E,
        _ => panic!("Cannot calculate opposite on a combined direction")
    }
}

#[derive(Clone, Copy)]
struct PipeNode {
    directions: Directions,
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Coord(usize, usize);

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        return Coord(self.0 + rhs.0, self.1 + rhs.1);
    }
}
impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        return Coord(self.0 - rhs.0, self.1 - rhs.1);
    }
}
impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.0, self.1))
    }
}

struct PipeMap {
    nodes: Vec<Vec<PipeNode>>,
    start: Coord
}
impl PipeMap {
    fn get(&self, coord: Coord) -> &PipeNode {
        &self.nodes[coord.0][coord.1]
    }
}

fn parse_pipe_map(input: BufReader<File>) -> PipeMap {
    let mut nodes = Vec::new();
    let mut start = None;
    for (row, line) in input.lines().enumerate() {
        let mut node_row = Vec::new();
        for (col, c) in line.unwrap_or_default().chars().enumerate() {
            let mut directions = Directions::empty();
            if c == '|' {
                directions |= Directions::N | Directions::S;
            } else if c == '-' {
                directions |= Directions::E | Directions::W;
            } else if c == 'L' {
                directions |= Directions::N | Directions::E;
            } else if c == 'J' {
                directions |= Directions::N | Directions::W;
            } else if c == '7' {
                directions |= Directions::S | Directions::W;
            } else if c == 'F' {
                directions |= Directions::S | Directions::E;
            } else if c == 'S' {
                directions |= Directions::all();
                start = Some(Coord(row, col));
            }
            node_row.push(PipeNode {
                directions
            });
        }
        nodes.push(node_row);
    }
    PipeMap {
        nodes,
        start: start.expect("Expected start node to exist")
    }
}

fn follow_direction(map: &PipeMap, location: Coord, direction: Directions) -> Option<Coord> {
    match direction {
        Directions::N => if location.0 == 0 {None} else {Some(location - Coord(1, 0))},
        Directions::S => if location.0 == map.nodes.len() - 1 {None} else {Some(location + Coord(1, 0))},
        Directions::W => if location.1 == 0 {None} else {Some(location - Coord(0, 1))},
        Directions::E => if location.1 == map.nodes[0].len() - 1 {None} else {Some(location + Coord(0, 1))},
        _ => None
    }
}

fn is_valid_way(map: &PipeMap, location: Coord, direction: Directions) -> bool {
    let curr = map.get(location);

    if !curr.directions.contains(direction) {
        return false;
    }

    if let Some(next) = follow_direction(map, location, direction) {
        return map.get(next).directions.contains(opposite_direction(direction))
    }

    return false;
}

struct DfsEntry {
    coord: Coord,
    coming_from: Directions,
    remaining_directions: Directions
}

fn find_loop(map: &PipeMap) -> Vec<Coord> {
    
    let mut path_stack = vec![DfsEntry {
        coord: map.start,
        coming_from: Directions::empty(),
        remaining_directions: 
            map.get(map.start).directions.iter().filter(|d| is_valid_way(map, map.start, *d)).collect()
    }];

    while let Some(entry) = path_stack.pop() {
        if path_stack.len() > 1 && entry.coord.eq(&map.start) {
            return path_stack.iter().map(|item| item.coord).collect()
        }

        if let Some(next_dir) = entry.remaining_directions.iter().next() {
            path_stack.push(DfsEntry { 
                coord: entry.coord, 
                coming_from: entry.coming_from,
                remaining_directions: entry.remaining_directions.difference(next_dir) 
            });
            let next = follow_direction(map, entry.coord, next_dir).unwrap();
            path_stack.push(DfsEntry {
                coord: next,
                coming_from: next_dir,
                remaining_directions: map.get(next)
                    .directions.iter()
                    .filter(|d| !opposite_direction(*d).eq(&next_dir) && is_valid_way(map, next, *d))
                    .collect()
            })
        }
    }

    panic!("No loop found");
}

fn is_space_inside_loop(map: &PipeMap, location: Coord, loop_set: &HashSet<Coord>) -> bool {
    
    // Enclosed by the loop does not mean on the edge
    if loop_set.contains(&location) {
        return false;
    }

    let items_to_test = vec![
        (Directions::N, location.0),
        (Directions::S, map.nodes.len() - location.0 - 1),
        (Directions::W, location.1),
        (Directions::E, map.nodes[0].len() - location.1 - 1)
    ];

    // This is such a pointless optimisattion btw
    let (direction, _) = items_to_test.iter().min_by_key(|item| item.1).unwrap();

    let line_dirs = *direction | opposite_direction(*direction);

    let perp_dirs = Directions::all() ^ line_dirs;

    let mut loop_intersections = 0;
    let mut curr = location;
    let mut curr_path_segment = None;
    while let Some(next) = follow_direction(map, curr, *direction) {
        if !is_valid_way(map, curr, *direction) {
            if let Some(path_segment) = curr_path_segment {
                let total_loop_dirs = map.get(path_segment).directions | map.get(curr).directions;
                if total_loop_dirs & perp_dirs == perp_dirs {
                    loop_intersections += 1;
                }
                curr_path_segment = None;
            }

            if loop_set.contains(&next) {
                curr_path_segment = Some(next);
            }
        }
        curr = next;
    }
    if let Some(path_segment) = curr_path_segment {
        let total_loop_dirs = map.get(path_segment).directions | map.get(curr).directions;
        if total_loop_dirs & perp_dirs == perp_dirs {
            loop_intersections += 1;
        }
    }
    
    return loop_intersections.is_odd();
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let map = parse_pipe_map(input);

        let l = find_loop(&map);

        let loop_set = l.iter().cloned().collect::<HashSet<Coord>>();

        let all_locations = (0..map.nodes.len()).flat_map(|row| {
            (0..map.nodes[0].len()).map(move |col| Coord(row, col))
        });

        let enclosed = all_locations.filter(|location| is_space_inside_loop(&map, *location, &loop_set));

        // println!("Enclosed: {}", enclosed.clone().join("\n"));
        let total_enclosed = enclosed.count(); 

        println!("Enclosed: {}", total_enclosed);

    }
};