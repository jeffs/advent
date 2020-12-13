use super::Vector;
use std::fmt;
use std::ops::{Add, Sub};

#[derive(Clone, Copy, Debug)]
pub struct Point {
    pub x: isize, // Cartesian longitude
    pub y: isize, // Cartesian latitude
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
