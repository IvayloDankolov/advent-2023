use bitflags::bitflags;

bitflags! {
    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct Directions: u32 {
        const N = 0b0001;
        const E = 0b0010;
        const S = 0b0100;
        const W = 0b1000;
    }
}

pub fn opposite_direction(direction: Directions) -> Directions {
    match direction {
        Directions::N => Directions::S,
        Directions::E => Directions::W,
        Directions::S => Directions::N,
        Directions::W => Directions::E,
        _ => panic!("Cannot calculate opposite on a combined direction")
    }
}

pub fn direction_vector(direction: Directions) -> (i64, i64) {
    match direction {
        Directions::N => (-1, 0),
        Directions::E => (0, 1),
        Directions::S => (1, 0),
        Directions::W => (0, -1),
        _ => panic!("Cannot calculate vector on a combined direction")
    }
}

pub fn direction_vectors(directions: Directions) -> Vec<(i64, i64)> {
    let mut vectors = Vec::new();

    if directions.contains(Directions::N) {
        vectors.push((-1, 0));
    }
    if directions.contains(Directions::E) {
        vectors.push((0, 1));
    }
    if directions.contains(Directions::S) {
        vectors.push((1, 0));
    }
    if directions.contains(Directions::W) {
        vectors.push((0, -1));
    }

    vectors
}