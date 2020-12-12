use super::{Angle, Point, Vector};

#[derive(Debug)]
pub struct Waypoint {
    pos: Point,
}

impl Waypoint {
    pub fn new() -> Waypoint {
        Waypoint {
            pos: Point { x: 10, y: 1 },
        }
    }

    pub fn north(self, distance: usize) -> Waypoint {
        Waypoint {
            pos: self.pos.north(distance),
        }
    }

    pub fn east(self, distance: usize) -> Waypoint {
        Waypoint {
            pos: self.pos.east(distance),
        }
    }

    pub fn south(self, distance: usize) -> Waypoint {
        Waypoint {
            pos: self.pos.south(distance),
        }
    }

    pub fn west(self, distance: usize) -> Waypoint {
        Waypoint {
            pos: self.pos.west(distance),
        }
    }

    pub fn left(self, center: Point, degrees: usize) -> Waypoint {
        let old = center - self.pos;
        let angle = old.angle() + Angle::from_degrees(degrees);
        Waypoint {
            pos: center + Vector::from_polar(old.magnitude(), angle),
        }
    }

    pub fn right(self, center: Point, degrees: usize) -> Waypoint {
        let old = center - self.pos;
        let angle = old.angle() - Angle::from_degrees(degrees);
        Waypoint {
            pos: center + Vector::from_polar(old.magnitude(), angle),
        }
    }

    pub fn pos(&self) -> Point {
        self.pos
    }
}
