use std::{io::{BufRead, BufReader}, fs::File, collections::HashSet, cmp::{min, max}};

use super::Solver;

#[derive(PartialEq, Eq)]
enum MapEntry {
    Empty,
    Galaxy
}

struct GalaxyMap {
    map: Vec<Vec<MapEntry>>
}
impl GalaxyMap {
    fn width(&self) -> usize {
        self.map[0].len()
    }
    fn height(&self) -> usize {
        self.map.len()
    }

    fn empty_rows(&self) -> HashSet<usize> {
        let mut empty_rows = HashSet::new();
        for (i, row) in self.map.iter().enumerate() {
            if row.iter().all(|entry| *entry == MapEntry::Empty) {
                empty_rows.insert(i);
            }
        }
        return empty_rows;
    }
    fn empty_cols(&self) -> HashSet<usize> {
        let mut empty_cols = HashSet::new();
        for col in 0..self.width() {
            if self.map.iter().all(|row| row[col] == MapEntry::Empty) {
                empty_cols.insert(col);
            }
        }
        return empty_cols;
    }

    fn galaxies(&self) -> HashSet<(usize, usize)> {
        let mut galaxies = HashSet::new();
        for (y, row) in self.map.iter().enumerate() {
            for (x, entry) in row.iter().enumerate() {
                if *entry == MapEntry::Galaxy {
                    galaxies.insert((x, y));
                }
            }
        }
        return galaxies;
    }

    fn shortest_path(&self, start: (usize, usize), end: (usize, usize), empty_rows: &HashSet<usize>, empty_cols: &HashSet<usize>) -> usize {
        let mut cost = 0;
        
        for row in min(start.0, end.0)+1..=max(start.0, end.0) {
            if empty_cols.contains(&row) {
                cost += 1000000;
            } else {
                cost += 1;
            }
        }
        for col in min(start.1, end.1)+1..=max(start.1, end.1) {
            if empty_rows.contains(&col) {
                cost += 1000000;
            } else {
                cost += 1;
            }
        }

        cost
    }

    fn total_shortest_galaxy_paths_cost(&self) -> usize {
        let empty_rows = self.empty_rows();
        let empty_cols = self.empty_cols();
        let galaxies = self.galaxies();
        let galaxy_vec = galaxies.iter().collect::<Vec<_>>();

        let mut cost = 0;
        for (i, (x, y)) in galaxy_vec.iter().enumerate() {
            for (x2, y2) in galaxy_vec[i+1..].iter() {
                cost += self.shortest_path((*x, *y), (*x2, *y2), &empty_rows, &empty_cols);
            }
        }

        cost
    }

    fn from_input(input: BufReader<File>) -> GalaxyMap {
        let map = input.lines().map(|line| {
            let bytes = line.unwrap().into_bytes();
            let map_row = bytes.iter().map(|c| {
                match c {
                    b'.' => MapEntry::Empty,
                    b'#' => MapEntry::Galaxy,
                    _ => panic!("Invalid map entry")
                }
            }).collect();
            return map_row;
        }).collect();

        return GalaxyMap {
            map
        };
    }
}

pub const SOLVER: Solver = Solver {
    solve: |input| {
        let map = GalaxyMap::from_input(input);

        let shortest = map.total_shortest_galaxy_paths_cost();

        println!("Shortest: {}", shortest);
    }
};