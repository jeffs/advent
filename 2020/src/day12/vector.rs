use std::fmt;
use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy)]
pub struct Vector {
    pub dx: isize,
    pub dy: isize,
}

impl Vector {
    pub fn rotate(&self, degrees: isize) -> Vector {
        let (sin, cos) = match degrees {
            0 => (0, 1),
            90 | -270 => (1, 0),
            180 | -180 => (0, -1),
            270 | -90 => (-1, 0),
            _ => panic!("{}°: bad angle", degrees),
        };
        Vector {
            dx: self.dx * cos - self.dy * sin,
            dy: self.dx * sin + self.dy * cos,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, addend: Self) -> Self {
        Vector {
            dx: self.dx + addend.dx,
            dy: self.dy + addend.dy,
        }
    }
}

impl AddAssign for Vector {
    fn add_assign(&mut self, addend: Self) {
        *self = *self + addend;
    }
}

impl Mul<isize> for Vector {
    type Output = Self;

    fn mul(self, multiplier: isize) -> Self {
        Self {
            dx: self.dx * multiplier,
            dy: self.dy * multiplier,
        }
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}]", self.dx, self.dy)
    }
}

pub const NORTH: Vector = Vector { dx: 0, dy: 1 };
pub const SOUTH: Vector = Vector { dx: 0, dy: -1 };
pub const EAST: Vector = Vector { dx: 1, dy: 0 };
pub const WEST: Vector = Vector { dx: -1, dy: 0 };
