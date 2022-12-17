use std::ops;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// A basic coordinate with some convenient operators - really this should be called vec2 or something like that
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

impl ops::Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl ops::Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl ops::AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl Coord {
    pub fn manhattan_distance(&self, other: &Self) -> u32 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
