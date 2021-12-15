pub type Point = (usize, usize);

pub struct CardinalNeighbors {
    places: [Point; 4],
    index: usize,
}

#[rustfmt::skip]
impl CardinalNeighbors {
    pub fn of<T>(grid: &[Vec<T>], p: Point) -> Self {
        assert!(p.0 < grid.len() && p.1 < grid[0].len());
        let (i, j) = p;
        let (m, n) = (grid.len(), grid[0].len());
        let (y, x) = (m - 1, n - 1);
        let (mut places, mut index) = ([(0, 0); 4], 4);
        if j > 0 { index -= 1; places[index] = (i, j - 1); } // West
        if i < y { index -= 1; places[index] = (i + 1, j); } // South
        if j < x { index -= 1; places[index] = (i, j + 1); } // East
        if i > 0 { index -= 1; places[index] = (i - 1, j); } // North
        CardinalNeighbors { places, index }
    }
}

impl Iterator for CardinalNeighbors {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(&p) = self.places.get(self.index) {
            self.index += 1;
            Some(p)
        } else {
            None
        }
    }
}

