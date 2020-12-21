/// A 3D grid coordinate.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Point3d(pub isize, pub isize, pub isize);

impl Point3d {
    pub fn neighbors(self) -> Neighbors {
        Neighbors::around(self)
    }
}

pub struct Neighbors {
    origin: Point3d,
    offset: (isize, isize, isize),
    done: bool,
}

impl Neighbors {
    fn around(origin: Point3d) -> Neighbors {
        Neighbors {
            origin,
            offset: (-1, -1, -1),
            done: false,
        }
    }
}

impl Iterator for Neighbors {
    type Item = Point3d;

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
            (0, 0, -1) => (0, 0, 1), // skip self
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
            }
            _ => unreachable!(),
        };
        let neighbor = Point3d(
            self.origin.0 + self.offset.0,
            self.origin.1 + self.offset.1,
            self.origin.2 + self.offset.2,
        );
        self.offset = new_offset;
        Some(neighbor)
    }
}
