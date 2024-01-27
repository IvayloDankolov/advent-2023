use std::ops::{Add, AddAssign, DivAssign, Div, MulAssign, Mul, SubAssign, Sub};

use num::Num;



#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point<T = f32>
where T: Num + Copy {
    pub x: T,
    pub y: T,
}

impl<T> Point<T>
where T: Num + Copy
{
    pub fn new(x: T, y: T) -> Self { Self { x, y } }
}

impl <T> Add<Point<T>> for Point<T> 
where T: Num + Copy {
    type Output = Point<T>;

    fn add(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl <T> AddAssign<Point<T>> for Point<T> 
where T: Num + Copy {
    fn add_assign(&mut self, other: Point<T>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl <T> Sub<Point<T>> for Point<T> 
where T: Num + Copy {
    type Output = Point<T>;

    fn sub(self, other: Point<T>) -> Point<T> {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl <T> SubAssign<Point<T>> for Point<T> 
where T: Num + Copy {
    fn sub_assign(&mut self, other: Point<T>) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }
}

impl <T> Mul<T> for Point<T> 
where T: Num + Copy {
    type Output = Point<T>;

    fn mul(self, other: T) -> Point<T> {
        Point {
            x: self.x * other,
            y: self.y * other,
        }
    }
}
impl <T> MulAssign<T> for Point<T> 
where T: Num + Copy {
    fn mul_assign(&mut self, other: T) {
        self.x = self.x * other;
        self.y = self.y * other;
    }
}

impl <T> Div<T> for Point<T> 
where T: Num + Copy {
    type Output = Point<T>;

    fn div(self, other: T) -> Point<T> {
        Point {
            x: self.x / other,
            y: self.y / other,
        }
    }
}
impl <T> DivAssign<T> for Point<T> 
where T: Num + Copy {
    fn div_assign(&mut self, other: T) {
        self.x = self.x / other;
        self.y = self.y / other;
    }
}
