/// A 3D grid coordinate.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point(pub isize, pub isize, pub isize);

impl Point {
    pub fn neighborhood(self) -> Neighborhood {
        Neighborhood::around(self)
    }
}

pub struct Neighborhood {
    origin: Point,
    offset: (isize, isize, isize),
    done: bool,
}

impl Neighborhood {
    fn around(origin: Point) -> Neighborhood {
        Neighborhood {
            origin,
            offset: (-1, -1, -1),
            done: false,
        }
    }
}

impl Iterator for Neighborhood {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let new_offset = match self.offset {
            (-1, -1, -1) => (-1, -1, 0),
            (-1, -1, 0) => (-1, -1, 1),
            (-1, -1, 1) => (-1, 0, -1),
            (-1, 0, -1) => (-1, 0, 0),
            (-1, 0, 0) => (-1, 0, 1),
            (-1, 0, 1) => (-1, 1, -1),
            (-1, 1, -1) => (-1, 1, 0),
            (-1, 1, 0) => (-1, 1, 1),
            (-1, 1, 1) => (0, -1, -1),
            (0, -1, -1) => (0, -1, 0),
            (0, -1, 0) => (0, -1, 1),
            (0, -1, 1) => (0, 0, -1),
            (0, 0, -1) => (0, 0, 0),
            (0, 0, 0) => (0, 0, 1),
            (0, 0, 1) => (0, 1, -1),
            (0, 1, -1) => (0, 1, 0),
            (0, 1, 0) => (0, 1, 1),
            (0, 1, 1) => (1, -1, -1),
            (1, -1, -1) => (1, -1, 0),
            (1, -1, 0) => (1, -1, 1),
            (1, -1, 1) => (1, 0, -1),
            (1, 0, -1) => (1, 0, 0),
            (1, 0, 0) => (1, 0, 1),
            (1, 0, 1) => (1, 1, -1),
            (1, 1, -1) => (1, 1, 0),
            (1, 1, 0) => (1, 1, 1),
            (1, 1, 1) => {
                self.done = true;
                (1, 1, 1)
            },
            _ => unreachable!(),
        };
        let neighbor = Point(
            self.origin.0 + self.offset.0,
            self.origin.1 + self.offset.1,
            self.origin.2 + self.offset.2,
        );
        self.offset = new_offset;
        Some(neighbor)
    }
}
