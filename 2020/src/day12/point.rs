use super::Vector;
use std::fmt;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: isize, // Cartesian longitude
    pub y: isize, // Cartesian latitude
}

impl Point {
    pub fn north(self, distance: usize) -> Point {
        Point {
            x: self.x,
            y: self.y + distance as isize,
        }
    }

    pub fn east(self, distance: usize) -> Point {
        Point {
            x: self.x + distance as isize,
            y: self.y,
        }
    }

    pub fn south(self, distance: usize) -> Point {
        Point {
            x: self.x,
            y: self.y - distance as isize,
        }
    }

    pub fn west(self, distance: usize) -> Point {
        Point {
            x: self.x - distance as isize,
            y: self.y,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, addend: Vector) -> Point {
        Point {
            x: self.x + addend.dx,
            y: self.y + addend.dy,
        }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, subtrahend: Point) -> Vector {
        Vector {
            dx: self.x - subtrahend.x,
            dy: self.y - subtrahend.y,
        }
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
