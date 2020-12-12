use super::Angle;

#[derive(Clone, Copy)]
pub struct Vector {
    pub dx: isize,
    pub dy: isize,
}

impl Vector {
    pub fn from_polar(magnitude: f64, angle: Angle) -> Vector {
        Vector {
            dx: (magnitude * angle.radians().cos()) as isize,
            dy: (magnitude * angle.radians().sin()) as isize,
        }
    }

    pub fn magnitude(&self) -> f64 {
        let dx = self.dx as f64;
        let dy = self.dy as f64;
        dx.hypot(dy)
    }

    pub fn angle(&self) -> Angle {
        let dx = self.dx as f64;
        let dy = self.dy as f64;
        Angle::from_radians((dy / dx).atan())
    }

    pub fn rotate(&self, angle: Angle) -> Vector {
        Vector::from_polar(self.magnitude(), self.angle() + angle)
    }
}
