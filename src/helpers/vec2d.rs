
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

    pub fn iter_col(&self, col: usize) -> impl Iterator<Item=&T> {
        (0..self.height).map(move |row| &self.data[row * self.width + col])
    }
    pub fn iter_col_mut(&mut self, col: usize) -> impl Iterator<Item=&mut T> {
        let w = self.width;
        self.iter_mut().skip(col-1).step_by(w)
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
}