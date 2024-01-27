use super::point::Point;

pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new(points: Vec<Point>) -> Self {
        Self { points }
    }

    fn iter_segments(&self) -> impl Iterator<Item = (&Point, &Point)> {
        self.points.iter().zip(self.points.iter().skip(1).chain(std::iter::once(&self.points[0])))
    }
}

