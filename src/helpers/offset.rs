use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

use num::integer::gcd;

use super::direction::Directions;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Offset {
    pub rows: i64,
    pub cols: i64,
}

impl Add for Offset {
    type Output = Offset;

    fn add(self, other: Offset) -> Offset {
        Offset {
            rows: self.rows + other.rows,
            cols: self.cols + other.cols,
        }
    }
}
impl AddAssign for Offset {
    fn add_assign(&mut self, other: Offset) {
        self.rows += other.rows;
        self.cols += other.cols;
    }
}

impl Sub for Offset {
    type Output = Offset;

    fn sub(self, other: Offset) -> Offset {
        Offset {
            rows: self.rows - other.rows,
            cols: self.cols - other.cols,
        }
    }
}
impl SubAssign for Offset {
    fn sub_assign(&mut self, other: Offset) {
        self.rows -= other.rows;
        self.cols -= other.cols;
    }
}

impl Mul<i64> for Offset {
    type Output = Offset;

    fn mul(self, other: i64) -> Offset {
        Offset {
            rows: self.rows * other,
            cols: self.cols * other,
        }
    }
}
impl MulAssign<i64> for Offset {
    fn mul_assign(&mut self, other: i64) {
        self.rows *= other;
        self.cols *= other;
    }
}
impl Div<i64> for Offset {
    type Output = Offset;

    fn div(self, other: i64) -> Offset {
        Offset {
            rows: self.rows / other,
            cols: self.cols / other,
        }
    }
}
impl DivAssign<i64> for Offset {
    fn div_assign(&mut self, other: i64) {
        self.rows /= other;
        self.cols /= other;
    }
}

impl Offset {
    pub fn new(rows: i64, cols: i64) -> Offset {
        Offset { rows, cols }
    }

    pub fn from_direction(direction: Directions) -> Offset {
        match direction {
            Directions::N => Offset::new(-1, 0),
            Directions::E => Offset::new(0, 1),
            Directions::S => Offset::new(1, 0),
            Directions::W => Offset::new(0, -1),
            _ => panic!("Cannot create offset from combined direction"),
        }
    }

    pub fn from_positions(from: (usize, usize), to: (usize, usize)) -> Offset {
        let (from_row, from_col) = from;
        let (to_row, to_col) = to;

        Offset {
            rows: to_row as i64 - from_row as i64,
            cols: to_col as i64 - from_col as i64,
        }
    }

    pub fn abs(&self) -> Offset {
        Offset {
            rows: self.rows.abs(),
            cols: self.cols.abs(),
        }
    }

    pub fn discrete_normalalized(&self) -> (Offset, i64) {
        let stride = gcd(self.rows.abs(), self.cols.abs());

        if stride == 0 {
            return (*self, 0);
        }

        return (*self / stride, stride);
    }

    pub fn is_same_direction(&self, other: Offset) -> bool {
        let (self_norm, _) = self.discrete_normalalized();
        let (other_norm, _) = other.discrete_normalalized();

        self_norm == other_norm
    }
}