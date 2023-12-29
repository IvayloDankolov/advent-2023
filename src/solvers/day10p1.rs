use core::panic;
use std::{io::{BufRead, BufReader}, fs::File, ops::{Sub, Add}, fmt::Display};

use bitflags::bitflags;

use super::Solver;


bitflags! {
    #[derive(PartialEq, Eq, Clone, Copy)]
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

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
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
    let curr = map.nodes[location.0][location.1];

    if !curr.directions.contains(direction) {
        return false;
    }

    if let Some(next) = follow_direction(map, location, direction) {
        return map.nodes[next.0][next.1].directions.contains(opposite_direction(direction))
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
            map.nodes[map.start.0][map.start.1].directions.iter().filter(|d| is_valid_way(map, map.start, *d)).collect()
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
                remaining_directions: map.nodes[next.0][next.1]
                    .directions.iter()
                    .filter(|d| !opposite_direction(*d).eq(&next_dir) && is_valid_way(map, next, *d))
                    .collect()
            })
        }
    }

    panic!("No loop found");
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let map = parse_pipe_map(input);

        let l = find_loop(&map);

        let total_len = l.len() + 1; // Accounting coming back to the start

        let furthest = total_len/2 + total_len%2 - 1;

        println!("Furthest steps: {}", furthest);

    }
};