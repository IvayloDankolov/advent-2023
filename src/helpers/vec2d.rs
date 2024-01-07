use super::offset::{Offset, self};


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Vec2d<T> {
    pub width: usize,
    pub height: usize,

    data: Vec<T>
}

impl<T> Vec2d<T> {
    pub fn new(width: usize, height: usize, default: T) -> Vec2d<T> where T: Clone {
        Vec2d {
            width,
            height,
            data: vec![default; width * height]
        }
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.data[row * self.width + col]
    }
    pub fn get_mut(&mut self, row: usize, col: usize) -> &mut T {
        &mut self.data[row * self.width + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        self.data[row * self.width + col] = value;
    }

    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.data.iter()
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item=&mut T> {
        self.data.iter_mut()
    }

    pub fn iter_row(&self, row: usize) -> impl Iterator<Item=&T> {
        self.data[row * self.width..(row+1) * self.width].iter()
    }
    pub fn iter_row_mut(&mut self, row: usize) -> impl Iterator<Item=&mut T> {
        self.data[row * self.width..(row+1) * self.width].iter_mut()
    }

    pub fn iter_rows(&self) -> impl Iterator<Item=impl Iterator<Item=&T>> {
        (0..self.height).map(move |row| self.iter_row(row))
    }

    pub fn iter_col(&self, col: usize) -> impl Iterator<Item=&T> {
        (0..self.height).map(move |row| &self.data[row * self.width + col])
    }
    pub fn iter_col_mut(&mut self, col: usize) -> impl Iterator<Item=&mut T> {
        let w = self.width;
        self.iter_mut().skip(col-1).step_by(w)
    }

    pub fn iter_cols(&self) -> impl Iterator<Item=impl Iterator<Item=&T>> {
        (0..self.width).map(move |col| self.iter_col(col))
    }

    pub fn iter_between(&self, start: (usize, usize), end: (usize, usize)) -> impl Iterator<Item=&T> {
        let (start_row, start_col) = start;
        let offset = offset::Offset::from_positions(start, end);
        let (norm, steps) = offset.discrete_normalalized();

        let positions = (0..steps+1).filter_map(
            move |step| self.offset_position(start_row, start_col, norm * step)
        );

        positions.map(|(row, col)| self.get(row, col))
    }

    pub fn from_strings<It: Iterator<Item = String>>(strings: It, mapper: impl Fn(char) -> T) -> Option<Vec2d<T>> {
        let mut width = 0;
        let mut height = 0;
        let mut data = Vec::new();
        for line in strings {
            width = line.len();
            height += 1;
            for c in line.chars() {
                data.push(mapper(c));
            }
        }
        
        if width == 0 || height == 0 {
            return None;
        }

        Some(Vec2d {
            width,
            height,
            data
        })
    }

    pub fn offset_position(&self, row: usize, col: usize, offset: Offset) -> Option<(usize, usize)> {
        let (row_offset, col_offset) = (offset.rows, offset.cols);
        let row = row as i64 + row_offset;
        let col = col as i64 + col_offset;
        if self.is_in_bounds(row, col) {
            Some((row as usize, col as usize))
        } else {
            None
        }
    }

    pub fn is_in_bounds(&self, row: i64, col: i64) -> bool {
        row >= 0 && col >= 0 && row < self.height as i64 && col < self.width as i64
    }
    pub fn try_get(&self, row: i64, col: i64) -> Option<&T> {
        if self.is_in_bounds(row, col) {
            Some(self.get(row as usize, col as usize))
        } else {
            None
        }
    }
    pub fn enumerate(&self) -> impl Iterator<Item=(usize, usize, &T)> {
        self.data.iter().enumerate().map(move |(i, v)| (i / self.width, i % self.width, v))
    }
}