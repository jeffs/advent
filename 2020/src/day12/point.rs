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
