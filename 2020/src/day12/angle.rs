use std::f64::consts::PI;
use std::ops::{Add, Sub};

#[derive(Clone, Copy)]
pub struct Angle {
    radians: f64,
}

#[allow(dead_code)]
impl Angle {
    pub fn from_degrees(degrees: usize) -> Angle {
        Angle::from_radians(degrees as f64 * PI / 180.0)
    }

    pub fn from_radians(radians: f64) -> Angle {
        Angle { radians }
    }

    pub fn degrees(&self) -> usize {
        (self.radians * 180.0 / PI) as usize
    }

    pub fn radians(&self) -> f64 {
        self.radians
    }
}

impl Add for Angle {
    type Output = Angle;

    fn add(self, addend: Angle) -> Angle {
        Angle::from_radians(self.radians + addend.radians)
    }
}

impl Sub for Angle {
    type Output = Angle;

    fn sub(self, subtrahend: Angle) -> Angle {
        Angle::from_radians(self.radians - subtrahend.radians)
    }
}
